use zoon::*;

static CHANNEL_MESSAGE: Lazy<Mutable<Option<String>>> = lazy::default();
static GREET_EVENT_NAMES: Lazy<MutableVec<String>> = lazy::default();

fn main() {
    start_app("app", root);
    Task::start(async {
        // https://github.com/tauri-apps/tauri/issues/5170
        Timer::sleep(100).await;
        tauri_bridge::show_window().await;
    });
    Task::start(async {
        tauri_bridge::listen_greet_events(|name| GREET_EVENT_NAMES.lock_mut().push_cloned(name))
            .await;
    });
    Task::start(async {
        tauri_bridge::send_ipc_channel(|message| CHANNEL_MESSAGE.set(Some(message))).await;
        tauri_bridge::greet_through_channel("Jane").await;
    });
}

fn root() -> impl Element {
    El::new()
        .s(Height::fill())
        .s(Background::new().color(color!("DarkSlateBlue", 0.7)))
        .s(Font::new().color(color!("Lavender")))
        .child(
            Column::new()
                .s(Align::center())
                .item(
                    El::new()
                        .child_signal(signal::from_future(Box::pin(tauri_bridge::greet("John")))),
                )
                .item(El::new().child_signal(CHANNEL_MESSAGE.signal_cloned()))
                .items_signal_vec(
                    GREET_EVENT_NAMES
                        .signal_vec_cloned()
                        .map(|name| El::new().child(format!("Hello {name}! [from event]"))),
                ),
        )
}

mod tauri_bridge {
    use super::*;

    pub async fn show_window() {
        tauri_glue::show_window().await
    }

    pub async fn greet(name: &str) -> String {
        tauri_glue::greet(name).await.as_string().unwrap_throw()
    }

    pub async fn send_ipc_channel(on_message: impl FnMut(String) + 'static) {
        tauri_glue::send_ipc_channel(Closure::new(on_message).into_js_value()).await
    }

    pub async fn greet_through_channel(name: &str) {
        tauri_glue::greet_through_channel(name).await
    }

    pub async fn listen_greet_events(on_event: impl FnMut(String) + 'static) {
        tauri_glue::listen_greet_events(Closure::new(on_event).into_js_value()).await
    }

    mod tauri_glue {
        use super::*;
        #[wasm_bindgen(module = "/js/tauri_glue.js")]
        extern "C" {
            pub async fn show_window();

            pub async fn greet(name: &str) -> JsValue;

            pub async fn send_ipc_channel(on_message: JsValue);

            pub async fn greet_through_channel(name: &str);

            pub async fn listen_greet_events(on_event: JsValue);
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
