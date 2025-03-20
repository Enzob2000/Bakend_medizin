use std::{future::Future, pin::Pin};
use async_trait::async_trait;
use serde::{ser::{SerializeStruct, SerializeStructVariant}, Serialize, Serializer};

#[async_trait]
pub trait Irepository_orden<Tinput>:Send+Sync {

    


   async  fn create(&mut self);
    async fn delete(&self, id: String) ->Result<String, String>;
    async fn update(&self, entidad: String, cliente:Tinput, id: String);
}
