use serde::{Deserialize, Serialize};

mod clients_and_projects;

#[derive(Serialize, Deserialize)]
pub enum UpMsg {
    GetClientsAndProjectsClients,
}

#[derive(Serialize, Deserialize)]
pub enum DownMsg {
    ClientsAndProjectsClients(Vec<clients_and_projects::Client>)
}

