use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize,Clone,Default)]
pub struct ModelCliente{
   pub tipo:String,
   pub id:String,
   pub nombre_cli:String,
   pub cedula:String,
   
}