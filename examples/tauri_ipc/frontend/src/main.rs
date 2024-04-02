use zoon::*;

static MESSAGE: Lazy<Mutable<Option<String>>> = lazy::default();

fn main() {
    Task::start(async {
        command::send_ipc_channel(|message| MESSAGE.set(Some(message))).await;
        command::greet_through_channel("Jane").await;
    });
    start_app("app", root);
}

fn root() -> impl Element {
    Column::new()
        .item(El::new().child_signal(signal::from_future(Box::pin(command::greet("John")))))
        .item(El::new().child_signal(MESSAGE.signal_cloned()))
}

mod command {
    use super::*;

    pub async fn greet(name: &str) -> String {
        js_bridge::greet(name).await.as_string().unwrap_throw()
    }

    pub async fn send_ipc_channel(on_message: impl FnMut(String) + 'static) {
        js_bridge::send_ipc_channel(Closure::new(on_message).into_js_value()).await
    }

    pub async fn greet_through_channel(name: &str) {
        js_bridge::greet_through_channel(name).await
    }

    mod js_bridge {
        use super::*;
        #[wasm_bindgen(module = "/js/commands.js")]
        extern "C" {
            pub async fn greet(name: &str) -> JsValue;

            pub async fn send_ipc_channel(on_message: JsValue);

            pub async fn greet_through_channel(name: &str);
        }
    }
}

// @TODO Test `tauri-sys` crate later once Tauri is more mature.
// @TODO Resolve `url` dependency conflict and ideally get rid of the `url` dependency somehow.
// @TODO When it works, update docs - the code + the need to install `esbuild`.
//
// fn root() -> impl Element {
//     Column::new()
//         .item("Tauri IPC")
//         .item_signal(signal::from_future(Box::pin(tauri_sys::tauri::invoke("greet", "Jonas"))))
// }

// @TODO Test `tauri-bindgen` crate again once both Tauri and the bindgen are more mature.
//
// Error: `Fetch API cannot load ipc://localhost/greet/greet. URL scheme "ipc" is not supported.``
// https://github.com/tauri-apps/tauri-bindgen/issues/147
//
// tauri_bindgen_guest_rust::generate!({
//     path: "../greet.wit"
// });
//
// fn root() -> impl Element {
//     Column::new()
//         .item("Tauri IPC")
//         .item_signal(signal::from_future(Box::pin(greet::greet("Jonas"))))
// }
