# MoonZoon Development

---

_WARNING:_ MoonZoon is in the phase of early development and a CI pipeline / linters haven't been configured yet.

## 1. Required tools

- [Rust](https://www.rust-lang.org/)
  ```bash
  rustup update
  rustc -V # rustc 1.54.0 (a178d0322 2021-07-26)
  ```

- [cargo-make](https://sagiegurari.github.io/cargo-make/)
  ```bash
  cargo install cargo-make
  makers -V # makers 0.35.0
  ```
  - _Note_: `cargo-make` is needed only for MoonZoon development and running its examples, you don't need it for your apps.

## 2. VS Code settings

- Install [Rust Analyzer](https://rust-analyzer.github.io/)
- My current `.vscode/settings.json`:

```json
{
    "rust-analyzer.linkedProjects": [
        // rust-analyzer ignores `main.rs` when `linkedProjects` are set
        "crates/mzoon/Cargo.toml",
        // examples are ignored because they have own workspaces
        "examples/canvas/Cargo.toml",
        "examples/chat/Cargo.toml",
        "examples/counter/Cargo.toml",
        "examples/counters/Cargo.toml",
        "examples/js-framework-benchmark/keyed/Cargo.toml",
        "examples/pages/Cargo.toml",
        "examples/svg/Cargo.toml",
        "examples/viewport/Cargo.toml",
        "examples/timer/Cargo.toml",
        "examples/todomvc/Cargo.toml",
    ],
    "rust-analyzer.diagnostics.disabled": [
        "missing-unsafe",
        "add-reference-here",
    ],
    "rust-analyzer.procMacro.enable": true,
    "rust-analyzer.cargo.allFeatures": true,
    "rust-analyzer.completion.autoimport.enable": false,
    "rust-analyzer.updates.channel": "nightly"
}
```

</details>

## 3. Start example

- Runnable examples (the list is continuously updated):
  - `canvas`
  - `chat`
  - `counter`
  - `counters`
  - `js-framework-benchmark/keyed`
  - `pages`
  - `svg`
  - `viewport`
  - `timer`
  - `todomvc`

```sh
cd examples
cd chat # or another example
makers mzoon start -o # add -r for the release mode
```

## 5. Have fun!

--

_Question:_ Do you think a [code tour](https://github.com/microsoft/codetour) would be useful for you?
