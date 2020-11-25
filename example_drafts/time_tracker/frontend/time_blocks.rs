use zoon::*;
use ulid::Ulid;
use std::borrow::Cow;
use std::collections::HashSet;
use chrono::{prelude::*, Duration};
use crate::app;
pub use shared::{ClientId, TimeBlockId, InvoiceId, TimeBlockStatus, DownMsg};

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
        listen(|msg: Option<DownMsg>| {
            if let Some(DownMsg::TimeBlocksClients(clients)) = msg {
                set_clients(Some(clients));
                return None
            }
            msg
        })
    }

    // ------ Client ------

    #[derive(Debug)]
    pub struct Client {
        id: ClientId,
        name: String,
        time_blocks: Vec<VarC<TimeBlock>>,
        tracked: Duration,
        statistics: VarC<Statistics>,
    }

    #[var]
    fn client_update_handler() -> VarUpdateHandler<Client> {
        VarUpdateHandler::new(|client| notify(RecomputeStatistics(client)))
    }

    #[var]
    fn clients() -> Option<Vec<VarC<Client>>> {
        None
    }

    #[update]
    fn set_clients(clients: Vec<shared::time_blocks::Client>) {
        let clients = match {
            Some(clients) => clients,
            None => return clients().set(None);
        };
        stop!{
            let new_invoice = |time_block: Var<TimeBlock>, invoice: Option<shared::time_blocks::Invoice>| {
                invoice.map(|invoice| {
                    var(Invoice {
                        id: invoice.id,
                        custom_id: invoice.custom_id,
                        url: invoice.url, 
                        time_block,
                    })
                })
            };
            let new_time_blocks = |client: Var<Client>, time_blocks: Vec<shared::time_blocks::TimeBlock>| {
                time_blocks.into_iter().map(|time_block| {
                    let time_block_var = var(TimeBlock {
                        id: time_block.id,
                        name: time_block.name,
                        status: time_block.status,
                        duration: time_block.duration,
                        invoice: None,
                        client,
                    });
                    time_block_var.update_mut(|new_time_block| {
                        new_time_block.invoice = new_invoice(time_block_var.var(), time_block.invoice);
                    });
                    time_block_var
                }).collect()
            };
            let new_clients = |clients: Vec<shared::time_blocks::Client>| {
                clients.into_iter().map(|client| {
                    let client_var = var(Client {
                        iid: client.id,
                        name: client.name,
                        time_blocks: Vec::new(),
                        tracked: client.tracked,
                        statistics: var(Statistics::default()),
                    });
                    client_var.update_mut(|new_client| {
                        new_client.time_blocks = new_time_blocks(client_var.var(), client.time_blocks);
                    });
                    notify(RecomputeStatistics(client.var()))
                    client_var
                }).collect()
            };
            clients().set(Some(new_clients(clients)));
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

    struct RecomputeStatistics(Client);

    #[subscription]
    fn recompute_statistics() {
        listen(|RecomputeStatistics(client)| {
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
        })
    }

    // ------ TimeBlock ------

    #[derive(Debug)]
    struct TimeBlock {
        id: TimeBLockId,
        name: String,
        status: TimeBlockStatus,
        duration: Duration,
        invoice: Option<VarC<Invoice>>,
        client: Var<Client>, 
    }

    #[var]
    fn time_block_update_handler() -> VarUpdateHandler<TimeBlock> {
        VarUpdateHandler::new(|time_block| {
            let client = time_block.map(|time_block| time_block.client);
            client.mark_updated();
        })
    }

    #[var]
    fn added_time_block() -> Option<Var<TimeBlock>> {
        None
    }

    #[update]
    fn add_time_block(client: Var<Client>) {
        let previous_duration = client.map(|client| {
            client.time_blocks
                .iter()
                .next_back()?
                .map(|time_block| time_block.duration)
        });

        let duration = previous_duration.unwrap_or_else(|| Duration::hours(20));
        let client_id = client.map(|client| client.id);
        let time_block_id = TimeBlockId::new();

        let time_block = var(TimeBlock {
            id: time_block_id,
            name: String::new(),
            status: TimeBlockStatus::default(),
            duration,
            invoice: None,
            client,
        });
        added_time_block().set(Some(time_block.var()));
        client().update_mut(|client| {
            client.time_blocks.push(time_block);
        });
        app::send_up_msg(
            true, 
            UpMsg::AddTimeBlock(client_id, time_block_id, duration)
        );
    }

    #[update]
    fn remove_time_block(time_block: Var<TimeBlock>) {
        let client = time_block.map(|time_block| time_block.client);
        let id = client.update_mut(|client| {
            let time_blocks = &mut client.time_blocks;
            let position = time_blocks.iter_vars().position(|tb| tb == time_block).unwrap();
            let id = time_blocks[position].id;
            time_blocks.remove(position);
            id
        });
        app::send_up_msg(true, UpMsg::RemoveTimeBlock(id));
    }

    #[update]
    fn rename_time_block(time_block: Var<TimeBlock>, name: &str) {
        time_block.update_mut(|time_block| {
            time_block.name = name.to_owned();
            app::send_up_msg(true, UpMsg::RenameTimeBlock(time_block.id, Cow::from(name)));
        });
    }

    #[update]
    fn set_time_block_status(time_block: Var<TimeBlock>, status: TimeBlockStatus) {
        time_block.update_mut(|time_block| {
            time_block.status = status;
            app::send_up_msg(true, UpMsg::SetTimeBlockStatus(time_block.id, status));
        });
    }

    #[update]
    fn set_time_block_duration(time_block: Var<TimeBlock>, duration: Duration) {
        time_block.update_mut(|time_block| {
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

    #[update]
    fn add_invoice(time_block: Var<TimeBlock>) {
        let time_block_id = time_block.map(|time_block| time_block.id);
        let invoice_id = InvoiceId::new();

        let invoice = var(Invoice {
            id: invoice_id,
            custom_id: String::new(),
            url: String::new(),
            time_block,
        });
        time_block().update_mut(|time_block| {
            time_block.invoice = Some(invoice);
        });
        app::send_up_msg(true, UpMsg::AddInvoice(time_block_id, invoice_id));
    }

    #[update]
    fn remove_invoice(invoice: Var<Invoice>) {
        let (time_block, id) = invoice.map(|invoice| (invoice.time_block, invoice.id));
        time_block().update_mut(|time_block| time_block.invoice = None);
        app::send_up_msg(true, UpMsg::RemoveInvoice(id));
    }

    #[update]
    fn set_invoice_custom_id(invoice: Var<Invoice>, custom_id: &str) {
        invoice.update_mut(|invoice| {
            invoice.custom_id = custom_id.to_owned();
            app::send_up_msg(true, UpMsg::SetInvoiceCustomId(invoice.id, Cow::from(custom_id)));
        });
    }

    #[update]
    fn set_invoice_url(invoice: Var<Invoice>, url: &str) {
        invoice.update_mut(|invoice| {
            invoice.url = url.to_owned();
            app::send_up_msg(true, UpMsg::SetInvoiceUrl(invoice.id, Cow::from(url)));
        });
    }

}
