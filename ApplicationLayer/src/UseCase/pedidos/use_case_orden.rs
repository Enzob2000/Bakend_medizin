use serde::Serialize;
use EnterpriseLayer::Entity::entity_entidades::Entidades;

use crate::Interface::{
    imapper::Imapper,
    pedidos::irepository::{
        irepository_fa::Irepository_pe, irepository_info::IrepositoryInfo,
        irepository_orden::Irepository_orden, irepository_rai::Irepository_rai,
    },
};

pub struct UseCaseGestionar<Strut> {
    repositori_orden: Box<dyn Irepository_orden<Strut, Vec<Entidades>>>,
    repositori_info: Box<dyn IrepositoryInfo<Strut>>,
}

impl<Strut> UseCaseGestionar<Strut> {
    pub async fn notificar(&self, id_pe: String) -> Result<Vec<Entidades>, String> {
        match self.repositori_orden.read(id_pe).await {
            Ok(o) => Ok(o),
            Err(e) => Err(e),
        }
    }

    pub async fn crear_pedido(&mut self, cliente: String) -> Result<String, String> {
        let cliente_info = self.repositori_info.read(cliente).await.unwrap();

        match self.repositori_orden.create(cliente_info).await {
            Ok(id) => Ok(id),
            Err(e) => Err(e),
        }
    }

    pub async fn actualizr_info(&self, id_pe: String, id_ente: String) -> Result<(), String> {
        let info = match self.repositori_info.read(id_ente.clone()).await {
            Ok(inf) => inf,
            Err(e) => return Err(e),
        };

        match self.repositori_orden.update(info, id_pe).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn eliminar_info(&self, id_ente: String, id_pe: String) -> Result<String, String> {
        match self.repositori_orden.delete(id_pe, id_ente).await {
            Ok(o) => Ok(o),
            Err(e) => Err(e),
        }
    }
}
