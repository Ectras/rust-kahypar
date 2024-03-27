use cmake::Config;
use std::{env, path::PathBuf};

fn main() {
    // Build kahypar
    let dst = Config::new("kahypar")
        .configure_arg("-DBUILD_TESTING=False")
        .configure_arg("-DBUILD_SHARED_LIBS=False")
        .profile("Release")
        .build();

    // Link it
    println!("cargo:rustc-link-search={}", dst.join("lib").display());
    println!("cargo:rustc-link-search={}", dst.join("lib64").display());
    println!("cargo:rustc-link-lib=kahypar");
    println!("cargo:rustc-link-lib=boost_program_options");
    println!("cargo:rustc-link-lib=stdc++");

    // Generate bindings
    let header = "kahypar/include/libkahypar.h";
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
