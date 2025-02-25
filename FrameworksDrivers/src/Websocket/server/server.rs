use std::{collections::{HashMap}, sync::{Arc, RwLock}};
use actix::{Actor, Context, Recipient};
use super::message::message;


#[derive(Debug)]
pub struct Server {
   pub sessions:Arc<RwLock<HashMap<usize, Recipient<message>>>>,
}

impl Server {
    pub fn new()->Self{
     Self{
     sessions:Arc::new(RwLock::new(HashMap::new()))
     }

    }
}

impl Actor for  Server{
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}