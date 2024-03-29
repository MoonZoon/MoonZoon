// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  // https://github.com/eclipse-platform/eclipse.platform.swt/issues/843
  // std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");

  // https://github.com/tauri-apps/tauri/issues/8462
  std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
  
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
