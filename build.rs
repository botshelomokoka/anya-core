use std::env;

fn main() {
    // Detect OS and set appropriate feature flags
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_OS");
    
    match target_os.as_str() {
        "linux" => {
            println!("cargo:rustc-cfg=feature=\"secure-storage-linux\"");
            println!("cargo:rustc-cfg=unix");
        },
        "windows" => {
            println!("cargo:rustc-cfg=feature=\"secure-storage-windows\"");
            println!("cargo:rustc-cfg=windows");
        },
        "macos" => {
            println!("cargo:rustc-cfg=feature=\"secure-storage-macos\"");
            println!("cargo:rustc-cfg=unix");
        },
        _ => {
            println!("cargo:rustc-cfg=feature=\"secure-storage-fallback\"");
        }
    }
}
