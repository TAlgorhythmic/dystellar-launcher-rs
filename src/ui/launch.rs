use std::{cell::RefCell, error::Error, fs, path::PathBuf, rc::Rc, str::FromStr, sync::{Arc, Mutex, atomic::Ordering}};

use itertools::Itertools;

use crate::{api::{config::Config, control::{dir_provider::{get_cache_dir, get_data_dir}, http::{BACKEND_URL, CLIENT_ID, fetch_manifest, get, get_jre_manifest}}, typedef::{implementation::HttpDownloadTask, manifest::{JavaManifest, Library, MinecraftManifest}, ms_session::MicrosoftSession, task_manager::TaskManager}}, generated::DialogSeverity, ui::dialogs::present_dialog_standalone};

pub fn get_manifest() -> Result<(MinecraftManifest, Box<str>), Box<dyn Error>> {
    let launcher_specs = get(format!("{BACKEND_URL}/launcher").as_str());
    if launcher_specs.is_err() {
        return Err(launcher_specs.err().unwrap());
    }

    let launcher_specs = launcher_specs.unwrap();
    let minecraft_version = launcher_specs["minecraft_version"].as_str();
    if minecraft_version.is_none() {
        return Err("Malformed response, minecraft_version not found".into());
    }

    let minecraft_version: Box<str> = minecraft_version.unwrap().into();
    let manifest = fetch_manifest(&minecraft_version);

    if manifest.is_err() {
        return Err(format!("Failed to fetch minecraft manifest: {}", manifest.err().unwrap()).into());
    }

    Ok((manifest.unwrap(), minecraft_version))
}

fn get_args(manifest: &MinecraftManifest, config: &Arc<Config>, version: &str, session: &MicrosoftSession) -> Vec<Box<str>> {
    let mut args: Vec<Box<str>> = vec![
        format!("-Xmx{}M", config.ram_allocation_mb).into_boxed_str(),
        "-Xss1M".into(),
        format!("-Djava.library.path={}", get_data_dir().join("natives").to_str().unwrap()).into_boxed_str(),
        format!("-Djna.tmpdir={}", get_data_dir().join("natives").join("tmp").to_str().unwrap()).into_boxed_str(),
        format!("-Dorg.lwjgl.system.SharedLibraryExtractPath={}", get_data_dir().join("natives").to_str().unwrap()).into_boxed_str(),
        format!("-Dio.netty.native.workdir={}", get_data_dir().join("natives").join("tmp").to_str().unwrap()).into_boxed_str(),
        "--Dminecraft.launcher.brand=Dystellar".into(),
        concat!("-Dminecraft.launcher.version=", env!("CARGO_PKG_VERSION")).into(),
    ];

    #[cfg(target_os = "windows")]
    jvm_args.push("-XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump");
    #[cfg(target_os = "macos")]
    jvm_args.push("-XstartOnFirstThread");

    config.jvm_args.split(' ').filter(|e| !e.is_empty()).for_each(|e| args.push(e.into()));

    #[cfg(target_os = "windows")]
    let separator = ";";
    #[cfg(not(target_os = "windows"))]
    let separator = ":";

    let classpath = manifest.libs
        .iter()
        .filter(|lib| {
            if lib.os.is_empty() { return true; }

            lib.os.iter().find(|os| {
                #[cfg(target_os = "linux")]
                return ***os == *"linux";
                #[cfg(target_os = "windows")]
                return ***os == *"windows";
                #[cfg(target_os = "macos")]
                return ***os == *"osx";
            }).is_some()
        })
        .map(|lib| lib.downloads
            .iter()
            .map(|d| d.path.as_ref().unwrap())
            .join(separator))
        .join(separator);

    args.push("-cp".into());
    args.push(classpath.into_boxed_str());
    args.push(manifest.main_class.clone());

    let mut game_args: Vec<Box<str>> = vec![
        "--username".into(), session.get_username().into(),
        "--version".into(), version.into(),
        "--gameDir".into(), config.game_dir.clone(),
        "--assetsDir".into(), format!("{}/assets", config.game_dir).into_boxed_str(),
        "--uuid".into(), session.uuid.clone(),
        "--accessToken".into(), session.minecraft_token.clone(),
        "--clientId".into(), CLIENT_ID.into(),
        "--xuid".into(), session.xuid.clone(),
        "--versionType".into(), "release".into(),
        "--width".into(), config.resolution.x.to_string().into_boxed_str(),
        "--height".into(), config.resolution.y.to_string().into_boxed_str(),
    ];

    args.append(&mut game_args);

    args
}

pub fn setup_library(lib: &Library, task_manager: &Rc<RefCell<TaskManager>>) -> Result<(), Box<dyn Error + Send + Sync>> {
    #[cfg(target_os = "linux")] {
        if !lib.os.is_empty() && lib.os.iter().find(|os| ***os == *"linux").is_none() {
            return Ok(());
        }
    }
    #[cfg(target_os = "windows")] {
        if !lib.os.is_empty() && lib.os.iter().find(|os| ***os == *"windows").is_none() {
            return Ok(());
        }
    }
    #[cfg(target_os = "macos")] {
        if !lib.os.is_empty() && lib.os.iter().find(|os| ***os == *"osx").is_none() {
            return Ok(());
        }
    }

    let folder = get_data_dir().join("libs");
    let _ = fs::create_dir(&folder);

    for download in &lib.downloads {
        let output = folder.join(download.path.as_ref().unwrap().as_ref());

        if *download.id == *"artifact" && !fs::exists(&output)? {
            let _ = fs::create_dir_all(&output);
            let output_cl = output.clone();
            let sha1 = download.sha1.clone();

            let mut post_scripts: Vec<Box<dyn Fn(&mut HttpDownloadTask) -> Result<(), Box<dyn Error + Send + Sync>> + Send + Sync>>
                = vec![Box::new(move |download_task| download_task.post_verify_sha1(output_cl.clone(), &sha1))];
            if lib.name.contains("native") {
                let output_cl = output.clone();

                post_scripts.push(Box::new(move |download_task| download_task.post_unpack_natives(output_cl.clone(), get_data_dir().join("natives"))));
            }
            let task = HttpDownloadTask::new(&download.url, output, post_scripts)?;
            task_manager.borrow_mut().submit_task("Downloads", &lib.name, &download.url, task);
        }
    }

    Ok(())
}

pub fn setup_jre(java_manifest: JavaManifest, task_manager: &Rc<RefCell<TaskManager>>, conf: Arc<Config>) -> Result<(), Box<dyn Error + Send + Sync>> {
    if fs::exists(PathBuf::from_str(&conf.jdk_dir)?.join("jre").join("bin"))? {
        return Ok(());
    }

    let cache_jre = get_cache_dir().join(&*java_manifest.name);
    let task = HttpDownloadTask::new(&java_manifest.download_url, cache_jre.clone(), vec![
        Box::new(move |task| task.post_unpack_package(cache_jre.clone(), PathBuf::from_str(conf.jdk_dir.as_ref())?, true))
    ])?;

    task_manager.borrow_mut().submit_task("JRE Download", &java_manifest.name, &java_manifest.download_url, task);
    Ok(())
}

pub fn setup_assets(manifest: &MinecraftManifest, task_manager: &Rc<RefCell<TaskManager>>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let json = get(&manifest.asset_index.url)?;
    let objs = &json["objects"];

    for entry in objs.entries() {
        let (id, json) = entry;
        let hash = json["hash"].as_str().ok_or("asset.hash missing")?;
        let size = json["size"].as_usize().ok_or("asset.size missing")?;

        let slice = hash.get(0..2).unwrap();
        let output_folder = get_data_dir().join("assets/objects").join(slice);
        let output = output_folder.join(hash);

        if output.try_exists()? { continue; }

        let url = format!("https://resources.download.minecraft.net/{}/{hash}", slice);

        fs::create_dir_all(&output_folder)?;

        let post_hash: Box<str> = hash.into();
        let task = HttpDownloadTask::new(&url, output.clone(), vec![
            Box::new(move |t| t.post_verify_sha1(output.clone(), &post_hash))
        ])?;
        task.total.store(size, Ordering::Relaxed);

        task_manager.borrow_mut().submit_task("Downloads", id, &url, task);
    }

    Ok(())
}

pub fn launch(manifest: MinecraftManifest, version: &str, session: Arc<Mutex<Option<MicrosoftSession>>>, task_manager: Rc<RefCell<TaskManager>>, config: Arc<Config>) {
    let session = session.lock().unwrap();
    if session.is_none() {
        present_dialog_standalone("Session Error", "Seems like you are not logged in", DialogSeverity::Error);
        return;
    }

    if let Some(session) = &*session {
        let java = get_jre_manifest(&manifest);
        if let Err(err) = &java {
            present_dialog_standalone("JRE Error", format!("Failed to fetch java manifest from azul zulu: {}", err.to_string()).as_str(), DialogSeverity::Error);
            return;
        }
        let java = java.unwrap();
        let args = get_args(&manifest, &config, version, session);
        
        if let Err(err) = setup_jre(java, &task_manager, config.clone()) {
            present_dialog_standalone("JRE Error", format!("Failed to setup jre: {}", err.to_string()).as_str(), DialogSeverity::Error);
            return;
        }

        for lib in &manifest.libs {
            if let Err(err) = setup_library(lib, &task_manager) {
                present_dialog_standalone("Lib Error", format!("Failed to setup library {}: {}", lib.name, err.to_string()).as_str(), DialogSeverity::Error);
                return;
            }
        }
        if let Err(err) = setup_assets(&manifest, &task_manager) {
            present_dialog_standalone("Asset Error", format!("Failed to setup assets: {}", err.to_string()).as_str(), DialogSeverity::Error);
            return;
        }
    }
}
