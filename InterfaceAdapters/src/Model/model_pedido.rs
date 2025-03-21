use std::default;

use serde::{Deserialize, Serialize};

use super::{model_cliente::ModelCliente, model_farma::Model_farma, model_raideri::ModelRaider};

#[derive(Deserialize, Serialize, Clone, Default)]
pub enum EstadoPedido {
    #[default]
    PedidoEntrante, // El pedido recién se ha recibido
    BusquedaFarmacia, // Se está realizando la búsqueda de la farmacia idónea
    BusquedaRaideri,  // Se está realizando la búsqueda del raideri para la entrega
    Finalizado,       // El pedido ha completado todas las etapas
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Model_Pedido {
    pub id: String,
    pub fecha: Fecha,
    pub destino: Destino,
    pub cliente: ModelCliente,
    pub farma: Model_farma,
    pub raideri: ModelRaider,
    pub estado_pedido: EstadoPedido,
}

pub struct Model_Farma {
    pub id: String,
    pub nombre: String,
    pub numero_telef: String,
    pub direccion: String,
    pub ubicacion: [f64; 2],
}
#[derive(Deserialize, Serialize, Clone, Default)]

pub struct Destino {
    pub latitud: f64,
    pub longitud: f64,
    pub direccion: String,
}

#[derive(Deserialize, Serialize, Clone, Default)]

pub struct Fecha {
    pub dia: u8,
    pub mes: u8,
    pub ano: u16,
}
