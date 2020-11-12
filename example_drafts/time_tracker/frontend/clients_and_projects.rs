use zoon::*;
use ulid::Ulid;
use crate::app;

pub mod view;

type ClientId = Ulid;
type ProjectId = Ulid;

zoons!{
    append_zoons![view]

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

    #[subscription]
    fn handle_down_msg() {
        app::down_msg().inner().try_update_owned(|down_msg| {
            match down_msg {
                Some(DownMsg::ClientsAndProjectsClients(clients)) => {
                    set_clients(clients)
                }
                _ => return down_msg
            }
            None
        });
    }

    #[update]
    fn set_clients(clients: Vec<shared::ClientsAndProjectsClient>) {
        clients.set(....)
    }

}
