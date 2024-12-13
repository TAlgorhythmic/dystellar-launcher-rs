use std::fs;
use dotenv::dotenv;

fn main() {
    let global = fs::read_to_string("css_documents/global.css").expect("Error trying to read global.css.");
    let buttons = fs::read_to_string("css_documents/buttons.css").expect("Error trying to read buttons.css.");
    let components = fs::read_to_string("css_documents/components.css").expect("Error trying to read components.css.");

    let global_write = format!("pub const CSS: &str = \"{}\";", global);
    let buttons_write = format!("pub const CSS: &str = \"{}\";", buttons);
    let components_write = format!("pub const CSS: &str = \"{}\";", components);

    fs::write("src/css/global.rs", global_write).expect("Error trying to inject css.");
    fs::write("src/css/buttons.rs", buttons_write).expect("Error trying to inject css.");
    fs::write("src/css/components.rs", components_write).expect("Error trying to inject css.");

    dotenv().ok();

    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "unknown".to_string());

    let client_id: String;
    let callback_uri: String;

    if profile == "release" {
        client_id = std::env::var("PROD_CLIENT_ID").expect("Error getting production client id.");
        callback_uri = std::env::var("PROD_CALLBACK_URI").expect("Error getting production callback uri");
    } else {
        client_id = std::env::var("TEST_CLIENT_ID").expect("Error getting test client id.");
        callback_uri = std::env::var("TEST_CALLBACK_URI").expect("Error getting test calback uri.");
    }

    println!("cargo:rustc-env=CLIENT_ID={}", client_id);
    println!("cargo:rustc-env=CALLBACK_URI={}", callback_uri);
}
