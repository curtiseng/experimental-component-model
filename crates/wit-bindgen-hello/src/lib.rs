use wit_bindgen::generate;
generate!("host");

struct MyHost;

impl Host for MyHost {
    fn run() {
        println!("hello world");
    }
}

export_host!(MyHost);