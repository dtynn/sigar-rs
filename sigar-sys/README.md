### sigar-sys

[![Crates.io](https://img.shields.io/crates/v/sigar-sys.svg)](https://crates.io/crates/sigar-sys)

[![Docs](https://docs.rs/sigar-sys/badge.svg)](https://docs.rs/sigar-sys)



low level binding to [AlexYaruki/sigar](https://github.com/AlexYaruki/sigar), which is a independent fork version of official [hyperic/sigar](https://github.com/hyperic/sigar).


#### Usage
Add this to your `Cargo.toml`
```toml
[dependencies]
sigar-sys = "1"
```

and this to your crate root:

```rust
extern crate sigar_sys;
```



#### Notes

- As a `-sys` crate, this provides only the binding to sigar.

  The sigar documentation itself is at [SourceForge/sigar](https://sourceforge.net/projects/sigar/).

- The build script for this crate always makes a static linking.

- This crate has been tested to work on Windows(with msvc), Linux and macOS.

  Other platforms or envs may work, and PRs to support them are welcome.



#### Example

```rust
extern crate sigar_sys;

use sigar_sys as ffi;
use std::slice::from_raw_parts;

fn main() {
    let usage = unsafe { cpuusage().unwrap() };
    println!("cpu usage: {:?}", usage);

    let pids = unsafe { pidlist().unwrap() };
    println!("pids: {:?}", pids);
}

unsafe fn cpuusage() -> Result<ffi::sigar_cpu_t, String> {
    let mut sigar_ptr: *mut ffi::sigar_t = std::ptr::null_mut();

    let res = ffi::sigar_open(&mut sigar_ptr);
    if res != ffi::SIGAR_CODE_OK {
        return Err(ffi::error_string(sigar_ptr, res));
    }

    let mut info: ffi::sigar_cpu_t = Default::default();
    let res = ffi::sigar_cpu_get(sigar_ptr, &mut info);
    if res != ffi::SIGAR_CODE_OK {
        return Err(ffi::error_string(sigar_ptr, res));
    }

    let res = ffi::sigar_close(sigar_ptr);
    if res != ffi::SIGAR_CODE_OK {
        return Err("failed to close sigar_t".to_string());
    }

    Ok(info)
}

unsafe fn pidlist() -> Result<Vec<ffi::sigar_pid_t>, String> {
    let mut sigar_ptr: *mut ffi::sigar_t = std::ptr::null_mut();

    let res = ffi::sigar_open(&mut sigar_ptr);
    if res != ffi::SIGAR_CODE_OK {
        return Err(ffi::error_string(sigar_ptr, res));
    }

    let mut raw: ffi::sigar_proc_list_t = Default::default();
    let res = ffi::sigar_proc_list_get(sigar_ptr, &mut raw);
    if res != ffi::SIGAR_CODE_OK {
        return Err(ffi::error_string(sigar_ptr, res));
    }

    let infos = from_raw_parts(raw.data, raw.number as usize)
        .to_vec()
        .clone();

    let res = ffi::sigar_proc_list_destroy(sigar_ptr, &mut raw);
    if res != ffi::SIGAR_CODE_OK {
        return Err("failed to destroy sigar_t".to_string());
    }

    let res = ffi::sigar_close(sigar_ptr);
    if res != ffi::SIGAR_CODE_OK {
        return Err("failed to close sigar_t".to_string());
    }

    Ok(infos)
}

```



**to run the example code on windows, we should link 3 more libs: `ws2_32` `netapi32`, `version`, maybe in the build scripts**