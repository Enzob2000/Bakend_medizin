use actix::fut::err;
use mongodb::{
    bson::{doc, Document},
    error::Error,
    options::ClientOptions,
    Client, Collection, Database,
};

use ApplicationLayer::Interface::Irepository::Irepository;
use InterfaceAdapters::Model::model_farma::Model_farma;

pub struct Repositori_farma {
    database: Database,
}

impl Irepository for Repositori_farma {
    type Error = mongodb::error::Error;
    type Tinput = String;
    type Touput = Model_farma;

    async fn new() -> Self {
        // Replace the placeholder with your Atlas connection string
        let uri = "mongodb://localhost:27017";

        // Create a new client and connect to the server
        let client = ClientOptions::parse(uri).await.unwrap();

        let client = Client::with_options(client).unwrap();

        // Get a handle on the movies collection
        let database = client.database("farmacias");

        Self { database }
    }

    async fn search(&self, farmacias: Vec<Self::Tinput>) -> Result<Vec<Self::Touput>, Self::Error> {
        let mut lista = vec![];

        let collection = self.database.collection::<Self::Touput>("farmacias");

        for farma in farmacias.iter().clone() {
            let filter = doc! {
                "id": farma
            };

            let curso = collection.find_one(filter).await?;

            if let Some(far) = curso {
                lista.push(far);
            }
        }
        Ok(lista)
    }
}
