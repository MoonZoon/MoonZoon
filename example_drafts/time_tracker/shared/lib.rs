use serde::{Deserialize, Serialize};
use ulid::Ulid;
use chrono::{prelude::*, Duration};
use std::borrow::Cow;

mod clients_and_projects;
mod time_blocks;
mod time_tracker;

pub type ClientId = Ulid;
pub type ProjectId = Ulid;
pub type TimeBlockId = Ulid;
pub type InvoiceId = Ulid;
pub type TimeEntryId = Ulid;

pub struct User {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub enum UpMsg<'a> {
    // ------ Auth ------
    Login(Cow<'a, str>),
    Logout,
    // ------ Page data ------
    GetClientsAndProjectsClients,
    GetTimeBlocksClients,
    GetTimeTrackerClients,
    // ------ Client ------
    AddClient(ClientId),
    RemoveClient(ClientId),
    RenameClient(ClientId, Cow<'a, str>),
    // ------ Project ------
    AddProject(ClientId, ProjectId),
    RemoveProject(ProjectId),
    RenameProject(ProjectId, Cow<'a, str>),
    // ------ TimeBlock ------
    AddTimeBlock(ClientId, TimeBlockId, Duration),
    RemoveTimeBlock(TimeBlockId),
    RenameTimeBlock(TimeBlockId, Cow<'a, str>),
    SetTimeBlockStatus(TimeBlockId, time_blocks::TimeBlockStatus),
    SetTimeBlockDuration(TimeBlockId, Duration),
    // ------ Invoice ------
    AddInvoice(TimeBlockId, InvoiceId),
    RemoveInvoice(InvoiceId),
    SetInvoiceCustomId(InvoiceId, Cow<'a, str>),
    SetInvoiceUrl(InvoiceId, Cow<'a, str>),
    // ------ TimeEntry ------
    AddTimeEntry(ProjectId, time_tracker::TimeEntry),
    RemoveTimeEntry(TimeEntryId),
    RenameTimeEntry(TimeEntryId, Cow<'a, str>),
    SetTimeEntryStarted(TimeEntryId, DateTime<Local>),
    SetTimeEntryStopped(TimeEntryId, DateTime<Local>),
}

#[derive(Serialize, Deserialize)]
pub enum DownMsg {
    // ------ Auth ------
    InvalidPassword,
    LoggedIn(User),
    LoggedOut,
    // ------ Page data ------
    ClientsAndProjectsClients(Vec<clients_and_projects::Client>),
    TimeBlocksClients(Vec<time_blocks::Client>),
    TimeTrackerClients(Vec<time_tracker::Client>),
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

