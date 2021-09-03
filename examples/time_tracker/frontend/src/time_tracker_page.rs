use zoon::{*, eprintln};
use crate::connection::connection;
use shared::{UpMsg, ClientId, ProjectId, TimeEntryId, InvoiceId, time_tracker};
use std::sync::Arc;

mod view;

// ------ ------
//     Types
// ------ ------

#[derive(Default)]
struct Client {
    id: ClientId,
    name:String,
    projects: MutableVec<Arc<Project>>,
}

#[derive(Default)]
struct Project {
    id: ProjectId,
    name: Mutable<String>,
    time_entries: MutableVec<Arc<TimeEntry>>,
}

#[derive(Default)]
struct TimeEntry {
    id: TimeEntryId,
    name: Mutable<String>,
    started: Mutable<Wrapper<DateTime<Local>>>,
    stopped: Mutable<Option<Wrapper<DateTime<Local>>>>,
    is_old: bool,
}

// ------ ------
//    States
// ------ ------

#[static_ref]
fn clients() -> &'static MutableVec<Arc<Client>> {
    MutableVec::new()
}

// ------ ------
//   Commands
// ------ ------

pub fn request_clients() {
    Task::start(async {
        let msg = UpMsg::GetTimeBlocksClients;
        if let Err(error) = connection().send_up_msg(msg).await {
            eprintln!("get TimeBlocks clients request failed: {}", error);
        }
    });
}

pub fn convert_and_set_clients(new_clients: Vec<time_tracker::Client>) {
    // fn convert_clients(clients: Vec<time_blocks::Client>) -> Vec<Arc<Client>> {
    //     clients.into_iter().map(|client| {
    //         Arc::new(Client {
    //             id: client.id,
    //             name: client.name,
    //             time_blocks: MutableVec::new_with_values(convert_time_blocks(client.time_blocks)),
    //             tracked: client.tracked,
    //         })
    //     }).collect()
    // }
    // fn convert_time_blocks(time_blocks: Vec<time_blocks::TimeBlock>) -> Vec<Arc<TimeBlock>> {
    //     time_blocks.into_iter().map(|time_block| {
    //         Arc::new(TimeBlock {
    //             id: time_block.id,
    //             name: Mutable::new(time_block.name),
    //             status: Mutable::new(time_block.status),
    //             duration: Mutable::new(time_block.duration),
    //             invoice: Mutable::new(time_block.invoice.map(convert_invoice)),
    //             is_old: true,
    //         })
    //     }).collect()
    // }
    // fn convert_invoice(invoice: time_blocks::Invoice) -> Arc<Invoice> {
    //     Arc::new(Invoice {
    //         id: invoice.id,
    //         custom_id: Mutable::new(invoice.custom_id),
    //         url: Mutable::new(invoice.url),
    //         is_old: true,
    //     })
    // }
    // clients().lock_mut().replace_cloned(convert_clients(new_clients));
}

// -- time_block --

// fn add_time_block(client: &Client) {
//     // @TODO send up_msg
//     client.time_blocks.lock_mut().insert_cloned(0, Arc::new(TimeBlock::default()))
// }

// fn delete_time_block(client: &Client, time_block_id: TimeBlockId) {
//     // @TODO send up_msg + confirm dialog
//     client.time_blocks.lock_mut().retain(|time_block| time_block.id != time_block_id);
// }

// fn rename_time_block(time_block_id: TimeBlockId, name: &str) {
//     // @TODO send up_msg
//     zoon::println!("rename_time_block not implemented yet");
// }

// fn set_time_block_status(time_block: &TimeBlock, status: TimeBlockStatus) {
//     // @TODO send up_msg
//     time_block.status.set(status);
// }

// fn set_time_block_duration(time_block: &TimeBlock, duration: Wrapper<Duration>) {
//     // @TODO send up_msg
//     time_block.duration.set(duration);
// }

// // -- invoice --

// fn add_invoice(time_block: &TimeBlock) {
//     // @TODO send up_msg
//     time_block.invoice.set(Some(Arc::new(Invoice::default())));
// }

// fn delete_invoice(time_block: &TimeBlock) {
//     // @TODO send up_msg + confirm dialog
//     time_block.invoice.take();
// }

// fn set_invoice_custom_id(invoice_id: InvoiceId, custom_id: &str) {
//     // @TODO send up_msg
//     zoon::println!("set_invoice_custom_id not implemented yet");
// }

// fn set_invoice_url(invoice_id: InvoiceId, url: &str) {
//     // @TODO send up_msg
//     zoon::println!("set_invoice_url not implemented yet");
// }

// ------ ------
//     View
// ------ ------

pub fn view() -> RawElement {
    view::page().into_raw_element()
}






// blocks!{
//     append_blocks![els]

//     #[subscription]
//     fn on_route_change() {
//         if let app::Route::TimeTracker = route() {
//             set_clients(None);
//             app::send_up_msg(false, UpMsg::GetTimeTrackerClients);
//         }
//     }

//     #[subscription]
//     fn handle_down_msg() {
//         listen(|msg: Option<DownMsg>| {
//             if let Some(DownMsg::TimeTrackerClients(clients)) = msg {
//                 set_clients(Some(clients));
//                 return None
//             }
//             msg
//         })
//     }

