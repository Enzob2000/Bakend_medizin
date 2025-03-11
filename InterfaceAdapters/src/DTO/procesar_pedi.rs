use serde::{Deserialize, Serialize};

use super::{cliente::pedido::Pedido, raideri::validar_rai::Validar_rai};

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Procesar_pedi {
    Cliente_pe(Pedido),
    Raideri_pe(Validar_rai),
    Faemacia_pe(Validar_rai),
    
}