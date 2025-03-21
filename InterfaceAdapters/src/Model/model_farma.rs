use serde::{Deserialize, Serialize};

use super::model_inventory::Model_inventory;

#[derive(Deserialize, Serialize,Clone,Debug,Default)]
pub struct Model_farma {

    pub tipo:String,
    pub id: String,
    pub nombre: String,
    pub numero_telef:String,
    pub direccion: String,
    pub ubicacion:GeoJsonPoint,
    pub inventario:Vec<Model_inventory>,

}


#[derive(Debug, Serialize, Deserialize,Clone,Default)]
pub struct GeoJsonPoint {
    #[serde(rename = "type")]
    pub geo_type: String, // Siempre será "Point"
    pub coordinates: [f64; 2], // [longitud, latitud]
}