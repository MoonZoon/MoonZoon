# CLI - mzoon
---

`mzoon` is a MoonZoon CLI tool. 

_

You'll be able to install or update it with _Cargo_: 
```sh
cargo install --locked mzoon
```
([why _--locked_?](https://github.com/rust-lang/cargo/issues/7169))

_Note:_ Faster installation options will be added later.

---

## Commands

### 1. `new`

- Example: `mzoon new my_project` 
- Creates a new directory with a MoonZoon project.
- _Question:_ Do you think a [code tour](https://github.com/microsoft/codetour) would be useful for you?
- _Note:_ Not implemented yet, use [MoonZoon/demo](https://github.com/MoonZoon/demo) as a starting project.

### 2. `start`

- Example: `mzoon start`
- Compiles the app in the debug mode and then starts the Moon's server.
- Both Moon and Zoon apps are automatically recompiled on a file change.
- The Moon app auto-reloads the Zoon app on a change.
- Optional parameters:
   1. **`--release` / `-r`**
      - Example: `mzoon start --release`
      - Compiles in the release mode and compress frontend files.
   1. **`--open` / `-o`**
      - Example: `mzoon start --open`
      - Opens the Zoon's URL in a new browser tab (e.g. `localhost:8080`)
      - _Note:_ Not implemented yet.

### 3. `build`

- Example: `mzoon build`
- Compiles the app in the debug mode.
- Optional parameters:
   1. **`--release` / `-r`**
      - Example: `mzoon build --release`
---

## MoonZoon.toml

- The configuration file located in the app root directory.

```toml
port = 8080
# port = 8443
https = false
cache_busting = true

[redirect_server] # useful for HTTP -> HTTPS redirect
port = 8081
enabled = false

[watch]
frontend = [
    "frontend/Cargo.toml",
    "frontend/src",
]
backend = [
    "backend/Cargo.toml",
    "backend/src",
]
```

---

## FAQ
1. _"What about other commands like `deploy`, `test`, `generate`, etc.?"_
   - Other commands will be added later.

1. _"How can I change the port number or enable HTTPS?"_

   - _For development_: Update settings in `MoonZoon.toml` (see `MoonZoon.toml` above or `/examples/counters/MoonZoon.toml`)

   - _For production_: Set environment variables (see the function `load_config` in `/crates/moon/src/lib.rs`)

1. _"Some commands or parameters mentioned above don't work!"_
   - They are probably not implemented yet.

1. _"What is a new project file structure?"_
   - See the content of [MoonZoon/demo](https://github.com/MoonZoon/demo) to have an idea.
    



