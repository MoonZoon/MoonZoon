1.
    ```
    cargo install --git https://github.com/bytecodealliance/cargo-component

    cargo install --git https://github.com/bytecodealliance/wit-bindgen wit-bindgen-cli
    ```

1.
    ```
    cargo component build
    ```

1.
    ```
    wit-bindgen host js --no-typescript --no-nodejs-compat --out-dir/debug out target/wasm32-unknown-unknown/debug/hello.wasm

    ```
