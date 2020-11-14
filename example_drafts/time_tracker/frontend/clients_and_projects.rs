use zoon::*;
use ulid::Ulid;
use std::borrow::Cow;
use crate::app;
use shared::{ClientId, ProjectId};

pub mod els;

blocks!{
    append_blocks![els]

    #[subscription]
    fn on_route_change() {
        if let app::Route::ClientsAndProjects = route() {
            added_project().set(None);
            added_client().set(None);
            app::send_up_msg(false, UpMsg::GetClientsAndProjectsClients);
        }
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

    // ------ Client ------

    #[derive(Debug)]
    pub struct Client {
        id: ClientId,
        name: String,
        projects: Vec<Var<Project>>,
    }

    #[var]
    fn clients() -> Option<Vec<Var<Client>>> {
        None
    }

    #[var]
    fn added_client() -> Option<Var<Client>> {
        None
    }

    #[update]
    fn set_clients(clients: Vec<shared::clients_and_projects::Client>) {
        let clients = clients.into_iter().map(|client| {
            let client_var = Var::new(Client {
                id: client.id,
                name: client.name,
                projects: Vec::new(),
            });
            client_var.update_mut(|new_client| {
                new_client.projects = client.projects.into_iter().map(|project| {
                    Var::new(Project {
                        id: project.id,
                        name: project.name,
                        client: client_var,
                    })
                }).collect()
            });
            client_var
        }).collect();
        clients().set(Some(clients));
    }

    #[update]
    fn add_client() {
        let client = Client {
            id: ClientId::new(),
            name: String::new(),
            projects: Vec::new(),
        };
        app::send_up_msg(true, UpMsg::AddClient(client.id));
        clients().update_mut(move |clients| {
            if let Some(clients) = clients {
                let client = Var::new(client);
                added_client().set(Some(client));
                clients.push(Var::new(client));
            }
        });
    } 

    #[update]
    fn remove_client(client: Var<Client>) {
        clients().update_mut(|clients| {
            if let Some(clients) = clients {
                if let Some(position) = clients.iter().position(|c| c == client) {
                    clients.remove(position);
                }
            }
        });
        if let Some(client) = client.try_remove() {
            for project in client.projects {
                remove_project(project);
            }
            app::send_up_msg(true, UpMsg::RemoveClient(client.id));
        }
    }

    #[update]
    fn rename_client(client: Var<Project>, name: &str) {
        client.try_use_ref(|client| {
            app::send_up_msg(true, UpMsg::RenameClient(client.id, Cow::from(name)));
        });
    }

    // ------ Project ------

    #[derive(Debug)]
    struct Project {
        id: ProjectId,
        name: String,
        client: Var<Client>, 
    }

    #[var]
    fn added_project() -> Option<Var<Project>> {
        None
    }

    #[update]
    fn add_project(client: Var<Client>) {
        let project = Project {
            id: ProjectId::new(),
            name: String::new(),
            client,
        };
        client.try_update_mut(move |client| {
            app::send_up_msg(true, UpMsg::AddProject(client.id, project.id));
            let project = Var::new(project);
            added_project().set(Some(project));
            client.projects.push(project);
        });
    }

    #[update]
    fn remove_project(project: Var<Project>) {
        if let Some(removed_project) = project.try_remove() {
            app::send_up_msg(true, UpMsg::RemoveProject(removed_project.id));
            removed_project.client.try_update_mut(|client| {
                if let Some(position) = client.projects.iter().position(|p| p == project) {
                    clients.projects.remove(position);
                }
            });
        }
    }

    #[update]
    fn rename_project(project: Var<Project>, name: &str) {
        project.try_use_ref(|project| {
            app::send_up_msg(true, UpMsg::RenameProject(project.id, Cow::from(name)));
        });
    }

}
