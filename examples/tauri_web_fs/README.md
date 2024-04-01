# Tauri Web FS
> MoonZoon example

---

### Start:

1. `cargo install tauri-cli@=2.0.0-beta.11`
2. `cargo tauri dev`

Troubleshooting:
- In case of Tauri compilation errors, install system dependencies: https://beta.tauri.app/guides/prerequisites/

- Possible Tauri runtime errors in terminal of VSCode installed from Linux Snap package manager:
    ```
    Failed to load module "colorreload-gtk-module"

    /usr/lib/x86_64-linux-gnu/webkit2gtk-4.1/WebKitNetworkProcess: symbol lookup error: /snap/core20/current/lib/x86_64-linux-gnu/libpthread.so.0: undefined symbol: __libc_pthread_init, version GLIBC_PRIVATE
    ```
    Fix it by installing VSCode directly from official `.deb` bundle or try to unset multiple env variables - more info in https://stackoverflow.com/questions/75921414/java-symbol-lookup-error-snap-core20-current-lib-x86-64-linux-gnu-libpthread

---

### Production build:

1. `cargo tauri build`
2. Runnable executable is in `target/release`
3. Installable bundles specific for the platform are in `target/release/bundle`

---

### Integration steps for a standard Tauri Web FS example to make this example:

1. Install Tauri CLI: `cargo install tauri-cli@=2.0.0-beta.11`
2. `cargo tauri init`
3. App name: `Tauri Web FS`
4. Window title: `Tauri Web FS`
5. Web assets relative path: `../frontend_dist`
6. Dev server url: `http://localhost:8080`
7. Frontend dev command: `makers mzoon start`
8. Frontend build command: `makers mzoon build -r -f`
9. Add `"src-tauri"` to `Cargo.toml` workspace members.
10. Change `identifier` in `src-tauri/tauri.conf.json` to `"com.example.moonzoon.tauri-web-fs"`
11. Set env var `WEBKIT_DISABLE_DMABUF_RENDERER=1` in `src-tauri/lib.rs` because WebKitGTK (2.42) is not compatible with NVIDIA drivers on Linux.
12. Enable `tauri` crate feature `linux-ipc-protocol` in `src-tauri/Cargo.toml` to make IPC faster on Linux.
13. Change `app.withGlobalTauri` in `src-tauri/tauri.conf.json` to `true`.
