# Tauri TodoMVC
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

Properties of an `msi` bundle on Windows:
- Size of `TodoMVC_0.1.0_x64_en-US.msi` is **2 MB**.
- Size of `C:\Program Files\TodoMVC\TodoMVC.exe` is **3.7 MB**.
- Process `TodoMVC.exe` uses ~**65.2 MB** RAM from start.

Properties of a `deb` bundle on Kubuntu:
- Size of `todo-mvc_0.1.0_amd64.deb` is **2.7 MB**.
- Size of `/usr/bin/todo-mvc` is **7.9**.
- Process `todo-mvc` in System Monitor uses ~**34 MB** RAM.
- Application `TodoMVC` in System Monitor uses ~**168.3 MB** RAM from start.

---

### Integration steps for a standard TodoMVC example to make this example:

1. Install Tauri CLI: `cargo install tauri-cli@=2.0.0-beta.11`
2. `cargo tauri init`
3. App name: `TodoMVC`
4. Window title: `TodoMVC`
5. Web assets relative path: `../frontend_dist`
6. Dev server url: `http://localhost:8080`
7. Frontend dev command: `makers mzoon start`
8. Frontend build command: `makers mzoon build -r -f`
9. Add `"src-tauri"` to `Cargo.toml` workspace members.
10. Change `identifier` in `src-tauri/tauri.conf.json` to `"com.example.moonzoon.tauri-todomvc"`
11. Set env var `WEBKIT_DISABLE_DMABUF_RENDERER=1` in `src-tauri/lib.rs` because WebKitGTK (2.42) is not compatible with NVIDIA drivers on Linux.
