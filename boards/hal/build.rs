use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    #[cfg(feature = "nrf9160")]
    // trustzone_m_tools::generate_bindings("../bootloaders/nrf9160/non_secure/src/main.rs", true).unwrap();
    // trustzone_m_tools::generate_bindings("../bootloaders/nrf9160/secure/src/main.rs", true).unwrap();
    
    println!("cargo:rustc-link-search=../bootloaders/nrf9160/target/thumbv8m.main-none-eabihf/release");
    println!("cargo:rustc-link-search=../bootloaders/nrf9160/target/thumbv8m.main-none-eabihf/release/deps");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rustc-link-lib=non_secure");
}