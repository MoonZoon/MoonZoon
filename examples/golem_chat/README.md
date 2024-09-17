# Golem Chat
> MoonZoon example

### How basic Golem files has been created and how to start Golem

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
11. `makers mzoon start`

### How component `message_store` has been created and how to deploy the component

- https://learn.golem.cloud/docs/rust-language-guide/setup

1. `cargo install --force --locked cargo-component@0.13.2`
2. `cargo component --version` => `cargo-component-component 0.13.2 (wasi:040ec92)`
3. 
