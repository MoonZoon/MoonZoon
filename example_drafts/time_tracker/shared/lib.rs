use serde::{Deserialize, Serialize};
use ulid::Ulid;

mod clients_and_projects;

type ClientId = Ulid;
type ProjectId = Ulid;

#[derive(Serialize, Deserialize)]
pub enum UpMsg {
    GetClientsAndProjectsClients,
    AddClient(ClientId),
}

#[derive(Serialize, Deserialize)]
pub enum DownMsg {
    ClientsAndProjectsClients(Vec<clients_and_projects::Client>),
    ClientAdded(ClientId),
}

