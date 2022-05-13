use crate::connection::connection;
use shared::{
    time_blocks::{self, TimeBlockStatus},
    ClientId, InvoiceId, TimeBlockId, UpMsg,
};
use std::sync::Arc;
use zoon::{eprintln, *};

mod view;

// ------ ------
//     Types
// ------ ------

struct Client {
    id: ClientId,
    name: String,
    time_blocks: MutableVec<Arc<TimeBlock>>,
    stats: Arc<Stats>,
    _time_block_change_handler: TaskHandle,
}

#[derive(Default, Debug)]
struct Stats {
    tracked: f64,
    blocked: Mutable<f64>,
    paid: Mutable<f64>,
    unpaid: Mutable<f64>,
    to_block: Mutable<f64>,
}

#[derive(Default, Debug)]
struct TimeBlock {
    id: TimeBlockId,
    name: Mutable<String>,
    status: Mutable<TimeBlockStatus>,
    duration: Mutable<Wrapper<Duration>>,
    invoice: Mutable<Option<Arc<Invoice>>>,
    is_old: bool,
}

#[derive(Default, Debug)]
pub struct Invoice {
    id: InvoiceId,
    custom_id: Mutable<String>,
    url: Mutable<String>,
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

pub fn convert_and_set_clients(new_clients: Vec<time_blocks::Client>) {
    fn convert_clients(clients: Vec<time_blocks::Client>) -> Vec<Arc<Client>> {
        clients
            .into_iter()
            .map(|client| {
                let time_blocks =
                    MutableVec::new_with_values(convert_time_blocks(client.time_blocks));
                let stats = Arc::new(Stats {
                    tracked: client.tracked.num_hours() as f64,
                    ..Default::default()
                });
                let time_block_change_signal = time_blocks
                    .signal_vec_cloned()
                    .map_signal(|time_block| {
                        map_ref! {
                            let _ = time_block.status.signal(),
                            let _ = time_block.duration.signal() => move {
                                time_block.clone()
                            }
                        }
                    })
                    .to_signal_cloned()
                    .map(clone!((stats) move |time_blocks| (stats.clone(), time_blocks)));

                let time_block_change_handler =
                    time_block_change_signal.for_each_sync(|(stats, time_blocks)| {
                        let mut non_billable = 0.;
                        let mut unpaid = 0.;
                        let mut paid = 0.;
                        for time_block in time_blocks {
                            let duration = (time_block.duration.get().num_seconds() as f64) / 3600.;
                            match time_block.status.get() {
                                TimeBlockStatus::NonBillable => non_billable += duration,
                                TimeBlockStatus::Unpaid => unpaid += duration,
                                TimeBlockStatus::Paid => paid += duration,
                            }
                        }
                        let blocked = non_billable + unpaid + paid;
                        stats.blocked.set_neq(blocked);
                        stats.unpaid.set_neq(unpaid);
                        stats.paid.set_neq(paid);
                        stats.to_block.set_neq(stats.tracked - blocked);
                    });
                Arc::new(Client {
                    id: client.id,
                    name: client.name,
                    time_blocks,
                    stats,
                    _time_block_change_handler: Task::start_droppable(time_block_change_handler),
                })
            })
            .collect()
    }
    fn convert_time_blocks(time_blocks: Vec<time_blocks::TimeBlock>) -> Vec<Arc<TimeBlock>> {
        time_blocks
            .into_iter()
            .map(|time_block| {
                Arc::new(TimeBlock {
                    id: time_block.id,
                    name: Mutable::new(time_block.name),
                    status: Mutable::new(time_block.status),
                    duration: Mutable::new(time_block.duration),
                    invoice: Mutable::new(time_block.invoice.map(convert_invoice)),
                    is_old: true,
                })
            })
            .collect()
    }
    fn convert_invoice(invoice: time_blocks::Invoice) -> Arc<Invoice> {
        Arc::new(Invoice {
            id: invoice.id,
            custom_id: Mutable::new(invoice.custom_id),
            url: Mutable::new(invoice.url),
            is_old: true,
        })
    }
    clients()
        .lock_mut()
        .replace_cloned(convert_clients(new_clients));
}

// -- time_block --

fn add_time_block(client: &Client) {
    // @TODO send up_msg
    client
        .time_blocks
        .lock_mut()
        .insert_cloned(0, Arc::new(TimeBlock::default()))
}

fn delete_time_block(client: &Client, time_block_id: TimeBlockId) {
    // @TODO send up_msg + confirm dialog
    client
        .time_blocks
        .lock_mut()
        .retain(|time_block| time_block.id != time_block_id);
}

fn rename_time_block(time_block_id: TimeBlockId, name: &str) {
    // @TODO send up_msg
    zoon::println!("rename_time_block not implemented yet");
}

fn set_time_block_status(time_block: &TimeBlock, status: TimeBlockStatus) {
    // @TODO send up_msg
    time_block.status.set(status);
}

fn set_time_block_duration(time_block: &TimeBlock, duration: Wrapper<Duration>) {
    // @TODO send up_msg
    time_block.duration.set(duration);
}

// -- invoice --

fn add_invoice(time_block: &TimeBlock) {
    // @TODO send up_msg
    time_block.invoice.set(Some(Arc::new(Invoice::default())));
}

fn delete_invoice(time_block: &TimeBlock) {
    // @TODO send up_msg + confirm dialog
    time_block.invoice.take();
}

fn set_invoice_custom_id(invoice_id: InvoiceId, custom_id: &str) {
    // @TODO send up_msg
    zoon::println!("set_invoice_custom_id not implemented yet");
}

fn set_invoice_url(invoice_id: InvoiceId, url: &str) {
    // @TODO send up_msg
    zoon::println!("set_invoice_url not implemented yet");
}

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
//         if let app::Route::ClientsAndProjects = route() {
//             set_clients(None);
//             app::send_up_msg(false, UpMsg::GetTimeBlocksClients);
//         }
//     }

//     #[subscription]
//     fn handle_down_msg() {
//         listen(|msg: Option<DownMsg>| {
//             if let Some(DownMsg::TimeBlocksClients(clients)) = msg {
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
//         time_blocks: Vec<VarC<TimeBlock>>,
//         tracked: Duration,
//         statistics: VarC<Statistics>,
//     }

//     #[s_var]
//     fn client_update_handler() -> VarUpdateHandler<Client> {
//         VarUpdateHandler::new(|client| notify(RecomputeStatistics(client)))
//     }

//     #[s_var]
//     fn clients() -> Option<Vec<VarC<Client>>> {
//         None
//     }

//     #[s_var]
//     fn setting_clients() -> bool {
//         false
//     }

//     #[update]
//     fn set_clients(clients: Vec<shared::time_blocks::Client>) {
//         let clients = match {
//             Some(clients) => clients,
//             None => return clients().set(None);
//         };
//         setting_clients().set(true);
//         stop!{
//             let new_invoice = |time_block: Var<TimeBlock>, invoice: Option<shared::time_blocks::Invoice>| {
//                 invoice.map(|invoice| {
//                     new_var_c(Invoice {
//                         id: invoice.id,
//                         custom_id: invoice.custom_id,
//                         url: invoice.url,
//                         time_block,
//                     })
//                 })
//             };
//             let new_time_blocks = |client: Var<Client>, time_blocks: Vec<shared::time_blocks::TimeBlock>| {
//                 time_blocks.into_iter().map(|time_block| {
//                     let time_block_var = new_var_c(TimeBlock {
//                         id: time_block.id,
//                         name: time_block.name,
//                         status: time_block.status,
//                         duration: time_block.duration,
//                         invoice: None,
//                         client,
//                     });
//                     time_block_var.update_mut(|new_time_block| {
//                         new_time_block.invoice = new_invoice(time_block_var.var(), time_block.invoice);
//                     });
//                     time_block_var
//                 }).collect()
//             };
//             let new_clients = |clients: Vec<shared::time_blocks::Client>| {
//                 clients.into_iter().map(|client| {
//                     let client_var = new_var_c(Client {
//                         iid: client.id,
//                         name: client.name,
//                         time_blocks: Vec::new(),
//                         tracked: client.tracked,
//                         statistics: new_var_c(Statistics::default()),
//                     });
//                     client_var.update_mut(|new_client| {
//                         new_client.time_blocks = new_time_blocks(client_var.var(), client.time_blocks);
//                     });
//                     notify(RecomputeStatistics(client.var()))
//                     client_var
//                 }).collect()
//             };
//             clients().set(Some(new_clients(clients)));
//         }
//         setting_clients().set(false);
//     }

//     // ------ Statistics ------

//     #[derive(Default, Copy, Clone)]
//     struct Statistics {
//         tracked: f64,
//         to_block: f64,
//         blocked: f64,
//         unpaid: f64,
//         paid: f64,
//     }

//     struct RecomputeStatistics(Client);

//     #[subscription]
//     fn recompute_statistics() {
//         listen(|RecomputeStatistics(client)| {
//             client.use_ref(|client| {
//                 let tracked = client.tracked.num_seconds() as f64 / 3600.;
//                 let mut non_billable = 0.;
//                 let mut unpaid = 0.;
//                 let mut paid = 0.;

//                 for time_block in &client.time_blocks {
//                     time_block.use_ref(|time_block| {
//                         let duration = time_block.duration as f64;
//                         use TimeBlockStatus::*;
//                         match time_block.status {
//                             NonBillable => non_billable += duration,
//                             Unpaid => unpaid += duration,
//                             NonBillable => paid += duration,
//                         }
//                     })
//                 }
//                 let blocked = non_billable + unpaid + paid;

//                 client.statistics.set(Statistics {
//                     tracked,
//                     to_block: tracked - blocked,
//                     blocked,
//                     unpaid,
//                     paid,
//                 });
//             })
//         })
//     }

//     // ------ TimeBlock ------

//     #[derive(Debug)]
//     struct TimeBlock {
//         id: TimeBLockId,
//         name: String,
//         status: TimeBlockStatus,
//         duration: Duration,
//         invoice: Option<VarC<Invoice>>,
//         client: Var<Client>,
//     }

//     #[s_var]
//     fn time_block_update_handler() -> VarUpdateHandler<TimeBlock> {
//         VarUpdateHandler::new(|time_block| {
//             let client = time_block.map(|time_block| time_block.client);
//             client.mark_updated();
//         })
//     }

//     #[update]
//     fn add_time_block(client: Var<Client>) {
//         let previous_duration = client.map(|client| {
//             client.time_blocks
//                 .iter()
//                 .next_back()?
//                 .map(|time_block| time_block.duration)
//         });

//         let duration = previous_duration.unwrap_or_else(|| Duration::hours(20));
//         let client_id = client.map(|client| client.id);
//         let time_block_id = TimeBlockId::new();

//         let time_block = new_var_c(TimeBlock {
//             id: time_block_id,
//             name: String::new(),
//             status: TimeBlockStatus::default(),
//             duration,
//             invoice: None,
//             client,
//         });
//         client().update_mut(|client| {
//             client.time_blocks.push(time_block);
//         });
//         app::send_up_msg(
//             true,
//             UpMsg::AddTimeBlock(client_id, time_block_id, duration)
//         );
//     }

//     #[update]
//     fn remove_time_block(time_block: Var<TimeBlock>) {
//         let (client, id) = time_block.map(|time_block| (time_block.client, time_block.id));
//         client.update_mut(|client| {
//             let time_blocks = &mut client.time_blocks;
//             let position = time_blocks.iter_vars().position(|tb| tb == time_block);
//             time_blocks.remove(position.unwrap());
//         });
//         app::send_up_msg(true, UpMsg::RemoveTimeBlock(id));
//     }

//     #[update]
//     fn rename_time_block(time_block: Var<TimeBlock>, name: &str) {
//         time_block.update_mut(|time_block| {
//             time_block.name = name.to_owned();
//             app::send_up_msg(true, UpMsg::RenameTimeBlock(time_block.id, Cow::from(name)));
//         });
//     }

//     #[update]
//     fn set_time_block_status(time_block: Var<TimeBlock>, status: TimeBlockStatus) {
//         time_block.update_mut(|time_block| {
//             time_block.status = status;
//             app::send_up_msg(true, UpMsg::SetTimeBlockStatus(time_block.id, status));
//         });
//     }

//     #[update]
//     fn set_time_block_duration(time_block: Var<TimeBlock>, duration: Duration) {
//         time_block.update_mut(|time_block| {
//             time_block.duration = duration;
//             app::send_up_msg(true, UpMsg::SetTimeBlockDuration(time_block.id, duration));
//         });
//     }

//     // ------ Invoice ------

//     #[derive(Debug)]
//     struct Invoice {
//         id: InvoiceId,
//         custom_id: String,
//         url: String,
//         time_block: Var<TimeBlock>,
//     }

//     #[update]
//     fn add_invoice(time_block: Var<TimeBlock>) {
//         let time_block_id = time_block.map(|time_block| time_block.id);
//         let invoice_id = InvoiceId::new();

//         let invoice = new_var_c(Invoice {
//             id: invoice_id,
//             custom_id: String::new(),
//             url: String::new(),
//             time_block,
//         });
//         time_block().update_mut(|time_block| {
//             time_block.invoice = Some(invoice);
//         });
//         app::send_up_msg(true, UpMsg::AddInvoice(time_block_id, invoice_id));
//     }

//     #[update]
//     fn remove_invoice(invoice: Var<Invoice>) {
//         let (time_block, id) = invoice.map(|invoice| (invoice.time_block, invoice.id));
//         time_block().update_mut(|time_block| time_block.invoice = None);
//         app::send_up_msg(true, UpMsg::RemoveInvoice(id));
//     }

//     #[update]
//     fn set_invoice_custom_id(invoice: Var<Invoice>, custom_id: &str) {
//         invoice.update_mut(|invoice| {
//             invoice.custom_id = custom_id.to_owned();
//             app::send_up_msg(true, UpMsg::SetInvoiceCustomId(invoice.id, Cow::from(custom_id)));
//         });
//     }

//     #[update]
//     fn set_invoice_url(invoice: Var<Invoice>, url: &str) {
//         invoice.update_mut(|invoice| {
//             invoice.url = url.to_owned();
//             app::send_up_msg(true, UpMsg::SetInvoiceUrl(invoice.id, Cow::from(url)));
//         });
//     }

// }
