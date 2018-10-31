use sigar_sys;
use std::{error::Error as stdError, fmt};

pub(crate) use sigar_sys::SIGAR_CODE_OK;

/// Type alias for [`Result<T, Error>`]
pub type SigarResult<T> = Result<T, Error>;

/// Wraps inner reason from sigar::sigar_strerror
#[derive(Debug)]
pub struct Error(String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.0.as_str())
    }
}

impl stdError for Error {}

impl Error {
    pub(crate) fn new(t: *mut sigar_sys::sigar_t, code: i32) -> Self {
        Error(sigar_sys::error_string(t, code.into()))
    }

    pub(crate) fn from_str(s: &str) -> Self {
        Error(s.to_string())
    }

    /// Returns the error reason
    pub fn reason(&self) -> String {
        self.0.clone()
    }
}
