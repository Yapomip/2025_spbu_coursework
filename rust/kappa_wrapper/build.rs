
use cmake::Config;
use std::path::{Path, PathBuf};

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=./build.rs");
    println!("cargo:rerun-if-changed=./src/");
    println!("cargo:rerun-if-changed=./kappa++/");

    let out_dir: PathBuf = std::env::var_os("OUT_DIR").expect("OUT_DIR PUPUPU").into();
    
    // static lib
    println!("cargo:rustc-link-search={}", out_dir.as_path().display());
    // dynamic lib
    println!("cargo:rustc-link-search={}/lib", out_dir.as_path().display());
    
    
    let dst = Config::new("libfoo").build(); 
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=foo");

    let main_dir: PathBuf = std::env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR PUPUPU").into();
    let lib_name = "kappa++";
    let lib_git_url = "https://github.com/lkampoli/kappa";
    let path_to_lib = main_dir.join(Path::new(lib_name));

    if !path_to_lib.exists() {
        git2::Repository::clone(lib_git_url, path_to_lib.as_path())?;
    }
    let dst = Config::new(path_to_lib.as_path())
        .env("CMAKE_INSTALL_PREFIX", out_dir.as_path())
        .build();

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++14")
        .clang_arg(format!("-I{}/install/include/kappa++", out_dir.as_path().display()))
        .enable_cxx_namespaces()
        .wrap_unsafe_ops(true)
        .layout_tests(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()?;

    let path_to_out = main_dir.join(Path::new("src/hellomod.rs"));
    bindings.write_to_file(path_to_out)?;

    println!("cargo:rustc-link-search={}/install/lib", dst.display());
    println!("cargo:rustc-link-lib={}", lib_name);


    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=dylib=openblas");
    println!("cargo:rustc-link-lib=dylib=yaml-cpp");
    println!("cargo:rustc-link-lib=dylib=armadillo");

    Ok(())
}

