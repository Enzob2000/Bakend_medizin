use crate::DTO::pedidos::cliente_pe::Medicamento;


pub struct ViewmodelCliente{
    pub id:String,
    pub medicamentos:Vec<Medicamento>,
    pub precioraideri:f32,
    pub preciofinal:f32,
    
}