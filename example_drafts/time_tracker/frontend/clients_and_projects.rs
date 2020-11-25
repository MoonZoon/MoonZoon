use zoon::*;
use ulid::Ulid;
use std::borrow::Cow;
use crate::app;
use shared::{ClientId, ProjectId, DownMsg};

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
        listen(|msg: Option<DownMsg>| {
            if let Some(DownMsg::ClientsAndProjectsClients(clients)) = msg {
                set_clients(Some(clients));
                return None
            }
            msg
        })
    }

    // ------ Client ------

    #[derive(Debug)]
    pub struct Client {
        id: ClientId,
        name: String,
        projects: Vec<VarC<Project>>,
    }

    #[var]
    fn clients() -> Option<Vec<VarC<Client>>> {
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
            let new_projects = |client: Var<Client>, projects: Vec<shared::clients_and_projects::Project>| {
                projects.into_iter().map(|project| {
                    var(Project {
                        id: project.id,
                        name: project.name,
                        client,
                    })
                }).collect()
            };
            let new_clients = |clients: Vec<shared::clients_and_projects::Client>| {
                clients.into_iter().map(|client| {
                    let client_var = var(Client {
                        id: client.id,
                        name: client.name,
                        projects: Vec::new(),
                    });
                    client_var.update_mut(|new_client| {
                        new_client.projects = new_projects(client_var.var(), client.projects);
                    });
                    client_var
                }).collect()
            };
            clients().set(Some(new_clients(clients)));
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
        added_client().set(Some(client.var()));
        clients().update_mut(move |clients| {
            clients.unwrap().push(client);
        });
        app::send_up_msg(true, UpMsg::AddClient(id));
    } 

    #[update]
    fn remove_client(client: Var<Client>) {
        let id = client.map(|client| client.id);
        clients().update_mut(|clients| {
            let clients = clients.as_mut().unwrap();
            let position = clients.iter_vars().position(|c| c == client);
            clients.remove(position.unwrap());
        });
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
        added_project().set(Some(project.var()));
        client().update_mut(|client| {
            client.projects.push(project);
        });
        app::send_up_msg(true, UpMsg::AddProject(client_id, project_id));
    }

    #[update]
    fn remove_project(project: Var<Project>) {
        let client = project.map(|project| project.client);
        let id = client().map_mut(|client| {
            let projects = &mut client.projects;
            let position = projects.iter().position(|p| p == project).unwrap();
            let id = projects[position].id;
            projects.remove(position);
            id
        });
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
