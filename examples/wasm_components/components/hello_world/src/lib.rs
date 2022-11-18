wit_bindgen_guest_rust::generate!("say.wit");

struct Say;

export_say!(Say);

impl say::Say for Say {
    fn say_something() -> String {
        // console::log("I'm saying!");
        "Hello, World!".to_string()
    }
}
