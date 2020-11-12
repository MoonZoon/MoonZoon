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
        projects: Vec<Var<Project>>,
    }

    #[derive(Debug)]
    struct Project {
        id: ProjectId,
        name: String,
        client: Var<Client>, 
    }

    #[var]
    fn clients() -> Option<Vec<Var<Client>>> {
        None
    }

    #[subscription]
    fn handle_down_msg() {
        app::down_msg().inner().try_update(|down_msg| {
            match down_msg {
                Some(DownMsg::ClientsAndProjectsClients(clients)) => {
                    set_clients(clients);
                    None
                }
                _ => down_msg
            }
        });
    }

    #[update]
    fn set_clients(clients: Vec<shared::ClientsAndProjectsClient>) {
        clients.set(....)
    }

}
