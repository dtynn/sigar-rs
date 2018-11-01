macro_rules! ffi_wrap {
    ($fname:ident, $target:ident) => {{
        let result: SigarResult<$target> = unsafe {
            let mut sigar_ptr: *mut sigar_t = std::ptr::null_mut();

            let res = sigar_open(&mut sigar_ptr);
            if res != SIGAR_CODE_OK {
                return Err(Error::new(sigar_ptr, res));
            }

            let mut info: $target = Default::default();

            let res = $fname(sigar_ptr, &mut info);
            if res != SIGAR_CODE_OK {
                return Err(Error::new(sigar_ptr, res));
            }

            let res = sigar_close(sigar_ptr);
            if res != SIGAR_CODE_OK {
                return Err(Error::from_str("failed to close sigar"));
            }

            Ok(info)
        };

        result
    }};

    ($fname:ident, ($($arg:expr), +), $target:ident) => {{
        let result: SigarResult<$target> = unsafe {
            let mut sigar_ptr: *mut sigar_t = std::ptr::null_mut();

            let res = sigar_open(&mut sigar_ptr);
            if res != SIGAR_CODE_OK {
                return Err(Error::new(sigar_ptr, res));
            }

            let mut info: $target = Default::default();

            let res = $fname(
                sigar_ptr,
                $(
                    $arg,
                )+
                &mut info,
            );
            if res != SIGAR_CODE_OK {
                return Err(Error::new(sigar_ptr, res));
            }

            let res = sigar_close(sigar_ptr);
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
            let mut sigar_ptr: *mut sigar_t = std::ptr::null_mut();

            let res = sigar_open(&mut sigar_ptr);
            if res != SIGAR_CODE_OK {
                return Err(Error::new(sigar_ptr, res));
            }

            let mut info: $target = Default::default();

            let res = $fnget(sigar_ptr, &mut info);
            if res != SIGAR_CODE_OK {
                return Err(Error::new(sigar_ptr, res));
            }

            let entity = $trans(&info);

            let res = $fndestroy(sigar_ptr, &mut info);
            if res != SIGAR_CODE_OK {
                return Err(Error::from_str("failed to destroy sigar"));
            }

            let res = sigar_close(sigar_ptr);
            if res != SIGAR_CODE_OK {
                return Err(Error::from_str("failed to close sigar"));
            }

            Ok(entity)
        }
    };

    ($fnget:ident, $fndestroy:ident, $target:ident, $trans:tt) => {
        unsafe {
            let mut sigar_ptr: *mut sigar_t = std::ptr::null_mut();

            let res = sigar_open(&mut sigar_ptr);
            if res != SIGAR_CODE_OK {
                return Err(Error::new(sigar_ptr, res));
            }

            let mut info: $target = Default::default();

            let res = $fnget(sigar_ptr, &mut info);
            if res != SIGAR_CODE_OK {
                return Err(Error::new(sigar_ptr, res));
            }

            let entity = $trans(&info);

            let res = $fndestroy(sigar_ptr, &mut info);
            if res != SIGAR_CODE_OK {
                return Err(Error::from_str("failed to destroy sigar"));
            }

            let res = sigar_close(sigar_ptr);
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

macro_rules! ffi_extract_list {
    ($raw:ident, $trans:ident) => {{
        let mut list = Vec::with_capacity($raw.number as usize);

        let data = from_raw_parts($raw.data, $raw.number as usize);
        for one in data {
            list.push($trans(one));
        }

        list
    }};

    ($raw:ident, $trans:path) => {{
        let mut list = Vec::with_capacity($raw.number as usize);

        let data = from_raw_parts($raw.data, $raw.number as usize);
        for one in data {
            list.push($trans(one));
        }

        list
    }};

    ($raw:ident, $trans:tt) => {{
        let mut list = Vec::with_capacity($raw.number as usize);

        let data = from_raw_parts($raw.data, $raw.number as usize);
        for one in data {
            list.push($trans(one));
        }

        list
    }};
}
