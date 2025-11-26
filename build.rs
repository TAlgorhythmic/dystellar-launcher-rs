use std::path::PathBuf;

use dotenv::dotenv;
use slint_build::CompilerConfiguration;

fn main() {
    dotenv().ok();

    let profile = std::env::var("PROFILE").unwrap_or("unknown".to_string());

    let client_id: String;
    let backend_url: String;
    let crypt_pass = std::env::var("CRYPT_PASS").expect("Failed to get CRYPT_PASS env");

    if profile == "release" {
        client_id = std::env::var("PROD_CLIENT_ID").expect("Error getting production client id.");
        backend_url = std::env::var("PROD_BACKEND_URL").expect("Error getting production callback uri");
    } else {
        client_id = std::env::var("TEST_CLIENT_ID").expect("Error getting test client id.");
        backend_url = std::env::var("TEST_BACKEND_URL").expect("Error getting test calback uri.");
    }

    println!("cargo:rustc-env=CLIENT_ID={}", client_id);
    println!("cargo:rustc-env=BACKEND_URL={}", backend_url);
    println!("cargo:rustc-env=CRYPT_PASS={}", crypt_pass);

    let conf = CompilerConfiguration::new();
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let app_out = out_dir.join("app.rs");

    slint_build::compile_with_output_path("ui/app.slint", app_out, conf).unwrap();
}
