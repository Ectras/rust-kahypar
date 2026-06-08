use cmake::Config;
use std::{env, path::PathBuf};

fn main() {
    // Build kahypar
    let dst = Config::new("extern/kahypar")
        .configure_arg("-DBUILD_TESTING=False")
        .configure_arg("-DSTATICCOMPILE=True")
        .configure_arg("-DKAHYPAR_USE_MINIMAL_BOOST=True")
        .profile("Release")
        .build();

    // Link it
    println!("cargo:rustc-link-search={}", dst.join("lib").display());
    println!("cargo:rustc-link-search={}", dst.join("build").display());
    println!("cargo:rustc-link-lib=kahypar");
    println!("cargo:rustc-link-lib=mini_boost");

    if let Ok(boost_dir) = env::var("BOOST_DIR") {
        println!("cargo:rustc-link-search={boost_dir}/lib");
    }

    // Generate bindings
    let header = "extern/kahypar/include/libkahypar.h";
    println!("cargo:rerun-if-changed={header}");
    let bindings = bindgen::Builder::default()
        .header(header)
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bind_path = out_path.join("bindings.rs");
    bindings
        .write_to_file(bind_path)
        .unwrap_or_else(|_| panic!("Unable to write bindings to {}", out_path.to_str().unwrap()));
}
