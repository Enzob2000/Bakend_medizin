use actix_web::{get, web, HttpRequest, HttpResponse};
use InterfaceAdapters::DTO::websokect::{ WebsocketDTO};
use actix::prelude::*;
use actix_web::Error;
use super::super::Server::server::Server;
use super::session::Session;
use actix_web_actors::ws;


#[get("/ws")]
pub async fn ws_index(
    data:web::Json<WebsocketDTO>,
    r: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<Server>>,
) -> Result<HttpResponse, Error> {



    let ws = Session{
        id:data.id.clone(),
        name:data.usuario.clone(),
        rol:data.rol,
        addr: srv.get_ref().clone(),
    };
    ws::start(ws, &r, stream)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(ws_index);
       
}