use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Model_farma {
    id: String,
    nombre: String,
    direccion: String,
    latitud: u128,
    longitud: u128,
}
