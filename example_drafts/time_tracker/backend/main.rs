use moon::*;
use shared::{UpMsg, DownMsg, Message, ClientId, ProjectId};

mod client;
mod invoice;
mod project;
mod time_block;
mod time_entry;

async fn up_msg_handler(msg: UpMsg) -> Option<DownMsg> {
    let down_msg = match msg {
        // ------ Page data ------
        UpMsg::GetClientsAndProjectsClients => {
            let shared_clients_futs = client::by_id().iter().map(|(client_id, client)| {
                async {
                    let (client_name, projects) = join(client.name(), client.projects()).await;
                    let shared_projects_futs = projects.iter().map(|(project_id, project)| {
                        async {
                            let project_name = project.name().await;
                            shared::clients_and_projects::Project {
                                id: project_id,
                                name: project_name
                            }
                        }
                    });
                    shared::clients_and_projects::Client {
                        id: client_id,
                        name: client_name,
                        projects: join_all(shared_projects_futs).await 
                    }
                }
            });
            DownMsg::ClientsAndProjectsClients(
                join_all(shared_clients_futs).await
            )
        }

        // ------ Client ------
        UpMsg::AddClient(id) => {
            new_actor(Client { id }).await;
            DownMsg::ClientAdded
        },
        UpMsg::RemoveClient(id) => {
            client::by_id().get(id)[0].remove().await;
            DownMsg::ClientRemoved
        },
        UpMsg::RenameClient(id, name) => {
            client::by_id().get(id)[0].rename(name.to_string()).await;
            DownMsg::ClientRenamed
        },

        // ------ Project ------
        UpMsg::AddProject(client, id) => {
            new_actor(Project { client, id }).await;
            DownMsg::ProjectAdded
        },
        UpMsg::RemoveProject(id) => {
            project::by_id().get(id)[0].remove().await;
            DownMsg::ProjectRemoved
        },
        UpMsg::RenameProject(id, name) => {
            project::by_id().get(id)[0].rename(name.to_string()).await;
            DownMsg::ProjectRenamed
        },

        // ------ TimeBlock ------
        UpMsg::AddTimeBlock(client, id, duration) => {
            new_actor(TimeBlock { client, id, duration }).await;
            DownMsg::TimeBlockAdded
        },
        UpMsg::RemoveTimeBlock(id) => {
            time_block::by_id().get(id)[0].remove().await;
            DownMsg::TimeBlockRemoved
        },
        UpMsg::RenameTimeBlock(id, name) => {
            time_block::by_id().get(id)[0].rename(name.to_string()).await;
            DownMsg::TimeBlockRenamed
        },
        UpMsg::SetTimeBlockStatus(id, status) => {
            time_block::by_id().get(id)[0].set_status(status).await;
            DownMsg::TimeBlockStatusSet
        },
        UpMsg::SetTimeBlockDuration(id, duration) => {
            time_block::by_id().get(id)[0].set_duration(duration).await;
            DownMsg::TimeBlockDurationSet
        },

        // ------ Invoice ------
        UpMsg::AddInvoice(time_block, id) => {
            new_actor(Invoice { time_block, id }).await;
            DownMsg::InvoiceAdded
        },
        UpMsg::RemoveInvoice(id) => {
            invoice::by_id().get(id)[0].remove().await;
            DownMsg::InvoiceRemoved
        },
        UpMsg::SetInvoiceCustomId(id, custom_id) => {
            invoice::by_id().get(id)[0].set_custom_id(custom_id.to_string()).await;
            DownMsg::InvoiceCustomIdSet
        },
        UpMsg::SetInvoiceUrl(id, url) => {
            invoice::by_id().get(id)[0].set_url(url.to_string()).await;
            DownMsg::InvoiceUrlSet
        },

        // ------ TimeEntry ------
        UpMsg::AddTimeEntry(project, time_entry) => {
            new_actor(TimeEntry { project, time_entry }).await;
            DownMsg::TimeEntryAdded
        },
        UpMsg::RemoveTimeEntry(id) => {
            time_entry::by_id().get(id)[0].remove().await;
            DownMsg::TimeEntryRemoved
        },
        UpMsg::RenameTimeEntry(id, name) => {
            time_entry::by_id().get(id)[0].rename(name.to_string()).await;
            DownMsg::TimeEntryRenamed
        },
        UpMsg::SetTimeEntryStarted(id, started) => {
            time_entry::by_id().get(id)[0].set_started(started).await;
            DownMsg::TimeEntryStartedSet
        },
        UpMsg::SetTimeEntryStopped(id, stopped) => {
            time_entry::by_id().get(id)[0].set_stopped(stopped).await;
            DownMsg::TimeEntryStoppedSet
        },
    };
    Some(down_msg)
}

fn main() {
    start!(up_msg_handler, actors![client, project]);
}
