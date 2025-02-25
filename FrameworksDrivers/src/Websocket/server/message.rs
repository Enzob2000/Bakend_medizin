use actix::prelude::*;


#[derive(Message)]
#[rtype(result = "()")]
pub struct message(pub String);

