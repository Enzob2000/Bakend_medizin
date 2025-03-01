use redis::{aio::MultiplexedConnection, AsyncCommands, Client, Commands, Connection};
use ApplicationLayer::Interface::Irepository::Irepository;

pub struct Repositori_localizacion {
    conexion: MultiplexedConnection,
}

impl Irepository for Repositori_localizacion {


     type Tinput = ;
     type Touput = ;
     type Error= ;



     async fn new() -> Self {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let mut con = client.get_multiplexed_async_connection().await.unwrap();

        Self { conexion: con }
    }

     async fn search(&self,list:Vec<Self::Tinput>)->Result<Vec<Self::Touput>,Self::Error> {

        
    }
}
