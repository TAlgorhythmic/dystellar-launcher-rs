use std::{cell::RefCell, error::Error, ffi::OsString, fs, path::PathBuf, process::Command, rc::Rc, str::FromStr, sync::{Arc, Mutex, atomic::Ordering}};

use itertools::Itertools;

use crate::api::{config::Config, control::{dir_provider::{get_cache_dir, get_data_dir}, http::{CLIENT_ID, fetch_manifest, get, get_jre_manifest, get_json}}, typedef::{implementation::{HttpDownloadTask, post_unpack_natives, post_unpack_package, post_verify_sha1}, manifest::{JavaManifest, Library, MinecraftManifest}, ms_session::MicrosoftSession, task_manager::{SharedTaskState, TaskManager}}};

pub fn get_manifest() -> Result<(MinecraftManifest, Box<str>), Box<dyn Error>> {
    let launcher_specs = get("/launcher");
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

fn get_args(manifest: &MinecraftManifest, config: &Arc<Config>, version: &str, session: &MicrosoftSession) -> Result<Vec<OsString>, Box<dyn Error + Send + Sync>> {
    let mut args: Vec<OsString> = vec![
        format!("-Xmx{}M", config.ram_allocation_mb).into(),
        "-Xss1M".into(),
        format!("-Djava.library.path={}", get_data_dir().join("natives").to_str().unwrap()).into(),
        format!("-Djna.tmpdir={}", get_data_dir().join("natives").join("tmp").to_str().unwrap()).into(),
        format!("-Dorg.lwjgl.system.SharedLibraryExtractPath={}", get_data_dir().join("natives").to_str().unwrap()).into(),
        format!("-Dio.netty.native.workdir={}", get_data_dir().join("natives").join("tmp").to_str().unwrap()).into(),
        "-Dminecraft.launcher.brand=Dystellar".into(),
        concat!("-Dminecraft.launcher.version=", env!("CARGO_PKG_VERSION")).into(),
    ];

    #[cfg(target_os = "windows")]
    args.push("-XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump".into());
    #[cfg(target_os = "macos")]
    args.push("-XstartOnFirstThread");

    config.jvm_args.split(' ').filter(|e| !e.is_empty()).for_each(|e| args.push(e.into()));

    #[cfg(target_os = "windows")]
    let separator = ";";
    #[cfg(not(target_os = "windows"))]
    let separator = ":";

    let libs = get_data_dir().join("libs");

    let mut classpath = manifest.libs
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
            .map(|d| libs.join(d.path.as_ref().unwrap().as_ref()).to_string_lossy().to_string())
            .join(separator))
        .join(separator);
    classpath.push_str(separator);
    classpath.push_str(get_data_dir().join("client.jar").to_str().ok_or("Failed to convert client path to string. Are you using invalid characters in your filesystem?")?);

    args.push("-cp".into());
    args.push(classpath.into());
    args.push(manifest.main_class.as_ref().into());

    let mut game_args: Vec<OsString> = vec![
        "--username".into(), session.get_username().into(),
        "--version".into(), version.into(),
        "--gameDir".into(), config.game_dir.as_ref().into(),
        "--assetsDir".into(), format!("{}/assets", config.game_dir).into(),
        "--uuid".into(), session.uuid.as_ref().into(),
        "--accessToken".into(), session.minecraft_token.as_ref().into(),
        "--clientId".into(), CLIENT_ID.into(),
        "--xuid".into(), session.uhs.as_ref().into(),
        "--versionType".into(), "release".into(),
        "--width".into(), config.resolution.x.to_string().into(),
        "--height".into(), config.resolution.y.to_string().into(),
    ];

    args.append(&mut game_args);

    Ok(args)
}

