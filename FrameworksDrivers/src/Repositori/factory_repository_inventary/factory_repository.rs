use ApplicationLayer::Interface::Irepository_farma::Irepository;

use super::cliente::Clienteoption;
use super::repository::Repositori_inv;
use std::collections::HashMap;

pub struct Factory_repository {
    pub state: HashMap<String, Repositori_inv>,
}

impl Factory_repository {
    pub async fn new(estados: Vec<&str>) -> Self {
        let mut states = HashMap::new();

        let cliente = Clienteoption::new().await.unwrap();

        for estado in estados {
            let new = Repositori_inv::new(&cliente.cliente, estado).await;


            states.insert(estado.to_owned(), new);
        }

        Self { state: states }
    }

    pub  fn get_estado(&mut self, estado: &str) -> Option<&mut Repositori_inv> {
        
        self.state.get_mut(estado)
        
        

        
    }
}
