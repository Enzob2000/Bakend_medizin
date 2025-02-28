
use  FrameworksDrivers::{Api::Server::server,Repositori::Repositori_inv::Repositori_inv};


#[actix_web::main]
async fn main() {
    
//server().await;
let mut repo=Repositori_inv::new().await;

repo.search(vec!["juan".to_owned()]).await;


}
