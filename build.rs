#[cfg(debug_assertions)]
include!("./src/resource/ids.rs");

fn main() {
    println!("cargo:rustc-link-arg=/SUBSYSTEM:console");
    println!("cargo:rustc-link-arg=/ENTRY:entry");
    println!("cargo:rustc-link-arg=/nodefaultlib");

    // Statically link libs
    println!("cargo:rustc-link-lib=kernel32");
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=shell32");
    println!("cargo:rustc-link-lib=oleaut32");

    cc::Build::new().file("./interop.c").compile("interop");

    #[cfg(debug_assertions)]
    {
        use which::which;

        let sfsu_path = which("echo").expect("echo not found");
        let path = sfsu_path.display().to_string().replace("\\", "\\\\");

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
