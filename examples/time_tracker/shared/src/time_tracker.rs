use crate::*;

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
    pub time_entries: Vec<TimeEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct TimeEntry {
    pub id: TimeEntryId,
    pub name: String,
    pub started: Wrapper<DateTime<Local>>,
    pub stopped: Option<Wrapper<DateTime<Local>>>,
}
