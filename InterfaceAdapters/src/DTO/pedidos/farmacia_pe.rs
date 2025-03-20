use serde::{Deserialize, Serialize};

use crate::Model::model_farma::GeoJsonPoint;

use super::cliente_pe::{Medicamento};

#[derive(Deserialize, Serialize,Clone,Default)]
pub struct Validar_far {
    pub id_orden: String,
    pub id_far: String,
    
}
