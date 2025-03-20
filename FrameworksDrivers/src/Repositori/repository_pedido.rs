use actix::fut::ok;
use async_trait::async_trait;
use mongodb::{
    bson::{doc, to_document, Document},
    options::ClientOptions,
    Client, Collection,
};
use serde::Serialize;
use std::{
    any::{type_name, type_name_of_val},
    future::Future,
    pin::Pin,
    process::Output,
};
use ulid::{Generator, Ulid};
use ApplicationLayer::Interface::irepository::irepository_orden::Irepository_orden;

use InterfaceAdapters::{
    Model::{model_cliente::ModelCliente, model_pedido::Model_Pedido},
    DTO::pedidos::{cliente_pe::Pedido, procesar_pedi::Procesar_pedi},
};

use super::factory_repository_inventary::cliente::{self, Clienteoption};

pub struct Repository_orde {
    colletion: Collection<Model_Pedido>,
    ids: Generator,
}

impl Repository_orde {
    pub async fn new(cliente: Clienteoption) -> Self {
        let collection = cliente
            .cliente
            .database("info")
            .collection::<Model_Pedido>("pedidos");

        let id = ulid::Generator::new();

        Self {
            colletion: collection,
            ids: id,
        }
    }
}

#[async_trait]
impl Irepository_orden<Document> for Repository_orde {
    async fn create(&mut self) {
        let mut pedido_defa = Model_Pedido::default();

        let id = self.ids.generate().unwrap().to_string();
        pedido_defa.id = id;

        self.colletion.insert_one(pedido_defa).await.unwrap();
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
