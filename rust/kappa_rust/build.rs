use cmake::Config;
use std::path::{Path, PathBuf};

fn main() -> anyhow::Result<()> {
    
    println!("cargo:rerun-if-changed=./build.rs");
    println!("cargo:rerun-if-changed=./src/");
    
    let main_dir: PathBuf = std::env::var_os("CARGO_MANIFEST_DIR").unwrap().into();
    let bin_dir: PathBuf = std::env::var_os("OUT_DIR").unwrap().into();
    let kappa_intall_dir = main_dir.join("../kappa/install");
    let kappa_c_wrap_intall_dir = main_dir.join("../kappa_c_wrap/install");

    if cfg!(windows) {
        let dst = Config::new("./CMakeLists.txt")
            .define("KAPPA_C_WRAP_DIR", kappa_c_wrap_intall_dir.clone())
            .define("COPY_TO_DIR", bin_dir)
            .build_target("dll_collector")
            .generator("Ninja")
            .build();
        println!("cargo:rustc-link-search={}", dst.display());
    }
    // path to libs
    println!("cargo:rustc-link-search={}/lib", kappa_intall_dir.display());
    println!("cargo:rustc-link-search={}/lib", kappa_c_wrap_intall_dir.display());
    // add libraryes
    println!("cargo:rustc-link-lib=kappa_c_wrap");
    println!("cargo:rustc-link-lib=dylib=kappa++");
    // path to installed data
    println!("cargo:rustc-env=KAPPA_RESOURCES_PATH={}/data", kappa_intall_dir.display());

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()?;

    let path_to_out = main_dir.join(Path::new("src/hellomod.rs"));
    bindings.write_to_file(path_to_out)?;

    Ok(())
}
