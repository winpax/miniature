#[cfg(debug_assertions)]
include!("./src/resource/ids.rs");

fn main() {
    println!("cargo:rustc-link-arg=/SUBSYSTEM:console");
    println!("cargo:rustc-link-arg=/ENTRY:entry");
    println!("cargo:rustc-link-arg=/nodefaultlib");

    cc::Build::new().file("interop.c").compile("interop");
    println!("cargo:rerun-if-changed=interop.c");

    #[cfg(debug_assertions)]
    {
        let exe_path =
            std::path::PathBuf::from("C:\\Users\\julie\\scoop\\apps\\figma\\current\\Figma.exe");
        // let exe_path = which::which("echo").expect("echo not found");
        let path = exe_path.display().to_string().replace("\\", "\\\\");

        let mut res = winres::WindowsResource::new();
        res.append_rc_content(&format!(
            r##"
            #define IDS_PATH {IDS_PATH}
            #define IDS_ARGS {IDS_ARGS}

            STRINGTABLE
            {{
            IDS_PATH, "{path}"
            IDS_ARGS, "search sfsu"
            }}
            "##,
        ));
        res.compile().unwrap();
    }
}
