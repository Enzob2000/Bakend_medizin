use std::{clone, thread::sleep};

use futures::stream::Collect;
use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Collection,
};
use ApplicationLayer::Interface::pedidos::irepository::irepository_info::IrepositoryInfo;

use super::factory_repository_inventary::cliente::Clienteoption;
use async_trait::async_trait;


pub struct RepositoryInfo {
    pub collection_cli: Collection<Document>,
    pub collection_raide: Collection<Document>,
    pub collection_farma: Collection<Document>,
}

impl RepositoryInfo {
    pub async fn new(cliente: Clienteoption) -> Self {
        let collection_raide = cliente
            .cliente
            .database("raideri")
            .collection::<Document>("info");

        let collection_farma = cliente
            .cliente
            .database("farmacia")
            .collection::<Document>("info");

        let collection_cli = cliente
            .cliente
            .database("cliente")
            .collection::<Document>("info");

        Self {
            collection_cli,
            collection_raide,
            collection_farma,
        }
    }
}

#[async_trait]
impl IrepositoryInfo<Document> for RepositoryInfo {
    async fn read(&self, id: String) -> Result<Document, String> {
        let filter = doc! {"id":id.clone()};

        let collection = match id{
            s if s.starts_with("cli") => &self.collection_cli,
            s if s.starts_with("fa") => &self.collection_farma,
            _ => &self.collection_raide,
        };

        let result = collection
            .find_one(filter)
            .projection(doc! {"inventario":0})
            .await;

        let mut document=match result {
            Ok(Some(j)) => j,
            Ok(None) => return Err("No exixtes el documento".to_string()),
            Err(_) =>return Err("No exixtes el documento".to_string()),
        };
          
       match document.remove("_id") {
           Some(_) => (),
           None =>return  Err("No existe el _id".to_string())
       };

       Ok(document)
    }
}
