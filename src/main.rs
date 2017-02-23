#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate libc;

use libc::c_char;
use std::ffi::CStr;

fn main() {
    let av_version = unsafe { CStr::from_ptr(av_version_info()) };

    println!("Yer AV Version from C library: {}", av_version.to_str().unwrap());
}
