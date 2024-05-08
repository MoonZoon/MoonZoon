# MoonZoon Development

---

_WARNING:_ MoonZoon is in the phase of early development and a CI pipeline / linters haven't been configured yet.

## 1. Required tools

- [Rust](https://www.rust-lang.org/)
  ```bash
  rustup update
  rustc -V # rustc 1.78.0 (9b00956e5 2024-04-29)
  ```

- [cargo-make](https://sagiegurari.github.io/cargo-make/)
  ```bash
  cargo install cargo-make --no-default-features
  makers -V # cargo-make 0.37.12
  ```

  - _Note_: `cargo-make` is needed only for MoonZoon development and running its examples, you don't need it for your apps.

## 2. VS Code settings

- Install [Rust Analyzer](https://rust-analyzer.github.io/)
- The current `.vscode/settings.json`: 
  - (Most examples are commented out to reduce the amount of operating memory consumed by RA.)

```json
{
    "rust-analyzer.linkedProjects": [
        // rust-analyzer ignores `main.rs` when `linkedProjects` are set
        "crates/mzoon/Cargo.toml",
        // examples are ignored because they have own workspaces
        // "examples/align/Cargo.toml",
        // "examples/align_content/Cargo.toml",
        // "examples/blocking_task/Cargo.toml",
        // "examples/canvas/Cargo.toml",
        // "examples/chat/Cargo.toml",
        "examples/counter/Cargo.toml",
        // "examples/counter_with_app_events/Cargo.toml",
        // "examples/counters/Cargo.toml",
        // "examples/custom_config/Cargo.toml",
        // "examples/custom_http_client/Cargo.toml",
        // "examples/image/Cargo.toml",
        // "examples/js_text_editor/Cargo.toml",
        // "examples/js_framework_benchmark/Cargo.toml",
        // "examples/keyboard/Cargo.toml",
        // "examples/layers/Cargo.toml",
        // "examples/local_search/Cargo.toml",
        // "examples/markup/Cargo.toml",
        // "examples/pages/Cargo.toml",
        // "examples/pan_zoom/Cargo.toml",
        // "examples/paragraph/Cargo.toml",
        // "examples/resize_drag/Cargo.toml",
        // "examples/row_wrap/Cargo.toml",
        // "examples/signal_params/Cargo.toml",
        // "examples/slider/Cargo.toml",
        // "examples/spinner/Cargo.toml",
        // "examples/start_with_app/Cargo.toml",
        // "examples/svg/Cargo.toml",
        // "examples/tauri_ipc/Cargo.toml",
        // "examples/tauri_local_search/Cargo.toml",
        // "examples/tauri_todomvc/Cargo.toml",
        // "examples/tauri_web_workers/Cargo.toml",
        // "examples/text_area/Cargo.toml",
        // "examples/three_d/Cargo.toml",
        // "examples/timer/Cargo.toml",
        "examples/todomvc/Cargo.toml",
        // "examples/todomvc_with_app_events/Cargo.toml",
        // "examples/tweened/Cargo.toml",
        // "examples/video/Cargo.toml",
        // "examples/viewport/Cargo.toml",
        // "examples/wasm_components/Cargo.toml",
        // "examples/web_components/Cargo.toml",
        // "examples/web_workers/Cargo.toml",
        "examples/wgpu/Cargo.toml",
    ],
    "rust-analyzer.diagnostics.disabled": [
        "missing-unsafe",
        "add-reference-here",
    ],
    "rust-analyzer.procMacro.enable": true,
    "rust-analyzer.completion.autoimport.enable": false,
    "rust-analyzer.cargo.extraEnv": {
        "CARGO_PROFILE_RUST_ANALYZER_INHERITS": "dev"
    },
    "rust-analyzer.cargo.extraArgs": [
        "--profile",
        "rust-analyzer"
    ],
    "rust-analyzer.cargo.features": ["zoon/frontend_multithreading"],
}
```

</details>

## 3. Start example

```sh
cd examples
cd chat # or another example from the `examples` directory
makers mzoon start -o # add -r for the release mode
```

## 4. Rebuild all examples

```sh
# in the root:
makers in_examples clean
makers in_examples mzoon build
```

## 5. Have fun!

--

_Dev note_: You can kill a zombie server on Linux with `kill -9 $(lsof -t -i:8080)`
