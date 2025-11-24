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
    let main_out = out_dir.join("main_ui.rs");
    let welcome_out = out_dir.join("welcome_ui.rs");
    let fallback_dialog_out = out_dir.join("fallback_dialog_ui.rs");
    let mods_out = out_dir.join("mods_ui.rs");
    let modinfo_out = out_dir.join("modinfo_ui.rs");

    slint_build::compile_with_output_path("ui/main.slint", main_out, conf.clone()).unwrap();
    slint_build::compile_with_output_path("ui/welcome_ui.slint", welcome_out, conf.clone()).unwrap();
    slint_build::compile_with_output_path("ui/fallback_dialog.slint", fallback_dialog_out, conf.clone()).unwrap();
    slint_build::compile_with_output_path("ui/modinfo_ui.slint", modinfo_out, conf.clone()).unwrap();
    slint_build::compile_with_output_path("ui/mods_ui.slint", mods_out, conf).unwrap();
}
