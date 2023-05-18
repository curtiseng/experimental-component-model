# experimental-component-model

use `wasm-tools` `cargo-component` and `wit-bindgen`.

```shell
wasm-tools print .\target\wasm32-wasi\debug\wit_bindgen_hello.wasm -o hello.wat
cargo component build
```