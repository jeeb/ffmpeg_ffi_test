extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    let used_libraries = ["libavcodec", "libavformat", "libavutil"];
    let mut include_paths = Vec::new();

    for library in used_libraries.iter() {
        if let Ok(lib) = pkg_config::find_library(library) {
            for path in &lib.include_paths {
                include_paths.push(format!("-I{}", path.display()));
                println!("cargo:include={}", path.display());
            }

            for path in &lib.link_paths {
                println!("cargo:rustc-link-search=native={}", path.display());
            }

            for lib in &lib.libs {
                println!("cargo:rustc-link-lib={}", lib);
            }
        }
    }

    let bindings = bindgen::Builder::default()
        .no_unstable_rust()
        .clang_args(include_paths)
        .header("./helpers/bindings_required.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
		.expect("Couldn't write bindings!");
}
