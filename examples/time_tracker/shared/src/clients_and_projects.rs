use crate::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct Client {
    pub id: ClientId,
    pub name: String,
    pub projects: Vec<Project>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
}
