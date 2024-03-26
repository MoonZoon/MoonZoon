# Tauri TodoMVC
> MoonZoon example

_Note:_ Tested with **Tauri v1**.

---

### Start:

1. `cargo install tauri-cli`
2. `cargo tauri dev`

---

### Production build:

1. `cargo tauri build`
2. Installable bundles specific for the platform are at `target/release/bundle`

See related Tauri docs:
- https://tauri.app/v1/guides/distribution/publishing
- https://tauri.app/v1/guides/building/

Cross-platform compilation: https://tauri.app/v1/guides/building/cross-platform

Properties of an `msi` bundle on Windows:
- Size of `TodoMVC_0.1.0_x64_en-US.msi` is **2048 KB**.
- Size of `C:\Program Files\TodoMVC\TodoMVC.exe` is **3730 KB**.
- Process `TodoMVC.exe` uses ~**65.2 MB** RAM.

---

### Integration steps for a standard TodoMVC example to make this example:

See https://tauri.app/v1/guides/getting-started/setup/integrate/

1. Install Tauri CLI: `cargo install tauri-cli`
2. `cargo tauri init`
3. App name: `TodoMVC`
4. Window title: `TodoMVC`
5. Web assets relative path: `../frontend_dist`
6. Dev server url: `http://localhost:8080`
7. Frontend dev command: `makers mzoon start`
8. Frontend build command: `makers mzoon build -r -f`
9. Add `"src-tauri"` to `Cargo.toml` workspace members.
10. Change `tauri.bundle.identifier` in `src-tauri/tauri.conf.json` to "com.example.moonzoon.tauri-todomvc"`

The config is saved to `src-tauri/tauri.conf.json`, more info here https://tauri.app/v1/api/config/

How to generate custom icons: https://tauri.app/v1/guides/features/icons/


