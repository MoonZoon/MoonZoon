#[tauri::command(rename_all = "snake_case")]
fn greet(name: String) -> String {
    format!("Hello {name}!")
}

// window.__TAURI__.core.invoke('greet', { name: 'John' }).then((message) => console.log(message))
// @TODO fn exchange_channels(channel/jschannelid + webview) -> Channel
// @TODO document adding `"withGlobalTauri": true,` to `tauri.conf.json`

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // https://github.com/tauri-apps/tauri/issues/8462
    #[cfg(target_os = "linux")]
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
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