pub fn setup_library(lib: &Library, task_manager: &Rc<RefCell<TaskManager>>) -> Result<(), Box<dyn Error + Send + Sync>> {
    #[cfg(target_os = "linux")] {
        if !lib.os.is_empty() && lib.os.iter().find(|os| &***os == "linux").is_none() {
            return Ok(());
        }
    }
    #[cfg(target_os = "windows")] {
        if !lib.os.is_empty() && lib.os.iter().find(|os| &***os == "windows").is_none() {
            return Ok(());
        }
    }
    #[cfg(target_os = "macos")] {
        if !lib.os.is_empty() && lib.os.iter().find(|os| &***os == "osx").is_none() {
            return Ok(());
        }
    }

    let folder = get_data_dir().join("libs");
    let _ = fs::create_dir_all(&folder);

    for download in &lib.downloads {
        let output = folder.join(download.path.as_ref().unwrap().as_ref());

        if &*download.id == "artifact" && !fs::exists(&output)? {
            let _ = fs::create_dir_all(output.parent().unwrap());
            let output_cl = output.clone();
            let sha1 = download.sha1.clone();

            let mut post_scripts: Vec<Box<dyn Fn(&SharedTaskState) -> Result<(), Box<dyn Error + Send + Sync>> + Send + Sync>>
                = vec![Box::new(move |s| post_verify_sha1(s, output_cl.clone(), &sha1))];
            if lib.name.contains("native") {
                let output_cl = output.clone();

                post_scripts.push(Box::new(move |s| post_unpack_natives(s, output_cl.clone(), get_data_dir().join("natives"))));
            }
            let task = HttpDownloadTask::new(&download.url, output, post_scripts)?;
            task_manager.borrow_mut().submit_task("Downloads", &lib.name, &download.url, task);
        }
    }

    Ok(())
}

pub fn setup_jre(java_manifest: JavaManifest, task_manager: &Rc<RefCell<TaskManager>>, conf: Arc<Config>) -> Result<(), Box<dyn Error + Send + Sync>> {
    if fs::exists(PathBuf::from_str(&conf.jdk_dir)?.join("bin"))? {
        return Ok(());
    }

    let cache_jre = get_cache_dir().join(&*java_manifest.name);
    let task = HttpDownloadTask::new(&java_manifest.download_url, cache_jre.clone(), vec![
        Box::new(move |s| post_unpack_package(s, cache_jre.clone(), PathBuf::from_str(conf.jdk_dir.as_ref())?, true))
    ])?;

    task_manager.borrow_mut().submit_task("JRE Download", &java_manifest.name, &java_manifest.download_url, task);
    Ok(())
}

pub fn setup_assets(manifest: &MinecraftManifest, task_manager: &Rc<RefCell<TaskManager>>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let json = get_json(&manifest.asset_index.url)?;
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
            Box::new(move |t| post_verify_sha1(t, output.clone(), &post_hash))
        ])?;
        task.shared_state.total.store(size, Ordering::Relaxed);

        task_manager.borrow_mut().submit_task("Downloads", id, &url, task);
    }

    Ok(())
}

pub fn setup_client(manifest: &MinecraftManifest, task_manager: &Rc<RefCell<TaskManager>>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let output = get_data_dir().join("client.jar");
    if output.try_exists()? { return Ok(()); }

    if let Some(client) = manifest.downloads.iter().find(|entry| entry.id.as_ref() == "client") {
        let hash: Box<str> = client.sha1.clone();
        let task = HttpDownloadTask::new(&client.url, output.clone(), vec![ Box::new(move |t| post_verify_sha1(t, output.clone(), hash.as_ref())) ])?;
        task.shared_state.total.store(client.size, Ordering::Relaxed);

        task_manager.borrow_mut().submit_task("Downloads", "Client Download", &client.url, task);
        return Ok(());
    }

    Err("Failed to install client.".into())
}

pub fn launch(manifest: MinecraftManifest, version: &str, session: Arc<Mutex<Option<MicrosoftSession>>>, task_manager: Rc<RefCell<TaskManager>>, config: Arc<Config>) -> Result<Command, Box<dyn Error + Send + Sync>> {
    let session = session.lock().unwrap();

    if let Some(session) = &*session {
        let java = get_jre_manifest(&manifest)?;

        setup_client(&manifest, &task_manager)?;
        setup_jre(java, &task_manager, config.clone())?;

        for lib in &manifest.libs {
            setup_library(lib, &task_manager)?;
        }
        setup_assets(&manifest, &task_manager)?;

        let args = get_args(&manifest, &config, version, session)?;

        for arg in &args {
            println!("{:?}", arg);
        }

        let mut process_cmd = Command::new("java");
        process_cmd.args(args);

        return Ok(process_cmd);
    }

    Err("Seems like you are not logged in".into())
}
