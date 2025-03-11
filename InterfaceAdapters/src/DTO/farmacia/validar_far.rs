use serde::{Deserialize, Serialize};

use crate::DTO::cliente::pedido::Ubicacion;

#[derive(Deserialize, Serialize)]
pub struct Validar_far{
    id_orden:String,
    id_far:String,
    nombre_far:String,
    direccion:String,
    geolocalizacion:Ubicacion,
    validar:bool
}