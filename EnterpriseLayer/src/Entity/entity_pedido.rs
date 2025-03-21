use super::entity_medicamento::Entity_medicamento;


pub  struct Pedido{

   pub id_cli:String,
   pub latitud:f64,
   pub longitud:f64,
   pub medicamentos:Vec<Entity_medicamento>

}