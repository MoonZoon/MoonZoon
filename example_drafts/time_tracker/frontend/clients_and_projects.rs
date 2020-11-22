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
        app::down_msg().inner().update(|down_msg| {
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
                        clients.unwrap().push(client);
                    });
                },
                VarUpdated => (),
                VarRemoved => {
                    client.use_ref(|client| {
                        stop!{
                            for project in &client.projects {
                                project.remove();
                            }
                        }
                    });
                    clients().update_mut(|clients| {
                        let clients = clients.unwrap();
                        let position = clients.iter().position(|c| c == client);
                        clients.remove(position.unwrap());
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
            for client in clients {
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
            }
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
        let id = client.remove().id;
        app::send_up_msg(true, UpMsg::RemoveClient(id));
    }

    #[update]
    fn rename_client(client: Var<Project>, name: &str) {
        client.update_mut(|client| {
            client.name = name.to_owned();
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
            let client = || project.map(|project| project.client);
            match event {
                VarAdded => {
                    client().update_mut(|client| {
                        client.projects.push(project);
                    });
                },
                VarUpdated => (),
                VarRemoved => {
                    client().update_mut(|client| {
                        let projects = &mut client.projects;
                        let position = projects.iter().position(|p| p == project);
                        projects.remove(position.unwrap());
                    });
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
        let client_id = client.map(|client| client.id);
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
        let id = project.remove().id;
        app::send_up_msg(true, UpMsg::RemoveProject(id));
    }

    #[update]
    fn rename_project(project: Var<Project>, name: &str) {
        project.update_mut(|project| {
            project.name = name.to_owned();
            app::send_up_msg(true, UpMsg::RenameProject(project.id, Cow::from(name)));
        });
    }

}
