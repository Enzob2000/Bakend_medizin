use actix::fut::ok;
use redis::{aio::MultiplexedConnection, geo, AsyncCommands, Client, Commands, Connection};

use ApplicationLayer::Interface::irepository::irepository_rai::Irepository_rai;
use EnterpriseLayer::Entity::entity_geolocalizacion::Entity_geolocalizacion;
use async_trait::async_trait;


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

type Touput = String;
type Error = redis::RedisError;

#[async_trait]
impl  Irepository_rai<Entity_geolocalizacion> for Repositori_localizacion {
 

    async fn search(&mut self, list: Entity_geolocalizacion) -> Result<Vec<Touput>, ()> {

        let lista=self.conexion.geo_radius(
            "lista",
            list.latitud,
            list.longitud,
            10.0,
            geo::Unit::Kilometers,
            geo::RadiusOptions::default(),
        ).await.unwrap();


     Ok(lista)
    }
}
