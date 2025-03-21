use serde::{Deserialize, Serialize};

use super::{cliente_pe::DTOPedido, farmacia_pe::DTOfarma, raideri_pe::DTOrai};





#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum DTOprocesar_pedi {
    Cliente_pe(DTOPedido),
    Raideri_pe(DTOrai),
    Faemacia_pe(DTOfarma),
    
}