use super::result::{Error, SigarResult};
use sigar_sys::{sigar_close, sigar_open, sigar_t, SIGAR_CODE_OK};
use std::{self, os::raw::c_char};

#[allow(unused_imports)]
pub(crate) use super::integer::CastTo;
pub(crate) use std::slice::from_raw_parts;

pub(crate) fn chars_to_bytes(chars: &[c_char]) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::with_capacity(chars.len());

    for i in chars {
        bytes.push(*i as u8);
    }

    bytes
}

pub(crate) struct SigarPtr {
    pub(crate) ptr: *mut sigar_t,
}

impl SigarPtr {
    pub(crate) fn new() -> SigarResult<Self> {
        let mut ptr: *mut sigar_t = std::ptr::null_mut();

        let res = unsafe { sigar_open(&mut ptr) };
        if res != SIGAR_CODE_OK {
            return Err(Error::new(ptr, res));
        }

        Ok(SigarPtr { ptr })
    }
}

impl Drop for SigarPtr {
    fn drop(&mut self) {
        unsafe { sigar_close(self.ptr) };
    }
}

/// Strips ending zeros in the bytes
pub fn strip_bytes(bytes: &[u8]) -> &[u8] {
    let mut len = bytes.len();

    while len > 0 {
        if bytes[len - 1] != 0 {
            break;
        }

        len -= 1;
    }

    &bytes[..len]
}

pub(crate) fn u32_reverse(src: u32) -> u32 {
    const MASK_U8: u32 = !(0u32) >> 24;
    (src & MASK_U8) << 24
        | (src >> 8 & MASK_U8) << 16
        | (src >> 16 & MASK_U8) << 8
        | (src >> 24 & MASK_U8)
}
