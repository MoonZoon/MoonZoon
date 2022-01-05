use zoon::{*, println, routing::origin};

fn send_request() {
    Task::start(async {
        let url = origin() + "/_api/hello";
        let greeting = reqwest::get(url)
            .await.unwrap_throw()
            .text()
            .await.unwrap_throw();
        println!("Greeting: '{}'", greeting);
    });
}

fn root() -> impl Element {
    Column::new()
        .item(Button::new().label("Send request").on_press(send_request))
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
