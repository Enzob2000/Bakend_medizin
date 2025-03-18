use std::future::Future;

use serde::{ser::{SerializeStruct, SerializeStructVariant}, Serialize, Serializer};

pub trait Irepository_orden<Tinput>:Send+Sync {

    type DeleteFuture: Future<Output = Result<String, String>> + Send;


    fn create(&mut self);
    fn delete(&self, id: String) ->Self::DeleteFuture;
    fn update(&self, entidad: String, cliente:Tinput, id: String);
}
