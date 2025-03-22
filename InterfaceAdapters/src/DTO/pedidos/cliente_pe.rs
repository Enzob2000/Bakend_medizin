use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize,Clone)]
pub struct Medicamento {
    pub medicamento: String,
    pub cantidad: i8,
 
}


#[derive(Serialize, Deserialize,Clone)]

pub struct DTOPedido {
    
    pub id_cliente:String,
    pub tipo:Tipo,
    pub latitud:f64,
    pub longitud:f64,
    pub direccion:String,
    pub medicamentos: Vec<Medicamento>,
}

#[derive(Serialize, Deserialize,Clone)]
enum Tipo {
    Delivery,
    PickUp
    
}