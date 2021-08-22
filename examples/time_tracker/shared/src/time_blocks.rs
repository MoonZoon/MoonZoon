use crate::*;

#[derive(Serialize, Deserialize)]
pub struct Client {
    pub id: ClientId,
    pub name: String,
    pub time_blocks: Vec<TimeBlock>,
    // pub tracked: Duration,
}

#[derive(Serialize, Deserialize)]
pub struct TimeBlock {
    pub id: TimeBlockId,
    pub name: String,
    pub status: TimeBlockStatus,
    // pub duration: Duration,
    pub invoice: Option<Invoice>,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum TimeBlockStatus {
    NonBillable,
    Unpaid,
    Paid,
}

#[derive(Serialize, Deserialize)]
pub struct Invoice {
    pub id: InvoiceId,
    pub custom_id: String,
    pub url: String,
}
