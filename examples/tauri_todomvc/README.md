# Tauri TodoMVC
> MoonZoon example

Start:

1. `cargo install tauri-cli`
2. `cargo tauri dev`

---

Integration steps for a standard TodoMVC example to make this example:

See https://tauri.app/v1/guides/getting-started/setup/integrate/

1. Install Tauri CLI: `cargo install tauri-cli`
2. `cargo tauri init`
3. App name: `TodoMVC`
4. Window title: `TodoMVC`
5. Web assets relative path from `<current dir>/src-tauri/tauri.conf.json`: `../../frontend_dist`
6. Dev server url: `http://localhost:8080`
7. Frontend dev command: `makers mzoon start`
8. Frontend build command: `makers mzoon build -r -f`

The config is saved to `src-tauri/tauri.conf.json`, more info here https://tauri.app/v1/api/config/

You can generate custom icons: https://tauri.app/v1/guides/features/icons/


