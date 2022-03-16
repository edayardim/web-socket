use crate::messages::StatisticRecord;
use crate::{ws::WebSocketSession, messages::BroadcastMessage};
use crate::lobby::Lobby;
use actix::Addr;
use actix_web::web::Json;
use actix_web::{get, web::Data, web, web::Payload, Error, HttpResponse, HttpRequest, post};
use actix_web_actors::ws;
use serde_json::json;
use uuid::Uuid;

#[get("/{topic_name}")]
pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    topic_name: web::Path<String>,
    srv: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {

    println!("client");

    let topic_name = topic_name.into_inner();
    let ws = WebSocketSession::new(
        topic_name,
        srv.get_ref().clone(),
    );
    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}

#[post("/dailyDashBoard")]
pub async fn send_statistics(
    websocket_srv: Data<Addr<Lobby>>,
    params: Json<Vec<StatisticRecord>>,
) -> Result<HttpResponse, Error> {

    let _msg =params.into_inner();
    let msg = BroadcastMessage{
        id:Uuid::parse_str("470bb217-ffa7-43d8-a0cc-b3d30421d1a9").unwrap(),
        msg:json!(_msg) ,
        room_id:  "dailyCollection".to_string()
    };
    websocket_srv.do_send(msg);
    return  Ok(HttpResponse::Ok().json(()));
}