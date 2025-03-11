use std::{collections::HashMap, sync::{Arc, Mutex, RwLock}};
use actix::{Actor, Context, Recipient};
use super::message::message;


#[derive(Debug)]
pub struct Server {
   pub farmacias:Arc<Mutex<HashMap<String, Recipient<message>>>>,
   pub raideris:Arc<Mutex<HashMap<String, Recipient<message>>>>,
   pub clientes:Arc<Mutex<HashMap<String, Recipient<message>>>>,

}

impl Server {
    pub fn new()->Self{
     Self{
     farmacias:Arc::new(Mutex::new(HashMap::new())),
     raideris:Arc::new(Mutex::new(HashMap::new())),
     clientes:Arc::new(Mutex::new(HashMap::new()))
     
     }

    }
}

impl Actor for  Server{
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}