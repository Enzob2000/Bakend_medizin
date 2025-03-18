use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize,Clone,Default)]
pub struct Validar_rai{
    id_orden:String,
    id_rai:String,
    nombre_rai:String,
    numero_tele:String,
    localizacion:String
    
}