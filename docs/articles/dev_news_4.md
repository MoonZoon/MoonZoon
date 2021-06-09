# MoonZoon Dev News (4): Actix, Async CLI, Error handling, Fails

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
- It uses [Tokio](https://crates.io/crates/tokio) under the hood. It's the most popular async runtime and we can use it also in mzoon.
- The API feels more intuitive than the [Warp](https://crates.io/crates/warp)'s one to me. And we were [fighting with Warp](https://github.com/MoonZoon/MoonZoon/pull/6#issuecomment-840037580) during the Moon development.
- [Tide](https://crates.io/crates/tide) supports only HTTP/1.x.
- Async [Rocket](https://crates.io/crates/rocket) working on stable Rust hasn't been released yet. (A Git version would block Moon publishing to [creates.io](https://crates.io/)).

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
   - _Note_: [fern](https://crates.io/crates/fern) looks like a good alternative if we find out `env_logger` isn't good enough. (Thanks [azzamsa](https://github.com/azzamsa) for the suggestion.)

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

## Async runtime

mzoon was rewritten with [Tokio](https://crates.io/crates/tokio). The main goal was to remove spaghetti code and boilerplate caused by manual handling of threads, channels and signals. The secondary goal was error handling and improved performance.

There are also other async runtimes like [async-std](https://crates.io/crates/async-std) or [smol](https://crates.io/crates/smol) but I've decided to choose the most battle-tested and popular one. Another reason for Tokio is Actix, because Actix is based on Tokio so there should be less context switching during the MoonZoon development.

## Error handling

During the mzoon refactor, I've decided to integrate two nice libraries to eliminate boilerplate:

The first one is [**anyhow**](https://crates.io/crates/anyhow). It basically allows you write `?` where you want to return an error early. No need to write error mappers or similar stuff. 

`anyhow` also provides the method `context` (and its lazy version `with_context`) and a macro `anyhow!` for creating errors. An example:

```rust
// `anyhow::Result<T>` is an alias for a standard `Result<T, anyhow::Error>`
use anyhow::{anyhow, Context, Result};  

pub async fn build_backend(release: bool, https: bool) -> Result<()> {
    ...
    Command::new("cargo")
        .args(&args)
        .status()
        .await
        .context("Failed to get frontend build status")?
        .success()
        .err(anyhow!("Failed to build backend"))?;
    ...
}
```
- _Notes_: 
   - The method `.err` from the example above is implemented in the crate [bool_ext](https://crates.io/crates/bool_ext).
   - `anyhow` is useful mostly for apps. If you are writing a library, look at [thiserror](https://crates.io/crates/thiserror) (written by the same author).


The second error handling library is [**fehler**](https://github.com/withoutboats/fehler). I've decided to integrate it into mzoon once I noticed that many functions were returning `Ok(())` and their signature was `... -> Result<()>`. `Ok(())` is a side-effect of `anyhow` because you want to use `?` as much as possible to automatically convert specific errors to `anyhow::Error`. The second reason why there were many `Ok(())`s is the fact that mzoon does many file operations. 

I recommend to read these articles about `fehler` - [A brief apology of Ok-Wrapping](https://without.boats/blog/why-ok-wrapping/) and [From failure to Fehler](https://without.boats/blog/failure-to-fehler/). 

So when we combine both libraries, we can write a clean code without boilerplate:
```rust
use anyhow::Error;
use fehler::throws;
// ...
#[throws]
#[tokio::main]
async fn main() {
    // ...
    match opt {
        Opt::New { project_name, here } => command::new(project_name, here).await?,
        Opt::Start { release, open } => command::start(release, open).await?,
        Opt::Build { release } => command::build(release).await?,
    }
}
```

- `#[throws]` automatically converts the return type from `-> ()` to  `-> Result<(), Error>` and you don't have to write ugly `Ok(())` or wrap the entire `match` into `Ok()`.

- All errors before `?` are automatically converted to `Error` and nicely written to the terminal with their contexts thanks to `anyhow`.

Let's look at another example from mzoon where we integrated the crate [apply](https://crates.io/crates/apply) to help with chaining:

```rust
// ...
use anyhow::{Context, Error};
use apply::{Also, Apply};
use fehler::throws;

#[throws]
pub fn run_backend(release: bool) -> Child {
    println!("Run backend");
    MetadataCommand::new()
        .no_deps()
        .exec()?
        .target_directory
        .also(|directory| directory.push(if release { "release" } else { "debug" }))
        .also(|directory| directory.push("backend"))
        .apply(Command::new)
        .spawn()
        .context("Failed to run backend")?
}
```

- _Tip_: Don't try to write "functional chains" at all costs. It's easy to get lost in long chains, they may be difficult to change and they may increase cognitive load because the reader has to keep intermediate steps/states in his working memory. The example above is very close to the case where clean code is uncomfortable to read.

- _Note_: We have to find the `target` directory and call the Moon app binary (`backend`) manually because `cargo run` always tries to build the project even if the project has been already built. It slows down the build pipeline and writes unnecessary messages to the terminal. [Related issue](https://github.com/rust-lang/cargo/issues/3773#issuecomment-787782106).

## File Watchers

While I was rewriting `std` channels to the `tokio` ones, I encountered the problem with the [notify](https://crates.io/crates/notify) API. Also its event [debouncing](https://css-tricks.com/debouncing-throttling-explained-examples/) wasn't working properly in mzoon. Fortunately `notify` maintainers are working on a new major version and they've already published `5.0.0-pre.x` versions. The API is more flexible but debouncing is still missing in the new `notify` and in the crate [futures-rs](https://github.com/rust-lang/futures-rs/issues/210). So I had to write a custom debouncer.

The snippets below belong to the current `ProjectWatcher` implementation in [/crates/mzoon/src/watcher/project_watcher.rs](https://github.com/MoonZoon/MoonZoon/blob/32362a38a35e0d57b291503516de0de2c1c55fc6/crates/mzoon/src/watcher/project_watcher.rs).

```rust
use notify::{immediate_watcher, Event, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
// ...

pub struct ProjectWatcher {
    watcher: RecommendedWatcher,
    debouncer: JoinHandle<()>,
}

impl ProjectWatcher {
    #[throws]
    pub fn start(paths: &[String], debounce_time: Duration) -> (Self, UnboundedReceiver<()>) {
        let (sender, receiver) = mpsc::unbounded_channel();
        let watcher = start_immediate_watcher(sender, paths)?;
        let (debounced_sender, debounced_receiver) = mpsc::unbounded_channel();

        let this = ProjectWatcher {
            watcher,
            debouncer: spawn(debounced_on_change(
                debounced_sender,
                receiver,
                debounce_time,
            )),
        };
        (this, debounced_receiver)
    }
    // ...
```
1. `ProjectWatcher` is a general watcher based on the `notify`'s watcher. It's used in mzoon's `BackendWatcher` and `FrontendWatcher`.

1. `start_immediate_watcher` calls the `notify`'s `immediate_watcher` function to register watched paths and the callback that is invoked when `notify` observes a file change. The callback sends `()` (aka _unit_) through the `sender`.

1. The `sender`'s second half - `receiver` - is passed to the `debouncer`. It means the debouncer is able to listen for all registered file system events.

1. The `debounced_sender` represents the `debouncer`'s output - basically a stream of debounced _units_ (we can replace units with `Event`s if needed in the future).

```rust
async fn debounced_on_change(
    debounced_sender: UnboundedSender<()>,
    mut receiver: UnboundedReceiver<()>,
    debounce_time: Duration,
) {
    let mut debounce_task = None::<JoinHandle<()>>;
    let debounced_sender = Arc::new(debounced_sender);

    while receiver.recv().await.is_some() {
        if let Some(debounce_task) = debounce_task {
            debounce_task.abort();
        }
        debounce_task = Some(spawn(debounce(
            Arc::clone(&debounced_sender),
            debounce_time,
        )));
    }

    if let Some(debounce_task) = debounce_task {
        debounce_task.abort();
    }
}

async fn debounce(debounced_sender: Arc<UnboundedSender<()>>, debounce_time: Duration) {
    sleep(debounce_time).await;
    if let Err(error) = debounced_sender.send(()) {
        return eprintln!("Failed to send with the debounced sender: {:#?}", error);
    }
}
```

1. When the _unit_ from the `notify`'s callback is received, then a new task is spawned. The task `sleep`s for the `debounce_time` and then a _unit_ is sent through the `debounced_sender`.

1. When another _unit_ is received, then the sleeping task is aborted and a new one is created. You can understand it as "debounce time reset".

Notice two same code blocks in the previous snippet:
```rust
if let Some(debounce_task) = debounce_task {
    debounce_task.abort();
}
```
The first usage "resets debounce time", but the second one is basically an alternative to `drop`. Unfortunately neither Rust nor `tokio` is able to automatically clean all garbage so we have to do it manually - the task handle does nothing when dropped in most cases.

So... how we can stop the watcher?

The `ProjectWatcher` doesn't have only one method (`start`) - there is another one:
```rust
#[throws]
pub async fn stop(self) {
    let watcher = self.watcher;
    drop(watcher);
    self.debouncer.await?;
}
```
1. Drop `notify`'s `RecommendedWatcher`.
1. Dropped `watcher` means that also our `sender` has been dropped because it was closed by the closure used as a callback / event handler owned by the watcher.
1. When the `sender` is dropped, then `receiver.recv().await.is_some()` returns `false` to break the `while` loop in the debouncer.
1. The debounce task is aborted if there was one running. 

Yeah, it's already quite complicated and error prone but we haven't finished yet.

`FrontendWatcher` and `BackendWatcher` have the similar relationship to `ProjectWatcher` as `ProjectWatcher` to `notify`'s `Watcher`. Let's look at the `FrontendWatcher` skeleton:

```rust
pub struct FrontendWatcher {
    watcher: ProjectWatcher,
    task: JoinHandle<Result<()>>,
}

impl FrontendWatcher {
    #[throws]
    pub async fn start(config: &Config, release: bool, debounce_time: Duration) -> Self {
        let (watcher, debounced_receiver) =
            ProjectWatcher::start(&config.watch.frontend, debounce_time)
                .context("Failed to start the frontend project watcher")?;
        // ...        
        Self {
            watcher,
            task: spawn(on_change(
                debounced_receiver,
                // ...
            )),
        }
    }

    #[throws]
    pub async fn stop(self) {
        self.watcher.stop().await?;
        self.task.await??;
    }
}
```

As you can see, there is another `stop` method that calls the previous `stop` method and the remaining code is very similar to the `ProjectWatcher` implementation. 

Let's look at the last snippet to know the whole watcher story ([crates/mzoon/src/command/start.rs](https://github.com/MoonZoon/MoonZoon/blob/32362a38a35e0d57b291503516de0de2c1c55fc6/crates/mzoon/src/command/start.rs)):
```rust
#[throws]
pub async fn start(release: bool, open: bool) {
    // ...
    let frontend_watcher = build_and_watch_frontend(&config, release).await?;
    let backend_watcher = build_run_and_watch_backend(&config, release, open).await?;

    signal::ctrl_c().await?;
    println!("Stopping watchers...");
    let _ = join!(
        frontend_watcher.stop(),
        backend_watcher.stop(),
    );
    println!("Watchers stopped");
}

#[throws]
async fn build_and_watch_frontend(config: &Config, release: bool) -> FrontendWatcher {
    if let Err(error) = build_frontend(release, config.cache_busting).await {
        eprintln!("{}", error);
    }
    FrontendWatcher::start(&config, release, DEBOUNCE_TIME).await?
}
```

So I can imagine there are some opportunities for another refactor round:
- "Hide" loops and debouncer inside `Stream`s.
- Use `notify`'s debouncer once it's integrated into the library.
- Use async drops once Rust supports them or an alternative.
   - See the related article [Asynchronous Destructors](https://boats.gitlab.io/blog/post/poll-drop/) from the `fehler`'s author.
- If you want to investigate the option "Wait until all task done" so we can just abort all tasks in a standard `drop` and then wait for async runtime to finish, there is [the entrance](https://github.com/tokio-rs/tokio/issues/2053) to the rabbit hole.

Feel free to create a PR when you manage to simplify the code.

## File Compressors

Frontend files are served compressed to get them quickly to users and to reduce network traffic and server load. Only app files (in the `pkg` directory) are compressed at the moment but we'll probably compress the entire `public` folder in the future.

mzoon compresses files when the app has been built in the release mode. The result is three files instead of one: `file.xxx` (the original), `file.xxx.gz` and `file.xxx.br`. Then Moon serves them according to the `ACCEPT_ENCODING` header sent by clients.  

We would use only [Brotli](https://developer.mozilla.org/en-US/docs/Glossary/brotli_compression) algorithm because it produces the smallest files but Firefox supports only [Gzip](https://developer.mozilla.org/en-US/docs/Glossary/brotli_compression) over HTTP. All browsers support Brotli with HTTPS.

_Note:_ If we decide to compress non-cacheable dynamic content - like messages between frontend and backend - then we will probably choose Gzip because it's faster than Brotli.

Let's look at the implementation. The first snippet is from [/crates/mzoon/src/helper/file_compressor.rs](https://github.com/MoonZoon/MoonZoon/blob/32362a38a35e0d57b291503516de0de2c1c55fc6/crates/mzoon/src/helper/file_compressor.rs):

```rust
use crate::helper::ReadToVec;
use async_trait::async_trait;
use brotli::{enc::backward_references::BrotliEncoderParams, CompressorReader as BrotliEncoder};
use flate2::{bufread::GzEncoder, Compression as GzCompression};
// ...

#[async_trait]
pub trait FileCompressor {
    async fn compress_file(content: Arc<Vec<u8>>, path: &Path, extension: &str) -> Result<()> {
        let path = compressed_file_path(path, extension);
        let mut file_writer = fs::File::create(&path)
            .await
            .with_context(|| format!("Failed to create the file {:#?}", path))?;

        let compressed_content = spawn_blocking(move || Self::compress(&content)).await??;

        file_writer.write_all(&compressed_content).await?;
        file_writer.flush().await?;
        Ok(())
    }

    fn compress(bytes: &[u8]) -> Result<Vec<u8>>;
}
//...
// ------ Brotli ------

pub struct BrotliFileCompressor;

#[async_trait]
impl FileCompressor for BrotliFileCompressor {
    fn compress(bytes: &[u8]) -> Result<Vec<u8>> {
        BrotliEncoder::with_params(bytes, 0, &BrotliEncoderParams::default()).read_to_vec()
    }
}

// ------ Gzip ------

pub struct GzipFileCompressor;

#[async_trait]
impl FileCompressor for GzipFileCompressor {
    fn compress(bytes: &[u8]) -> Result<Vec<u8>> {
        GzEncoder::new(bytes, GzCompression::best()).read_to_vec()
    }
}
```
- `#[async_trait]` allows us to write `async` methods in traits. (The crate [async_trait](https://crates.io/crates/async-trait), from the author of `anyhow` and `thiserror`.)

- The combination of `async-trait` and `fehler` is deadly for the Rust compiler. That's why you see `Ok(())` + `Result<()>` instead of `#[throws]`. I'm not sure if it's `async-trait` or `fehler` problem, feel free to investigate it more and let me know.

- We need to call `spawn_blocking` instead of `spawn` to move compression to a new thread because both encoders / compressors are blocking. I was trying to use [async-compression](https://crates.io/crates/async-compression), but there was a bug probably somewhere close to the `GzEncoder` - the MZ example `counter` was producing a wasm file that had always only `9KB` instead of `16KB`. Also I had to use `async-compression`'s `futures` encoders with the [compat layer](https://docs.rs/tokio-util/0.6.7/tokio_util/compat/index.html) to resolve the problem with incompatible `tokio` versions. Feel free to investigate it more and let me know.

- _Tip_: Don't forget to call `.flush()` after `.write_all()`. Sometimes it works without `.flush()`, sometimes it doesn't, so it's difficult to debug.

- `read_to_vec` is a custom helper - see [/crates/mzoon/src/helper/read_to_vec.rs](https://github.com/MoonZoon/MoonZoon/blob/32362a38a35e0d57b291503516de0de2c1c55fc6/crates/mzoon/src/helper/read_to_vec.rs).

- Both encoders are set to compress in the best quality (i.e. to produce the smallest files at the cost of speed).

The second and the last snippet is from [/crates/mzoon/src/build_frontend.rs](https://github.com/MoonZoon/MoonZoon/blob/32362a38a35e0d57b291503516de0de2c1c55fc6/crates/mzoon/src/build_frontend.rs):

```rust
use futures::TryStreamExt;

#[throws]
async fn compress_pkg(wasm_file_path: impl AsRef<Path>, js_file_path: impl AsRef<Path>) {
    try_join!(
        create_compressed_files(wasm_file_path),
        create_compressed_files(js_file_path),
        visit_files("frontend/pkg/snippets")
            .try_for_each_concurrent(None, |file| create_compressed_files(file.path()))
    )?
}

#[throws]
async fn create_compressed_files(file_path: impl AsRef<Path>) {
    let file_path = file_path.as_ref();
    let content = Arc::new(fs::File::open(&file_path).await?.read_to_vec().await?);

    try_join!(
        BrotliFileCompressor::compress_file(Arc::clone(&content), file_path, "br"),
        GzipFileCompressor::compress_file(content, file_path, "gz"),
    )
    .with_context(|| format!("Failed to create compressed files for {:#?}", file_path))?
}
```
- All files are compressed and generated in parallel thanks to `spawn_blocking` (explained before) and thanks to `tokio::fs` (we don't block the working thread by waiting for OS file operations). 

- `visit_files` is a stream of files (explained in the next section). It works nice with the function [try_for_each_concurrent](https://docs.rs/futures/0.3.15/futures/stream/trait.TryStreamExt.html#method.try_for_each_concurrent).

## File Visitor

When you want to iterate over all files in the given directory and its nested folders, then it's relatively straightforward with the standard Rust library. Just go to the Rust docs for `std::fs::read_dir` and copy the provided [example](https://doc.rust-lang.org/std/fs/fn.read_dir.html#examples). Also there is chance we'll see the function [fs::read_dir_all](https://github.com/rust-lang/rust/issues/69684) in `std`. Or you can use the crate [walkdir](https://crates.io/crates/walkdir) from a very experienced maintainer of many Rust libraries.

However the Rust async world is still pretty new and messy. If I chose `smol` instead of `tokio` and was brave enough to use the library with only 602 downloads, then I would probably integrate the crate [async_walkdir](https://crates.io/crates/async-walkdir).

Another approach would be to use `walkdir` to create a list of files and then process the list as needed in parallel. However it doesn't sound as a clean solution and in the case of a large directory tree, you want to return early when the processing fails or when your file search is complete.

I'm not a big fan or recursive functions because:
- They often lead to increased cognitive load.
- Stack overflow is difficult to catch and debug.
- Rust doesn't have a good support for TCO/TCE (_tail call optimization / elimination_), although there are some libraries like [Tailcall](https://crates.io/crates/tailcall) and maybe promising news in [rust-lang/rfcs](https://github.com/rust-lang/rfcs/issues/2691).
- You often need to use `Box` in Rust recursive constructs (both functions and types need boxed items). The crate [async-recursion](https://crates.io/crates/async-recursion) basically just wraps the `Future` into a `Box`.
- [Why does NASA not allow recursion?](https://craftofcoding.wordpress.com/2021/03/08/why-does-nasa-not-allow-recursion/)

Fortunately during intensive reading and searching for a better solution, I've found a nice answer on [stackoverflow.com](https://stackoverflow.com/a/58825638) compatible with `tokio` and `futures`. I've refactored it a little bit and saved to [/crates/mzoon/src/helper/visit_files.rs](https://github.com/MoonZoon/MoonZoon/blob/32362a38a35e0d57b291503516de0de2c1c55fc6/crates/mzoon/src/helper/visit_files.rs). The code:

```rust
pub fn visit_files(path: impl Into<PathBuf>) -> impl Stream<Item = Result<DirEntry>> + Send + 'static {
    #[throws]
    async fn one_level(path: PathBuf, to_visit: &mut Vec<PathBuf>) -> Vec<DirEntry> {
        let mut dir = fs::read_dir(path).await?;
        let mut files = Vec::new();

        while let Some(child) = dir.next_entry().await? {
            if child.metadata().await?.is_dir() {
                to_visit.push(child.path());
            } else {
                files.push(child)
            }
        }
        files
    }

    stream::unfold(vec![path.into()], |mut to_visit| {
        async {
            let path = to_visit.pop()?;
            let file_stream = match one_level(path, &mut to_visit).await {
                Ok(files) => stream::iter(files).map(Ok).left_stream(),
                Err(error) => stream::once(async { Err(error) }).right_stream(),
            };
            Some((file_stream, to_visit))
        }
    })
    .flatten()
}
```
(Let me know if you know a better solution or a suitable library.)

## Wasm-pack installer



---

# Fails

---

And that's all for today! 
Thank You for reading and I hope you are looking forward to the next episode.

Martin

P.S.
We are waiting for you on [Discord](https://discord.gg/eGduTxK2Es).


