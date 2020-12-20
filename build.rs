use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{env, fs};

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let name = env::var("CARGO_PKG_NAME").unwrap();

    if target.starts_with("mips") {
        let suffix = if env::var_os("CARGO_FEATURE_LINKER_PLUGIN_LTO").is_some() {
            "-lto"
        } else {
            ""
        };

        fs::copy(
            format!("bin/{}{}.a", target, suffix),
            out_dir.join(format!("lib{}.a", name)),
        )
        .unwrap();

        println!("cargo:rustc-link-lib=static={}", name);
        println!("cargo:rustc-link-search={}", out_dir.display());
    }


    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("link.ld"))
        .unwrap()
        .write_all(include_bytes!("link.ld"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
}
