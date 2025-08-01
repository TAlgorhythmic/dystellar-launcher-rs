use std::{error::Error, sync::LazyLock, time::Duration};

use json::{object, stringify, JsonValue};
use ureq::Agent;
use webbrowser;
use uuid::Uuid;

use crate::{api::{control::callbacks::exec_safe_gtk, typedef::ms_session::MicrosoftSession}, ui::{components::{show_confirmation_dialog, show_dialog, ICON_ERROR}, welcome_ui::login_callback}};

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

pub fn login_existing(access_token: &str, refresh_token: &str) -> Result<MicrosoftSession, Box<dyn Error + Send + Sync>> {
    let res = post(format!("/api/microsoft/login_existing").as_str(), object! {
        access_token: access_token, refresh_token: refresh_token
    })?;

    let uuid_opt = res["uuid"].as_str();
    let mc_token_opt = res["minecraft_token"].as_str();
    let access_token_opt = res["access_token"].as_str();
    let refresh_token_opt = res["refresh_token"].as_str();

    if uuid_opt.is_none() || mc_token_opt.is_none() || access_token_opt.is_none() || refresh_token_opt.is_none() {
        return Err("Malformed or incomplete data. Please contact support.".into());
    }

    Ok(MicrosoftSession {
        uuid: uuid_opt.unwrap().into(),
        username: "TODO?".into(),
        access_token: access_token_opt.unwrap().into(),
        refresh_token: refresh_token_opt.unwrap().into(),
        minecraft_token: mc_token_opt.unwrap().into()
    })
}

pub fn login() {
    std::thread::spawn(move || {
        let uuid = Uuid::new_v4();
        let callback = format!("{BACKEND_URL}/api/microsoft/callback");

        let ms_url = format!("https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize?client_id={CLIENT_ID}&response_type=code&redirect_uri={callback}&scope=XboxLive.signin%20offline_access&state={uuid}");

        let uuid_str = uuid.to_string();
        let lsopt = post("/api/microsoft/loginsession", object! { uuid: uuid_str.clone() });

        if lsopt.is_err() {
            println!("Error connecting to server.");
            exec_safe_gtk(|| {
                show_confirmation_dialog(
                    "Connection Error",
                    "Error connecting to server, please check your internet connection.",
                    Some("Try Again"),
                    ICON_ERROR,
                    || login());
            });
            return;
        }
        
        let lsession_res = lsopt.unwrap();
        
        if !lsession_res["ok"].as_bool().unwrap() {
            // TODO: Server error
            exec_safe_gtk(|| {
                show_confirmation_dialog(
                    "Server Error",
                    "An unexpected error occured, please try again later. \nSorry for the inconvenience.",
                    Some("Try Again"),
                    ICON_ERROR,
                    || login());
            });
            println!("Server error");
            return;
        }

        if webbrowser::open(&ms_url).is_ok() {
            let login_url = format!("/api/microsoft/login?uuid={uuid_str}");

            loop {
                let _ = std::thread::sleep(Duration::from_secs(2));
                let res = get(login_url.as_str());

                if let Err(err) = &res {
                    let str_err = err.to_string();
                    exec_safe_gtk(move || show_dialog("Fatal Error", format!("Failed to check login status: {str_err} \nIs your internet down?").as_str(), None, ICON_ERROR));
                    break;
                }

                let body_res = res.unwrap();
                let ok = body_res["ok"].as_bool();

                if !ok.unwrap_or(false) {
                    let body_err_msg: Box<str> = body_res["error"].as_str().unwrap_or("Cannot provide error message").into();
                    exec_safe_gtk(move || show_dialog("Server Error", format!("Login failed: {}. Please contact support.", body_err_msg).as_str(), None, ICON_ERROR));
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
                    exec_safe_gtk(|| show_dialog("Session Error", "Data received from server is incomplete.\nPlease contact support.", None, ICON_ERROR));
                    break;
                }
                let session = MicrosoftSession {
                    uuid: uuid_opt.unwrap().into(),
                    username: "TODO".into(),
                    access_token: access_token_opt.unwrap().into(),
                    refresh_token: refresh_token_opt.unwrap().into(),
                    minecraft_token: mc_token_opt.unwrap().into()
                };
                exec_safe_gtk(move || login_callback(session));

                println!("Logged in");
                break;
            }
        }
    });
}
