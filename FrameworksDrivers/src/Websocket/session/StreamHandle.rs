use super::session::Session;
use actix::prelude::*;
use actix_web_actors::ws;
use InterfaceAdapters::{Model::model_farma::Model_farma, DTO::procesar_pedi::Procesar_pedi};

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                // Manejar mensajes de texto recibidos
            let  msg:Model_farma=serde_json::from_str::(text);
                
               
                
            }
            Ok(ws::Message::Binary(bin)) => {
                // Manejar mensajes binarios recibidos
                ctx.binary(bin);
            }
            Ok(ws::Message::Close(_)) => {
                // Manejar la desconexión
                ctx.stop();
            }
            _ => (),
        }
    }
}

impl Session {

    pub fn procesar_m(&self,) {


        
    }
    
}