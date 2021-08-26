use crate::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    pub id: ClientId,
    pub name: String,
    pub projects: Vec<Project>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
}
