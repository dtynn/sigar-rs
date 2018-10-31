//! Provides low level binding to [`AlexYaruki/sigar`], which is a independent fork version of official [`hyperic/sigar`]
//!
//! [`AlexYaruki/sigar`]: https://github.com/AlexYaruki/sigar
//! [`hyperic/sigar`]: https://github.com/hyperic/sigar

extern crate libc;

use libc::c_int;
use std::ffi::CStr;
use std::str;

#[allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
mod ffi;

/// ffi automatically generated by rust-bindgen
pub use ffi::*;

/// Returns an error string from given code
pub fn error_string(sigar: *mut ffi::sigar_t, code: c_int) -> String {
    unsafe {
        let ptr = ffi::sigar_strerror(sigar, code);
        let bytes = CStr::from_ptr(ptr).to_bytes();
        str::from_utf8(bytes)
            .ok()
            .expect("Invalid UTF8 string")
            .to_string()
    }
}

/// Represents OK returned from functions
pub const SIGAR_CODE_OK: i32 = ffi::SIGAR_OK as i32;
