use async_trait::async_trait;
use futures::future::join_all;
use futures::stream::{self, StreamExt};
use mongodb::options::{
    Collation, CollationAlternate, CollationCaseFirst, CollationMaxVariable, CollationStrength,
    FindOneOptions, FindOptions, IndexOptions,
};
use mongodb::IndexModel;
use std::fs::read_to_string;
use std::future::Future;
use std::pin::Pin;
use std::{fs, sync};
// Para buffering y streams
use mongodb::{
    bson::{doc, Document},
    error::Error,
    Collection, Database,
};
use redis::geo;

use ApplicationLayer::Interface::irepository_fa::Irepository_pe;
use InterfaceAdapters::Model::model_farma::{GeoJsonPoint, Model_farma};
use InterfaceAdapters::{
    Model::model_inventory::Model_inventory,
    DTO::pedidos::cliente_pe::{Medicamento, Pedido},
};

// Asumiendo que la estructura y demás código ya se definieron
pub struct Repositori_inv {
    database: Database,
    collection: Collection<Document>,
}

impl Repositori_inv {
    pub async fn new(cliente: &mongodb::Client, estado: &str) -> Self {
        let database = cliente.database("anzo");

        let collection = database.collection::<Document>(&estado);

        Self {
            database: database,
            collection,
        }
    }

    pub async fn indexar(&self) {
        let pharmacies = self.database.list_collection_names().await.unwrap();

        let index_model = IndexModel::builder()
            .keys(doc! {

                 "ubicacion": "2dsphere"

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
type Geo = GeoJsonPoint;
type Tinput = Medicamento;
type Touput = String;

#[async_trait]
impl Irepository_pe<Tinput, Geo> for Repositori_inv {
    

   async fn search(&self, list_m: Vec<Tinput>, geo: Geo) -> Result<Vec<String>, ()> {
        // Obtener la lista de nombres de colección (cada farmacia)

        let mut list_f = list_m
            .into_iter()
            .map(|req| {
                doc! {
                    "inventario": {
                        "$elemMatch": {
                            "nombre": req.medicamento,
                            "cantidad": { "$gte": req.cantidad }
                        }
                    }
                }
            })
            .collect::<Vec<Document>>();

        let fil_geo = doc! {"ubicacion": {
            "$near": {
                "$geometry": {
                    "type": "Point",
                    "coordinates": [geo.coordinates[0], geo.coordinates[1]]
                },
                "$maxDistance": 5000
            }
        }
        };
        list_f.push(fil_geo);
        let filtro: Document = doc! {
                    "$and":list_f



        };

        let option = FindOptions::builder()
            .projection(doc! {"id":1,"_id":0})
            .build();


       
            let mut farma = self.collection
                .find(filtro)
                .with_options(option)
                .await
                .unwrap();

            let mut validas = Vec::new();

            while farma.advance().await.unwrap() {
                let filtro = farma.current().get_str("id");

                match filtro {
                    Ok(far) => validas.push(far.to_string()),
                    Err(_) => (),
                }
            }
            Ok(validas)
        
    }
}

impl Repositori_inv {
    pub async fn cargar(&self) {
        let path = "C:/Users/HP/proyectos rust/Backed_medizin/FrameworksDrivers/enzo.json";
        let DatabaseOptions = read_to_string(path).unwrap();

        let datos: Model_farma = serde_json::from_str(DatabaseOptions.as_str()).unwrap();

        let coll = self.database.collection::<Model_farma>("anzo");

        for i in 0..500 {
            let j = coll.insert_one(datos.clone()).await.unwrap();

            println!("{:?}", j);
        }
    }

    pub async fn prueba(&self) {
        let command = doc! {
            "explain": {
                "find": "anzo", // nombre de la colección
                "filter": {
                    "inventario": {
                        "$elemMatch": {
                            "nombre": "Producto 1",
                            "cantidad": { "$gte": 10 }
                        }
                    }
                },
                "projection": { "id": 1, "_id": 0 }
            },
            // La propiedad "verbosity" puede ser "queryPlanner", "executionStats" o "allPlansExecution"
            "verbosity": "executionStats"
        };

        // Ejecuta el comando en la base de datos
        let explain_result = self.database.run_command(command).await.unwrap();

        // Imprime el resultado del explain
        println!("Resultado de explain: {:#?}", explain_result);
    }
}
