use actix::prelude::*;
use super::wsmessage::wsmessage;
use super::wsServer::Server;

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<wsmessage>,
}



impl Handler<Connect> for Server {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        
      println!("conectado");

      1
    }
}