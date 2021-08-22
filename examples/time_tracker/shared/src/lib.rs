use moonlight::{
    serde_lite::{self, Deserialize, Serialize},
    chrono::{prelude::*, Duration},
    AuthToken,
};

mod entity_id;
pub use entity_id::EntityId;

// mod clients_and_projects;
// mod time_blocks;
// mod time_tracker;

pub type ClientId = EntityId;
pub type ProjectId = EntityId;
pub type TimeBlockId = EntityId;
pub type InvoiceId = EntityId;
pub type TimeEntryId = EntityId;
pub type UserId = EntityId;

#[derive(Serialize, Deserialize)]
pub struct User {
    id: UserId,
    name: String,
    auth_token: AuthToken,
}

// ------ UpMsg ------

#[derive(Serialize, Deserialize)]
pub enum UpMsg {
    // ------ Auth ------
    Login(String),
    Logout,
    // ------ Page data ------
    GetClientsAndProjectsClients,
    GetTimeBlocksClients,
    GetTimeTrackerClients,
    // ------ Client ------
    AddClient(ClientId),
    RemoveClient(ClientId),
    RenameClient(ClientId, String),
    // ------ Project ------
    AddProject(ClientId, ProjectId),
    RemoveProject(ProjectId),
    RenameProject(ProjectId, String),
    // ------ TimeBlock ------
    // AddTimeBlock(ClientId, TimeBlockId, Duration),
    RemoveTimeBlock(TimeBlockId),
    RenameTimeBlock(TimeBlockId, String),
    // SetTimeBlockStatus(TimeBlockId, time_blocks::TimeBlockStatus),
    // SetTimeBlockDuration(TimeBlockId, Duration),
    // ------ Invoice ------
    AddInvoice(TimeBlockId, InvoiceId),
    RemoveInvoice(InvoiceId),
    SetInvoiceCustomId(InvoiceId, String),
    SetInvoiceUrl(InvoiceId, String),
    // ------ TimeEntry ------
    // AddTimeEntry(ProjectId, time_tracker::TimeEntry),
    RemoveTimeEntry(TimeEntryId),
    RenameTimeEntry(TimeEntryId, String),
    // SetTimeEntryStarted(TimeEntryId, DateTime<Local>),
    // SetTimeEntryStopped(TimeEntryId, DateTime<Local>),
}

// ------ DownMsg ------

#[derive(Serialize, Deserialize)]
pub enum DownMsg {
    // ------ Auth ------
    InvalidPassword,
    LoggedIn(User),
    LoggedOut,
    AccessDenied,
    // ------ Page data ------
    // ClientsAndProjectsClients(Vec<clients_and_projects::Client>),
    // TimeBlocksClients(Vec<time_blocks::Client>),
    // TimeTrackerClients(Vec<time_tracker::Client>),
    // ------ Client ------
    ClientAdded,
    ClientRemoved,
    ClientRenamed,
    // ------ Project ------
    ProjectAdded,
    ProjectRemoved,
    ProjectRenamed,
    // ------ TimeBlock ------
    TimeBlockAdded,
    TimeBlockRemoved,
    TimeBlockRenamed,
    TimeBlockStatusSet,
    TimeBlockDurationSet,
    // ------ Invoice ------
    InvoiceAdded,
    InvoiceRemoved,
    InvoiceCustomIdSet,
    InvoiceUrlSet,
    // ------ TimeEntry ------
    TimeEntryAdded,
    TimeEntryRemoved,
    TimeEntryRenamed,
    TimeEntryStartedSet,
    TimeEntryStoppedSet,
}

