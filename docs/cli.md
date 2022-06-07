# CLI - mzoon
---

`mzoon` is a MoonZoon CLI tool.

_

```sh
cargo install mzoon --git https://github.com/MoonZoon/MoonZoon --locked
```

_Notes:_ 
   - Why [_--locked_](https://github.com/rust-lang/cargo/issues/7169) ?
   - `mzoon` hasn't been published to [crates.io](https://crates.io/) yet.
   - Faster installation methods with pre-compiled binaries will be added later.

---

## Commands

### 1. `new`

- Example A: `mzoon new my_project` 
   - Creates a new directory with a new MoonZoon project.
- Example B: `mzoon new .` 
   - The new project files will be created in the current directory.
- Optional parameters:
   1. **`--local-deps` / `-l`**
      - Example: `mzoon new my_project --local-deps`
      - `moon` and `zoon` dependencies in `Cargo.toml`s will be defined with `path` instead of `version`. It's useful especially for MoonZoon development.

### 2. `start`

- Example: `mzoon start`
- Compiles the app in the debug mode and then starts the Moon's server.
- Both Moon and Zoon apps are automatically recompiled on a file change.
- The Moon app auto-reloads the Zoon app on a change.
- You can scan a generated QR code to open the app on your phone.
- Optional parameters:
   1. **`--release` / `-r`**
      - Example: `mzoon start --release`
      - Compiles in the release mode and compresses frontend files.
   1. **`--profiling` / `-p`**
      - Example: `mzoon start --profiling`
      - The same as the release mode but debugging info isn't removed from the binary.
   1. **`--open` / `-o`**
      - Example: `mzoon start --open`
      - Opens the Zoon's URL in a new browser tab (e.g. `localhost:8080`)

### 3. `build`

- Example: `mzoon build`
- Compiles the app in the debug mode.
- Optional parameters:
   1. **`--release` / `-r`**
      - Example: `mzoon build --release`
      - Compiles in the release mode and compresses frontend files.
   1. **`--profiling` / `-p`**
      - Example: `mzoon build --profiling`
      - The same as the release mode but debugging info isn't removed from the binary.
   1. **`--frontend-dist` / `-f`**
      - Example: `mzoon build --release --frontend-dist`
      - Generates a new folder `frontend_dist` in the project root.
      - You can deploy the content of the `frontend_dist` folder to your favorite frontend hosting.
      - You can also generate some hosting-specific files with the `mzoon` argument `<HOSTING>`
         - Example: `mzoon build -r -f netlify`
---

## FAQ
1. _"What about other commands like `deploy` and `test`?"_
   - Other commands will be added later.
   - You should be able to use the standard `cargo test` until there is a native `mzoon` support.
   - Write your ideas on the MoonZoon [Discord](https://discord.gg/eGduTxK2Es), please.
