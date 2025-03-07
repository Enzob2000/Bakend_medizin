use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Medicamento {
    pub medicamento: String,
    pub cantidad: i32,
}
#[derive(Serialize, Deserialize)]

pub struct Ubicacion {
    pub latitud: f64,
    pub longitud: f64,
}

#[derive(Serialize, Deserialize)]

pub struct Pedido {
    pub id: String,
    pub nombre: String,
    pub direccion: String,
    pub geolocalizacion: Ubicacion,
    pub medicamentos: Vec<Medicamento>,
}
