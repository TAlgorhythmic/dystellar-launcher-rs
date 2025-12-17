use std::{error::Error, sync::{Arc, Mutex, MutexGuard}, thread};

use itertools::Itertools;

use crate::{api::{config::Config, control::{dir_provider::get_data_dir, http::{BACKEND_URL, CLIENT_ID, fetch_manifest, get}}, typedef::{manifest::{JavaManifest, MinecraftManifest}, ms_session::MicrosoftSession, task_manager::TaskManager}}, generated::{DialogSeverity, Main}, logic::safe, ui::dialogs::present_dialog_standalone};

pub fn get_manifest_async(callback: impl Fn(Result<(MinecraftManifest, Box<str>), Box<dyn Error>>) + Send + 'static) {
    thread::spawn(move || {
        let launcher_specs = get(format!("{BACKEND_URL}/launcher").as_str());
        if launcher_specs.is_err() {
            safe(move || callback(Err(launcher_specs.err().unwrap())));
            return;
        }

        let launcher_specs = launcher_specs.unwrap();
        let minecraft_version = launcher_specs["minecraft_version"].as_str();
        if minecraft_version.is_none() {
            safe(move || callback(Err("Malformed response, minecraft_version not found".into())));
            return;
        }

        let minecraft_version: Box<str> = minecraft_version.unwrap().into();
        let manifest = fetch_manifest(&minecraft_version);

        if manifest.is_err() {
            safe(move || callback(Err(format!("Failed to fetch minecraft manifest: {}", manifest.err().unwrap()).into())));
            return;
        }

        safe(move || callback(Ok((manifest.unwrap(), minecraft_version))));
    });
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

pub fn get_jre_manifest(manifest: &MinecraftManifest) -> Result<JavaManifest, Box<dyn Error + Send + Sync>> {
    #[cfg(target_os = "linux")]
    let os = "linux-glibc";
    #[cfg(target_os = "macos")]
    let os = "macos";
    #[cfg(target_os = "windows")]
    let os = "windows";

    #[cfg(target_os = "windows")]
    let archive_type = "zip";
    #[cfg(not(target_os = "windows"))]
    let archive_type = "tar.gz";

    #[cfg(target_arch = "x86_64")]
    let arch = "amd64";
    #[cfg(target_arch = "aarch64")]
    let arch = "aarch64";

    let java_version = &manifest.java_version;

    let url = format!("https://api.azul.com/metadata/v1/zulu/packages/?java_version={java_version}&os={os}&arch={arch}&archive_type={archive_type}&java_package_type=jre&javafx_bundled=false&crac_supported=false&support_term=lts&latest=true&java_package_features=headfull&availability_types=CA&certifications=tck");

    JavaManifest::try_from(get(&url)?)
}

pub fn launch(manifest: MinecraftManifest, version: &str, session: Arc<Mutex<Option<MicrosoftSession>>>, task_manager: Arc<TaskManager>, config: Arc<Config>) {
    let session = session.lock().unwrap();
    if session.is_none() {
        present_dialog_standalone("Session Error", "Seems like you are not logged in", DialogSeverity::Error);
        return;
    }

    if let Some(session) = &*session {
        let args = get_args(&manifest, &config, version, session);
        
    }
}
