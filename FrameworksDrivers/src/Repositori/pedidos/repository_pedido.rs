use actix::fut::ok;
use async_trait::async_trait;
use mongodb::{
    bson::{doc, to_document, Document},
    options::ClientOptions,
    Client, Collection,
};
use serde::Serialize;
use ApplicationLayer::Interface::pedidos::irepository::irepository_orden::Irepository_orden;
use std::{
    any::{type_name, type_name_of_val},
    future::Future,
    pin::Pin,
    process::Output,
};
use ulid::{Generator, Ulid};


use InterfaceAdapters::{
    Model::{model_cliente::ModelCliente, model_pedido::Model_Pedido},
    
};

use super::factory_repository_inventary::cliente::{self, Clienteoption};

pub struct Repository_orde {
    colletion: Collection<Model_Pedido>,
   
}

impl Repository_orde {
    pub async fn new(cliente: Clienteoption) -> Self {
        let collection = cliente
            .cliente
            .database("info")
            .collection::<Model_Pedido>("pedidos");

        

        Self {
            colletion: collection,
           
        }
    }
}

#[async_trait]
impl Irepository_orden<Document,ModelCliente> for Repository_orde {


    async fn create(&mut self,cliente:ModelCliente)->Result<String,()> {

        let mut pedido_defa = Model_Pedido::default();

        pedido_defa.cliente=cliente;

    
        let id = Ulid::new().to_string();
        pedido_defa.id = id.clone();

        self.colletion.insert_one(pedido_defa).await.map_err(|_|())?;

        Ok(id)
    }

    async fn delete(&self, id: String) -> Result<String, String> {
        let filtro = doc! { "id": id };

        let collection = self.colletion.clone();

        match collection.delete_one(filtro).await {
            Ok(_) => return Ok("El pedido fue eliminado".to_string()),
            Err(_) => return Err("El pedido no pudo ser eliminado".to_string()),
        }
    }

    async fn update(&self, entidad: String, info: Document, id: String) {
        let filtro = doc! {"id":id};

        let update = doc! {
           "$set":{
          entidad:info
           }
        };

        self.colletion.update_one(filtro, update).await.unwrap();
    }
}
