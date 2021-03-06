# MoonZoon Dev News (1): CLI, Build pipeline, Live-reload, HTTPS

It's alive! It runs!

[![Auto-reload](images/autoreload.gif)](https://raw.githubusercontent.com/MoonZoon/MoonZoon/main/docs/images/autoreload.gif)

It... doesn't do anything useful yet. Just like an average kitten. We have to wait until they grow up - but who doesn't like to watch progress?

---

<p align="center">
  <strong>Welcome to the MoonZoon Dev News!</strong>
</p>

<p align="center">
  <img src="images/MoonZoon.png" width="200" title="MoonZoon logo">
</p>


[MoonZoon](https://moonzoon.rs) is a [Rust](https://www.rust-lang.org/) full-stack framework. If you want to read about new MZ features, architecture and interesting problems & solutions - Dev News is the right place.

---

There are two big news. I've written my first [tweet](https://twitter.com/MartinKavik/status/1362863940175241216) ever! And also a couple MoonZoon lines of code - a build pipeline, live-reload, certificate generator and servers ([GitHub PR](https://github.com/MoonZoon/MoonZoon/pull/1)).

Awesome [Discord](https://discord.gg/eGduTxK2Es) friends tested it on _Fedora_, _Ubuntu_, _Windows_ and _macOS_ with _Chrome_, _Firefox_ and _Safari_. Live-reload works also on my older iPhone SE. Thanks `@adsick`, `@UberIntuiter` and `@EvenWei`!

**Follow [these steps](https://github.com/MoonZoon/MoonZoon/blob/main/docs/development.md) to try it by yourself.**

---

# How the build process works?

When you run in *`examples/counter`* the command
```sh
cargo run --manifest-path "../../crates/mzoon/Cargo.toml" start
# or in the future:
mzoon start
```
then:

1. MZoon (aka MoonZoon CLI) loads the project's `MoonZoon.toml`. It contains only configuration for file watchers atm:
    ```toml
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

1. MZoon checks if [wasm-pack](https://rustwasm.github.io/wasm-pack/) exists and panics if doesn't. 
   - _Note:_ MZoon will automatically install the required `wasm-pack` version defined in `MoonZoon.toml` and check the compatible Rust version in the future.

1. MZoon generates a certificate for the `localhost` domain using [rcgen](https://crates.io/crates/rcgen). The result is two files - `private.pem` and `public.pem` - saved in the `backend/private` directory. 
   - _Note:_ Git ignores the `private` directory content.
   - _Warning_: I recommend to set the certificate's serial number explicitly to a unique value (you can use [the current unix time](https://www.unixtimestamp.com)). Otherwise Firefox may fail to load your app with the error code [SEC_ERROR_REUSED_ISSUER_AND_SERIAL](https://support.mozilla.org/en-US/kb/Certificate-contains-the-same-serial-number-as-another-certificate).

1. `wasm-pack` builds the frontend part. If you used the parameter `-r / --release` together with the command `start`, then it builds in the release mode and also optimizes the output (`.wasm` file) for size.

1. A unique `frontend_build_id` is generated and saved to the `examples/pkg/build_id`. The _build id_ is added as a name suffix to some files in the `pkg` directory. It's a cache busting mechanism because `pkg` files are served by the backend.
   - _Note_: `pkg` is generated by `wasm-pack` and its content is ignored by Git.

1. The frontend file watcher is set according to the paths in `MoonZoon.toml`. It sends an empty POST request to Moon (`https://127.0.0.1:8443/api/reload`) on a file change.
   - _Warning_: Browsers treat unregistered self-signed certificates as invalid so we must allow the acceptance of such certificates before we fire the request:
      ```rust
      reqwest::blocking::ClientBuilder::new()
          .danger_accept_invalid_certs(true)
      ```

1. `cargo run` builds and starts the backend. MZoons sets the backend file watcher and saves a generated `backend_build_id` to `backend/private/build_id`.
   - _Note_: If you like async spaghetti, you won't be disappointed by looking at the [related code](https://github.com/MoonZoon/MoonZoon/blob/7fe6957bf6dcf6ea4d53ce8eee3ef4fb883ae3cb/crates/mzoon/src/main.rs#L173-L231). Why?
     - We can't easily split `cargo run` to standalone "build" and "run" parts. We ideally need something like [cargo run --no-build](https://github.com/rust-lang/cargo/issues/3773#issuecomment-787782106).
     - We need to handle "Ctrl+C signal".
     - We need to somehow find out when the backend has been started or turned off (from the MZoon's point of view).
     - (Don't worry, I'll refactor it later and probably rewrite with an async runtime.)

---

# How the backend works?

The backend part consists two [Warp](https://crates.io/crates/warp) servers.
  - The HTTPS one runs on the port `8443`. It uses generated certificates.
  - The HTTP one runs on the port `8080` and redirects to the HTTPS one.
     - _Question_: What's the best way of HTTP -> HTTPS redirection? I don't like the [current code](https://github.com/MoonZoon/MoonZoon/blob/7fe6957bf6dcf6ea4d53ce8eee3ef4fb883ae3cb/crates/moon/src/lib.rs#L122-L136).

We need HTTPS server because otherwise browsers can't use HTTP/2. And we need HTTP/2 to eliminate [SSE limitations](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#receiving_events_from_the_server). Also it's better to use HTTPS on the local machine to make the dev environment similar to the production one.

Both servers binds to `0.0.0.0` (instead of `127.0.0.1`) to make servers accessible outside of your development machine. It means you can connect to your dev servers with your phone on the address like `https://192.168.0.1:8443`.

I assume some people will need to use custom dev domains, sub-domains, ports, etc. Let me know when it happens.

_Tip:_ I recommend to test server performance through an IP address and not a domain (`localhost`) because DNS resolving could be slow.

I've chosen Warp because I wanted a simpler server with HTTP/2 support. Also I have a relatively good experience with [hyper](https://crates.io/crates/hyper) (Warp's HTTP implementation) from writing a proxy server for my client.

---

# How live-reload works?

When you go to `https://127.0.0.1:8443`, the `frontend` route is selected by Warp in the Moon. Moon responds with a generated [HTML + Javascript code](https://github.com/MoonZoon/MoonZoon/blob/7fe6957bf6dcf6ea4d53ce8eee3ef4fb883ae3cb/crates/moon/src/lib.rs#L173-L221). 

HTML and JS for app initialization aren't very interesting so let's focus on the live-reload code (_Note_: I know, the code needs refactor, but it should be good enough for explanation):

```js
<script type="text/javascript">
    {reconnecting_event_source}
    var uri = location.protocol + '//' + location.host + '/sse';
    var sse = new ReconnectingEventSource(uri);
    ...
    sse.addEventListener("reload", function(msg) {
        sse.close();
        location.reload();
    });
</script>
```

What's `{reconnecting_event_source}`? And why I see `ReconnectingEventSource` instead of [EventSource](https://developer.mozilla.org/en-US/docs/Web/API/EventSource)? 

Well, welcome to the world of browsers where nothing works as expected, specs are unreadable and jQuery and polyfills are still needed.

`{reconnecting_event_source}` is a placeholder for the [ReconnectingEventSource](https://github.com/fanout/reconnecting-eventsource) code. The library description:

> This is a small wrapper library around the JavaScript EventSource API to ensure it maintains a connection to the server. Normally, EventSource will reconnect on its own, however there are some cases where it may not. This library ensures a reconnect always happens.

I've already found such "edge-case" - just run the app in Firefox and restart backend. Firefox permanently closes the connection. Chrome (and I hope other browsers) try to reconnect as expected. _Question_: Do you know a better solution?

Let's move forward and look at this snippet:

```js
sse.addEventListener("reload", function(msg) {
    sse.close();
    location.reload();
});
```
It means we listen for messages with the event name `reload`. Moon creates them in the `POST /api/reload` endpoint this way:
```rust
Event::default().event("reload").data(""))
``` 
_Warning_: Notice the empty string in `data("")`. It doesn't work without it.

We should call `sse.close()` if we don't want to see an ugly error message in some console logs when the browser kills the connection on reload.

The last part, hidden under the `...` mark in the first code snippet, is:
```js
var backendBuildId = null;
sse.addEventListener("backend_build_id", function(msg) {
    var newBackendBuildId = msg.data;
    if(backendBuildId === null) {
        backendBuildId = newBackendBuildId;
    } else if(backendBuildId !== newBackendBuildId) {
        sse.close();
        location.reload();
    }
});
``` 
The only purpose of this code is to reload the frontend when the backend has been changed. The backend sends the message `backend_build_id` automatically when the client connects to `/sse` endpoint - i.e. when the SSE connection has been opened.

---

And that's all for today! 
Thank You for reading and I hope you are looking forward to the next episode.

Martin

P.S.
We are waiting for you on [Discord](https://discord.gg/eGduTxK2Es).

