use serde::{Deserialize, Serialize};

use crate::DTO::pedidos::cliente_pe::Medicamento;

#[derive(Deserialize, Serialize, Clone, Default)]

pub struct ViewmodelRaider {
    pub id: String,
    pub medicamentos: Vec<Medicamento>,
    pub nombre_farma: String,
    pub direccion_farma: String,
    pub geolocalizacion_farma: [f64; 2],
    pub nombre_cli: String,
    pub direccion_cli: String,
    pub geolocalizacion_cli: [f64; 2],
}
