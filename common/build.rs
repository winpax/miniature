fn main() {
    let mut build = cc::Build::new();
    build.file("interop/macros.c");
    println!("cargo:rerun-if-changed=interop/macros.c");

    #[cfg(feature = "crt_functions")]
    build.file("interop/crt_fns.c");
    println!("cargo:rerun-if-changed=interop/crt_fns.c");

    build.compile("interop");
    println!("cargo:rerun-if-changed=interop.c");
}
