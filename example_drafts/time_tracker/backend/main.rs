use moon::*;
use shared::{UpMsg, DownMsg, Message, ClientId, ProjectId};

mod client;
mod project;

blocks!{
    append_actors![client, project]

    #[var]
    fn connector() -> Connector<UpMsg, DownMsg> {
        Connector::new("9000", up_msg_handler)
    }

    // @TODO `cor_id`?
    // @TODO async + Result?
    // @TODO #[index] in actor! or here?

    fn up_msg_handler(msg: UpMsg) -> DownMsg {
        match msg {
            // ------ Page data ------
            UpMsg::GetClientsAndProjectsClients => {
                // let mut shared_clients: Vec<shared::clients_and_projects::Client> = Vec::new();
                // for (id, client) in client::by_id() {
                //     let projects
                // }
            }
            // ------ Client ------
            UpMsg::AddClient(client_id) => {
                new_actor(Client { id: client_id }).await;
                DownMsg::ClientAdded
            },
            UpMsg::RemoveClient(client_id) => {
                client::by_id().get(client_id)[0].remove().await;
                DownMsg::ClientRemoved
            },
            UpMsg::RenameClient(client_id, name) => {
                client::by_id().get(client_id)[0].rename(name.to_string()).await;
                DownMsg::ClientRenamed
            },
            // ------ Project ------
            UpMsg::AddProject(client_id, project_id) => {
                new_actor(Project { client: client_id, id: project_id }).await;
                DownMsg::ProjectAdded
            },
            UpMsg::RemoveProject(project_id) => {
                project::by_id().get(project_id)[0].await().remove().await;
                DownMsg::ProjectRemoved
            },
            UpMsg::RenameProject(project_id, name) => {
                project::by_id().get(project_id)[0].rename(name.to_string()).await;
                DownMsg::ProjectRenamed
            },
        }
    }
}

fn main() {
    start!()
}
