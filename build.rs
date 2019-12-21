fn build_detours() {
    cc::Build::new()
        .flag("/W4")
        .flag("/WX")
        .flag("/MT")
        .flag("/Gy")
        .flag("/Gm-")
        .flag("/Zl")
        .flag("/Od")
        .define("WIN32_LEAN_AND_MEAN", None)
        .define("_WIN32_WINNT", "0x501")
        .file("deps/Detours/src/detours.cpp")
        .file("deps/Detours/src/modules.cpp")
        .file("deps/Detours/src/disasm.cpp")
        .file("deps/Detours/src/image.cpp")
        .file("deps/Detours/src/creatwth.cpp")
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
        .expect("Unable to generate bindings");
    //
    bindings
        .write_to_file(out_path)
}

fn main() {
    build_detours();
    generate_bindings();
}
