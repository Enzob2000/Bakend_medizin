
use std::{sync::Arc, time::Instant, vec};
use ApplicationLayer::Interface::irepository_farmacia::Irepository;
use  FrameworksDrivers::{Api::Server::server,Repositori::factory_repository_inventary::factory_repository::Factory_repository};
use InterfaceAdapters::DTO::pedidos::cliente_pe::{Medicamento, Ubicacion};
use std::io;

#[actix_web::main]
 async fn main() {


    let inicio=Instant::now();
//server().
let mut factory_q=Factory_repository::new(vec!["prueba"]).await;

let mut estado=factory_q.get_estado("prueba").unwrap();
//estado.prueba().await;
//estado.cargar().await;
estado.indexar().await;
let lista_medi=vec![("Producto 100".to_string(),10),("Producto 2".to_string(),10),("Producto 3".to_string(),10)];

let mut list=Vec::new();

for (i,j) in lista_medi.into_iter(){

let medicamentos=Medicamento{medicamento:i,cantidad:j,precio:3};

list.push(medicamentos);
    
}



let resultado =estado.search(list,Ubicacion{latitud:-73.808577,longitud: 40.848447}).await.unwrap();

println!("{:?}",resultado);

let  time=inicio.elapsed();

println!("duracion de {:?}",time);






}
