use actix::prelude::{Message, Recipient};
use uuid::Uuid;
use serde_json::Value;
use serde::{Deserialize, Serialize};

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub lobby_id: String,
    pub self_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
    pub room_id: String,
}

/*
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    pub id: Uuid,
    pub msg: String,
    pub room_id: String
}
 */

#[derive(Message, Deserialize, Serialize, Clone)]
#[rtype(result = "()")]
pub struct BroadcastMessage {
    pub id: Uuid,
    pub msg: Value,
    pub room_id: String
}

impl BroadcastMessage {
    pub fn _new(id: Uuid, data: Value, r_id: String) -> Self {
        Self {
            id,
            msg :data,
            room_id: r_id
        }
    }
}





#[derive(Serialize, Deserialize,Debug, Clone)]
pub struct StatisticRecord {
    pub product_line: Option<String>,
    pub product_line_id: i32,
    pub total_trx: Option<i64>,
    pub total_amount: Option<f64>,
    pub currency: Option<String>,
    pub group_status: Option<String>
}