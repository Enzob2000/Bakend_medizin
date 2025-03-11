use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Validar_rai{
    id_orden:String,
    id_rai:String,
    nombre_rai:String,
    validar:bool
}