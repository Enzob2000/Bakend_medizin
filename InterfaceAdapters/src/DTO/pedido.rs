use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Pedido{
    pub medicamento: String,
    pub cantidad: i32,
}
