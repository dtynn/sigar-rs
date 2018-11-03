use sigar_sys;
use std::{error::Error as stdError, fmt};

pub(crate) use sigar_sys::SIGAR_CODE_OK;

/// Type alias for [`Result<T, Error>`]
pub type SigarResult<T = ()> = Result<T, Error>;

/// Wraps inner reason from sigar::sigar_strerror
#[derive(Debug)]
pub enum Error {
    NotImplementd,
    ENOENT,
    EACCES,
    ENXIO,
    CString(String),
    Others(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(self.description())
    }
}

impl stdError for Error {}

impl Error {
    fn description(&self) -> &str {
        match self {
            Error::NotImplementd => "not implemented on current os",
            Error::ENOENT => "no such file or directory",
            Error::EACCES => "permission denied",
            Error::ENXIO => "no such device or address",
            Error::CString(ref reason) => reason,
            Error::Others(ref reason) => reason,
        }
    }

    pub(crate) fn new(t: *mut sigar_sys::sigar_t, code: i32) -> Self {
        if let Some(e) = match_code(code) {
            return e;
        }

        Error::Others(sigar_sys::error_string(t, code.into()))
    }

    pub(crate) fn from_str(s: &str) -> Self {
        Error::Others(s.to_string())
    }

    pub(crate) fn from_string(s: String) -> Self {
        Error::Others(s)
    }

    /// Returns the error reason
    pub fn reason(&self) -> String {
        self.description().to_string()
    }
}

#[cfg(windows)]
fn match_code(code: i32) -> Option<Error> {
    match code as u32 {
        sigar_sys::SIGAR_ENOTIMPL => Some(Error::NotImplementd),
        0x2 => Some(Error::ENOENT),
        0x5 => Some(Error::EACCES),
        0x77 => Some(Error::ENXIO),
        _ => None,
    }
}

#[cfg(unix)]
fn match_code(code: i32) -> Option<Error> {
    match code as u32 {
        sigar_sys::SIGAR_ENOTIMPL => Some(Error::NotImplementd),
        2 => Some(Error::ENOENT),
        13 => Some(Error::EACCES),
        6 => Some(Error::ENXIO),
        _ => None,
    }
}
