macro_rules! ffi_wrap {
    ($fname:ident, $target:ident) => {{
        let result: SigarResult<ffi::$target> = unsafe {
            let mut sigar_ptr: *mut ffi::sigar_t = std::ptr::null_mut();

            let res = ffi::sigar_open(&mut sigar_ptr);
            if res != SIGAR_CODE_OK {
                return Err(Error::new(sigar_ptr, res));
            }

            let mut info: ffi::$target = Default::default();

            let res = ffi::$fname(sigar_ptr, &mut info);
            if res != SIGAR_CODE_OK {
                return Err(Error::new(sigar_ptr, res));
            }

            let res = ffi::sigar_close(sigar_ptr);
            if res != SIGAR_CODE_OK {
                return Err(Error::from_str("failed to close sigar"));
            }

            Ok(info)
        };

        result
    }};
}

macro_rules! ffi_wrap_destroy {
    ($fnget:ident, $fndestroy:ident, $target:ident, $trans:ident) => {
        unsafe {
            let mut sigar_ptr: *mut ffi::sigar_t = std::ptr::null_mut();

            let res = ffi::sigar_open(&mut sigar_ptr);
            if res != SIGAR_CODE_OK {
                return Err(Error::new(sigar_ptr, res));
            }

            let mut info: ffi::$target = Default::default();

            let res = ffi::$fnget(sigar_ptr, &mut info);
            if res != SIGAR_CODE_OK {
                return Err(Error::new(sigar_ptr, res));
            }

            let entity = $trans(info);

            let res = ffi::$fndestroy(sigar_ptr, &mut info);
            if res != SIGAR_CODE_OK {
                return Err(Error::from_str("failed to destroy sigar"));
            }

            let res = ffi::sigar_close(sigar_ptr);
            if res != SIGAR_CODE_OK {
                return Err(Error::from_str("failed to close sigar"));
            }

            Ok(entity)
        }
    };
}

macro_rules! value_convert {
    ($struct:ident, $src:ident, $($field:ident), +) => {
        $struct {
            $(
            $field: $src.$field.into(),
            )+
        }
    };
    ($struct:ident, $src:expr, $($field:ident), +) => {
        $struct {
            $(
            $field: $src.$field.into(),
            )+
        }
    };
    ($struct:ident, $src:ident, $($field:ident), +, $(($ofield:ident : $oexpr:expr),) *) => {
        $struct {
            $(
            $field: $src.$field.into(),
            )+
            $(
            $ofield: $oexpr,
            )*
        }
    };
    ($struct:ident, $src:expr, $($field:ident), +, $($ofield:ident : $oexpr:expr,) *) => {
        $struct {
            $(
            $field: $src.$field.into(),
            )+
            $(
            $ofield: $oexpr,
            )*
        }
    };
}
