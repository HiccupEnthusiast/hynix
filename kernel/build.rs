fn main() {
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH")
        .expect("Couldn't find CARGO_CFG_TARGET_ARCH while building kernel");
    println!("cargo:rustc-link-arg=-Tlinker-{arch}.ld");
    println!("cargo:rerun-if-changed=linker-{arch}.ld");
}
