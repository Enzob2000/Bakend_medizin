use EnterpriseLayer::Entity::entity_medicamento::Medicamento;



use std::collections::HashMap;

use crate::Repositori::pedidos::repository_medica::RepositoriMedi;

use super::cliente::Clienteoption;

pub struct Factory_repository {
    pub state: HashMap<String, RepositoriMedi>,
}

impl Factory_repository {
    pub async fn new(estados: Vec<&str>, cliente: Clienteoption) -> Self {
        let mut states = HashMap::new();

        for estado in estados {
            let new = RepositoriMedi::new(&cliente.cliente, estado).await;

            states.insert(estado.to_owned(), new);
        }

        Self { state: states }
    }

    pub fn get_estado(&mut self, estado: &str) -> Option<&mut RepositoriMedi> {
        self.state.get_mut(estado)
    }
}
