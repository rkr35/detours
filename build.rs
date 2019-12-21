//! Build detours.
fn build_detours() {
    cc::Build::new()
        .include("deps/Detours/src/")
        .flag("/MT")
        .flag("/W4")
        .flag("/WX")
        .flag("/Gy")
        .flag("/Gm-")
        .flag("/Zl")
        .flag("/Od")
        .define("WIN32_LEAN_AND_MEAN", "1")
        .define("_WIN32_WINNT", "0x501")
        .file("deps/Detours/src/detours.cpp")
        .file("deps/Detours/src/modules.cpp")
        .file("deps/Detours/src/disasm.cpp")
        .file("deps/Detours/src/image.cpp")
        .file("deps/Detours/src/creatwth.cpp")
        .compile("detours");
}

fn generate_bindings() {
    use std::{env, fs, path::PathBuf};
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::copy("deps/Detours/src/detours.h", out_path.join("detours.h")).unwrap();
    //
    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", out_path.to_str().expect("OUTDIR is weird")))
        .clang_arg("-fms-compatibility")
        .clang_arg("-fms-extensions")

        .whitelist_function("DetourTransactionBegin")
        .whitelist_function("DetourUpdateThread")
        .whitelist_function("DetourAttach")
        .whitelist_function("DetourDetach")
        .whitelist_function("DetourTransactionCommit")

        .header("build/wrapper.h")
        .generate()
        .expect("Unable to generate bindings");
    //
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    build_detours();
    generate_bindings();
}
