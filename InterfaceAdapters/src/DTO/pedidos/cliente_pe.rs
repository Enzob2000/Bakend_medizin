use serde::{Deserialize, Serialize};

use crate::Model::model_farma::GeoJsonPoint;

#[derive(Serialize, Deserialize,Clone,Default)]
pub struct Medicamento {
    pub medicamento: String,
    pub cantidad: i32,
    pub precio:i32,
}


#[derive(Serialize, Deserialize,Clone,Default)]

pub struct Pedido {
    pub id:String,
    pub id_cliente:String,
    pub nombre: String,
    pub direccion: String,
    pub geolocalizacion: GeoJsonPoint,
    pub medicamentos: Vec<Medicamento>,
}
