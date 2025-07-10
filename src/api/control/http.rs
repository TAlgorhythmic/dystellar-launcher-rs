use std::{error::Error, sync::LazyLock, thread::spawn, time::Duration};

use json::{object, stringify, JsonValue};
use ureq::Agent;
use webbrowser;
use uuid::Uuid;

static CLIENT_ID: &str = env!("CLIENT_ID");
static BACKEND_URL: &str = env!("BACKEND_URL");
static AGENT: LazyLock<Agent> = LazyLock::new(|| Agent::config_builder().timeout_global(Some(Duration::from_secs(6))).build().into());

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

pub fn login() {
    let uuid = Uuid::new_v4();

    let callback = format!("{BACKEND_URL}/api/microsoft/callback");

    let ms_url = format!("https://login.live.com/oauth20_authorize.srf?client_id={}&response_type=code&redirect_uri={}&scope=XboxLive.signin%20offline_access&state={}", CLIENT_ID, callback, &uuid);

    spawn(move || {
        let uuid_str = uuid.to_string();
        let lsopt = post("/api/microsoft/loginsession", object! { uuid: uuid_str.clone() });

        if lsopt.is_err() {
            // TODO: Error trying to connect to server
            println!("Error connecting to server.");
            return;
        }
        
        let lsession_res = lsopt.unwrap();
        
        if !lsession_res["ok"].as_bool().unwrap() {
            // TODO: Server error
            println!("Server error");
            return;
        }

        if webbrowser::open(&ms_url).is_ok() {
            let login_url = format!("/api/microsoft/login?uuid={uuid_str}");

            loop {
                let _ = std::thread::sleep(Duration::from_secs(2));
                let res = get(login_url.as_str());

                if let Err(_) = &res {
                    // TODO: Error
                    println!("Error ege");
                }

                let body_res = res.unwrap();

                if body_res["ok"].as_bool().unwrap() {
                    println!("Logged in");
                    break;
                }
            }
        }
    });
}
