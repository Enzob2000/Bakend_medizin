use actix::prelude::*;
use super::server::Server;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}


impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
       
       self.sessions.write().unwrap().remove(&msg.id);

    }
}