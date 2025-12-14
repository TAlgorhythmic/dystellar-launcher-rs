use std::{error::Error, sync::{Arc, Mutex}, thread};

use crate::{api::{config::Config, control::http::{BACKEND_URL, fetch_manifest, get}, typedef::{manifest::MinecraftManifest, ms_session::MicrosoftSession, task_manager::TaskManager}}, generated::Main, logic::safe};

pub fn get_manifest_async(callback: impl Fn(Result<MinecraftManifest, Box<dyn Error>>) + Send + 'static) {
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

        let minecraft_version = minecraft_version.unwrap();
        let manifest = fetch_manifest(minecraft_version);

        if manifest.is_err() {
            safe(move || callback(Err(format!("Failed to fetch minecraft manifest: {}", manifest.err().unwrap()).into())));
            return;
        }

        safe(move || callback(Ok(manifest.unwrap())));
    });
}

pub fn launch(ui: Main, manifest: MinecraftManifest, session: Arc<Mutex<Option<MicrosoftSession>>>, task_manager: Arc<TaskManager>, config: Arc<Config>) {
    todo!()
}
