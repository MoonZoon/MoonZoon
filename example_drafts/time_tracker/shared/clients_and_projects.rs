use serde::{Deserialize, Serialize};
use ulid::Ulid;

type ClientId = Ulid;
type ProjectId = Ulid;

#[derive(Serialize, Deserialize)]
pub struct Client {
    pub id: ClientId,
    pub name: String,
    pub projects: Vec<Project>
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
}
