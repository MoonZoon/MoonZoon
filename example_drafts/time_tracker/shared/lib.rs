use serde::{Deserialize, Serialize};
use ulid::Ulid;
use chrono::{prelude::*, Duration};
use std::borrow::Cow;

mod clients_and_projects;
mod time_blocks;

pub type ClientId = Ulid;
pub type ProjectId = Ulid;
pub type TimeBlockId = Ulid;
pub type InvoiceId = Ulid;

#[derive(Serialize, Deserialize)]
pub enum UpMsg<'a> {
    // ------ Client ------
    GetClientsAndProjectsClients,
    GetTimeBlocksClients,
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
}

#[derive(Serialize, Deserialize)]
pub enum DownMsg {
    // ------ Client ------
    ClientsAndProjectsClients(Vec<clients_and_projects::Client>),
    TimeBlocksClients(Vec<time_blocks::Client>),
    ClientAdded,
    ClientRemoved,
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
}

