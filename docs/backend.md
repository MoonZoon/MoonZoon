# Backend - Moon

---

## Basics

The **Chat** example:

```rust
use moon::*;
use shared::{UpMsg, DownMsg, Message};

async fn init() {}

async fn frontend() -> Frontend {
    Frontend::new().title("Chat example")
}

async fn up_msg_handler(req: UpMsgRequest) {
    if let UpMsg::SendMessage(message) = req.up_msg {
        join_all(connected_client::by_id().iter().map(|(_, client)| {
            client.send_down_msg(message, req.cor_id)
        })).await
    }
}

fn main() {
    start!(init, frontend, up_msg_handler);
}
```

### 1. The App Initialization

1. The function `main` is invoked automatically when the Moon app is started on the server.
1. The function `init` is invoked when the Moon is ready to work.
1. The function `frontend` is invoked when the web browser wants to download and run the client (Zoon) app.
1. The function `up_msg_handler` handles requests from the Zoon.

### 2. Calling Actor Functions
1. We need to find the needed _actors_. They are stored in _indices_. `connected_client::by_id` is a Moon's system index where each value is an actor representing a connected Zoon app.
1. All public actor functions are asynchronous so we have to `await` them an ideally call them all at once in this example to improve the performance a bit. 
   - _Note_: The requested actor may live in another server or it doesn't live at all - then the Moon has to start it and load its state into the memory before it can process your call. And all those operations and the business logic processing take some time, so asynchronicity allows you to spend the time in better ways than just waiting.  

---

## Actors

The **Time Tracker** example parts:

```rust
fn main() {
    start!(init, frontend, up_msg_handler, actors![
        client, 
        invoice, 
        ...
    ]);
}
...

async fn up_msg_handler(req: UpMsgRequest) {
    let down_msg = match req.up_msg {
        ...
        // ------ Invoice ------
        UpMsg::AddInvoice(time_block, id) => {
            check_access!(req);
            new_actor(InvoiceArgs { time_block, id }).await;
            DownMsg::InvoiceAdded
        },
        ...
    }
}
```

```rust
use shared::{InvoiceId, TimeBlockId};

actor!{
    #[args]
    struct InvoiceArgs {
        time_block: TimeBlockId,
        id: InvoiceId,
    }  

    // ------ Indices ------

    #[index]
    fn by_id() -> Index<InvoiceId, InvoiceActor> {
        index("invoice_by_id", |_| id())
    }

    #[index]
    fn by_time_block() -> Index<ClientId, InvoiceActor> {
        index("invoice_by_time_block", |_| time_block())
    }

    // ------ PVars ------

    #[p_var]
    fn id() -> PVar<InvoiceId> {
        p_var("id", |_| args().map(|args| args.id))
    }

    #[p_var]
    fn custom_id() -> PVar<String> {
        p_var("custom_id", |_| String::new())
    }

    #[p_var]
    fn url() -> PVar<String> {
        p_var("url", |_| String::new())
    }

    #[p_var]
    fn time_block() -> PVar<ClientId> {
        p_var("time_block", |_| args().map(|args| args.time_block))
    }

    // ------ Actor ------

    #[actor]
    struct InvoiceActor;
    impl InvoiceActor {
        async fn remove(&self) {
            self.remove_actor().await
        }
    
        async fn set_custom_id(&self, custom_id: String) {
            custom_id().set(custom_id).await
        }

        async fn set_url(&self, url: String) {
            url().set(url).await
        }

        async fn id(&self) -> InvoiceId {
            id().inner().await
        }

        async fn custom_id(&self) -> String {
            custom_id().inner().await
        }

        async fn url(&self) -> String {
            url().inner().await
        }
    }
}

```

### Args

### Indices

### PVars
