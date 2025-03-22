use std::{future::Future, pin::Pin};
use async_trait::async_trait;
use serde::{ser::{SerializeStruct, SerializeStructVariant}, Serialize, Serializer};

#[async_trait]
pub trait IrepositoryOrden<Cliente,Ente>:Send+Sync {

    
    async fn read(& self,id_pe:String,)->Result<Ente,String>;
    async  fn create(&mut self,cliente:Cliente)->Result<String,String>;
    async fn delete(&self, id_pe: String,id_ente:String) ->Result<String, String>;
    async fn update(&self, cliente:Cliente, id: String)->Result<(),String>;
}
