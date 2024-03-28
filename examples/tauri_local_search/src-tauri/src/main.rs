// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let mut context = tauri::generate_context!();
    let first_window_config = {
        if context.config_mut().tauri.windows.is_empty() {
            <_>::default()
        } else {
            context.config_mut().tauri.windows.remove(0)
        }
    };
    tauri::Builder::default()
        .setup(|app| {
            tauri::WindowBuilder::from_config(app, first_window_config)
                .on_web_resource_request(|req, resp| {
                    if req.uri().starts_with("tauri://") {
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
