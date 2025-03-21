use serde::{Deserialize, Serialize};

use crate::Model::model_farma::GeoJsonPoint;

#[derive(Serialize, Deserialize,Clone,Default)]
pub struct Medicamento {
    pub medicamento: String,
    pub cantidad: u8,
 
}


#[derive(Serialize, Deserialize,Clone,Default)]

pub struct DTOPedido {
    
    pub id_cliente:String,
    pub latitud:f64,
    pub longitud:f64,
    pub direccion:String,
    pub medicamentos: Vec<Medicamento>,
}
