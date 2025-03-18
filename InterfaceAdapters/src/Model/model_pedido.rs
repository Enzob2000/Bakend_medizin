use std::default;

use serde::{Deserialize,Serialize};

use crate::DTO::pedidos::{cliente_pe::{self, Pedido}, farmacia_pe::Validar_far, raideri_pe::Validar_rai};


#[derive(Deserialize,Serialize,Clone,Default)]
pub enum EstadoPedido {
    #[default]
    PedidoEntrante,     // El pedido recién se ha recibido
    BusquedaFarmacia,   // Se está realizando la búsqueda de la farmacia idónea
    BusquedaRaideri,    // Se está realizando la búsqueda del raideri para la entrega
    Finalizado,         // El pedido ha completado todas las etapas
}

#[derive(Deserialize,Serialize,Clone,Default)]
pub struct Model_pedido{

pub id:String,
pub cliente:Pedido,
pub farma:Validar_far,
pub raideri:Validar_rai,
pub estado_pedido:EstadoPedido


}

