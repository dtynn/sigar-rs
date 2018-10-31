ffi:
	rm -f src/ffi.rs
	bindgen --with-derive-default --distrust-clang-mangling include/wrapper.h -o src/ffi.rs -- -I sigar/include
