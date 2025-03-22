use EnterpriseLayer::Entity::{entity_geolocalizacion::Geolocalizacion, entity_pedido::Pedido};

use crate::Interface::{imapper::Imapper, pedidos::irepository::{irepository_fa::IrepositoryFa, irepository_rai::IrepositoryRai}};


pub struct UseCaseBusque<TinputF,TinputR>{

    repository_farma:Box<dyn IrepositoryFa<Pedido>>,
    mapper_farma:Box<dyn Imapper<TinputF,Pedido>>,
    repository_rai:Box<dyn IrepositoryRai<Geolocalizacion>>,
    mapper_rai:Box<dyn Imapper<TinputR,Geolocalizacion>>
}

impl <TinputF,TinputR> UseCaseBusque<TinputF,TinputR>{

    pub async fn busqueda_far(&self, medica: TinputF) -> Result<Vec<String>, String> {
        let pedido = self.mapper_farma.mapper(medica);

        match self.repository_farma.search(pedido).await {
            Ok(far) => Ok(far),
            Err(e) => Err(e),
        }
    }

    pub async fn busquda_rai(&mut self,geo_fa:TinputR)->Result<Vec<String>, String>{

     let geo=self.mapper_rai.mapper(geo_fa);

     match self.repository_rai.search(geo).await {
         Ok(o) => Ok(o),
         Err(e) =>Err(e) 
     }


    }
    
}