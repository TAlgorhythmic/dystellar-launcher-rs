use webbrowser;

const CLIENT_ID: &str = env!("CLIENT_ID");
const CALLBACK_URI: &str = env!("CALLBACK_URI");


pub fn login() {
    let url = format!("https://login.microsoftonline.com/common/oauth2/v2.0/authorize?client_id={}&response_type=code&redirect_uri={}&scope=XboxLive.signin%20offline_access&state=waiting", CLIENT_ID, CALLBACK_URI);

    if webbrowser::open(&url).is_ok() {

    }
}
