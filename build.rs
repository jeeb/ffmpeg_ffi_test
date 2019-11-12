extern crate bindgen;
extern crate pkg_config;

use std::collections::HashSet;
use std::env;
use std::path::PathBuf;

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

fn main() {
    let used_libraries = ["libavcodec", "libavformat", "libavutil"];
    let mut include_paths = Vec::new();
    let ignored_macros = IgnoreMacros(
        vec![
            "FP_INFINITE".into(),
            "FP_NAN".into(),
            "FP_NORMAL".into(),
            "FP_SUBNORMAL".into(),
            "FP_ZERO".into(),
            "IPPORT_RESERVED".into(),
        ]
        .into_iter()
        .collect(),
    );


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
        .clang_args(include_paths)
        .parse_callbacks(Box::new(ignored_macros))
        .header("./helpers/bindings_required.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
		.expect("Couldn't write bindings!");
}
