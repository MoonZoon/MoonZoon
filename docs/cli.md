# CLI - mzoon
---

`mzoon` is a MoonZoon CLI tool. 

_

You'll be able to install or update it with _Cargo_: 
```sh
cargo install mzoon
```

---

## Commands

### 1. `new`

- Example: `mzoon new my_project` 
- Creates a new directory with a MoonZoon project.

### 2. `start`

- Example: `mzoon start`
- Compiles the app in the debug mode and then starts the Moon's server.
- Both Moon and Zoon apps are automatically recompiled on a file change.
- The Moon app automatically reloads the Zoon app on a change.
- Optional parameters:
   1. **`--prod` / `-p`**
      - Example: `mzoon start --prod`
      - Compiles in the release mode; Zoon auto-reload is disabled.
   1. **`--open` / `-o`**
      - Example: `mzoon start --open`
      - Opens the Zoon's URL in a new browser tab (e.g. `localhost:8080`)

---

## FAQ
1. _"What about other commands like `deploy`, `test`, `generate`, etc.?"_
   - Other commands will be added later.

1. _"How can I change the port number?"_
   - The first idea: The function `init` in your Moon's app would return a struct `Config` with the optional property `port`. The default port is `8080`.

1. _"What is a new project file structure?"_
   - See the example `time_tracker` to have an idea.
    



