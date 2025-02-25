use actix::prelude::*;
use super::message::message;
use super::server::Server;

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<message>,
}



impl Handler<Connect> for Server {
    type Result = usize;

    fn handle(&mut self, _: Connect, _: &mut Context<Self>) -> Self::Result {
        
      println!("conectado");

      1
    }
}