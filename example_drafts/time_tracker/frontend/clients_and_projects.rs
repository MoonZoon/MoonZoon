use zoon::*;
use ulid::Ulid;
use crate::app;

pub mod view;

type ClientId = Ulid;
type ProjectId = Ulid;

zoons!{
    append_zoons![view]

    // ------ Client ------

    #[derive(Debug)]
    pub struct Client {
        id: ClientId,
        name: String,
        projects: Vec<Model<Project>>,
    }

    #[model]
    fn clients() -> Option<Vec<Model<Client>>> {
        None
    }

    #[update]
    fn set_clients(clients: Vec<shared::ClientsAndProjectsClient>) {
        clients.set(....)
    }

    // ------ Project ------

    #[derive(Debug)]
    struct Project {
        id: ProjectId,
        name: String,
        client: Model<Client>, 
    }

}
