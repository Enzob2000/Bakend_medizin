use actix::fut::ok;
use async_trait::async_trait;
use mongodb::{
    bson::{doc, from_bson_with_options, from_document, to_bson, to_document, Document},
    options::ClientOptions,
    Client, Collection,
};
use serde::{de::DeserializeOwned, Serialize};
use ApplicationLayer::Interface::pedidos::irepository::irepository_orden::Irepository_orden;
use std::{
    any::{type_name, type_name_of_val}, default, future::Future, pin::Pin, process::Output
};
use ulid::{Generator, Ulid};


use InterfaceAdapters::Model::{model_cliente::ModelCliente, model_pedido::{EstadoPedido, Model_Farma, Model_Pedido}, model_raideri::ModelRaider};

use super::factory_repository_inventary::cliente::{self, Clienteoption};

pub struct Repository_orde {
    colletion: Collection<Model_Pedido>,
   
}

impl Repository_orde {
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
impl Irepository_orden<Document> for Repository_orde {


    async fn create(&mut self,cliente:Document)->Result<String,()> {

        let mut pedido_defa = Model_Pedido::default();


    
        let id = Ulid::new().to_string();
        pedido_defa.id = id.clone();

        self.colletion.insert_one(pedido_defa).await.map_err(|_|())?;

        self.update(cliente, id.clone()).await.map_err(|_|())?;

        Ok(id)
    }

    async fn delete(&self, id_pe: String,id_ente:String) -> Result<String, String> {

        let default = match id_ente{
            s if s.starts_with("cli") => to_document(&ModelCliente::default()),
            s if s.starts_with("fa") => to_document(&Model_Farma::default()),
            _ => to_document(&ModelRaider::default()),
        };

        let docu=match default {
            Ok(d) => d,
            Err(_) => return Err("No se pudo borrar los datos".to_string()),
        };
            
    

        match self.update(docu, id_pe).await {
            Ok(_) => Ok("Se actualizo correctamente el pedido".to_string()),
            Err(e) => Err(e),
        }

        
    }

    async fn update(&self,  info: Document, id: String)->Result<(),String> {
        let filtro = doc! {"id":id};

        let infor=info.clone();

        let entidad=match info.get_str("tipo") {
            Ok(enti) => enti,
            Err(_) => return Err("No se encontro el tipo".to_string()),
        };
        
        let tipo=infor.get_str("tipo").unwrap();

        let estado=match tipo {
            "cliente"=>EstadoPedido::PedidoEntrante,
            "raider"=>EstadoPedido::BusquedaRaideri,
            _=>EstadoPedido::BusquedaFarmacia

            
        };

        let estado=to_bson(&estado).unwrap();

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
