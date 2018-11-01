use super::result::{Error, SigarResult};
use sigar_sys::{sigar_close, sigar_open, sigar_t, SIGAR_CODE_OK};
use std::{self, os::raw::c_char};

pub(crate) fn must_chars_to_string(chars: &[c_char]) -> String {
    let mut bytes: Vec<u8> = Vec::with_capacity(chars.len());

    for i in chars {
        if *i == 0 {
            break;
        }

        bytes.push(*i as u8);
    }

    String::from_utf8(bytes).expect("invalid utf8 string")
}

pub(crate) use std::slice::from_raw_parts;

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
