use std::env;
use std::path::PathBuf;

pub fn main() {
    let bindings = bindgen::Builder::default()
        .clang_args(&["-x", "c"])
        .clang_args(api_header())
        .rust_target(bindgen::RustTarget::Nightly)
        .layout_tests(false)
        .generate_inline_functions(false)
        .derive_debug(true)
        .header("src/api.h")
        .generate()
        .expect("unable to generate api.h bindings!");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out.join("api.rs"))
        .expect("could not write bindings!");
}

fn api_header() -> Vec<String> {
    vec![
        "-I./api/include".into(),
    ]
}
