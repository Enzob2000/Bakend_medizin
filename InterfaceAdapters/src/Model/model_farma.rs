use serde::{Deserialize, Serialize};

use super::model_inventory::Model_inventory;

#[derive(Deserialize, Serialize,Clone,Debug)]
pub struct Model_farma {
    pub id: String,
    pub nombre: String,
    pub numero_telef:String,
    pub direccion: String,
    pub latitud: f64,
    pub longitud: f64,
    pub inventario:Vec<Model_inventory>,

}
