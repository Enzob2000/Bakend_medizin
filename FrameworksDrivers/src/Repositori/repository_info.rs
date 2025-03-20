use futures::stream::Collect;
use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Collection,
};

use async_trait::async_trait;
use ApplicationLayer::Interface::irepository::irepository_info::IrepositoryInfo;
use super::factory_repository_inventary::cliente::Clienteoption;

pub struct RepositoryInfo {
    pub collection: Collection<Document>,
}

impl RepositoryInfo {
    pub async fn new(cliente: Clienteoption) -> Self {
        let collection = cliente
            .cliente
            .database("info")
            .collection::<Document>("info");

        Self { collection }
    }
}

#[async_trait]
impl IrepositoryInfo<Document> for RepositoryInfo {
    async fn read(&self, id: String) -> Result<Document, ()> {
        let filter = doc! {"id":id};

        let result = self
            .collection
            .find_one(filter)
            .projection(doc! {"inventario":0})
            .await;

        match result {
            Ok(Some(j)) => Ok(j),
            Ok(None) => Err(()),
            Err(_) => Err(()),
        }
    }
}
