use zoon::{*, eprintln};
use crate::connection::connection;
use shared::{UpMsg, ClientId, ProjectId, clients_and_projects};

mod view;

// ------ ------
//     Types
// ------ ------

struct Client {
    id: ClientId,
    name: Mutable<String>,
    projects: MutableVec<Project>
}

struct Project {
    id: ProjectId,
    name: Mutable<String>,
}

// ------ ------
//    States
// ------ ------

#[static_ref]
fn clients() -> &'static Mutable<Option<Vec<Client>>> {
    Mutable::new(None)
}

// ------ ------
//   Commands
// ------ ------

pub fn request_clients() {
    Task::start(async {
        let msg = UpMsg::GetClientsAndProjectsClients;
        if let Err(error) = connection().send_up_msg(msg).await {
            eprintln!("get ClientsAndProjects clients request failed: {}", error);
        }
    });
}

pub fn convert_and_set_clients(new_clients: Vec<clients_and_projects::Client>) {
    fn convert_clients(clients: Vec<clients_and_projects::Client>) -> Vec<Client> {
        clients.into_iter().map(|client| {
            Client {
                id: client.id,
                name: Mutable::new(client.name),
                projects: MutableVec::new_with_values(convert_projects(client.projects)),
            }
        }).collect()
    }
    fn convert_projects(projects: Vec<clients_and_projects::Project>) -> Vec<Project> {
        projects.into_iter().map(|project| {
            Project {
                id: project.id,
                name: Mutable::new(project.name),
            }
        }).collect()
    }
    clients().set(Some(convert_clients(new_clients)));
}

// ------ ------
//     View
// ------ ------

pub fn view() -> RawElement {
    view::page().into_raw_element()
}

// blocks!{
//     append_blocks![els]

//     #[subscription]
//     fn on_route_change() {
//         if let app::Route::ClientsAndProjects = route() {
//             set_clients(None);
//             app::send_up_msg(false, UpMsg::GetClientsAndProjectsClients);
//         }
//     }

//     #[subscription]
//     fn handle_down_msg() {
//         listen(|msg: Option<DownMsg>| {
//             if let Some(DownMsg::ClientsAndProjectsClients(clients)) = msg {
//                 set_clients(Some(clients));
//                 return None
//             }
//             msg
//         })
//     }

//     // ------ Client ------

//     #[derive(Debug)]
//     pub struct Client {
//         id: ClientId,
//         name: String,
//         projects: Vec<VarC<Project>>,
//     }

//     #[s_var]
//     fn clients() -> Option<Vec<VarC<Client>>> {
//         None
//     }

//     #[s_var]
//     fn setting_clients() -> bool {
//         false
//     }

//     #[update]
//     fn set_clients(clients: Option<Vec<shared::clients_and_projects::Client>>) {
//         let clients = match {
//             Some(clients) => clients,
//             None => return clients().set(None);
//         };
//         setting_clients().set(true);
//         stop!{
//             let new_projects = |client: Var<Client>, projects: Vec<shared::clients_and_projects::Project>| {
//                 projects.into_iter().map(|project| {
//                     new_var_c(Project {
//                         id: project.id,
//                         name: project.name,
//                         client,
//                     })
//                 }).collect()
//             };
//             let new_clients = |clients: Vec<shared::clients_and_projects::Client>| {
//                 clients.into_iter().map(|client| {
//                     let client_var = new_var_c(Client {
//                         id: client.id,
//                         name: client.name,
//                         projects: Vec::new(),
//                     });
//                     client_var.update_mut(|new_client| {
//                         new_client.projects = new_projects(client_var.var(), client.projects);
//                     });
//                     client_var
//                 }).collect()
//             };
//             clients().set(Some(new_clients(clients)));
//         }
//         setting_clients().set(false);
//     }

//     #[update]
//     fn add_client() {
//         let id = ClientId::new();
//         let client = new_var_c(Client {
//             id,
//             name: String::new(),
//             projects: Vec::new(),
//         });
//         clients().update_mut(move |clients| {
//             clients.unwrap().push(client);
//         });
//         app::send_up_msg(true, UpMsg::AddClient(id));
//     } 

//     #[update]
//     fn remove_client(client: Var<Client>) {
//         let id = client.map(|client| client.id);
//         clients().update_mut(|clients| {
//             let clients = clients.as_mut().unwrap();
//             let position = clients.iter_vars().position(|c| c == client);
//             clients.remove(position.unwrap());
//         });
//         app::send_up_msg(true, UpMsg::RemoveClient(id));
//     }

//     #[update]
//     fn rename_client(client: Var<Project>, name: &str) {
//         client.update_mut(|client| {
//             client.name = name.to_owned();
//             app::send_up_msg(true, UpMsg::RenameClient(client.id, Cow::from(name)));
//         });
//     }

//     // ------ Project ------

//     #[derive(Debug)]
//     struct Project {
//         id: ProjectId,
//         name: String,
//         client: Var<Client>, 
//     }

//     #[update]
//     fn add_project(client: Var<Client>) {
//         let client_id = client.map(|client| client.id);
//         let project_id = ProjectId::new();

//         let project = new_var_c(Project {
//             id: project_id,
//             name: String::new(),
//             client,
//         });
//         client().update_mut(|client| {
//             client.projects.push(project);
//         });
//         app::send_up_msg(true, UpMsg::AddProject(client_id, project_id));
//     }

//     #[update]
//     fn remove_project(project: Var<Project>) {
//         let client = project.map(|project| project.client);
//         let id = client().map_mut(|client| {
//             let projects = &mut client.projects;
//             let position = projects.iter().position(|p| p == project).unwrap();
//             let id = projects[position].id;
//             projects.remove(position);
//             id
//         });
//         app::send_up_msg(true, UpMsg::RemoveProject(id));
//     }

//     #[update]
//     fn rename_project(project: Var<Project>, name: &str) {
//         project.update_mut(|project| {
//             project.name = name.to_owned();
//             app::send_up_msg(true, UpMsg::RenameProject(project.id, Cow::from(name)));
//         });
//     }

// }
