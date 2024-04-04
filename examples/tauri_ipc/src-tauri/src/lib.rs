use std::sync::Mutex;
use tauri::Manager;

#[derive(Default)]
struct Store {
    ipc_channel: Mutex<Option<tauri::ipc::Channel>>,
}

#[tauri::command(rename_all = "snake_case")]
fn show_window(window: tauri::Window) {
    window.show().unwrap();
}

#[tauri::command(rename_all = "snake_case")]
fn greet(name: &str) -> String {
    format!("Hello {name}! [from command]")
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
        .send(format!("Hello {name}! [from channel]"))
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
            show_window,
            greet,
            send_ipc_channel,
            greet_through_channel
        ])
        .setup(|app| {
            let greet_tom_menu_item =
                tauri::menu::MenuItem::new(app, "Greet Tom", true, None::<&str>)?;

            let quit_menu_item = tauri::menu::MenuItem::new(app, "Quit", true, None::<&str>)?;

            // Note: It's not possible to add a `MenuItem` directly to MacOS root menu (https://github.com/tauri-apps/tauri/issues/7981)
            // and you have to double-click `MenuItem`s in the root. So we always need to use `Submenu`s.
            // Also the first submenu is opened when the window title is clicked on MacOS so the first submenu title/text is basically ignored.
            let menu = tauri::menu::Menu::with_items(
                app,
                &[&tauri::menu::Submenu::with_items(
                    app,
                    "Tauri IPC",
                    true,
                    &[&greet_tom_menu_item, &quit_menu_item],
                )?],
            )?;

            app.on_menu_event(move |app_handle, tauri::menu::MenuEvent { id: menu_id }| {
                match menu_id {
                    id if id == greet_tom_menu_item.id() => {
                        app_handle.emit("greet", "Tom").unwrap()
                    }
                    id if id == quit_menu_item.id() => app_handle.exit(0),
                    _ => unreachable!("unhandled menu id"),
                }
            });
            app.set_menu(menu)?;
            Ok(())
        })
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
