extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut ffmpeg_lib_path     = PathBuf::from(env::var("FFMPEG_PREFIX").unwrap());
    ffmpeg_lib_path.push("lib");
    let mut ffmpeg_include_path = PathBuf::from(env::var("FFMPEG_PREFIX").unwrap());
    ffmpeg_include_path.push("include");

    println!("cargo:rustc-link-lib=avformat");
    println!("cargo:rustc-link-lib=avcodec");
    println!("cargo:rustc-link-lib=avutil");
    println!("cargo:rustc-link-search=native={}", ffmpeg_lib_path.display());

    let bindings = bindgen::Builder::default()
        .no_unstable_rust()
        .clang_arg(format!("-I{}", ffmpeg_include_path.display()))
        .header("./helpers/bindings_required.h")
        .opaque_type("pthread.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
		.expect("Couldn't write bindings!");
}