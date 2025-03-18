use EnterpriseLayer::Entity::{
    entity_geolocalizacion::{self, Entity_geolocalizacion},
    entity_medicamento::Entity_medicamento,
};

use crate::Interface::irepository_orden::Irepository_orden;

use super::super::Interface::irepository_fa::Irepository_pe;

pub struct Pedidos_cli<T>{
    pub repository_pedidos: Box<dyn Irepository_pe<Entity_medicamento, Entity_geolocalizacion>>,
    pub repoository_orden: Box<dyn Irepository_orden<T>>,
}
