use super::server::Server;
use super::message::message;
use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    /// Id of the client session
    pub id: usize,
    /// Peer message
    pub msg: String,
}

impl Handler<ClientMessage> for Server {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        if let Some(id) = self.sessions.write().unwrap().get(&msg.id) {
            id.do_send(message { 0: msg.msg });
        }
    }
}
