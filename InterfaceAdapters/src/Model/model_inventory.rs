use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize)]
pub struct Model_inventory {
    pub idd: u16,
    pub nombre: String,
    pub categoria: String,
    pub cantidad: u8,
    pub precio: f32,
}

