use std::{any::{type_name, type_name_of_val}, future::Future, pin::Pin, process::Output};

use mongodb::{
    bson::{doc, to_document},
    options::ClientOptions,
    Client, Collection,
};
use serde::Serialize;
use ulid::{Generator, Ulid};
use ApplicationLayer::Interface::irepository_orden::Irepository_orden;
use InterfaceAdapters::{
    Model::model_pedido::Model_pedido,
    DTO::pedidos::{cliente_pe::Pedido, procesar_pedi::Procesar_pedi},
};

use super::factory_repository_inventary::cliente::{self, Clienteoption};

pub struct Repository_orde {
    colletion: Collection<Model_pedido>,
    ids: Generator,
}

impl Repository_orde {
    pub async fn new(cliente: Clienteoption) -> Self {
        let collection = cliente
            .cliente
            .database("info")
            .collection::<Model_pedido>("pedidos");

        let id = ulid::Generator::new();

        Self {
            colletion: collection,
            ids: id,
        }
    }
}

impl Irepository_orden<Procesar_pedi> for Repository_orde {



    fn create(&mut self)->Pin<Box<dyn Future<Output = ()> + Send>> {
        let mut pedido_defa = Model_pedido::default();

        let id = self.ids.generate().unwrap().to_string();
        pedido_defa.id = id;

        let collle=self.colletion.clone();

       Box::pin( async move {
            collle.insert_one(pedido_defa).await.unwrap();
        })
    }

    fn delete(&self, id: String)->Pin<Box<dyn Future<Output = Result<String, String>> + Send>> {
        let filtro = doc! { "id": id };

        let collection = self.colletion.clone();

       Box::pin( async  move{
            match collection.delete_one(filtro).await {
                Ok(_) => return Ok("El pedido fue eliminado".to_string()),
                Err(_) => return Err("El pedido no pudo ser eliminado".to_string()),
            }
        })
    }

    fn update(&self, entidad: String, cliente: Procesar_pedi, id: String)->Pin<Box<dyn Future<Output = ()> + Send>> {
        let documento = match cliente {
            Procesar_pedi::Cliente_pe(cli) => to_document(&cli).unwrap(),
            Procesar_pedi::Faemacia_pe(fa) =>  to_document(&fa).unwrap(),
             Procesar_pedi::Raideri_pe(ra) =>  to_document(&ra).unwrap(),
        };

       
        let filtro = doc! {"id":id};

        let update = doc! {
           "$set":{
          entidad:documento
           }
        };
        let collle=self.colletion.clone();
        Box::pin(async move {
            collle.update_one(filtro, update).await.unwrap();
        })
    }
}
