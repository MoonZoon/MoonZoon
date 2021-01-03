use moon::*;
use shared::{UpMsg, DownMsg, Message, ClientId, ProjectId, AccessToken};

mod client;
mod invoice;
mod project;
mod time_block;
mod time_entry;
mod user;

use client::{self, ClientArgs};
use invoice::{self, InvoiceArgs};
use project::{self, ProjectArgs};
use time_block::{self, TimeBlockArgs};
use time_entry::{self, TimeEntryArgs};
use user::{self, UserArgs};

async fn init() {
    if user::by_id().is_empty() {
        new_actor(UserArgs).await;
    }
}

async fn frontend() -> Frontend {
    Frontend::new().title("Time tracker example")
}

async fn check_access(access_token: Option<AccessToken>) -> bool {
    if let Some(access_token) = access_token {
        user::by_id()
            .first_actor()
            .unwrap()
            .logged_in(req.access_token)
            .await
    }
    false
}

macro_rules! check_access {
    ($req:ident) => {
        if !check_access($req.access_token).await {
            return DownMsg::AccessDenied
        }
    };
}

async fn up_msg_handler(req: UpMsgRequest) {
    let down_msg = match req.up_msg {

        // ------ Auth ------
        UpMsg::Login(password) => {
            user::by_id()
                .first_actor()
                .unwrap()
                .login(password)
                .await
                .map(DownMsg::LoggedIn)
                .unwrap_or_else(|| DownMsg::InvalidPassword)
        }
        UpMsg::Logout => {
            check_access!(req);
            user::by_id()
                .first_actor()
                .unwrap()
                .logout(req.access_token.unwrap())
                .await;
            DownMsg::LoggedOut
        }

        // ------ Page data ------
        UpMsg::GetClientsAndProjectsClients => {
            check_access!(req);
            let shared_projects_futs = |projects| projects.iter().map(|project| {
                let (id, name) = join(project.id(), project.name()).await;
                async {
                    shared::clients_and_projects::Project { id, name }
                }
            });

            let shared_clients_futs = client::by_id().iter().map(|(id, client)| {
                async {
                    let (name, projects) = join!(
                        client.name(), 
                        client.projects().then(|projects| {
                            join_all(shared_projects_futs(projects))
                        }),
                    ).await;
                    shared::clients_and_projects::Client { id, name, projects }
                }
            });

            DownMsg::ClientsAndProjectsClients(
                join_all(shared_clients_futs).await
            )
        }
        UpMsg::TimeBlocksClients => {
            check_access!(req);
            let shared_invoice_fut = |invoice| {
                async {
                    if let Some(invoice) = invoice {
                        let (id, custom_id, url) = join!(
                            invoice.id(),
                            invoice.custom_id(), 
                            invoice.url(),
                        ).await;
                        Some(shared::time_blocks::Invoice { id, custom_id, url })
                    } else {
                        None
                    };
                }
            };

            let shared_time_blocks_futs = |time_blocks| time_blocks.iter().map(|time_block| {
                async {
                    let (id, name, status, duration, invoice) = join!(
                        time_block.id(),
                        time_block.name(), 
                        time_block.status(), 
                        time_block.duration(), 
                        time_block.invoice().then(shared_invoice_fut),
                    ).await;
                    shared::time_blocks::TimeBlock { id, name, status, duration, invoice }
                }
            });

            let shared_clients_futs = client::by_id().iter().map(|(id, client)| {
                async {
                    let (name, tracked, time_blocks) = join!(
                        client.name(), 
                        client.tracked(), 
                        client.time_blocks().then(|time_blocks| {
                            join_all(shared_time_blocks_futs(time_blocks))
                        }),
                    ).await;
                    shared::time_blocks::Client { id, name, time_blocks, tracked }
                }
            });
            DownMsg::ClientsAndProjectsClients(
                join_all(shared_clients_futs).await
            )
        }
        UpMsg::TimeTrackerClients => {
            check_access!(req);
            let shared_time_entries_futs = |time_entries| time_entries.iter().map(|time_entry| {
                async {
                    let (id, name, started, stopped) = join!(
                        time_entry.id(),
                        time_entry.name(), 
                        time_entry.started(), 
                        time_entry.stopped(),
                    );
                    shared::time_tracker::TimeEntry { id, name, started, stopped }
                }
            });

            let shared_projects_futs = |projects| projects.iter().map(|project| {
                async {
                    let (id, name, time_entries) = join!(
                        project.id(),
                        project.name(), 
                        project.time_entries().then(|time_entries| {
                            join_all(shared_time_entries_futs(time_entries))
                        }),
                    ).await;
                    shared::time_tracker::Project { id, name, time_entries }
                }
            });

            let shared_clients_futs = client::by_id().iter().map(|(id, client)| {
                async {
                    let (name, projects) = join(
                        client.name(), 
                        client.projects().then(|projects| { 
                            join_all(shared_projects_futs(projects))
                        }),
                    ).await;
                    shared::time_tracker::Client { id, name, projects }
                }
            });

            DownMsg::ClientsAndProjectsClients(
                join_all(shared_clients_futs).await
            )
        }

        // ------ Client ------
        UpMsg::AddClient(id) => {
            check_access!(req);
            new_actor(ClientArgs { id }).await;
            DownMsg::ClientAdded
        },
        UpMsg::RemoveClient(id) => {
            check_access!(req);
            client::by_id().actors(id).first().unwrap().remove().await;
            DownMsg::ClientRemoved
        },
        UpMsg::RenameClient(id, name) => {
            check_access!(req);
            client::by_id().actors(id).first().unwrap().rename(name.to_string()).await;
            DownMsg::ClientRenamed
        },

        // ------ Project ------
        UpMsg::AddProject(client, id) => {
            check_access!(req);
            new_actor(ProjectArgs { client, id }).await;
            DownMsg::ProjectAdded
        },
        UpMsg::RemoveProject(id) => {
            check_access!(req);
            project::by_id().actors(id).first().unwrap().remove().await;
            DownMsg::ProjectRemoved
        },
        UpMsg::RenameProject(id, name) => {
            check_access!(req);
            project::by_id().actors(id).first().unwrap().rename(name.to_string()).await;
            DownMsg::ProjectRenamed
        },

        // ------ TimeBlock ------
        UpMsg::AddTimeBlock(client, id, duration) => {
            check_access!(req);
            new_actor(TimeBlockArgs { client, id, duration }).await;
            DownMsg::TimeBlockAdded
        },
        UpMsg::RemoveTimeBlock(id) => {
            check_access!(req);
            time_block::by_id().actors(id).first().unwrap().remove().await;
            DownMsg::TimeBlockRemoved
        },
        UpMsg::RenameTimeBlock(id, name) => {
            check_access!(req);
            time_block::by_id().actors(id).first().unwrap().rename(name.to_string()).await;
            DownMsg::TimeBlockRenamed
        },
        UpMsg::SetTimeBlockStatus(id, status) => {
            check_access!(req);
            time_block::by_id().actors(id).first().unwrap().set_status(status).await;
            DownMsg::TimeBlockStatusSet
        },
        UpMsg::SetTimeBlockDuration(id, duration) => {
            check_access!(req);
            time_block::by_id().actors(id).first().unwrap().set_duration(duration).await;
            DownMsg::TimeBlockDurationSet
        },

        // ------ Invoice ------
        UpMsg::AddInvoice(time_block, id) => {
            check_access!(req);
            new_actor(InvoiceArgs { time_block, id }).await;
            DownMsg::InvoiceAdded
        },
        UpMsg::RemoveInvoice(id) => {
            check_access!(req);
            invoice::by_id().actors(id).first().unwrap().remove().await;
            DownMsg::InvoiceRemoved
        },
        UpMsg::SetInvoiceCustomId(id, custom_id) => {
            check_access!(req);
            invoice::by_id().actors(id).first().unwrap().set_custom_id(custom_id.to_string()).await;
            DownMsg::InvoiceCustomIdSet
        },
        UpMsg::SetInvoiceUrl(id, url) => {
            check_access!(req);
            invoice::by_id().actors(id).first().unwrap().set_url(url.to_string()).await;
            DownMsg::InvoiceUrlSet
        },

        // ------ TimeEntry ------
        UpMsg::AddTimeEntry(project, time_entry) => {
            check_access!(req);
            new_actor(TimeEntryArgs { project, time_entry }).await;
            DownMsg::TimeEntryAdded
        },
        UpMsg::RemoveTimeEntry(id) => {
            check_access!(req);
            time_entry::by_id().actors(id).first().unwrap().remove().await;
            DownMsg::TimeEntryRemoved
        },
        UpMsg::RenameTimeEntry(id, name) => {
            check_access!(req);
            time_entry::by_id().actors(id).first().unwrap().rename(name.to_string()).await;
            DownMsg::TimeEntryRenamed
        },
        UpMsg::SetTimeEntryStarted(id, started) => {
            check_access!(req);
            time_entry::by_id().actors(id).first().unwrap().set_started(started).await;
            DownMsg::TimeEntryStartedSet
        },
        UpMsg::SetTimeEntryStopped(id, stopped) => {
            check_access!(req);
            time_entry::by_id().actors(id).first().unwrap().set_stopped(stopped).await;
            DownMsg::TimeEntryStoppedSet
        },
    };
    connected_client::by_id()
        .actors(req.client_id)
        .first()
        .unwrap()
        .send_down_msg(down_msg, req.cor_id)
        .await
}

fn main() {
    start!(init, frontend, up_msg_handler, actors![
        client, 
        invoice, 
        project, 
        time_block, 
        time_entry,
        user,
    ]);
}
