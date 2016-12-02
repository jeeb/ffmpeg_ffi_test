extern crate libc;

use libc::c_char;
use std::ffi::CStr;

#[link(name = "avutil")]
extern "C" {
    fn av_version_info() -> *const c_char;
}

fn main() {
    let av_version = unsafe { CStr::from_ptr(av_version_info()) };

    println!("Yer AV Version from C library: {}", av_version.to_str().unwrap());
}
