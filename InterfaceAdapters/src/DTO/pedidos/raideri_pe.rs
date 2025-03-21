use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize,Clone,Default)]
pub struct DTOrai{
    id_orden:String,
    id_rai:String,
    localizacion:String
    
}