#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // https://github.com/tauri-apps/tauri/issues/8462
    #[cfg(target_os = "linux")]
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
                    if req
                        .uri()
                        .scheme()
                        .map(|scheme| scheme.as_str() == "tauri")
                        .unwrap_or_default()
                    {
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
