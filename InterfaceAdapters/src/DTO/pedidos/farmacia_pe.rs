use serde::{Deserialize, Serialize};

use super::cliente_pe::{Medicamento, Ubicacion};

#[derive(Deserialize, Serialize,Clone)]
pub struct Validar_far {
    id_orden: String,
    id_far: String,
    nombre_far: String,
    direccion: String,
    geolocalizacion: Ubicacion,
    medicamentos: Vec<Medicamento>,
}
