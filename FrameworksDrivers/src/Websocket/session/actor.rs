use actix::Actor;
use actix_web_actors::ws;
use actix::prelude::*;
use super::session::Session;
use super::super::{Server::connect::Connect,Server::disconnect::Disconnect};

impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
    
        let addr = ctx.address();
        self.addr
            .send(Connect {
                id:self.id.clone(),
                rol:self.rol,
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(_) =>(),
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
        

        self.addr.do_send(Disconnect{id:self.id.clone(),rol:self.rol });
        Running::Stop
    }
}