use crate::*;

#[derive(Serialize)]
pub struct DownMsgTransporterForSer<'a, DMsg: Serialize> {
    pub down_msg: &'a DMsg,
    pub cor_id: CorId,
}

#[derive(Deserialize)]
pub struct DownMsgTransporterForDe<DMsg: Deserialize> {
    pub down_msg: DMsg,
    pub cor_id: CorId,
}
