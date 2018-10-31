### sigar-sys
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