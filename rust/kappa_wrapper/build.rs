
use cmake::Config;
use conan2::ConanInstall;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");

    let dst = Config::new("libfoo").build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=foo");

    let dst = Config::new("libfoo").build();       

}

