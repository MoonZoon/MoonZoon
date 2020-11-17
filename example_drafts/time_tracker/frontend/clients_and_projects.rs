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
            set_clients(None);
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
                    set_clients(Some(clients));
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
    fn client_event_handler() -> VarEventHandler<Client> {
        VarEventHandler::new(|event, client| {
            match event {
                VarAdded => {
                    clients().update_mut(|clients| {
                        if let Some(clients) = clients {
                            clients.push(client);
                        }
                    });
                },
                VarChanged => (),
                VarRemoved => {
                    client.use_ref(|client| {
                        stop!{
                            for project in &client.projects {
                                project.try_remove();
                            }
                        }
                    });
                    clients().update_mut(|clients| {
                        if let Some(clients) = clients {
                            if let Some(position) = clients.iter().position(|c| c == client) {
                                clients.remove(position);
                            }
                        }
                    });
                },
            }
        })
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
    fn set_clients(clients: Option<Vec<shared::clients_and_projects::Client>>) {
        let clients = match {
            Some(clients) => clients,
            None => return clients().set(None);
        };
        stop!{
            clients().set(Some(Vec::new()));
            clients.into_iter().for_each(|client| {
                let client_var = var(Client {
                    id: client.id,
                    name: client.name,
                    projects: Vec::new(),
                });
                for project in client.projects {
                    var(Project {
                        id: project.id,
                        name: project.name,
                        client: client_var,
                    });
                }
            });
        }
    }

    #[update]
    fn add_client() {
        let id = ClientId::new();
        let client = var(Client {
            id,
            name: String::new(),
            projects: Vec::new(),
        });
        added_client().set(Some(client));
        app::send_up_msg(true, UpMsg::AddClient(id));
    } 

    #[update]
    fn remove_client(client: Var<Client>) {
        if let Some(client) = client.try_remove() {
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
    fn project_event_handler() -> VarEventHandler<Project> {
        VarEventHandler::new(|event, project| {
            match event {
                VarAdded => {
                    project.use_ref(|project| {
                        project.client.try_update_mut(|client| {
                            client.projects.push(project);
                        });
                    })
                },
                VarChanged => (),
                VarRemoved => {
                    project.use_ref(|project| {
                        project.client.try_update_mut(|client| {
                            if let Some(position) = client.projects.iter().position(|p| p == project) {
                                clients.projects.remove(position);
                            }
                        })
                    })
                },
            }
        })
    }

    #[var]
    fn added_project() -> Option<Var<Project>> {
        None
    }

    #[update]
    fn add_project(client: Var<Client>) {
        let client_id = client.try_map(|client| client.id).expect("client id");
        let project_id = ProjectId::new();

        let project = var(Project {
            id: project_id,
            name: String::new(),
            client,
        });
        added_project().set(Some(project));
        app::send_up_msg(true, UpMsg::AddProject(client_id, project_id));
    }

    #[update]
    fn remove_project(project: Var<Project>) {
        if let Some(project) = project.try_remove() {
            app::send_up_msg(true, UpMsg::RemoveProject(project.id));
        }
    }

    #[update]
    fn rename_project(project: Var<Project>, name: &str) {
        project.try_use_ref(|project| {
            app::send_up_msg(true, UpMsg::RenameProject(project.id, Cow::from(name)));
        });
    }

}
