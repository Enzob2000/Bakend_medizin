use ApplicationLayer::Interface::imapper::Imapper;
use EnterpriseLayer::Entity::{ entity_medicamento::Medicamento, entity_pedido:: Pedido};

use crate::{Mapper, DTO::pedidos::cliente_pe::DTOPedido};

pub struct MapperPedido;

impl Imapper<DTOPedido, Pedido> for MapperPedido {
    fn mapper(&self, dto: DTOPedido) -> Pedido {
        let new =Pedido  {
            latitud:dto.latitud,
            longitud:dto.longitud,
            medicamentos:dto.medicamentos.iter().map(

                |x| Medicamento{
                    nombre:x.medicamento.clone(),
                    cantidad:x.cantidad
                }
            ).collect()
        };

        new
    }
}
