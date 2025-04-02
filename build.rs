#[cfg(debug_assertions)]
include!("./src/resource/ids.rs");

fn main() {
    #[cfg(debug_assertions)]
    {
        use which::which;

        let sfsu_path = which("sfsu.exe").expect("sfsu.exe not found");

        let mut res = winres::WindowsResource::new();
        res.append_rc_content(&format!(
            r##"
            #define IDS_PATH {IDS_PATH}
            #define IDS_ARGS {IDS_ARGS}

            STRINGTABLE
            {{
            IDS_PATH, "{path}"
            IDS_ARGS, "--help"
            }}
            "##,
            path = sfsu_path.display(),
        ));
        res.compile().unwrap();
    }
}
