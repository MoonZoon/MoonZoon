use crate::connection::connection;
use shared::{clients_and_projects, ClientId, ProjectId, UpMsg};
use std::sync::Arc;
use zoon::{eprintln, *};

mod view;

// ------ ------
//     Types
// ------ ------

#[derive(Default)]
struct Client {
    id: ClientId,
    name: Mutable<String>,
    projects: MutableVec<Arc<Project>>,
    is_old: bool,
}

#[derive(Default)]
struct Project {
    id: ProjectId,
    name: Mutable<String>,
    is_old: bool,
}

// ------ ------
//    States
// ------ ------

#[static_ref]
fn clients() -> &'static MutableVec<Arc<Client>> {
    MutableVec::new()
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
    fn convert_clients(clients: Vec<clients_and_projects::Client>) -> Vec<Arc<Client>> {
        clients
            .into_iter()
            .map(|client| {
                Arc::new(Client {
                    id: client.id,
                    name: Mutable::new(client.name),
                    projects: MutableVec::new_with_values(convert_projects(client.projects)),
                    is_old: true,
                })
            })
            .collect()
    }
    fn convert_projects(projects: Vec<clients_and_projects::Project>) -> Vec<Arc<Project>> {
        projects
            .into_iter()
            .map(|project| {
                Arc::new(Project {
                    id: project.id,
                    name: Mutable::new(project.name),
                    is_old: true,
                })
            })
            .collect()
    }
    clients()
        .lock_mut()
        .replace_cloned(convert_clients(new_clients));
}

// -- client --

fn add_client() {
    // @TODO send up_msg
    clients()
        .lock_mut()
        .insert_cloned(0, Arc::new(Client::default()))
}

fn delete_client(client_id: ClientId) {
    // @TODO send up_msg + confirm dialog
    clients().lock_mut().retain(|client| client.id != client_id);
}

fn rename_client(client_id: ClientId, name: &str) {
    // @TODO send up_msg
    zoon::println!("rename_client not implemented yet");
}

// -- project --

fn add_project(client: &Client) {
    // @TODO send up_msg
    client
        .projects
        .lock_mut()
        .insert_cloned(0, Arc::new(Project::default()))
}

fn delete_project(client: &Client, project_id: ProjectId) {
    // @TODO send up_msg + confirm dialog
    client
        .projects
        .lock_mut()
        .retain(|project| project.id != project_id);
}

fn rename_project(project_id: ProjectId, name: &str) {
    // @TODO send up_msg
    zoon::println!("rename_project not implemented yet");
}

// ------ ------
//     View
// ------ ------

pub fn view() -> RawElement {
    view::page().into_raw_element()
}
