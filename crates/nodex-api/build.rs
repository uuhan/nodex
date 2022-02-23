#![allow(unreachable_code)]
use fs_extra::dir::copy;
use fs_extra::dir::CopyOptions;

pub fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    #[cfg(feature = "gen-api")]
    {
        use std::env;
        use std::path::PathBuf;

        let bindings = bindgen::Builder::default()
            .clang_args(&["-x", "c"])
            .clang_args(&["-Iapi/include"])
            .clang_args(api_version())
            .rust_target(bindgen::RustTarget::Nightly)
            .layout_tests(false)
            .generate_inline_functions(false)
            .derive_debug(true)
            .header("src/api.h")
            .generate()
            .expect("unable to generate api.h bindings!");

        let out = PathBuf::from("./src");
        bindings
            .write_to_file(out.join("out.rs"))
            .expect("could not write bindings!");
    }

    if target_os == "windows" {
        let out = std::env::var("OUT_DIR").unwrap();
        let out_dir = std::path::PathBuf::from(&out);
        let mut options = CopyOptions::new();
        options.skip_exist = true;
        copy("./lib", &out_dir, &options).unwrap();

        println!("cargo:rustc-link-search={}/lib", out_dir.to_str().unwrap());
        println!("cargo:rustc-link-lib=node-v16.0.0-x64");
    }

    if target_os == "macos" {
        // NB: macos link options
        println!("cargo:rustc-cdylib-link-arg=-Wl");
        println!("cargo:rustc-cdylib-link-arg=-undefined");
        println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");
    }
}

#[cfg(feature = "gen-api")]
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
