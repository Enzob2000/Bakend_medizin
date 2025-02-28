use redis::{aio::MultiplexedConnection, AsyncCommands, Client, Commands, Connection};

pub struct Repositori_localizacion {
    conexion: MultiplexedConnection,
}

impl Repositori_localizacion {
    pub async fn new() -> Self {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let mut con = client.get_multiplexed_async_connection().await.unwrap();

        Self { conexion: con }
    }

    pub async fn search(&self) {

        
    }
}
