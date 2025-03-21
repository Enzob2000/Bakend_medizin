use serde::{Deserialize, Serialize};

use crate::DTO::pedidos::cliente_pe::Medicamento;

#[derive(Serialize,Deserialize,Clone,Default)]
pub struct PresentFarmapart1{
    pub id:String,
    pub medicamentos:Vec<Medicamento>,
    
}


#[derive(Serialize,Deserialize,Clone,Default)]
pub struct PresentFarmapart2{
    pub id:String,
    pub medicamentos:Vec<Medicamento>,
    pub nombre_rai:String,
    pub cedula:String,
    pub latitud:f64,
    pub longitud:f64

}