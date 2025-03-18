use actix::fut::ok;
use redis::{aio::MultiplexedConnection, geo, AsyncCommands, Client, Commands, Connection};



pub struct Repositori_localizacion {
    conexion: MultiplexedConnection,
}

impl Repositori_localizacion {
    async fn new() -> Self {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let mut con = client.get_multiplexed_async_connection().await.unwrap();

    

        Self { conexion: con }
    }
}
type Tinput = (f64, f64);
type Touput = String;
type Error = redis::RedisError;

impl Repositori_localizacion {
 

    async fn search(& mut self, list: Vec<Tinput>) -> Result<Vec<Touput>, Error> {

        let lista=self.conexion.geo_radius(
            "lista",
            list[0].0,
            list[0].1,
            10.0,
            geo::Unit::Kilometers,
            geo::RadiusOptions::default(),
        ).await.unwrap();


     Ok(lista)
    }
}
