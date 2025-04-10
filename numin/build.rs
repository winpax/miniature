use std::{env, error::Error, fs, path::PathBuf};

fn handle_error<T, E: Error>(result: Result<T, E>) -> T {
    match result {
        Err(e) => {
            println!("cargo::error={}", e);
            std::process::exit(0);
        }
        Ok(v) => v,
    }
}

fn main() {
    let download_url = concat!(
        "https://github.com/winpax/miniature/releases/download/v",
        env!("CARGO_PKG_VERSION"),
        "/miniature.exe"
    );

    let resp = handle_error(reqwest::blocking::get(download_url));
    if !resp.status().is_success() {
        println!("cargo::error=Failed to download miniature.exe");
        println!("cargo::error={}", resp.status());
    } else {
        let binary_data = handle_error(resp.bytes());

        let output_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("miniature.exe");
        fs::write(&output_path, &binary_data).expect("Failed to write miniature.exe");
    }
}
