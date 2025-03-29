fn main() {
    println!("cargo:rerun-if-changed=include/interop.c");

    cc::Build::new()
        .file("include/interop.c")
        .compile("interop");
}
