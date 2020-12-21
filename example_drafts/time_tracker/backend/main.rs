use moon::*;
use shared::{UpMsg, DownMsg, Message};
use ulid::Ulid;

blocks!{
    #[var]
    fn connector() -> Connector<UpMsg, DownMsg> {
        Connector::new("9000", up_msg_handler)
    }

    // @TODO `cor_id`?
    // @TODO async + Result?
    // @TODO #[index] in actor! or here?

    fn up_msg_handler(msg: UpMsg) -> DownMsg {
        match msg {
            UpMsg::AddClient(client_id) => {
                actor(Client { id: client_id });
                DownMsg::ClientAdded
            },
            UpMsg::RemoveClient(client_id) => {
                let client = get_actor("Clients", client_id);
                client.send_in_msg(InMsg::Remove);
                DownMsg::ClientRemoved
            },
            UpMsg::RenameClient(client_id, name) => {
                let client = get_actor("Clients", client_id);
                client.send_in_msg(InMsg::Rename(name.to_string()));
                DownMsg::ClientRenamed
            },
        }
    }

    // #[update]
    // fn broadcast_message(message: Message) {
    //     connector().use_ref(move |connector| {
    //         connector.broadcast(DownMsg::MessageReceived(message))
    //     })
    // }
}

actor!{
    type ClientId = Ulid;

    #[new_actor]
    struct Client {
        id: ClientId,
    }  

    #[p_var]
    fn id() -> ClientId {
        p_var("id", &["Clients"], |_| new_actor().id)
    }

    #[p_var]
    fn name() -> String {
        p_var("name", &[], |_| String::new())
    }

    #[in_msg]
    enum InMsg {
        Remove,
        Rename(String),
    }

    #[in_msg_handler]
    fn in_msg_handler(in_msg: InMsg) {
        match msg {
            InMsg::Remove => {
                remove_actor();
            }
            InMsg::Rename(name) => {
                name().set(name);
            }
        }
    }
}

fn main() {
    start!()
}
