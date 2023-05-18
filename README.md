# experimental-component-model

use `wasm-tools` `cargo-component` and `wit-bindgen`.

```shell
wasm-tools print .\target\wasm32-wasi\debug\wit_bindgen_hello.wasm -o hello.wat
cargo component build
```

```shell
rustup default nightly
cargo expand
```

```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use wit_bindgen::generate;
#[allow(clippy::all)]
pub fn print(msg: &str) {
    #[allow(unused_imports)]
    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
    unsafe {
        let vec0 = msg;
        let ptr0 = vec0.as_ptr() as i32;
        let len0 = vec0.len() as i32;
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "$root_print"]
            fn wit_import(_: i32, _: i32);
        }
        wit_import(ptr0, len0);
    }
}
pub trait Host {
    fn run();
}
#[doc(hidden)]
pub unsafe fn call_run<T: Host>() {
    #[allow(unused_imports)]
    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
    T::run();
}
const _: &str = "default world host {\r\n  import print: func (msg: string)\r\n\r\n  export run: func()\r\n}";
struct MyHost;
impl Host for MyHost {
    fn run() {
        {
            ::std::io::_print(format_args!("hello world\n"));
        };
    }
}
const _: () = {
    #[doc(hidden)]
    #[export_name = "run"]
    #[allow(non_snake_case)]
    unsafe extern "C" fn __export_host_run() {
        call_run::<MyHost>()
    }
};
```
