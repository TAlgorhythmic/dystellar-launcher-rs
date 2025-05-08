use std::{collections::HashMap, error::Error, iter::Map, sync::LazyLock, thread::spawn, time::Duration};

use json::{object, JsonValue};
use ureq::Agent;
use webbrowser;
use uuid::Uuid;

const CLIENT_ID: &str = env!("CLIENT_ID");
const BACKEND_URL: &str = env!("BACKEND_URL");
const AGENT: LazyLock<Agent> = LazyLock::new(|| Agent::config_builder().timeout_global(Some(Duration::from_secs(6))).build().into());

pub fn get(path: &str) -> Result<JsonValue, Box<dyn Error + Send + Sync>> {
    let url = format!("{}{}", BACKEND_URL, path);

    let req = AGENT.get(url).call()?;
    let res = json::parse(req.into_body().read_to_string()?.as_str())?;

    Ok(res)
}

pub fn login() {
    let uuid = Uuid::new_v4();
    println!("{uuid}");

    let callback = "/api/microsoft/callback";

    let ms_url = format!("https://login.microsoftonline.com/common/oauth2/v2.0/authorize?client_id={}&response_type=code&redirect_uri={}&scope=XboxLive.signin%20offline_access&state={}", CLIENT_ID, callback, &uuid);

    spawn(move || {
        println!("sometit");
        let lsopt = request(format!("/api/microsoft/loginsession?uuid={uuid_owned}").as_str(), "POST", object! {});
        if lsopt.is_err() {
            // TODO: Error trying to connect to server
            println!("Error connecting to server.");
            return;
        }
        
        let lsession_res = lsopt.unwrap();
        let js = get_body(lsession_res).await.unwrap();
        
        if !js["ok"].as_bool().unwrap() {
            // TODO: Server error
            println!("Server error");
            return;
        }

        if webbrowser::open(&ms_url).is_ok() {
            let login_url = format!("/api/microsoft/login?uuid={uuid_owned}");

            loop {
                let _ = tokio::time::sleep(Duration::from_secs(2));
                let res = request(login_url.as_str(), "GET", object! {}).await;

                if let Err(_) = &res {
                    // TODO: Error
                    println!("Error ege");
                }

                let body_res = json::parse(String::from_utf8(res.unwrap().into_body().collect().await.unwrap().to_bytes().to_vec()).unwrap().as_str()).unwrap();

                if body_res["ok"].as_bool().unwrap() {
                    println!("Logged in");
                    break;
                }
            }
        }
    });
}
