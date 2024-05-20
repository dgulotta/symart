symart
======

A program that creates abstract art.

Running the program
===================

Currently, the easiest way to use symart is through the WebAssembly interface.
There is an online demo [here](https://dgulotta.github.io/symart_demo/).
It can also be run locally:
```sh
cd symart_wasm
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen ../target/wasm32-unknown-unknown/release/symart_wasm.wasm --out-dir pkg --target no-modules --no-typescript
cd pkg
python3 -m http.server
```

See also
========
[dgulotta/paintlines](https://github.com/dgulotta/paintlines), a similar program written in C++


License
=======
The `symart_base` and `symart_wasm` crates are dual licensed under the
[MIT License](LICENSE-MIT) and the
[Apache License, Version 2.0](LICENSE-APACHE).  The `symart_designs` crate is
licensed under the [GNU GPL, Version 2](LICENSE-GPL2) or later.
