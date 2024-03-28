# Tauri LocalSearch
> MoonZoon example

_Note:_ Tested with **Tauri v1**.

---

### Create a valid self-signed `localhost` certificate for dev server:

1. Download `mkcert`: https://github.com/FiloSottile/mkcert/releases
2. `mkcert -install`
3. `mkcert localhost`
4. Rename generated `localhost.pem` to `public.pem`.
5. Rename generated `localhost-key.pem` to `private.pem`.
6. Move `public.pem` and `private.pem` to `backend/private`.

---

### Start:

1. `cargo install tauri-cli`
2. `cargo tauri dev`

---

### Debug build:

1. `cargo tauri build --debug`
2. Executable is in `target/debug`

_Notes:_ 
- Only the Tauri app is built in debug mode, the app alone is still built with `mzoon build -r -f`.
- The debug mode enables the browser dev console / Inspect element feature.

---

### Production build:

1. `cargo tauri build`
2. Installable bundles specific for the platform are at `target/release/bundle`

See related Tauri docs:
- https://tauri.app/v1/guides/distribution/publishing
- https://tauri.app/v1/guides/building/

Cross-platform compilation: https://tauri.app/v1/guides/building/cross-platform

Properties of an `msi` bundle on Windows:
- Size of `LocalSearch_0.1.0_x64_en-US.msi` is **2112 KB**.
- Size of `C:\Program Files\LocalSearch\LocalSearch.exe` is **3815 KB**.
- Process `LocalSearch.exe` uses ~**71.8 MB** RAM at start and ~**1030 MB** when 10M companies are generated. 

_Notes_:
- Wasm memory cannot be freed.
- 4 GB should be maximum allocated memory.
- Tauri V1 frontend <-> backend communication uses JSON String, V2 communication should be faster.

---

### Integration steps for a standard LocalSearch example to make this example:

See https://tauri.app/v1/guides/getting-started/setup/integrate/

1. Install Tauri CLI: `cargo install tauri-cli`
2. `cargo tauri init`
3. App name: `LocalSearch`
4. Window title: `Local Search`
5. Web assets relative path: `../frontend_dist`
6. Dev server url: `https://localhost:8443`
7. Frontend dev command: `makers mzoon start`
8. Frontend build command: `makers mzoon build -r -f`
9. Add `"src-tauri"` to `Cargo.toml` workspace members.
10. Change `tauri.bundle.identifier` in `src-tauri/tauri.conf.json` to `"com.example.moonzoon.tauri-local-search"`

The config is saved to `src-tauri/tauri.conf.json`, more info here https://tauri.app/v1/api/config/

How to generate custom icons: https://tauri.app/v1/guides/features/icons/
