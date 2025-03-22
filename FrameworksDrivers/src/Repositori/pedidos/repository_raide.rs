use actix::fut::ok;
use redis::{aio::MultiplexedConnection, geo, AsyncCommands, Client, Commands, Connection};

use ApplicationLayer::Interface::pedidos::irepository::irepository_rai::Irepository_rai;

use async_trait::async_trait;
use EnterpriseLayer::Entity::{
    entity_geolocalizacion::Geolocalizacion, entity_medicamento::Medicamento,
};

pub struct RepositoriRai {
    conexion: MultiplexedConnection,
}

impl RepositoriRai {
    async fn new() -> Self {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let con = client.get_multiplexed_async_connection().await.unwrap();

        Self { conexion: con }
    }
}

#[async_trait]
impl Irepository_rai<Geolocalizacion> for RepositoriRai {
    async fn search(&mut self, list: Geolocalizacion) -> Result<Vec<String>, String> {
        let lista = self
            .conexion
            .geo_radius(
                "lista",
                list.latitud,
                list.longitud,
                10.0,
                geo::Unit::Kilometers,
                geo::RadiusOptions::default(),
            )
            .await;

        let lista = match lista {
            Ok(o) => o,
            Err(_) => return Err("No se encontro raideris disponibles".to_string()),
        };

        Ok(lista)
    }
}
