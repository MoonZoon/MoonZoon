# MoonZoon Development

---

_WARNING:_ MoonZoon is in the phase of early development! You may find ugly code in the repo!

## 1. Required tools

- [Rust](https://www.rust-lang.org/)
  ```bash
  rustup update
  rustc -V # rustc 1.52.0 (88f19c6da 2021-05-03)
  ```

- [cargo-make](https://sagiegurari.github.io/cargo-make/)
  ```bash
  cargo install cargo-make
  makers -V # makers 0.33.0
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

</details>

## 3. Start example

- _Note:_ Not all examples work at the moment

```sh
cd examples
cd counter # or `counters` or  ...
makers mzoon start # or `makers mzoon start -r`
```

## 4. Open example in the browser

- `Ctrl` + click (or equivalent) on the url in brackets in your terminal
- _Note:_ Tested with `git-bash` in VS Code
```bash
Main server is running on 0.0.0.0:8080 [http://127.0.0.1:8080]

```

## 5. Have fun!
