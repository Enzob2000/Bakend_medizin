
use  FrameworksDrivers::Framework::Server::server;


#[actix_web::main]
async fn main() {
    
server().await;

}
