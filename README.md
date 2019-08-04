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
cargo web start --release
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
