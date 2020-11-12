use zoon::*;
use ulid::Ulid;
use crate::app;

pub mod els;

type ClientId = Ulid;
type ProjectId = Ulid;

zoons!{
    append_zoons![els]

    #[subscription]
    fn on_route_change() {
        if let app::Route::ClientsAndProjects = route() {
            app::send_up_msg(UpMsg::GetClientsAndProjectsClients);
        }
    }

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
    fn set_clients(clients: Vec<shared::clients_and_projects::Client>) {
        let clients = clients.into_iter().map(|client| {

            let client_var = Var::new(Client {
                id: client.id,
                name: client.name,
                projects: Vec::new(),
            });

            let projects = client.projects.into_iter().map(|project| {
                let project_var = Var::new(Project {
                    id: project.id,
                    name: project.name,
                    client: client_var,
                });
            }).collect();

            client_var.update_mut(move |client| {
                client.projects = projects;
            });
            client_var

        }).collect();
        clients().set(Some(clients));
    }

}
