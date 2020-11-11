use zoon::*;
use ulid::Ulid;
use crate::app;

pub mod view;

type ClientId = Ulid;
type ProjectId = Ulid;

zoons!{
    append_zoons![view]

    #[subscription]
    fn on_down_msg_received() {
        app::down_msg_received();
        handle_down_msg();
    }

    #[update]
    fn handle_down_msg() {
        app::down_msg().update_owned(|down_msg| {
            if let Some(DownMsg::ClientsAndProjectsClients(clients)) = down_msg {
                set_clients(clients);
                return None;
            } 
            down_msg
        });
    }

    #[derive(Debug)]
    pub struct Client {
        id: ClientId,
        name: String,
        projects: Vec<Model<Project>>,
    }

    #[derive(Debug)]
    struct Project {
        id: ProjectId,
        name: String,
        client: Model<Client>, 
    }

    #[model]
    fn clients() -> Option<Vec<Model<Client>>> {
        None
    }

    #[update]
    fn set_clients(clients: Vec<shared::ClientsAndProjectsClient>) {
        clients.set(....)
    }

}
