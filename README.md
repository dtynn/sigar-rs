## sigar-rs
[![Crates.io](https://img.shields.io/crates/v/sigar-rs.svg)](https://crates.io/crates/sigar-rs)

[![Docs](https://docs.rs/sigar-rs/badge.svg)](https://docs.rs/sigar-rs)

[sigar](https://github.com/AlexYaruki/sigar) is a cross-platform system monitoring lib.

this crate is a high-level wrapper on top of [sigar-sys](https://github.com/dtynn/sigar-rs/tree/master/sigar-sys).





### Usage

Add this to your `Cargo.toml`
```toml
[dependencies]
sigar-rs = "0.2"
```

and this to your crate root:

```rust
extern crate sigar_rs;
```

### Examples

see [examples](https://github.com/dtynn/sigar-rs/tree/master/examples).



### Cross Platform

some of the functions are not implemented on all platforms.
the  `Error::NotImplementd` could be used to ident the missing methods.

### Contributing

PRs and issues are welcomed.
