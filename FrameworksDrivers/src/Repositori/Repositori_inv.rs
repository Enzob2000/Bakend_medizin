use mongodb::{
    bson::{doc, Document},
    error::Error,
    options::ClientOptions,
    Client, Collection, Database,
};
use InterfaceAdapters::Model::model_inventory::{self, Model_inventory};

pub struct Repositori_inv {
    database: Database,
}

impl Repositori_inv {
    pub async fn new() -> Self {
        // Replace the placeholder with your Atlas connection string
        let uri = "mongodb://localhost:27017";

        // Create a new client and connect to the server
        let client = ClientOptions::parse(uri).await.unwrap();

        let client = Client::with_options(client).unwrap();

        // Get a handle on the movies collection
        let database = client.database("pueba");

        Self { database }
    }

    pub async fn search(&mut self, list_m: Vec<String>) -> Vec<String> {
        let mut farma: Vec<String> = vec![];

        let list = self.database.list_collection_names().await.unwrap();

        for pharmacy in list.iter() {
            let mut cont = 0;
            let collection = self.database.collection::<Model_inventory>(pharmacy);

            for medicines in list_m.iter() {
                let filter = doc! {
                    "nombre": { "$regex": medicines, "$options": "i" },
                     "cantidad": { "$gte": 30 }
                };

                let cursor = collection.find_one(filter).await.unwrap();

                let Some(_) = cursor else {
                    cont = 0;

                    break;
                };

                cont = cont + 1;
            }

            if cont == list_m.len() {
                farma.push(pharmacy.to_owned());
            }
        }

        farma
    }
}
