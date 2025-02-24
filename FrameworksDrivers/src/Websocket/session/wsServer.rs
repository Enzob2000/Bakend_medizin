use std::collections::{HashMap, HashSet};
use actix::{Actor, Context, Recipient};
use super::wsmessage::wsmessage;


#[derive(Debug)]
pub struct Server {
    sessions: HashMap<usize, Recipient<wsmessage>>,
}

impl Actor for Server {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}