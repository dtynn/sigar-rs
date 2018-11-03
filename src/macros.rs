macro_rules! ffi_wrap {
    ($fname:ident, $target:ident) => {{
        let result: SigarResult<$target> = unsafe {
            let sigar_ptr = SigarPtr::new()?;

            let mut info: $target = Default::default();

            let res = $fname(sigar_ptr.ptr, &mut info);
            if res != SIGAR_CODE_OK {
                return Err(Error::new(sigar_ptr.ptr, res));
            }

            Ok(info)
        };

        result
    }};

    ($fname:ident, ($($arg:expr), +), $target:ident) => {{
        let result: SigarResult<$target> = unsafe {
            let sigar_ptr = SigarPtr::new()?;

            let mut info: $target = Default::default();

            let res = $fname(
                sigar_ptr.ptr,
                $(
                    $arg,
                )+
                &mut info,
            );
            if res != SIGAR_CODE_OK {
                return Err(Error::new(sigar_ptr.ptr, res));
            }

            Ok(info)
        };

        result
    }};

    ($func:tt, $target:ident) => {{
        let result: SigarResult<$target> = unsafe {
            let sigar_ptr = SigarPtr::new()?;

            let mut info: $target = Default::default();

            let res = $func(
                sigar_ptr.ptr,
                &mut info,
            );
            if res != SIGAR_CODE_OK {
                return Err(Error::new(sigar_ptr.ptr, res));
            }

            Ok(info)
        };

        result
    }};
}

macro_rules! ffi_wrap_destroy {
    ($fnget:ident, $fndestroy:ident, $target:ident, $trans:tt) => {
        unsafe {
            let sigar_ptr = SigarPtr::new()?;

            let mut info: $target = Default::default();

            let res = $fnget(sigar_ptr.ptr, &mut info);
            if res != SIGAR_CODE_OK {
                return Err(Error::new(sigar_ptr.ptr, res));
            }

            let entity = $trans(&info);

            let res = $fndestroy(sigar_ptr.ptr, &mut info);
            if res != SIGAR_CODE_OK {
                return Err(Error::from_str("failed to destroy sigar"));
            }

            Ok(entity)
        }
    };

    ($fnget:tt, $fndestroy:ident, $target:ident, $trans:tt) => {
        unsafe {
            let sigar_ptr = SigarPtr::new()?;

            let mut info: $target = Default::default();

            let res = $fnget(sigar_ptr.ptr, &mut info);
            if res != SIGAR_CODE_OK {
                return Err(Error::new(sigar_ptr.ptr, res));
            }

            let entity = $trans(&info);

            let res = $fndestroy(sigar_ptr.ptr, &mut info);
            if res != SIGAR_CODE_OK {
                return Err(Error::from_str("failed to destroy sigar"));
            }

            Ok(entity)
        }
    };
}

macro_rules! ffi_wrap_sigar_t {
    ($func:tt) => {{
        let sigar_ptr = SigarPtr::new()?;

        let ret = $func(sigar_ptr.ptr);

        Ok(ret)
    }};
}

macro_rules! value_convert {
    ($struct:ident, $src:expr, $($field:ident), *) => {
        $struct {
            $(
            $field: $src.$field.cast_to(),
            )*
        }
    };
    ($struct:ident, $src:ident, $($field:ident), *, $(($ofield:ident : $oexpr:expr),) *) => {
        $struct {
            $(
            $field: $src.$field.cast_to(),
            )*
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
