# Tauri LocalSearch
> MoonZoon example

---

**WARNING**: This example currently does NOT work on Linux when run inside Tauri. 

WebKitGTK hasn't reenabled `SharedArrayBuffer` support yet so it's not possible to use fast Zoon multithreading demonstrated in this example.

More info:
- https://webkitgtk.org/2018/01/10/webkitgtk2.18.5-released.html
- https://github.com/tauri-apps/tauri/issues/1522
- https://github.com/tauri-apps/tauri/discussions/6269

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

1. `cargo install tauri-cli@=2.0.0-beta.11`
2. `cargo tauri dev`

Troubleshooting:
- In case of Tauri compilation errors, install system dependencies: https://beta.tauri.app/guides/prerequisites/

- Examples of possible Tauri runtime errors in terminal of VSCode installed from Linux Snap package manager:
    ```
    Failed to load module "colorreload-gtk-module"

    /usr/lib/x86_64-linux-gnu/webkit2gtk-4.1/WebKitNetworkProcess: symbol lookup error: /snap/core20/current/lib/x86_64-linux-gnu/libpthread.so.0: undefined symbol: __libc_pthread_init, version GLIBC_PRIVATE
    ```
    Fix it by installing VSCode directly from official `.deb` bundle or try to unset multiple env variables - more info in https://stackoverflow.com/questions/75921414/java-symbol-lookup-error-snap-core20-current-lib-x86-64-linux-gnu-libpthread

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
2. Runnable executable is in `target/release`
3. Installable bundles specific for the platform are in `target/release/bundle`

Properties of an `msi` bundle on Windows:
- Size of `LocalSearch_0.1.0_x64_en-US.msi` is **2112 KB**.
- Size of `C:\Program Files\LocalSearch\LocalSearch.exe` is **3815 KB**.
- Process `LocalSearch.exe` uses ~**71.8 MB** RAM at start and ~**1030 MB** when 10M companies are generated. 

_Notes_:
- Wasm memory cannot be freed.
- 4 GB should be the frontend memory allocation limit.

---

### Integration steps for a standard LocalSearch example to make this example:

1. Install Tauri CLI: `cargo install tauri-cli@=2.0.0-beta.11`
2. `cargo tauri init`
3. App name: `LocalSearch`
4. Window title: `Local Search`
5. Web assets relative path: `../frontend_dist`
6. Dev server url (HTTPS is one of the requirements to enable `SharedArrayBuffer`): `https://localhost:8443`
7. Frontend dev command: `makers mzoon start`
8. Frontend build command: `makers mzoon build -r -f`
9. Add `"src-tauri"` to `Cargo.toml` workspace members.
10. Change `identifier` in `src-tauri/tauri.conf.json` to `"com.example.moonzoon.tauri-local-search"`
11. Set env var `WEBKIT_DISABLE_DMABUF_RENDERER=1` in `src-tauri/lib.rs` because WebKitGTK (2.42) is not compatible with NVIDIA drivers on Linux.
12. Set headers in `src-tauri/lib.rs` for frontend files served by Tauri in prod build to enable `SharedArrayBuffer`.
