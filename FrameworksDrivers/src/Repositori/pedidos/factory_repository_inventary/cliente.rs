
use mongodb::{error::Error, options::ClientOptions, Client};

pub struct Clienteoption {
    pub cliente: Client,
}

impl Clienteoption {
    pub async fn new() -> Result<Clienteoption, Error> {
        let mut clienteoption = ClientOptions::parse("mongodb://localhost:27017").await?;

        clienteoption.app_name = Some("backend_medizin".to_owned());
        clienteoption.max_connecting=Some(5);
        clienteoption.min_pool_size = Some(10);
        clienteoption.max_pool_size = Some(150);

        let cliente = Client::with_options(clienteoption)?;

        return Ok(Clienteoption { cliente })
    }
}
