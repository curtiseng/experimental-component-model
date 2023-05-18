struct Component;

impl bindings::Component for Component {
    /// Say hello!
    fn hello_world() -> String {
        "Hello, World!".to_string()
    }
}

bindings::export!(Component);
