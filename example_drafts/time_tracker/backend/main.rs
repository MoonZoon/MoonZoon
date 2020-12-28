use moon::*;
use shared::{UpMsg, DownMsg, Message, ClientId, ProjectId};

mod client;
mod project;

async fn up_msg_handler(msg: UpMsg) -> impl Into<Option<DownMsg>> {
    match msg {
        // ------ Page data ------
        UpMsg::GetClientsAndProjectsClients => {
            let shared_clients_futs = client::by_id().iter().map(|(client_id, client)| {
                async {
                    let (client_name, projects) = join(client.name(), client.projects()).await;
                    let shared_projects_futs = projects.iter().map(|(project_id, project)| {
                        async {
                            let project_name = project.name().await;
                            shared::clients_and_projects::Project {
                                id: project_id,
                                name: project_name
                            }
                        }
                    });
                    shared::clients_and_projects::Client {
                        id: client_id,
                        name: client_name,
                        projects: join_all(shared_projects_futs).await 
                    }
                }
            });
            DownMsg::ClientsAndProjectsClients(
                join_all(shared_clients_futs).await
            )
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

fn main() {
    start!(up_msg_handler, actors![client, project]);
}
