# MoonZoon Development

## 1. Required tools

- [Rust](https://www.rust-lang.org/)
  ```bash
  rustup update
  rustc -V # rustc 1.90.0 (1159e78c4 2025-09-14)
  ```

- [cargo-make](https://sagiegurari.github.io/cargo-make/)
  ```bash
  cargo install cargo-make
  makers -V # cargo-make 0.37.24
  ```

  - _Note_: `cargo-make` is needed only for MoonZoon development and running its examples, you don't need it for your apps.

## 2. VS Code settings

- Install [Rust Analyzer](https://rust-analyzer.github.io/)
- Uncomment examples as needed in `.vscode/settings.json` 
  - Don't commit the change.
  - Most examples are commented out to reduce compilation time and consumed operating memory.

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

_Note_: You can kill a zombie server on Linux with `kill -9 $(lsof -t -i:8080)`
