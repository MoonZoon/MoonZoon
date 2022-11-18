use bindings::interface;

struct Component;

impl interface::Interface for Component {
    fn say_something() -> String {
        "Hello, World!".to_string()
    }
}

bindings::export!(Component);
