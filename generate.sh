#/bin/bash
# generate the basic ffi bindings
bindgen --no-prepend-enum-name --no-layout-tests bgfx/include/bgfx/c99/bgfx.h -- -Ibx/include > src/ffi.rs
