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
                new_actor(Client { id: client_id });
                DownMsg::ClientAdded
            },
            UpMsg::RemoveClient(client_id) => {
                let client = client::by_id().get(client_id)[0];
                client.send_in_msg(InMsg::Remove);
                DownMsg::ClientRemoved
            },
            UpMsg::RenameClient(client_id, name) => {
                let client = client::by_id().get(client_id)[0];
                client.send_in_msg(InMsg::Rename(name.to_string()));
                DownMsg::ClientRenamed
            },
            // ------ Project ------
            UpMsg::AddProject(client_id, project_id) => {
                new_actor(Project { client: client_id, id: project_id });
                DownMsg::ProjectAdded
            },
            UpMsg::RemoveProject(project_id) => {
                let project = project::by_id().get(project_id)[0];
                project.send_in_msg(InMsg::Remove);
                DownMsg::ProjectRemoved
            },
            UpMsg::RenameProject(project_id, name) => {
                let project = project::by_id().get(project_id)[0];
                project.send_in_msg(InMsg::Rename(name.to_string()));
                DownMsg::ProjectRenamed
            },
        }
    }
}

fn main() {
    start!()
}
