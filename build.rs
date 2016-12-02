use std::env;
use std::path::PathBuf;

fn main() {
    let mut ffmpeg_path = PathBuf::from(env::var("FFMPEG_PREFIX").unwrap());
    ffmpeg_path.push("lib");

    println!("cargo:rustc-link-search=native={}", ffmpeg_path.display());
}