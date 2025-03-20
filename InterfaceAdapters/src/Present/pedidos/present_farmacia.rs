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
    pub id_rai:String,
    pub nombre_rai:String,
    pub  geolocalizacion:[f64;2]

}