use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
pub struct wsmessage(pub String);

