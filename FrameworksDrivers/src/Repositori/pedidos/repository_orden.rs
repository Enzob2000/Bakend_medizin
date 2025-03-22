use actix::fut::ok;
use async_trait::async_trait;
use mongodb::{
    bson::{doc, from_bson_with_options, from_document, to_bson, to_document, Document},
    options::ClientOptions,
    Client, Collection,
};
use serde::{de::DeserializeOwned, Serialize};
use EnterpriseLayer::Entity::entity_entidades::Entidades;
use std::{
    any::{type_name, type_name_of_val},
    default,
    future::Future,
    pin::Pin,
    process::Output,
};
use ulid::{Generator, Ulid};
use ApplicationLayer::Interface::pedidos::irepository::irepository_orden::{ IrepositoryOrden};

use InterfaceAdapters::Model::{
    model_cliente::ModelCliente,
    model_pedido::{EstadoPedido, Model_Farma, Model_Pedido},
    model_raideri::ModelRaider,
};

use super::factory_repository_inventary::cliente::{self, Clienteoption};

pub struct RepositoryOrde {
    colletion: Collection<Model_Pedido>,
}

impl RepositoryOrde {
    pub async fn new(cliente: Clienteoption) -> Self {
        let collection = cliente
            .cliente
            .database("info")
            .collection::<Model_Pedido>("pedidos");

        Self {
            colletion: collection,
        }
    }
}

#[async_trait]
impl IrepositoryOrden<Document,Vec<Entidades>> for RepositoryOrde {

    async fn read(&self,id_pe:String)->Result<Vec<Entidades>,String>{
     let filtro=doc! {
        "id":id_pe
     };

     let projection = doc! {
        "cliente.id": 1,
        "farma.id": 1,
        "raideri.id": 1,
        "_id": 0,
    };
    
     let pedi=match self.colletion.find_one(filtro).projection(projection).await {
         Ok(Some(o)) => o,
         Ok(None)=>return Err("No se encontraron los datos".to_string()),
         Err(_) => return Err("No se encontraron los datos".to_string()),
     };
      
      let farma=Entidades::Farmacia(pedi.farma.id);
      let cliente=Entidades::client(pedi.cliente.id);
      let raidei=Entidades::Raideri(pedi.raideri.id);

      Ok(vec![farma,cliente,raidei])



    }


    async fn create(&mut self, cliente: Document) -> Result<String, String> {
        let mut pedido_defa = Model_Pedido::default();

        let id = Ulid::new().to_string();
        pedido_defa.id = id.clone();

        match self.colletion.insert_one(pedido_defa).await {
            Ok(_) => (),
            Err(_) => return Err("No se pudo crear orden".to_string()),
        };

        match self.update(cliente, id.clone()).await {
            Ok(_) => (),
            Err(_) => return Err("No se pudo insertar el usuario".to_string()),
        };

        Ok(id)
    }

    async fn delete(&self, id_pe: String, id_ente: String) -> Result<String, String> {
        let default = match id_ente {
            s if s.starts_with("cli") => to_document(&ModelCliente::default()),
            s if s.starts_with("fa") => to_document(&Model_Farma::default()),
            _ => to_document(&ModelRaider::default()),
        };

        let docu = match default {
            Ok(d) => d,
            Err(_) => return Err("No se pudo borrar los datos".to_string()),
        };

        match self.update(docu, id_pe).await {
            Ok(_) => Ok("Se actualizo correctamente el pedido".to_string()),
            Err(e) => Err(e),
        }
    }

    async fn update(&self, info: Document, id: String) -> Result<(), String> {
        let filtro = doc! {"id":id};

        let infor = info.clone();

        let entidad = match info.get_str("tipo") {
            Ok(enti) => enti,
            Err(_) => return Err("No se encontro el tipo".to_string()),
        };

        let tipo = infor.get_str("tipo").unwrap();

        let estado = match tipo {
            "cliente" => EstadoPedido::PedidoEntrante,
            "raider" => EstadoPedido::BusquedaRaideri,
            _ => EstadoPedido::BusquedaFarmacia,
        };

        let estado = to_bson(&estado).unwrap();

        let update = doc! {
           "$set":{
          entidad:infor,
          "estado":estado
           }
        };

        match self.colletion.update_one(filtro, update).await {
            Ok(_) => Ok(()),
            Err(_) => Err("NO se pudo actualizar la orden".to_string()),
        }
    }
}
