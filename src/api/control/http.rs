use std::error::Error;

use hyper::{body::Incoming, client::conn::http1::handshake, Request, Response, Uri};
use hyper_util::rt::TokioIo;
use json::JsonValue;
use tokio::net::TcpStream;
use webbrowser;
use uuid::Uuid;

const CLIENT_ID: &str = env!("CLIENT_ID");
const BACKEND_URL: &str = env!("BACKEND_URL");

pub async fn request(url: &str, method: &str, body: JsonValue) -> Result<Response<Incoming>, Box<dyn Error + Send + Sync>> {
    let url: Uri = url.parse()?;

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

    let mut callback = String::from(BACKEND_URL);
    callback.push_str("/api/microsoft/callback");

    let ms_url = format!("https://login.microsoftonline.com/common/oauth2/v2.0/authorize?client_id={}&response_type=code&redirect_uri={}&scope=XboxLive.signin%20offline_access&state={}", CLIENT_ID, callback, &uuid);

    tokio::spawn(async move {
        if webbrowser::open(&ms_url).is_ok() {
            
        }
    });
}
