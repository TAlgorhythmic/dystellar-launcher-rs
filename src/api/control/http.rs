use webbrowser;
use uuid::Uuid;

const CLIENT_ID: &str = env!("CLIENT_ID");
const BACKEND_URL: &str = env!("BACKEND_URL");



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
