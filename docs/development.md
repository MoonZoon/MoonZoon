# MoonZoon Development
---

_WARNING:_ MoonZoon is in the phase of early development! You'll find hacks, ugly and spaghetti code in the repo!

But if you are brave enough:
```sh
cd examples
cd counter
cargo run --manifest-path "../../crates/mzoon/Cargo.toml" start
```
_Notes_:
  - Tested only with the latest stable Rust.
  - [Wasm-pack](https://rustwasm.github.io/wasm-pack/) auto-install hasn't been implemented yet. If you don't have it, CLI panics with the recommendation to run `cargo install wasm-pack` or to install a pre-built version.

Then, you can find paths which trigger auto-reload in the file: `examples/counter/MoonZoon.toml`.

And you can test it this way:

[![Autoreload demo](images/autoreload.gif)](https://raw.githubusercontent.com/MoonZoon/MoonZoon/main/docs/images/autoreload.gif)
