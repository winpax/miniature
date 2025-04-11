fn main() {
    cc::Build::new().file("interop.c").compile("interop");
    println!("cargo:rerun-if-changed=interop.c");
}