//     // ------ Client ------

//     #[derive(Debug)]
//     pub struct Client {
//         id: ClientId,
//         name: String,
//         projects: Vec<VarC<Project>>,
//     }

//     #[s_var]
//     fn clients() -> Option<Vec<VarC<Client>>> {
//         None
//     }

//     #[update]
//     fn set_clients(clients: Vec<shared::time_tracker::Client>) {
//         let clients = match {
//             Some(clients) => clients,
//             None => return clients().set(None);
//         };
//         stop!{
//             let new_time_entries = |project: Var<Project>, time_entries: Vec<shared::time_tracker::TimeEntry>| {
//                 time_entries.into_iter().map(|time_entry| {
//                     new_var_c(TimeEntry {
//                         id: time_entry.id,
//                         name: time_entry.name,
//                         started: time_entry.started,
//                         stopped: time_entry.stopped,
//                         project,
//                     })
//                 }).collect()
//             };
//             let new_projects = |client: Var<Client>, projects: Vec<shared::time_tracker::Project>| {
//                 projects.into_iter().map(|project| {
//                     let project_var = new_var_c(Project {
//                         id: project.id,
//                         name: project.name,
//                         active_time_entry: None,
//                         time_entries: vec::new(),
//                         client,
//                     });
//                     project_var.update_mut(|new_project| {
//                         new_project.time_entries = new_time_entries(project_var.var(), project.time_entries);
//                     });
//                     project_var
//                 }).collect()
//             };
//             let new_clients = |clients: Vec<shared::time_tracker::Client>| {
//                 clients.into_iter().map(|client| {
//                     let client_var = new_var_c(Client {
//                         id: client.id,
//                         name: client.name,
//                         projects: Vec::new(),
//                     });
//                     client_var.update_mut(|new_client| {
//                         new_client.projects = new_projects(client_var.var(), client.projects);
//                     });
//                     client_var
//                 }).collect()
//             };
//             clients().set(Some(new_clients(clients)));
//         }
//     }

//     // ------ Project ------

//     #[derive(Debug)]
//     pub struct Project {
//         id: ProjectId,
//         name: String,
//         active_time_entry: Option<Var<TimeEntry>>,
//         time_entries: Vec<VarC<TimeEntry>>,
//     }

//     // ------ TimeEntry ------

//     #[derive(Debug)]
//     struct TimeEntry {
//         id: TimeEntryId,
//         name: String,
//         started: DateTime<Local>,
//         stopped: Option<DateTime<Local>>,
//         project: Var<Project>,
//     }

//     #[update]
//     fn add_time_entry(project: Var<Project>) {
//         let project_id = project.map(|project| project.id);

//         let time_entry = shared::time_tracker::TimeEntry {
//             id: TimeEntryId::new(),
//             name: String::new(),
//             started: Local::now(),
//             stopped: None,
//         };
//         let time_entry_var = new_var_c(TimeEntry {
//             id: time_entry.id,
//             name: time_entry.name.clone(),
//             started: time_entry.started,
//             stopped: time_entry.stopped,
//             project,
//         });
//         project.update_mut(|project| {
//             project.active_time_entry = Some(time_entry_var.var());
//         });
//         app::send_up_msg(true, UpMsg::AddTimeEntry(project_id, time_entry));
//     }

//     #[update]
//     fn remove_time_entry(time_entry: Var<TimeEntry>) {
//         let (project, id) = time_entry.map(|time_entry| {
//             (time_entry.project.var(), time_entry.id)
//         });
//         project().update_mut(|project| {
//             let time_entries = &mut project.time_entries;
//             let position = time_entries.iter_vars().position(|te| te == time_entry);
//             time_entries.remove(position.unwrap());

//             if project.active_time_entry == Some(time_entry) {
//                 project.active_time_entry = None;
//             }
//         });
//         app::send_up_msg(true, UpMsg::RemoveTimeEntry(id));
//     }

//     #[update]
//     fn rename_time_entry(time_entry: Var<TimeEntry>, name: &str) {
//         time_entry.update_mut(|time_entry| {
//             time_entry.name = name.to_owned();
//             app::send_up_msg(true, UpMsg::RenameTimeEntry(time_entry.id, Cow::from(name)));
//         });
//     }

//     #[update]
//     fn set_time_entry_started(time_entry: Var<TimeEntry>, started: DateTime<Local>) {
//         time_entry.update_mut(|time_entry| {
//             time_entry.started = started;
//             app::send_up_msg(true, UpMsg::SetTimeEntryStarted(time_entry.id, started));
//         });
//     }

//     #[update]
//     fn set_time_entry_stopped(time_entry: Var<TimeEntry>, stopped: DateTime<Local>) {
//         time_entry.update_mut(|time_entry| {
//             if time_entry.stopped.is_none() {
//                 time_entry.project.update_mut(|project| {
//                     project.active_time_entry = None;
//                 })
//             }
//             time_entry.stopped = Some(stopped);
//             app::send_up_msg(true, UpMsg::SetTimeEntryStarted(time_entry.id, stopped));
//         });
//     }

// }
