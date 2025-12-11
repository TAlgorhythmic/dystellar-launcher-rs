use std::{error::Error, path::PathBuf, sync::LazyLock, thread, time::Duration};

use json::{object, stringify, JsonValue};
use ureq::Agent;
use webbrowser;
use uuid::Uuid;

use crate::{api::typedef::{manifest::MinecraftManifest, ms_session::{ErrorData, MicrosoftSession}}, logic::safe};

pub static BACKEND_URL: &str = env!("BACKEND_URL");

static CLIENT_ID: &str = env!("CLIENT_ID");
static AGENT: LazyLock<Agent> = LazyLock::new(|| Agent::config_builder().http_status_as_error(false).timeout_global(Some(Duration::from_secs(20))).build().into());

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

pub fn download(url: &str, output: PathBuf) -> Result<(), Box<dyn Error>> {
    Ok(())
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

        let uuid_opt: Option<Box<str>> = res["uuid"].as_str().map(|s| s.into());
        let mc_token_opt: Option<Box<str>> = res["minecraft_token"].as_str().map(|s| s.into());
        let access_token_opt: Option<Box<str>> = res["access_token"].as_str().map(|s| s.into());
        let refresh_token_opt: Option<Box<str>> = res["refresh_token"].as_str().map(|s| s.into());

        if uuid_opt.is_none() || mc_token_opt.is_none() || access_token_opt.is_none() || refresh_token_opt.is_none() {
            safe(move || f(Err("Failed to process session: Malformed or incomplete data, please contact support.".into())));
            return;
        }

        safe(move || f(Ok(MicrosoftSession {
            uuid: uuid_opt.unwrap(),
            username: "TODO?".into(),
            access_token: access_token_opt.unwrap(),
            refresh_token: refresh_token_opt.unwrap(),
            minecraft_token: mc_token_opt.unwrap()
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

            let body_res = res.unwrap();
            let ok = body_res["ok"].as_bool();

            if !ok.unwrap_or(false) {
                let body_err_msg: Box<str> = body_res["error"].as_str().unwrap_or("Cannot provide error message").into();
                let err_str = format!("Login failed: {}. Please contact support.", body_err_msg);

                safe(move || callback(Err(ErrorData { title: "Server Error", description: err_str.into() })));
                break;
            }

            if !body_res["authenticated"].as_bool().unwrap_or(false) {
                continue;
            }

            let uuid_opt = body_res["uuid"].as_str();
            let mc_token_opt = body_res["minecraft_token"].as_str();
            let access_token_opt = body_res["access_token"].as_str();
            let refresh_token_opt = body_res["refresh_token"].as_str();

            if uuid_opt.is_none() || mc_token_opt.is_none() || access_token_opt.is_none() || refresh_token_opt.is_none() {
                safe(move || callback(Err(ErrorData { title: "Session Error", description: "Data received from server is incomplete. Please contact support.".into() })));
                break;
            }
            let session = MicrosoftSession {
                uuid: uuid_opt.unwrap().into(),
                username: "TODO".into(),
                access_token: access_token_opt.unwrap().into(),
                refresh_token: refresh_token_opt.unwrap().into(),
                minecraft_token: mc_token_opt.unwrap().into()
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
    let json = get("https://piston-meta.mojang.com/mc/game/version_manifest.json")?;

    let version = json["versions"].members().find(|v| v["id"].as_str().map(|v| v == version).unwrap_or(false)).ok_or("Version not found")?;
    let url = version["url"].as_str().ok_or("Url not found in version")?;
    let manifest_json = get(url)?;

    Ok(MinecraftManifest::try_from(manifest_json)?)
}
