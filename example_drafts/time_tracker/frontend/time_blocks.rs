use zoon::*;
use ulid::Ulid;
use std::borrow::Cow;
use std::collections::HashSet;
use chrono::{prelude::*, Duration};
use crate::app;
pub use shared::{ClientId, TimeBlockId, InvoiceId, TimeBlockStatus};

pub mod els;

blocks!{
    append_blocks![els]

    #[subscription]
    fn on_route_change() {
        if let app::Route::ClientsAndProjects = route() {
            set_clients(None);
            added_time_block().set(None);
            app::send_up_msg(false, UpMsg::GetTimeBlocksClients);
        }
    }

    #[subscription]
    fn handle_down_msg() {
        app::down_msg().inner().try_update(|down_msg| {
            match down_msg {
                Some(DownMsg::TimeBlocksClients(clients)) => {
                    set_clients(Some(clients));
                    None
                }
                _ => down_msg
            }
        });
    }

    // ------ Client ------

    #[derive(Debug)]
    pub struct Client {
        id: ClientId,
        name: String,
        time_blocks: Vec<Var<TimeBlock>>,
        tracked: Duration,
        statistics: Var<Statistics>,
    }

    #[var]
    fn client_event_handler() -> VarEventHandler<Client> {
        VarEventHandler::new(|event, client| {
            match event {
                VarAdded => {
                    clients().update_mut(|clients| {
                        if let Some(clients) = clients {
                            clients.push(client);
                        }
                    });
                    add_client_to_recompute_queue(client);
                },
                VarChanged => {
                    add_client_to_recompute_queue(client);
                },
                VarRemoved => (),
            }
        })
    }

    #[var]
    fn clients() -> Option<Vec<Var<Client>>> {
        None
    }

    #[update]
    fn set_clients(clients: Vec<shared::time_blocks::Client>) {
        let clients = match {
            Some(clients) => clients,
            None => return clients().set(None);
        };
        stop!{
            clients().set(Some(Vec::new()));
            for client in clients {
                let client_var = var(Client {
                    id: client.id,
                    name: client.name,
                    time_blocks: Vec::new(),
                    tracked: client.tracked,
                    statistics: var(Statistics::default()),
                });
                for time_block in client.time_blocks {
                    let time_block_var = var(TimeBlock {
                        id: time_block.id,
                        name: time_block.name,
                        status: time_block.status,
                        duration: time_block.duration,
                        invoice: None,
                        client: client_var,
                    });
                    if let Some(invoice) = time_block.invoice {
                        var(Invoice {
                            id: invoice.id,
                            custom_id: invoice.custom_id,
                            url: invoice.url, 
                            time_block: time_block_var, 
                        });
                    }
                }
            }
        }
    }

    // ------ Statistics ------

    #[derive(Default, Copy, Clone)]
    struct Statistics {
        tracked: f64,
        to_block: f64,
        blocked: f64,
        unpaid: f64,
        paid: f64,
    }

    #[update]
    fn add_client_to_recompute_queue(client: Var<Client>) {
        recompute_queue().update_mut(|queue_var| {
            queue_var.try_update_mut(|queue| queue.insert(client))
        });
    }

    #[var]
    fn recompute_queue() -> Var<HashSet<Var<Client>>> {
        var(HashSet::new())
    }

    #[subscription]
    fn recompute_statistics() {
        let clients = recompute_queue().inner().map_mut(mem::take);
        for client in clients {
            client.use_ref(|client| {
                let tracked = client.tracked.num_seconds() as f64 / 3600.;
                let mut non_billable = 0.;
                let mut unpaid = 0.;
                let mut paid = 0.;

                for time_block in &client.time_blocks {
                    time_block.use_ref(|time_block| {
                        let duration = time_block.duration as f64;
                        use TimeBlockStatus::*;
                        match time_block.status {
                            NonBillable => non_billable += duration,
                            Unpaid => unpaid += duration,
                            NonBillable => paid += duration,
                        }
                    })
                }
                let blocked = non_billable + unpaid + paid;

                client.statistics.set(Statistics {
                    tracked,
                    to_block: tracked - blocked,
                    blocked,
                    unpaid,
                    paid,
                });
            })
        }
    }

    // ------ TimeBlock ------

    #[derive(Debug)]
    struct TimeBlock {
        id: TimeBLockId,
        name: String,
        status: TimeBlockStatus,
        duration: Duration,
        invoice: Option<Var<Invoice>>,
        client: Var<Client>, 
    }

    #[var]
    fn time_block_event_handler() -> VarEventHandler<TimeBlock> {
        VarEventHandler::new(|event, time_block| {
            let client = time_block.try_map(|time_block| time_block.client).expect("client");
            match event {
                VarAdded => {
                    client.try_update_mut(|client| {
                        client.time_blocks.push(time_block);
                    });
                },
                VarChanged => client.mark_changed(),
                VarRemoved => {
                    client.try_update_mut(|client| {
                        if let Some(position) = client.time_blocks.iter().position(|tb| tb == time_block) {
                            clients.time_blocks.remove(position);
                        }
                    })
                },
            }
        })
    }


    #[var]
    fn added_time_block() -> Option<Var<TimeBlock>> {
        None
    }

    #[update]
    fn add_time_block(client: Var<Client>) {
        let previous_duration = client.try_map(|client| {
            client.time_blocks
                .iter()
                .next_back()
                .map(|time_block| time_block.duration)
        }).flatten();

        let duration = previous_duration.unwrap_or_else(|| Duration::hours(20));
        let client_id = client.try_map(|client| client.id).expect("client id");
        let time_block_id = TimeBlockId::new();

        let time_block = var(TimeBlock {
            id: time_block_id,
            name: String::new(),
            status: TimeBlockStatus::default(),
            duration,
            invoice: None,
            client,
        });
        added_time_block().set(Some(time_block));
        app::send_up_msg(
            true, 
            UpMsg::AddTimeBlock(client_id, time_block_id, duration)
        );
    }

    #[update]
    fn remove_time_block(time_block: Var<TimeBlock>) {
        if let Some(time_block) = time_block.try_remove() {
            app::send_up_msg(true, UpMsg::RemoveTimeBlock(time_block.id));
        }
    }

    #[update]
    fn rename_time_block(time_block: Var<TimeBlock>, name: &str) {
        time_block.try_use_ref(|time_block| {
            app::send_up_msg(true, UpMsg::RenameTimeBlock(time_block.id, Cow::from(name)));
        });
    }

    #[update]
    fn set_time_block_status(time_block: Var<TimeBlock>, status: TimeBlockStatus) {
        time_block.try_update_mut(|time_block| {
            time_block.status = status;
            app::send_up_msg(true, UpMsg::SetTimeBlockStatus(time_block.id, status));
        });
    }

    #[update]
    fn set_time_block_duration(time_block: Var<TimeBlock>, duration: Duration) {
        time_block.try_update_mut(|time_block| {
            time_block.duration = duration;
            app::send_up_msg(true, UpMsg::SetTimeBlockDuration(time_block.id, duration));
        });
    }

    // ------ Invoice ------

    #[derive(Debug)]
    struct Invoice {
        id: InvoiceId,
        custom_id: String,
        url: String, 
        time_block: Var<TimeBlock>, 
    }

    #[var]
    fn invoice_event_handler() -> VarEventHandler<Invoice> {
        VarEventHandler::new(|event, invoice| {
            let time_block = || invoice.try_map(|invoice| invoice.time_block).expect("time_block");
            match event {
                VarAdded => {
                    time_block().try_update_mut(|time_block| {
                        time_block.invoice = Some(invoice);
                    });
                },
                VarChanged => (),
                VarRemoved => {
                    time_block().try_update_mut(|time_block| {
                        time_block.invoice = None;
                    });
                },
            }
        })
    }

    #[update]
    fn add_invoice(time_block: Var<TimeBlock>) {
        let time_block_id = time_block.try_map(|time_block| time_block.id).expect("time_block id");
        let invoice_id = InvoiceId::new();
        var(Invoice {
            id: invoice_id,
            custom_id: String::new(),
            url: String::new(),
            time_block,
        });
        app::send_up_msg(true, UpMsg::AddInvoice(time_block_id, invoice_id));
    }

    #[update]
    fn remove_invoice(invoice: Var<Invoice>) {
        if let Some(invoice) = invoice.try_remove() {
            app::send_up_msg(true, UpMsg::RemoveInvoice(invoice.id));
        }
    }

    #[update]
    fn set_invoice_custom_id(invoice: Var<Invoice>, custom_id: &str) {
        invoice.try_use_ref(|invoice| {
            app::send_up_msg(true, UpMsg::SetInvoiceCustomId(invoice.id, Cow::from(custom_id)));
        });
    }

    #[update]
    fn set_invoice_url(invoice: Var<Invoice>, url: &str) {
        invoice.try_use_ref(|invoice| {
            app::send_up_msg(true, UpMsg::SetInvoiceUrl(invoice.id, Cow::from(url)));
        });
    }

}
