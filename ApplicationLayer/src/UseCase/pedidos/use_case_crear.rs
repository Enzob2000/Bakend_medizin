use serde::Serialize;
use EnterpriseLayer::Entity::{entity_geolocalizacion::Entity_geolocalizacion, entity_medicamento::Entity_medicamento};

use crate::Interface::pedidos::irepository::{
    irepository_fa::Irepository_pe, irepository_info::IrepositoryInfo, irepository_orden::Irepository_orden
};

pub struct UseCaseGestionar<Strut,ModelCliente> {
    repositori_orden: Box<dyn Irepository_orden<ModelCliente, Strut>>,
    repositori_info: Box<dyn IrepositoryInfo<Strut>>,
    repository_pedido:Box<dyn Irepository_pe<Entity_medicamento,Entity_geolocalizacion>>
}

impl<Strut: Serialize,ModelCliente> UseCaseGestionar<Strut,ModelCliente>
 {
    pub async fn crear_pedido(&mut self, cliente: String) -> Result<String, String> {
         
        let cliente_info= self.repositori_info.read(cliente).await.unwrap();    
        
        match self.repositori_orden.create(cliente_info).await {
            Ok(id) => Ok(id),
            Err(_) => Err("No se pudo crear el pedido".to_string()),
        }

        self.repository_pedido.search(med, geo)


    }


    
}
