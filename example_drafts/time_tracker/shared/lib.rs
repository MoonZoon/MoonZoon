use serde::{Deserialize, Serialize};
use ulid::Ulid;
use std::borrow::Cow;

mod clients_and_projects;

type ClientId = Ulid;
type ProjectId = Ulid;

#[derive(Serialize, Deserialize)]
pub enum UpMsg<'a> {
    // ------ Client ------
    GetClientsAndProjectsClients,
    AddClient(ClientId),
    RemoveClient(ClientId),
    RenameClient(ClientId, Cow<'a, str>),
    // ------ Project ------
    AddProject(ClientId, ProjectId),
    RemoveProject(ProjectId),
    RenameProject(ProjectId, Cow<'a, str>),
}

#[derive(Serialize, Deserialize)]
pub enum DownMsg {
    // ------ Client ------
    ClientsAndProjectsClients(Vec<clients_and_projects::Client>),
    ClientAdded,
    ClientRemoved,
    // ------ Project ------
    ProjectAdded,
    ProjectRemoved,
    ProjectRenamed,
}

