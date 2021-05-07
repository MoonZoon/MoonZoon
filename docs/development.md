# MoonZoon Development

---

_WARNING:_ MoonZoon is in the phase of early development! You may find hacks and ugly code in the repo!

## 1. Required tools

- [Rust](https://www.rust-lang.org/)
  ```bash
  rustup update
  rustc -V # rustc 1.52.0 (88f19c6da 2021-05-03)
  ```

- [cargo-make](https://sagiegurari.github.io/cargo-make/)
  ```bash
  cargo install cargo-make
  makers -V # makers 0.32.17
  ```

## 2. VS Code settings

- Install [Rust Analyzer](https://rust-analyzer.github.io/)
- My current `.vscode/settings.json`:

```json
{
    "rust-analyzer.linkedProjects": [
        // rust-analyzer ignores `main.rs` when `linkedProjects` are set
        "crates/mzoon/Cargo.toml",
        // examples are ignored because they have own workspaces
        "examples/counter/Cargo.toml",
        "examples/counters/Cargo.toml",
        "examples/js-framework-benchmark/keyed/Cargo.toml",
        // ... add more examples as needed
    ],
    "rust-analyzer.diagnostics.disabled": [
        "missing-unsafe",
    ],
    "rust-analyzer.procMacro.enable": true,
    "rust-analyzer.cargo.allFeatures": true,
    "rust-analyzer.completion.autoimport.enable": false,
    "rust-analyzer.updates.channel": "nightly"
}
```

## 3. Configure example

- Update example's `Makefile.toml` or `MoonZoon.toml` if necessary.

<details>
<summary>Current configs files from <code>examples/counters/</code></summary>

```toml
# Makefile.toml

[config]
default_to_workspace = false
min_version = "0.32.15"

[config.modify_core_tasks]
private = true
namespace = "default"

[tasks.mzoon]
description = "Run MZoon"
command = "cargo"
args = ["run", "--manifest-path", "../../crates/mzoon/Cargo.toml", "${@}"]
dependencies = ["default::install-wasm-pack"]
```

```toml
# MoonZoon.toml

port = 8080
# port = 8443
https = false

[redirect_server]
port = 8081
enabled = false

[watch]
frontend = [
    "frontend/Cargo.toml",
    "frontend/src",
    "../../crates/zoon/Cargo.toml",
    "../../crates/zoon/src",
    "../../crates/blocks_macro/Cargo.toml",
    "../../crates/blocks_macro/src",
    "../../crates/update_macro/Cargo.toml",
    "../../crates/update_macro/src",
    "../../crates/s_var_macro/Cargo.toml",
    "../../crates/s_var_macro/src",
    "../../crates/cache_macro/Cargo.toml",
    "../../crates/cache_macro/src",
    "../../crates/tracked_call_macro/Cargo.toml",
    "../../crates/tracked_call_macro/src",
    "../../crates/cmp_macro/Cargo.toml",
    "../../crates/cmp_macro/src",
]
backend = [
    "backend/Cargo.toml",
    "backend/src",
    "../../crates/moon/Cargo.toml",
    "../../crates/moon/src",
]

```

</details>

## 4. Start example

```sh
cd examples
cd counter # or `counters` or  ...
makers mzoon start # or `makers mzoon start -r`
```

## 5. Have fun!
