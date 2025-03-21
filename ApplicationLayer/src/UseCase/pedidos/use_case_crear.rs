use serde::Serialize;
use EnterpriseLayer::Entity::{
    entity_geolocalizacion::Entity_geolocalizacion,
    entity_medicamento::Entity_medicamento,
    entity_pedido::{ Pedido},
};

use crate::Interface::{
    imapper::Imapper,
    pedidos::irepository::{
        irepository_fa::Irepository_pe, irepository_info::IrepositoryInfo,
        irepository_orden::Irepository_orden,
    },
};

pub struct UseCaseGestionar<Strut, ModelCliente, DTO> {
    repositori_orden: Box<dyn Irepository_orden<ModelCliente, Strut>>,
    repositori_info: Box<dyn IrepositoryInfo<Strut>>,
    repository_pedido: Box<dyn Irepository_pe<Pedido>>,
    mapper_pedido: Box<dyn Imapper<DTO, Pedido>>,
}

impl<Strut: Serialize, ModelCliente, DTO> UseCaseGestionar<Strut, ModelCliente, DTO> {
    pub async fn crear_pedido(&mut self, cliente: String) -> Result<String, String> {
        let cliente_info = self.repositori_info.read(cliente).await.unwrap();

        match self.repositori_orden.create(cliente_info).await {
            Ok(id) => Ok(id),
            Err(_) => Err("No se pudo crear el pedido".to_string()),
        }
    }

    pub async fn buscar_pedido(&self, cliente: DTO) -> Result<Vec<String>, String> {
        let pedido = self.mapper_pedido.mapper(cliente);

        match self.repository_pedido.search(pedido).await {
            Ok(med) => Ok(med),
            Err(_) => Err("la busqueda no fue exitosa".to_string()),
        }
    }
}
