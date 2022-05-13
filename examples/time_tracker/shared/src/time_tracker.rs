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
    pub time_entries: Vec<TimeEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct TimeEntry {
    pub id: TimeEntryId,
    pub name: String,
    pub started: DateTime<Local>,
    pub stopped: Option<DateTime<Local>>,
}
