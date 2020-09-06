#![warn(clippy::pedantic)]

use std::{
    env,
    fs,
    path::PathBuf,
};

const DETOURS_SOURCE_DIR: &str = "deps/Detours/src/";

fn build_detours() {
    let source_files = fs::read_dir(DETOURS_SOURCE_DIR)
        .expect("unable to read detours source directory")
        .filter_map(|f| f
            .ok()
            .map(|f| f.path())
            .filter(|p| {
                let p = p.to_string_lossy();
                p.ends_with(".cpp") && !p.ends_with("uimports.cpp")
            })
        );

    cc::Build::new()
        .include(DETOURS_SOURCE_DIR)

        // CFLAGS: https://github.com/microsoft/Detours/blob/64ec135a509884aa60ac6c19b59564f1da9cb2fa/src/Makefile#L19
        .flag("/W4")
        .flag("/WX")
        .flag("/MT")
        .flag("/Gy")
        .flag("/Gm-")
        .flag("/Zl")
        .flag("/Od")

        // https://github.com/microsoft/Detours/blob/64ec135a509884aa60ac6c19b59564f1da9cb2fa/src/Makefile#L24
        .define("WIN32_LEAN_AND_MEAN", None)
        .define("_WIN32_WINNT", "0x501")
        .files(source_files)
        .compile("detours");
}

fn generate_bindings() {
    let mut out_path: PathBuf = env::var("OUT_DIR")
        .expect("unable to fetch OUT_DIR environment variable")
        .into();

    out_path.push("bindings.rs");

    let bindings = bindgen::Builder::default()
        .whitelist_function("DetourTransactionBegin")
        .whitelist_function("DetourUpdateThread")
        .whitelist_function("DetourAttach")
        .whitelist_function("DetourDetach")
        .whitelist_function("DetourTransactionCommit")
        .use_core()
        .header("build/wrapper.h")
        .generate()
        .expect("unable to generate bindings");

    bindings
        .write_to_file(out_path)
        .expect("couldn't write bindings");
}

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=build/wrapper.h");
    
    build_detours();
    generate_bindings();
}
