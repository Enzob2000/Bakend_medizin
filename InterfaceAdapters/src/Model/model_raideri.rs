use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize,Clone,Default)]
pub struct ModelRaider{
    pub id:String,
    pub nombre_rai:String,
    pub cedula:String,
}