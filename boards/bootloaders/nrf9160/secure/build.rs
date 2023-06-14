use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("../memory.x"))
        .unwrap();
    // println!("cargo:rustc-link-search=/target/thumbv8m.main-none-eabihf/release");
    // println!("cargo:rustc-link-search=/target/thumbv8m.main-none-eabihf/release/deps");
    println!("cargo:rustc-link-search={}", out.display());
    // trustzone_m_tools::generate_bindings("../non_secure/src/main.rs", true).unwrap();
    
    // println!("cargo:rustc-link-lib=non_secure");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=memory.x");
}
