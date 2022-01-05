use zoon::*;

fn send_request() {
    zoon::println!("TODO send request");
}

fn root() -> impl Element {
    Column::new()
        .item(Button::new().label("Send request").on_press(send_request))
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
