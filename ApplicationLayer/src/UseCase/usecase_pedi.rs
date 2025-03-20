use std::future::Future;

use EnterpriseLayer::Entity::{
    entity_geolocalizacion::{self, Entity_geolocalizacion},
    entity_medicamento::Entity_medicamento,
};

use crate::Interface::{irepository_orden::Irepository_orden, irepository_rai::Irepository_rai};

use super::super::Interface::irepository_fa::Irepository_pe;

pub struct Pedidos_cli<T,U>{
    pub repository_farma: Box<dyn Irepository_pe<Entity_medicamento, Entity_geolocalizacion>> ,
    pub repository_orden: Box<dyn Irepository_orden<T>>,
    pub repository_raide:Box<dyn Irepository_rai<U>>
}
