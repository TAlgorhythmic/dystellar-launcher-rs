use std::{error::Error, sync::LazyLock, thread, time::Duration};

use json::{object, stringify, JsonValue};
use ureq::Agent;
use webbrowser;
use uuid::Uuid;

use crate::{api::typedef::{manifest::{JavaManifest, MinecraftManifest}, ms_session::{ErrorData, MicrosoftSession}}, logic::safe};

pub static BACKEND_URL: &str = env!("BACKEND_URL");

pub static CLIENT_ID: &str = env!("CLIENT_ID");
static AGENT: LazyLock<Agent> = LazyLock::new(|| Agent::config_builder().http_status_as_error(false).timeout_global(Some(Duration::from_secs(20))).build().into());

pub fn get_json(url: &str) -> Result<JsonValue, Box<dyn Error + Send + Sync>> {
    let req = AGENT.get(url).call()?;

    Ok(json::parse(&req.into_body().read_to_string()?)?)
}

pub fn get(path: &str) -> Result<JsonValue, Box<dyn Error + Send + Sync>> {
    let url = format!("{}{}", BACKEND_URL, path);

    let req = AGENT.get(url).call()?;
    let res = json::parse(req.into_body().read_to_string()?.as_str())?;

    Ok(res)
}

pub fn post(path: &str, json: JsonValue) -> Result<JsonValue, Box<dyn Error + Send + Sync>> {
    let url = format!("{}{}", BACKEND_URL, path);

    let req = AGENT.post(url)
        .content_type("application/json")
        .send(stringify(json))?
        .into_body()
        .read_to_string()?;

    let res = json::parse(req.as_str())?;

    Ok(res)
}

pub fn login_existing<F>(access_token: Box<str>, refresh_token: Box<str>, f: F)
where
    F: FnOnce(Result<MicrosoftSession, Box<dyn Error>>) + Send + 'static
{
    thread::spawn(move || {
        let result = post(format!("/api/microsoft/login_existing").as_str(), object! {
            access_token: access_token.as_ref(), refresh_token: refresh_token.as_ref()
        });

        if let Err(err) = &result {
            let err_str = format!("Failed to login: {}", err.to_string());

            safe(move || f(Err(err_str.into())));
            return;
        }

        let res = result.unwrap();

        let uuid: Option<Box<str>> = res["uuid"].as_str().map(|s| s.into());
        let username: Option<Box<str>> = res["username"].as_str().map(|s| s.into());
        let mc_token: Option<Box<str>> = res["minecraft_token"].as_str().map(|s| s.into());
        let access_token: Option<Box<str>> = res["access_token"].as_str().map(|s| s.into());
        let refresh_token: Option<Box<str>> = res["refresh_token"].as_str().map(|s| s.into());
        let uhs: Option<Box<str>> = res["uhs"].as_str().map(|s| s.into());

        if uuid.is_none() || username.is_none() || mc_token.is_none() || access_token.is_none() || refresh_token.is_none() || uhs.is_none() {
            safe(move || f(Err("Failed to process session: Malformed or incomplete data, please contact support.".into())));
            return;
        }

        safe(move || f(Ok(MicrosoftSession {
            uuid: uuid.unwrap(),
            username: username.unwrap(),
            access_token: access_token.unwrap(),
            refresh_token: refresh_token.unwrap(),
            minecraft_token: mc_token.unwrap(),
            uhs: uhs.unwrap(),
        })));
    });
}

