use dotenv::dotenv;

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
}
