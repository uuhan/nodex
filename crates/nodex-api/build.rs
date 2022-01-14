#![allow(unreachable_code)]

use std::env;
use std::path::PathBuf;

pub fn main() {
    let bindings = bindgen::Builder::default()
        .clang_args(&["-x", "c"])
        .clang_args(&["-I../../api/include"])
        .clang_args(api_version())
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

    // NB: macos link options
    // #[cfg(target_os="macos")]
    // {
    //     println!("cargo:rustc-cdylib-link-arg=-Wl");
    //     println!("cargo:rustc-cdylib-link-arg=-undefined");
    //     println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");
    // }
}

fn api_version<'a>() -> &'a [&'a str] {
    #[cfg(feature = "v8")]
    return &["-DNAPI_VERSION=8"];
    #[cfg(feature = "v7")]
    return &["-DNAPI_VERSION=7"];
    #[cfg(feature = "v6")]
    return &["-DNAPI_VERSION=6"];
    #[cfg(feature = "v5")]
    return &["-DNAPI_VERSION=5"];
    #[cfg(feature = "v4")]
    return &["-DNAPI_VERSION=4"];
    #[cfg(feature = "v3")]
    return &["-DNAPI_VERSION=3"];
    #[cfg(feature = "v2")]
    return &["-DNAPI_VERSION=2"];
    #[cfg(feature = "v1")]
    return &["-DNAPI_VERSION=1"];

    &["-DNAPI_VERSION=100"]
}