fn poll_uuid<F>(uuid: Uuid, callback: F)
where
    F: Fn(Result<MicrosoftSession, ErrorData>) + Send + 'static
{
    thread::spawn(move || {
        let callback_url = format!("{BACKEND_URL}/api/microsoft/callback");
        let uuid_str = uuid.to_string();
        let ms_url = format!("https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize?client_id={CLIENT_ID}&response_type=code&redirect_uri={callback_url}&scope=XboxLive.signin%20offline_access&state={uuid_str}");

        if let Err(err) = webbrowser::open(&ms_url) {
            let err_str = format!("Failed to open browser: {}", err.to_string());

            safe(move || callback(Err(ErrorData { title: "System Error", description: err_str.into() })));
            return;
        }

        loop {
            thread::sleep(Duration::from_millis(1500));

            let login_url = format!("/api/microsoft/login?uuid={uuid_str}");

            let res = get(login_url.as_str());

            if let Err(err) = &res {
                let str_err = err.to_string();

                safe(move || callback(Err(ErrorData { title: "Fatal Error", description: format!("Failed to check login status: {str_err} \nIs your internet down?").into() })));
                break;
            }

            let res = res.unwrap();
            let ok = res["ok"].as_bool();

            if !ok.unwrap_or(false) {
                let body_err_msg: Box<str> = res["error"].as_str().unwrap_or("Cannot provide error message").into();
                let err_str = format!("Login failed: {}. Please contact support.", body_err_msg);

                safe(move || callback(Err(ErrorData { title: "Server Error", description: err_str.into() })));
                break;
            }

            if !res["authenticated"].as_bool().unwrap_or(false) {
                continue;
            }


            let uuid: Option<Box<str>> = res["uuid"].as_str().map(|s| s.into());
            let username: Option<Box<str>> = res["username"].as_str().map(|s| s.into());
            let mc_token: Option<Box<str>> = res["minecraft_token"].as_str().map(|s| s.into());
            let access_token: Option<Box<str>> = res["access_token"].as_str().map(|s| s.into());
            let refresh_token: Option<Box<str>> = res["refresh_token"].as_str().map(|s| s.into());
            let uhs: Option<Box<str>> = res["uhs"].as_str().map(|s| s.into());

            if uuid.is_none() || username.is_none() || mc_token.is_none() || access_token.is_none() || refresh_token.is_none() || uhs.is_none() {
                safe(move || callback(Err(ErrorData { title: "Session Error", description: "Data received from server is incomplete. Please contact support.".into() })));
                return;
            }
            let session = MicrosoftSession {
                uuid: uuid.unwrap(),
                username: username.unwrap(),
                access_token: access_token.unwrap(),
                refresh_token: refresh_token.unwrap(),
                minecraft_token: mc_token.unwrap(),
                uhs: uhs.unwrap()
            };

            safe(move || callback(Ok(session)));

            println!("Logged in");
            break;
        }
    });
}

pub fn login<F>(callback: F)
where
    F: Fn(Result<MicrosoftSession, ErrorData>) + Send + 'static
{
    let uuid = Uuid::new_v4();
    let uuid_str = uuid.to_string();
    let lsopt = post("/api/microsoft/loginsession", object! { uuid: uuid_str });

    if lsopt.is_err() {
        callback(Err(ErrorData { title: "Connection Error", description: "Error connecting to server, please check your internet connection.".into() }));
        return;
    }

    let lsession_res = lsopt.unwrap();
    if !lsession_res["ok"].as_bool().unwrap_or(false) {
        callback(Err(ErrorData { title: "Server Error", description: format!("An unexpected error occured: {}\nplease try again later.\nSorry for the inconvenience.", lsession_res["error"].as_str().unwrap_or("Unknown Error")).into() }));
        return;
    }

    poll_uuid(uuid, callback);
}

pub fn fetch_manifest(version: &str) -> Result<MinecraftManifest, Box<dyn Error + Send + Sync>> {
    let json = get_json("https://piston-meta.mojang.com/mc/game/version_manifest.json")?;

    let version = json["versions"].members().find(|v| v["id"].as_str().map(|v| v == version).unwrap_or(false)).ok_or("Version not found")?;
    let url = version["url"].as_str().ok_or("Url not found in version")?;
    let manifest_json = get_json(url)?;

    Ok(MinecraftManifest::try_from(manifest_json)?)
}

pub fn get_jre_manifest(manifest: &MinecraftManifest) -> Result<JavaManifest, Box<dyn Error + Send + Sync>> {
    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    let os = "linux-glibc";
    #[cfg(all(target_os = "linux", target_env = "musl"))]
    let os = "linux-musl";
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

    JavaManifest::try_from(get_json(&url)?)
}
