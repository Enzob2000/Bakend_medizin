use std::fs::read_to_string;
use std::{fs, sync};


use futures::future::join_all;
use futures::stream::{self, StreamExt};
use mongodb::options::{Collation, CollationAlternate, CollationCaseFirst, CollationMaxVariable, IndexOptions,CollationStrength};
use mongodb::IndexModel;
// Para buffering y streams
use mongodb::{
    bson::{doc, Document},
    error::Error,
    Collection, Database,
};
use ApplicationLayer::Interface::Irepository::Irepository;
use InterfaceAdapters::{
    Model::model_inventory::Model_inventory,
    DTO::pedidos::cliente_pe::{Pedido,Medicamento}
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
                Ok(resp)=>{
                    let cop=resp.clone();
                    println!("{}     {:?}",i,cop)},
                Err(e)=>println!("{}",e)
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
        let pharmacies = self.database.list_collection_names().await?;

        // Para cada farmacia, creamos una tarea asíncrona que verificará que se encuentren todos los medicamentos
        let pharmacy_tasks = pharmacies.into_iter().map(|pharmacy| {
            let collection: Collection<Model_inventory> = self.database.collection(&pharmacy);

            // Clonamos la lista de medicamentos para cada tarea.
            let meds = list_m.clone();
            async move {
                // Disparamos de forma concurrente las búsquedas de cada medicamento en la farmacia actual.
                let med_futures = meds.into_iter().map(|med| {

                        let medicamento=med.medicamento;
                   
                        let filter = doc! {
                            "$and": [
                                { "nombre":  medicamento  },
                                { "cantidad": { "$gte": med.cantidad } }
                            ]
                        };
                    async {
                        // Si se encuentra algún error o no existe el documento, se refleja en el resultado.
                        collection.find_one(filter).await
                    }
                });
                // Ejecutamos todas las búsquedas de medicamentos en paralelo.
                let med_results = join_all(med_futures).await;

                // Verificamos que cada búsqueda haya retornado algún documento
                for res in med_results {
                    match res {
                        Ok(Some(_)) => continue,     // Se encontró el medicamento
                        Ok(None) => return Ok(None), // Falta algún medicamento, esta farmacia no cumple el criterio
                        Err(e) => return Err(e),     // Propagamos cualquier error
                    }
                }
                // Si llegó hasta aquí, es porque en esta farmacia se encontraron todos los medicamentos
                Ok(Some(pharmacy))
            }
        });

        // Ejecutamos las tareas de farmacia en paralelo, limitando la concurrencia para evitar saturar conexiones
        let results = stream::iter(pharmacy_tasks)
            .buffer_unordered(30) // Ajusta el número según tu contexto (por ejemplo, número de conexiones disponibles)
            .collect::<Vec<_>>()
            .await;

        // Procesamos los resultados: filtramos las farmacias válidas y propagamos errores si ocurrieron
        let mut farma = Vec::new();
        for result in results {
            match result {
                Ok(Some(pharmacy)) => farma.push(pharmacy),
                Ok(None) => (),          // La farmacia no cumplió con los criterios
                Err(e) => return Err(e), // Propagamos error desde cualquiera de las búsquedas
            }
        }
        Ok(farma)
    }
}


impl Repositori_inv {

    pub async  fn cargar(&self){


        let path="Jenzo.json";
    let DatabaseOptions=read_to_string(path).unwrap();

  let  datos:Vec<Model_inventory>=serde_json::from_str(DatabaseOptions.as_str()).unwrap();

  let mut id=ulid::Generator::new();

  for i in 0..7500{

    let idt=id.generate().unwrap().0.to_string();

  let coll:Collection<Model_inventory>=self.database.collection(&idt);

  let idt=id.generate().unwrap().0.to_string();

coll.insert_many(datos.clone()).await.unwrap();


  }

  





    }
    
}
