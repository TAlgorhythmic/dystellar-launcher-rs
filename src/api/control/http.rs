use std::{error::Error, fmt::Write, io::Read};

use http_body_util::BodyExt;
use hyper::{body::{self, Incoming}, client::conn::http1::handshake, Request, Response, Uri};
use hyper_util::rt::TokioIo;
use json::{object, JsonValue};
use tokio::net::TcpStream;
use webbrowser;
use uuid::Uuid;

const CLIENT_ID: &str = env!("CLIENT_ID");
const BACKEND_URL: &str = env!("BACKEND_URL");

pub async fn request(path: &str, method: &str, body: JsonValue) -> Result<Response<Incoming>, Box<dyn Error + Send + Sync>> {
    let abspath = format!("{}{}", BACKEND_URL, path);
    let url: Uri = abspath.parse()?;

    let host = url.host().unwrap();
    let port = url.port_u16().unwrap_or(80);

    let address = format!("{}:{}", host, port);
    
    let stream = TcpStream::connect(address).await?;
    let io = TokioIo::new(stream);

    let (mut sender, _) = handshake(io).await?;

    let authority = url.authority().unwrap();

    let req = Request::builder()
        .uri(&url)
        .header(hyper::header::HOST, authority.as_str())
        .method(method)
        .body(String::from(stringify!(body)))?;

    let res = sender.send_request(req).await?;

    Ok(res)
}

pub fn login() {
    let uuid = Uuid::new_v4();
    println!("{uuid}");

    let callback = format!("{BACKEND_URL}/api/microsoft/callback");

    let ms_url = format!("https://login.microsoftonline.com/common/oauth2/v2.0/authorize?client_id={}&response_type=code&redirect_uri={}&scope=XboxLive.signin%20offline_access&state={}", CLIENT_ID, callback, &uuid);

    tokio::spawn(async move {
        let uuid_owned = uuid;
        let lsopt = request(format!("/api/microsoft/loginsession?uuid={uuid_owned}").as_str(), "POST", object! {}).await;
        if lsopt.is_err() {
            // TODO: Error trying to connect to server
            println!("Error connecting to server.");
            return;
        }
        
        let lsession_res = lsopt.unwrap();
        let bod = lsession_res.into_body().collect().await.unwrap().to_bytes();
        let js = json::parse(String::from_utf8(bod.to_vec()).unwrap().as_str()).unwrap();
        
        if !js["ok"].as_bool().unwrap() {
            // TODO: Server error
            println!("Server error");
            return;
        }

        if webbrowser::open(&ms_url).is_ok() {
        }
    });
}
