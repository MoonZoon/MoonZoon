// @TODO rewrite to a proper actor (?)

pub async fn broadcast_down_msg<DMsg>(down_msg: DMsg, cor_id: CorId) {
    join_all(connected_client::by_id().iter().map(|(_, client)| {
        client.send_down_msg(message, req.cor_id)
    })).await
}

