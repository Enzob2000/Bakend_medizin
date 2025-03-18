use serde::{Deserialize,Serialize};

use crate::DTO::pedidos::cliente_pe::Medicamento;

#[derive(Deserialize,Serialize,Clone)]
pub struct view_model_raideri1{

   pub id:String,
   pub direccion:String,
   pub latitud:f64,
   pub longitud:f64,
   pub medicamentos:Vec<Medicamento>    

}