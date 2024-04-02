use std::sync::Mutex;

#[derive(Default)]
struct Store {
    ipc_channel: Mutex<Option<tauri::ipc::Channel>>,
}

#[tauri::command(rename_all = "snake_case")]
fn greet(name: &str) -> String {
    format!("Hello {name}!")
}

#[tauri::command(rename_all = "snake_case")]
fn send_ipc_channel(channel: tauri::ipc::Channel, store: tauri::State<Store>) {
    *store.ipc_channel.lock().unwrap() = Some(channel);
}

#[tauri::command(rename_all = "snake_case")]
fn greet_through_channel(name: &str, store: tauri::State<Store>) {
    store
        .ipc_channel
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(format!("Hello through channel {name}!"))
        .unwrap()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // https://github.com/tauri-apps/tauri/issues/8462
    #[cfg(target_os = "linux")]
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

    tauri::Builder::default()
        .manage(Store::default())
        .invoke_handler(tauri::generate_handler![
            greet,
            send_ipc_channel,
            greet_through_channel
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// @TODO test tauri-bindgen again once both Tauri and the bindgen are more mature

// use tauri_bindgen_host::ipc_router_wip::{BuilderExt, Router};

// tauri_bindgen_host::generate!({
//     path: "../greet.wit",
//     async: false,
//     tracing: true
// });

// #[derive(Clone, Copy)]
// struct GreetCtx;

// impl greet::Greet for GreetCtx {
//     fn greet(&self, name: String) -> String {
//         format!(
//             "Hello, {}! You've been greeted from code-generated Rust!",
//             name
//         )
//     }
// }

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
// pub fn run() {
//     // https://github.com/tauri-apps/tauri/issues/8462
//     #[cfg(target_os = "linux")]
//     std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

//     let mut router: Router<GreetCtx> = Router::new(GreetCtx {});
//     greet::add_to_router(&mut router, |ctx| ctx).unwrap();

//     tauri::Builder::default()
//         .ipc_router(router)
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }
