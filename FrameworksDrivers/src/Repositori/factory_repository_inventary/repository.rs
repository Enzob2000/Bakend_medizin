use actix::fut::ok;
use mongodb::{
    bson::{doc, Document},
    error::Error,
    options::ClientOptions,
    Client, Collection, Database,
};
use InterfaceAdapters::Model::model_inventory::{self, Model_inventory};
use InterfaceAdapters::DTO::pedido::Pedido;
use ApplicationLayer::Interface::Irepository::Irepository;
pub struct Repositori_inv {
    database: Database,
}



impl  Repositori_inv {

   pub async fn new(cliente:&Client,estado:&str) -> Self {
        // Replace the placeholder with your Atlas connection string
        // Get a handle on the movies collection
        let database =cliente.database(estado) ;

        Self { database }
    }
    
}

impl  Irepository for Repositori_inv {

    type Tinput = Pedido;
    type Touput = String;
    type Error = mongodb::error::Error;


    

    async fn search(& mut self, list_m: Vec<Self::Tinput>) ->Result< Vec<Self::Touput>,Error> {
        let mut farma: Vec<String> = vec![];

        let list = self.database.list_collection_names().await?;

        for pharmacy in list.iter() {
            let mut cont = 0;
            let collection = self.database.collection::<Model_inventory>(pharmacy);

            for medicines in list_m.iter() {

                let cantidad=medicines.cantidad;
                let medicina=medicines.medicamento.clone();

                let filter = doc! {
                    "nombre": { "$regex":medicina , "$options": "i" },
                     "cantidad": { "$gte": cantidad }
                };

                let cursor = collection.find_one(filter).await?;

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
    
        Ok(farma)
    }
}
