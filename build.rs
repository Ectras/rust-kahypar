use std::{env, path::PathBuf};

use cmake::Config;

fn main() {
    // Build hptt
    let dst = Config::new("kahypar").build();

    // Link it
    println!("cargo:rustc-link-search={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=hptt");
    println!("cargo:rustc-link-lib=stdc++");

    // Generate bindings
    let header = "hptt_c_api.h";
    println!("cargo:rerun-if-changed={header}");
    let bindings = bindgen::Builder::default()
        .header(header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
