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
    println!("cargo:rerun-if-env-changed=MINIATURE_EXE_PATH");
    if let Ok(executable_path) = std::env::var("MINIATURE_EXE_PATH") {
        let path = PathBuf::from(executable_path);
        if !path.exists() {
            println!("cargo::error=MINIATURE_EXE_PATH does not exist");
            std::process::exit(0);
        }
        if !path.is_file() {
            println!("cargo::error=MINIATURE_EXE_PATH is not a file");
            std::process::exit(0);
        }
        if path.extension().is_none_or(|ext| ext != "exe") {
            println!("cargo::error=MINIATURE_EXE_PATH is not a .exe file");
            std::process::exit(0);
        }

        let output_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("miniature.exe");

        if output_path.exists() {
            fs::remove_file(&output_path).expect("Failed to remove existing miniature.exe");
        }
        fs::copy(&path, &output_path).expect("Failed to copy miniature.exe");

        println!("cargo:rerun-if-changed={}", path.display());
    } else {
        if cfg!(not(debug_assertions)) {
            println!("cargo::warning=Downloading latest build in release mode");
        }

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
}
