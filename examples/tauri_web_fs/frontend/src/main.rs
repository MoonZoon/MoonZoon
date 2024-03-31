use zoon::*;

fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    Column::new()
        .item("Tauri Web FS")
}



// @TODO Test `tauri-sys` crate later once Tauri is more mature.
// @TODO Resolve `url` dependency conflict and ideally get rid of the `url` dependency somehow.
// @TODO When it works, update docs - the code + the need to install `esbuild`.
//
// fn root() -> impl Element {
//     Column::new()
//         .item("Tauri Web FS")
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
//         .item("Tauri Web FS")
//         .item_signal(signal::from_future(Box::pin(greet::greet("Jonas"))))
// }
