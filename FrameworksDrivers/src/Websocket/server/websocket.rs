use actix::prelude::*;
use actix_web_actors::ws;
use super::super::session::wsServer::Server;
use super::super::session::wsmessage::wsmessage;
use super::super::session::disconnect::Disconnect;
use super::super::session::connect::Connect;
pub struct WsChatSession {
    /// unique session id
    pub id: usize,

    /// peer name
    pub name: Option<String>,

    /// Chat server
    pub addr: Addr<Server>,
}


impl Actor for WsChatSession {
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
        self.addr.do_send(Disconnect{ id:1 });
        Running::Stop
    }
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<wsmessage> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: wsmessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}