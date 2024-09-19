# Backend - Moon

---

## Basics

The **Chat** example:

```rust
use moon::*;
use shared::{DownMsg, UpMsg};

async fn frontend() -> Frontend {
    Frontend::new().title("Chat example").append_to_head(
        "
        <style>
            html {
                background-color: black;
            }
        </style>",
    )
}

async fn up_msg_handler(req: UpMsgRequest<UpMsg>) {
    println!("{:?}", req);

    let UpMsgRequest { up_msg, cor_id, .. } = req;
    let UpMsg::SendMessage(message) = up_msg;

    sessions::broadcast_down_msg(&DownMsg::MessageReceived(message), cor_id).await;
}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
```

### 1. The App Initialization

1. The function `main` is invoked.

1. The `frontend` function returns HTML similar to a standard `index.html` with scripts for starting the frontend app back to the web browser.
   - Requests with the url path starting with `_api` won't trigger the function.

1. The function `up_msg_handler` handles message requests from the Zoon. Zoon sends in the `UpMsgRequest`:
   - Your `UpMsg`.
   - New `CorId` (aka [_correlation id_](https://www.rapid7.com/blog/post/2016/12/23/the-value-of-correlation-ids/)) generated for each request.
   - `SessionId` generated in the Zoon app before it connects to the Moon.
   - `Option<AuthToken>` containing `String` defined in your Zoon app.

### 2. Calling Actor functions

`sessions` are [_virtual actors_](https://www.microsoft.com/en-us/research/publication/orleans-distributed-virtual-actors-for-programmability-and-scalability/) managed by the Moon. Each `SessionActor` represents a live connection between Zoon and Moon apps.

You can send your `DownMsg` to all connected Zoon apps by calling `sessions::broadcast_down_msg` (demonstrated in the code snippet above).

If you want to send the message to only one `session` (e.g. to simulate a standard request-response mechanism):
```rust
let UpMsgRequest { up_msg, cor_id, session_id, .. } = req;
let UpMsg::SendMessage(message) = up_msg;

sessions::by_session_id()
    .get(session_id)
    .unwrap()
    .send_down_msg(&DownMsg::MessageReceived(message), cor_id).await;
```

Where `by_session_id()` returns an _actor index_. Then we try to find the actor and call its method `send_down_msg`.

_Notes_: 

- All actor methods are asynchronous because the requested actor may live in another server or it doesn't live at all - then the Moon app has to start it and load its state into the main memory before it can process your call. And all those operations and the business logic processing take some time so asynchronicity allows you to spend the time in better ways than just waiting.

- Index API will change a bit during the future development to support server clusters (e.g. `get` will be probably `async`).

---

## Moonlight

`moonlight` is the crate connecting Zoon and Moon worlds. It contains things like `CorId`, `SessionId` and `AuthToken` that are used on the both sides.

You need it to define your `UpMsg` and `DownMsg`. See the content of `/examples/chat/shared/src/lib.rs` below.

```rust
use moonlight::*;

// ------ UpMsg ------

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "serde")]
pub enum UpMsg {
    SendMessage(Message),
}

// ------ DownMsg ------

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "serde")]
pub enum DownMsg {
    MessageReceived(Message),
}

// ------ Message ------

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "serde")]
pub struct Message {
    pub username: String,
    pub text: String,
}
```

---

## Actix

Moon is based on [Actix](https://actix.rs/). And there is a way to use Actix directly:

```rust
use moon::{
    actix_web::{web, Responder},
    *,
};

async fn frontend() -> Frontend {
    Frontend::new().title("Actix example")
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

async fn hello() -> impl Responder {
    "Hello!"
}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |cfg| {
        cfg.route("/hello", web::get().to(hello));
    })
    .await
}
```

![Hello](images/hello.png)

- `cfg` in the example is [actix_web::web::ServiceConfig](https://docs.rs/actix-web/latest/actix_web/web/struct.ServiceConfig.html)

- You can also replace default middlewares and create an Actix `App` instance by yourself. It's often useful when you are migrating your Actix app to MoonZoon. See the example `start_with_app` for more info.

- We can no longer use native Actix proc macros (e.g. `#[get("hello")]`) with the latest Actix versions because of the changes in their implementations that break reimporting (it's a common problem in Rust).

---

## Golem integration

- https://github.com/golemcloud/golem
- MoonZoon example: `/examples/golem_chat` (WIP)
- TODO

