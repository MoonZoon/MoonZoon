// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // https://github.com/tauri-apps/tauri/issues/8462
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

    let mut context = tauri::generate_context!();
    let first_window_config = {
        if context.config_mut().app.windows.is_empty() {
            <_>::default()
        } else {
            context.config_mut().app.windows.remove(0)
        }
    };
    tauri::Builder::default()
        .setup(move |app| {
            tauri::WebviewWindowBuilder::from_config(app, &first_window_config)?
                .on_web_resource_request(|req, resp| {
                    if req.uri().scheme().map(|scheme| scheme.as_str() == "tauri").unwrap_or_default() {
                        resp.headers_mut().insert(
                            "Cross-Origin-Opener-Policy",
                            "same-origin".try_into().unwrap(),
                        );
                        resp.headers_mut().insert(
                            "Cross-Origin-Embedder-Policy",
                            "require-corp".try_into().unwrap(),
                        );
                    }
                })
                .build()?;
            Ok(())
        })
        .run(context)
        .expect("error while running tauri application");
}


// /usr/lib/x86_64-linux-gnu/webkit2gtk-4.1/WebKitNetworkProcess: symbol lookup error: /snap/core20/current/lib/x86_64-linux-gnu/libpthread.so.0: undefined symbol: __libc_pthread_init, version GLIBC_PRIVATE
