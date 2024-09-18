# Golem Chat
> MoonZoon example

### How basic Golem files has been created and How to start Golem and the MoonZoon parts

- https://learn.golem.cloud/docs/quickstart

1. https://grpc.io/docs/protoc-installation/
2. `protoc --version` => `libprotoc 28.1`
3. `cargo install golem-cli`
4. `mkdir golem`
5. `cd golem`
6. `curl -O https://raw.githubusercontent.com/golemcloud/golem/main/docker-examples/docker-compose-postgres.yaml -O  https://raw.githubusercontent.com/golemcloud/golem/main/docker-examples/.env`
7. `golem-cli init` => default values
8. Start Docker Desktop
9. `docker-compose -f docker-compose-postgres.yaml up`
10. Open new terminal window and go to this example root
11. `cargo install cargo-make`
12. `makers mzoon start`

### How component `message_store` has been created and How to deploy the component

- https://learn.golem.cloud/docs/rust-language-guide/setup

1. `cargo install --force --locked cargo-component@0.13.2`
2. `cargo component --version` => `cargo-component-component 0.13.2 (wasi:040ec92)`
3. `golem-cli new -l rust message_store`
4. `cd message_store`
5. Add `golem/message_store` to root Cargo.toml `workspace.members`
6. `cargo component build --release`
7. `golem-cli component add -c message_store ../../../../target/wasm32-wasi/release/message_store.wasm`
8. [Redeploy workers](https://learn.golem.cloud/docs/cli/components#redeploying-workers) if needed - `golem-cli component redeploy -c message_store`

### How to create Worker

- https://learn.golem.cloud/docs/quickstart

1. 


// @TODO remove
New component created with URN urn:component:023b0392-9848-42eb-99c7-c4423e6bec36, version 0, and size of 14585 bytes.
Component name: message_store.
Exports:
        golem:component/api.{add}(value: u64)
        golem:component/api.{get}() -> u64

