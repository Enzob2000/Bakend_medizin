
use std::{sync::Arc, time::Instant, vec};
use ApplicationLayer::Interface::Irepository::Irepository;
use  FrameworksDrivers::{Api::Server::server,Repositori::factory_repository_inventary::factory_repository::Factory_repository};
use InterfaceAdapters::DTO::pedidos::cliente_pe::Medicamento;
use std::io;

#[actix_web::main]
 async fn main() {


    let inicio=Instant::now();
//server().
let mut factory_q=Factory_repository::new(vec!["pueba"]).await;

let estado=factory_q.get_estado("pueba").unwrap();



let lista_medi=vec![("Paracetamol".to_string(),10),("Lisinopril".to_string(),20),("Azitromicina".to_string(),12)];

let mut list=Vec::new();

for (i,j) in lista_medi.into_iter(){

let medicamentos=Medicamento{medicamento:i,cantidad:j,precio:3};

list.push(medicamentos);
    
}



let resultado =estado.search(list).await.unwrap();

println!("{:?}",resultado);

let  time=inicio.elapsed();

println!("duracion de {:?}",time);






}
