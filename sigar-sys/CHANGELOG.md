### 1.0.3

- disable functions using `stdin` / `stdout` / `stderr` to avoid **undefined reference to `__imp___acrt_iob_func' error** when compiling on windows with gnu toolchain.

  related links: 

  [https://github.com/rust-lang/rust/issues/47048](https://github.com/rust-lang/rust/issues/47048)

  [https://github.com/rust-lang/rust/pull/51989](https://github.com/rust-lang/rust/pull/51989)

  [https://github.com/Alexpux/MINGW-packages/issues/3237](https://github.com/Alexpux/MINGW-packages/issues/3237)