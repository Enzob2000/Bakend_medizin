use std::fs::read_to_string;
use std::{fs, sync};

use futures::future::join_all;
use futures::stream::{self, StreamExt};
use mongodb::options::{
    Collation, CollationAlternate, CollationCaseFirst, CollationMaxVariable, CollationStrength,
    FindOneOptions, FindOptions, IndexOptions,
};
use mongodb::IndexModel;
// Para buffering y streams
use mongodb::{
    bson::{doc, Document},
    error::Error,
    Collection, Database,
};
use ApplicationLayer::Interface::Irepository::Irepository;
use InterfaceAdapters::Model::model_farma::Model_farma;
use InterfaceAdapters::{
    Model::model_inventory::Model_inventory,
    DTO::pedidos::cliente_pe::{Medicamento, Pedido},
};

// Asumiendo que la estructura y demás código ya se definieron
pub struct Repositori_inv {
    database: Database,
}

impl Repositori_inv {
    pub async fn new(cliente: &mongodb::Client, estado: &str) -> Self {
        let database = cliente.database(estado);
        Self { database }
    }

    pub async fn indexar(&self) {
        let pharmacies = self.database.list_collection_names().await.unwrap();

        let index_model = IndexModel::builder()
            .keys(doc! {

                "cantidad": "text"
            })
            .build();

        for i in pharmacies.into_iter() {
            let collection: Collection<Model_inventory> = self.database.collection(&i);

            let res = collection.create_index(index_model.clone()).await;
            match res {
                Ok(resp) => {
                    let cop = resp.clone();
                    println!("{}     {:?}", i, cop)
                }
                Err(e) => println!("{}", e),
            };
        }
    }
}

impl Irepository for Repositori_inv {
    type Tinput = Medicamento;
    type Touput = String;
    type Error = Error;

    async fn search(&mut self, list_m: Vec<Self::Tinput>) -> Result<Vec<Self::Touput>, Error> {
        // Obtener la lista de nombres de colección (cada farmacia)
        let collection = self.database.collection::<Model_farma>("anzo");

        let lista: Vec<Document> = list_m
            .into_iter()
            .map(|req| {
                doc! {
                        "inventario":{
                        "$elemMatch": {
                            "nombre": req.medicamento,
                            "cantidad": { "$gte": req.cantidad },

                        }
                    }
                }
            })
            .collect();

        let filtro: Document = doc! {
            "productos": { "$all": lista }
        };

        let option = FindOptions::builder().projection(doc! {"idd":1}).build();

        let farma=collection.find(filtro).with_options(option).await?;

        let validas=Vec::new();

        while let Ok(far) =farma.current()  {
            
        }


        Ok(())
    }
}

impl Repositori_inv {
    pub async fn cargar(&self) {
        let path = "C:/Users/HP/proyectos rust/Backed_medizin/FrameworksDrivers/enzo.json";
        let DatabaseOptions = read_to_string(path).unwrap();

        let datos: Model_farma = serde_json::from_str(DatabaseOptions.as_str()).unwrap();

        let coll = self.database.collection::<Model_farma>("anzo");

        for i in 0..7500 {
            let j = coll.insert_one(datos.clone()).await.unwrap();

            println!("{:?}", j);
        }
    }
}
