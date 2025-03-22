use ApplicationLayer::Interface::imapper::Imapper;
use EnterpriseLayer::Entity::entity_geolocalizacion::Geolocalizacion;

use crate::DTO::pedidos::farmacia_pe::DTOfarma;



pub struct MapperGeo;



impl Imapper<DTOfarma,Geolocalizacion> for MapperGeo {

    fn mapper(&self,geo:DTOfarma)->Geolocalizacion {

        let new=Geolocalizacion{
            latitud:geo.latitud,
            longitud:geo.longitud,
        };

        new
        
    }
    
}