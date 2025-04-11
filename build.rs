#[cfg(debug_assertions)]
include!("./src/resource/ids.rs");

fn main() {
    println!("cargo:rustc-link-arg=/SUBSYSTEM:console");
    println!("cargo:rustc-link-arg=/ENTRY:entry");
    println!("cargo:rustc-link-arg=/nodefaultlib");

    #[cfg(debug_assertions)]
    {
        use std::{env, path::PathBuf};

        let exe_path = PathBuf::from(env::var("DEBUG_SHIM_TARGET").unwrap_or("echo".to_string()));
        let args = env::var("DEBUG_SHIM_ARGS").unwrap_or("miniature called how cool".to_string());

        println!("cargo:rerun-if-env-changed=DEBUG_SHIM_TARGET");
        println!("cargo:rerun-if-env-changed=DEBUG_SHIM_ARGS");

        let absolute_path = if !exe_path.is_absolute() {
            which::which(&exe_path)
                .unwrap_or_else(|_| panic!("Could not find executable: {}", exe_path.display()))
        } else {
            exe_path
        };

        println!("cargo:rerun-if-changed={}", absolute_path.display());

        let mut res = winres::WindowsResource::new();
        res.append_rc_content(&format!(
            r##"
            #define IDS_PATH {IDS_PATH}
            #define IDS_ARGS {IDS_ARGS}

            STRINGTABLE
            {{
            IDS_PATH, "{path}"
            IDS_ARGS, "{args}"
            }}
            "##,
            path = absolute_path.display().to_string().replace("\\", "\\\\"),
        ));
        res.compile().unwrap();
    }
}
