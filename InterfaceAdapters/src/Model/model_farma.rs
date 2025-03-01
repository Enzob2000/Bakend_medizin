use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Model_farma {
    pub id: String,
    pub nombre: String,
    pub direccion: String,
    pub latitud: u128,
    pub longitud: u128,
}
