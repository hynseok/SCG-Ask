use std::env;

fn main() {
    let api_url = env::var("API_URL").expect("API_URL must be set");
    println!("cargo:rustc-env=API_URL={}", api_url);
}