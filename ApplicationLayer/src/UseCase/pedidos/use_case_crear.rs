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

pub struct UseCaseGestionar<Strut, DTO> {
    repositori_orden: Box<dyn Irepository_orden< Strut>>,
    repositori_info: Box<dyn IrepositoryInfo<Strut>>,
    repository_pedido: Box<dyn Irepository_pe<Pedido>>,
    mapper_pedido: Box<dyn Imapper<DTO, Pedido>>,

}

impl<Strut: Serialize,  DTO> UseCaseGestionar<Strut, DTO> {
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
            Ok(far) => Ok(far),
            Err(_) => Err("la busqueda no fue exitosa".to_string()),
        }
    }


    pub async  fn actualizr_info(&self,id_pe: String,id_ente:String)->Result<(),String>{

      let info= match self.repositori_info.read(id_ente.clone()).await {
          Ok(inf) => inf,
          Err(e) => return Err(e),
      };

      match self.repositori_orden.update(info,id_pe).await {
          Ok(_) =>Ok(()) ,
          Err(e) => Err(e),
      }

    }

    pub async fn eliminar_info(&self,id_ente:String, id_pe: String)->Result<String,String>{

    match self.repositori_orden.delete(id_pe,id_ente).await {
        Ok(o) => Ok(o),
        Err(e) => Err(e),
    }

    }
}
