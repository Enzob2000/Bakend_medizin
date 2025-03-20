use std::future::Future;

use serde::Serialize;
use EnterpriseLayer::Entity::{
    entity_geolocalizacion::{self, Entity_geolocalizacion},
    entity_medicamento::Entity_medicamento,
};

use crate::Interface::irepository::{irepository_fa::Irepository_pe, irepository_info::IrepositoryInfo, irepository_orden::Irepository_orden, irepository_rai::Irepository_rai};


pub struct Pedidos_cli<T,U,C>{
    pub repository_inven: Box<dyn Irepository_pe<Entity_medicamento, Entity_geolocalizacion>> ,
    pub repository_orden: Box<dyn Irepository_orden<T>>,
    pub repository_raide:Box<dyn Irepository_rai<U>>,
    pub reposiroty_info:Box<dyn IrepositoryInfo<C>>
}




impl<T,U,C> Pedidos_cli<T,U,C> {

  
pub async fn new_orden(&mut self){

    self.repository_orden.create().await;

    

    

}


    
}