use mongodb::{
    bson::{doc, Document},
    error::Error,
    options::ClientOptions,
    Client, Collection, Database,
};

use InterfaceAdapters::Model::model_farma::Model_farma;

pub struct Repositori_farma {
    database: Database,
}

impl Repositori_farma {
    pub async fn new() -> Self {
        // Replace the placeholder with your Atlas connection string
        let uri = "mongodb://localhost:27017";

        // Create a new client and connect to the server
        let client = ClientOptions::parse(uri).await.unwrap();

        let client = Client::with_options(client).unwrap();

        // Get a handle on the movies collection
        let database = client.database("farmacias");

        Self { database }
    }

    pub async fn search(&self, farmacias: Vec<String>) {

        let mut lista=vec![];

        let collection = self.database.collection::<Model_farma>("farmacias");

        for farma in  farmacias.iter(){
           
            let filter = doc! {
                "id": farma  
            };

            let curso=collection.find_one(filter).await.unwrap();

            if let Some(far)=curso{

                lista.push(far);
            }
        }
    }
}
