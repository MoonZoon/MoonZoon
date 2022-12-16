# MoonZoon Development

---

_WARNING:_ MoonZoon is in the phase of early development and a CI pipeline / linters haven't been configured yet.

## 1. Required tools

- [Rust](https://www.rust-lang.org/)
  ```bash
  rustup update stable
  rustc -V # rustc 1.66.0 (69f9c33d7 2022-12-12)
  ```

- [cargo-make](https://sagiegurari.github.io/cargo-make/)
  ```bash
  cargo install cargo-make --no-default-features
  makers -V # cargo-make 0.36.3
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
        // "examples/canvas/Cargo.toml",
        // "examples/chat/Cargo.toml",
        "examples/counter/Cargo.toml",
        // "examples/counters/Cargo.toml",
        // "examples/custom_config/Cargo.toml",
        // "examples/custom_http_client/Cargo.toml",
        // "examples/image/Cargo.toml",
        // "examples/js_text_editor/Cargo.toml",
        // "examples/js_framework_benchmark/Cargo.toml",
        // "examples/keyboard/Cargo.toml",
        // "examples/layers/Cargo.toml",
        // "examples/markup/Cargo.toml",
        // "examples/minesweeper/Cargo.toml",
        // "examples/pages/Cargo.toml",
        // "examples/pan_zoom/Cargo.toml",
        // "examples/paragraph/Cargo.toml",
        // "examples/resize_drag/Cargo.toml",
        // "examples/row_wrap/Cargo.toml",
        // "examples/slider/Cargo.toml",
        // "examples/start_with_app/Cargo.toml",
        // "examples/svg/Cargo.toml",
        // "examples/text_area/Cargo.toml",
        // "examples/time_tracker/Cargo.toml",
        // "examples/timer/Cargo.toml",
        // "examples/todomvc/Cargo.toml",
        // "examples/tweened/Cargo.toml",
        // "examples/video/Cargo.toml",
        // "examples/viewport/Cargo.toml",
        // "examples/wasm_components/Cargo.toml",
    ],
    "rust-analyzer.diagnostics.disabled": [
        "missing-unsafe",
        "add-reference-here",
    ],
    "rust-analyzer.procMacro.enable": true,
    "rust-analyzer.cargo.allFeatures": true,
    "rust-analyzer.completion.autoimport.enable": false,
    // "rust-analyzer.updates.channel": "nightly"
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
