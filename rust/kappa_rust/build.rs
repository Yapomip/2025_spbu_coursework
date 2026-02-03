use cmake::Config;
use std::path::{Path, PathBuf};

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=./build.rs");
    println!("cargo:rerun-if-changed=./src/");

    let main_dir: PathBuf = std::env::var_os("CARGO_MANIFEST_DIR").unwrap().into();
    let lib_name = "kappa_c_wrap";
    let path_to_lib = main_dir.join("../").join(Path::new(lib_name));

    // cmake build
    let dst = Config::new(path_to_lib.as_path()).build();

    // add libraryes
    println!("cargo:rustc-link-lib={}", lib_name);
    println!("cargo:rustc-link-lib=dylib=kappa++");
    // path to installed libraries
    println!("cargo:rustc-link-search={}/lib", dst.display());
    // path to installed data
    println!("cargo:rustc-env=KAPPA_RESOURCES_PATH={}/data", dst.display());

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++17")
        .enable_cxx_namespaces()
        .wrap_unsafe_ops(true)
        .layout_tests(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()?;

    let path_to_out = main_dir.join(Path::new("src/hellomod.rs"));
    bindings.write_to_file(path_to_out)?;

    // link kappa++ libs dependides
    // println!("cargo:rustc-link-lib=dylib=stdc++");
    // println!("cargo:rustc-link-lib=dylib=openblas");
    // println!("cargo:rustc-link-lib=dylib=yaml-cpp");
    // println!("cargo:rustc-link-lib=dylib=armadillo");

    Ok(())
}
