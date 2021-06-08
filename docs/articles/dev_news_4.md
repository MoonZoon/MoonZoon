# MoonZoon Dev News (4): Actix, Async CLI, Fails

Unlimited Actix power!

[![Hello from Actix](images/hello_from_actix.gif)](https://raw.githubusercontent.com/MoonZoon/MoonZoon/main/docs/images/demo.gif)

---

<p align="center">
  <strong>Welcome to the MoonZoon Dev News!</strong>
</p>

<p align="center">
  <img src="images/MoonZoon.png" width="200" title="MoonZoon logo">
</p>


[MoonZoon](https://moonzoon.rs) is a [Rust](https://www.rust-lang.org/) full-stack framework. If you want to read about new MZ features, architecture and interesting problems & solutions - Dev News is the right place.

---

# News

- Moon - [Warp](https://crates.io/crates/warp) replaced with [Actix](https://crates.io/crates/actix-web). There are API changes to allow you to use Actix directly from your apps.

- mzoon - Rewritten with Tokio, implemented `--open` parameter and `wasm-pack` installer.

- The entire MoonZoon codebase should be clean enough now. Comments are still missing and there should be more tests but if you wanted to know how it really works, you don't have to be afraid to read the code in the MZ [repo](https://github.com/MoonZoon/MoonZoon).

- You can select the required mzoon version for [heroku-buildpack-moonzoon](https://github.com/MoonZoon/heroku-buildpack-moonzoon) by adding the file [mzoon_commit](https://github.com/MoonZoon/demo/blob/main/mzoon_commit) to your repo with a MZ project.

You'll read about Moon and mzoon improvements in the following chapters. The last chapter is dedicated to my development fails, library fails and other notes from trenches. 

---

And I would like to thank:
- All Rust libraries maintainers. It's tough work but it allows us to write clean code and amazing products.

---

# Actix

## Why Actix?

- It's fast, async and popular.
- Supports HTTP/2 and probably also H3 in the future ([related issue](https://github.com/actix/actix-web/issues/309)).
- [Actix actor framework](https://crates.io/crates/actix) could be a good foundation for the first version of virtual actors. 
- The API feels more intuitive than the Warp's one to me. And we were [fighting with Warp](https://github.com/MoonZoon/MoonZoon/pull/6#issuecomment-840037580) during the Moon development.

## Moon API changes

The simplest Moon app:

```rust
use moon::*;

async fn frontend() -> Frontend {
    Frontend::new().title("Actix example")
}

async fn up_msg_handler(_: UpMsgRequest) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_|{}).await
}
```

- `main` is now `async` so we no longer need the `init` function - you can write your async code directly to the `main`'s body. 

- `start!` macro has been rewritten to a simple function `start`. The interesting is the third argument. See the next example:

```rust
use moon::*;
use moon::actix_web::{get, Responder};

async fn frontend() -> Frontend {
    Frontend::new().title("Actix example")
}

async fn up_msg_handler(_: UpMsgRequest) {}

#[get("hello")]
async fn hello() -> impl Responder {
    "Hello!"
}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |cfg|{
        cfg.service(hello);
    }).await
}
```

- It's the code used in the GIF at the top. 

- `cfg` in the example is [actix_web::web::ServiceConfig](https://docs.rs/actix-web/4.0.0-beta.6/actix_web/web/struct.ServiceConfig.html). It allows you to create custom Actix endpoints and configure the server as you wish.

- Multiple crates and items are reexported from `moon` to mitigate dependency hell caused by incompatible versions and to make your `Cargo.toml` simple enough. The current list looks like this:

```rust
pub use trait_set::trait_set;
pub use actix_files;
pub use actix_http;
pub use actix_web;
pub use actix_web::main;
pub use futures;
pub use mime;
pub use mime_guess;
pub use parking_lot;
pub use serde;
pub use tokio;
pub use tokio_stream;
pub use uuid;
```

## MoonZoon.toml changes

```toml
port = 8080
# port = 8443
https = false
cache_busting = true
backend_log_level = "warn" # "error" / "warn" / "info" / "debug" / "trace"

[redirect]
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

- There is a new property `backend_log_level`. It sets the [env_logger](https://crates.io/crates/env_logger) log level. 
   - `info` level is useful for debugging because it shows all requests (demonstrated in the GIF at the top).
   - _Note:_ There are also independent `404` and `500` error handlers that call `eprintl` with the error before they pass the response to the client. 

- `[redirect_server]` has been renamed to `[redirect]` because there is no longer a redirection server. The new [RedirectMiddleware](https://github.com/MoonZoon/MoonZoon/blob/32362a38a35e0d57b291503516de0de2c1c55fc6/crates/moon/src/redirect.rs) is activated when you enable the redirect.

Caching has been also improved:
- `cache_busting = true`:
   - mzoon generates files like `frontend_bg_[uuid].wasm`, where `uuid` is a _frontend build id_ with the type `u128`.
   - Moon serves the files with the header `CacheControl` set to `MaxAge(31536000)` (1 year).
- `cache_busting = false`
   - mzoon doesn't change the file names at all - e.g. `frontend_bg.wasm`.
   - Moon serves the files with the header `ETag` with a _strong_ etag set to the _frontend build id_. (See [MDN ETag docs](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/ETag) for more info.)

## Server-Sent Events

Actix unfortunately doesn't have an official SSE API so I've decided to write a custom one. The current implementation is in the file [crates/moon/src/sse.rs](https://github.com/MoonZoon/MoonZoon/blob/32362a38a35e0d57b291503516de0de2c1c55fc6/crates/moon/src/sse.rs). 

- It sends a `ping` to all connections every 10 seconds to recognize the disconnected ones.

- Integration:
  1. `let sse = SSE::start();`
  2. `App::new().app_data(sse.clone())`

Moon's SSE connector:

```rust
async fn sse_responder(
    sse: web::Data<Mutex<SSE>>,
    shared_data: web::Data<SharedData>,
) -> impl Responder {
    let (connection, event_stream) = sse.new_connection();
    let backend_build_id = shared_data.backend_build_id.to_string();

    if connection
        .send("backend_build_id", &backend_build_id)
        .is_err()
    {
        return HttpResponse::InternalServerError()
            .reason("sending backend_build_id failed")
            .finish();
    }

    HttpResponse::Ok()
        .insert_header(ContentType(mime::TEXT_EVENT_STREAM))
        .streaming(event_stream)
}
```

and the frontend reloader:

```rust
async fn reload_responder(sse: web::Data<Mutex<SSE>>) -> impl Responder {
    let _ = sse.broadcast("reload", "");
    HttpResponse::Ok()
}
```

_Warning_: Keep in mind that browsers can open only 6 SSE connections over HTTP/1.x to the same domain. It means when you open multiple browser tabs pointing to `http://localhost`, you may observe infinite loadings or similar problems. The limit for HTTP/2 is 100 connections by default, but can be negotiated between the client and the server.

## Moon endpoint changes

```rust
App::new()
    // ...
    .service(Files::new("_api/public", "public"))
    .service(
        web::scope("_api")
            .route(
                "up_msg_handler",
                web::post().to(up_msg_handler_responder::<UPH, UPHO>),
            )
            .route("reload", web::post().to(reload_responder))
            .route("pkg/{file:.*}", web::get().to(pkg_responder))
            .route("sse", web::get().to(sse_responder))
            .route("ping", web::to(|| async { "pong" })),
    )
    .route("*", web::get().to(frontend_responder::<FRB, FRBO>))
```

All backend endpoints are prefixed with `_api` to prevent conflicts with frontend routes. There are other solutions like hash routing or moving the frontend endpoint to another domain or a prefix for frontend urls but these solutions often lead to many unpredictable problems. Let's keep it simple.

There is a new simple endpoint `ping`. It's useful for testing if the server is alive. I can imagine we can also implement a _heartbeat_ later (Moon would call a predefined endpoint in a configured interval).

---

# mzoon

---

# Fails

---

And that's all for today! 
Thank You for reading and I hope you are looking forward to the next episode.

Martin

P.S.
We are waiting for you on [Discord](https://discord.gg/eGduTxK2Es).


