use super::session::Session;
use actix::prelude::*;
use super::super::Server::message::message;


impl Handler<message> for Session {
    type Result = ();

    fn handle(&mut self, msg: message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}