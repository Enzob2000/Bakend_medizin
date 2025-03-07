use std::collections::HashMap;
use super::repository::Repositori_inv;
use super::cliente::Clienteoption;


struct Factory_repository{
    state:HashMap<String,Repositori_inv>

}


impl Factory_repository {


    pub async fn new(estados:Vec<&str>)->Self{

    let mut states=HashMap::new();

    let cliente=Clienteoption::new().await.unwrap();

    for estado in estados  {


        let new=Repositori_inv::new(&cliente.cliente, estado).await;

        states.insert(estado.to_owned(), new);
        
    }

    Self{state:states}

    }

    pub async fn get(&self,estado:&str)->Option<&Repositori_inv>{


    self.state.get(estado)


    }
    
}