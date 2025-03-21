use serde::{Deserialize, Serialize};

use super::{cliente_pe::Pedido, farmacia_pe::Validar_far, raideri_pe::Validar_rai};



#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Procesar_pedi {
    Cliente_pe(Pedido),
    Raideri_pe(Validar_rai),
    Faemacia_pe(Validar_far),
    
}