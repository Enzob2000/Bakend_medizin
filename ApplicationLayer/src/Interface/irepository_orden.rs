use std::{future::Future, pin::Pin};

use serde::{ser::{SerializeStruct, SerializeStructVariant}, Serialize, Serializer};

pub trait Irepository_orden<Tinput>:Send+Sync {

    


    fn create(&mut self)->Pin<Box<dyn Future<Output = ()> + Send>>;
    fn delete(&self, id: String) ->Pin<Box<dyn Future<Output = Result<String, String>> + Send>>;
    fn update(&self, entidad: String, cliente:Tinput, id: String)->Pin<Box<dyn Future<Output = ()> + Send>>;
}
