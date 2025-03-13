use serde::{Deserialize, Serialize};

use super::model_inventory::Model_inventory;

#[derive(Deserialize, Serialize)]
pub struct Model_farma {
    pub id: String,
    pub nombre: String,
    pub numero_telef:String,
    pub direccion: String,
    pub latitud: u128,
    pub longitud: u128,
    pub inventario:Vec<Model_inventory>,

}
