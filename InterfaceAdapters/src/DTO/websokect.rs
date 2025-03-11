use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Clone, Copy)]

pub enum Roles {
    Raideri,
    Cliente,
    Farmacia
    
}


#[derive(Serialize, Deserialize,Clone)]
pub struct WebsocketDTO {
    pub id:String,
    pub usuario: String,
    pub rol:Roles
    
}
