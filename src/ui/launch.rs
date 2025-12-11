use std::{error::Error, thread};

use crate::{api::{control::http::{BACKEND_URL, fetch_manifest, get}, typedef::manifest::MinecraftManifest}, generated::Main, logic::safe};

pub fn get_manifest_async<F>(callback: F) where F: Fn(Result<MinecraftManifest, Box<dyn Error + Send + Sync>>) + Send + 'static {
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

pub fn launch(ui: Main, manifest: MinecraftManifest)
