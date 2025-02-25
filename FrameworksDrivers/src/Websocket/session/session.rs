use actix::prelude::*;
use actix_web::{get, web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use super::super::Server::server::Server;
use super::super::Server::message::message;
use super::super::Server::disconnect::Disconnect;
use super::super::Server::connect::Connect;
use actix_web::Error;

pub struct Session {
    /// unique session id
    pub id: usize,

    /// peer name
    pub name: String,

    /// Chat server
    pub addr: Addr<Server>,
}

impl Session {

    pub fn new(id:usize,name:String,addr:Addr<Server>   )->Self{

        Self{

            id,
            name,
            addr
        }
    }
    
}


impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.addr
            .send(Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat server

        println!("se detuvo el cliente {}",self.id);
        
        let id= self.id.clone();

        self.addr.do_send(Disconnect{id  });
        Running::Stop
    }
}


impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                // Manejar mensajes de texto recibidos
                ctx.text(text);
                
               
                
            }
            Ok(ws::Message::Binary(bin)) => {
                // Manejar mensajes binarios recibidos
                ctx.binary(bin);
            }
            Ok(ws::Message::Close(reason)) => {
                // Manejar la desconexión
                ctx.stop();
            }
            _ => (),
        }
    }
}


/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<message> for Session {
    type Result = ();

    fn handle(&mut self, msg: message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

#[get("/ws")]
pub async fn ws_index(
    r: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<Server>>,
) -> Result<HttpResponse, Error> {
    let ws = Session{
        id:1,
        name:"jose".to_owned(),
        // Define un ID de sesión si es necesario
        addr: srv.get_ref().clone(),
    };
    ws::start(ws, &r, stream)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(ws_index);
       
}