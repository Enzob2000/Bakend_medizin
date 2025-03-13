use super::session::Session;
use actix::prelude::*;
use actix_web::body::MessageBody;
use actix_web::web::Bytes;
use actix_web_actors::ws::{self, WebsocketContext};
use InterfaceAdapters::Model::model_farma::Model_farma;
use InterfaceAdapters::DTO::pedidos::cliente_pe::Pedido;
use InterfaceAdapters::DTO::pedidos::farmacia_pe::Validar_far;
use InterfaceAdapters::DTO::pedidos::procesar_pedi::Procesar_pedi;
use InterfaceAdapters::DTO::pedidos::raideri_pe::Validar_rai;

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                // Manejar mensajes de texto recibidos

                self.procesar_m(text, ctx);
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
    pub fn procesar_m<T: AsRef<str>>(&self, text: T, ctx: &mut WebsocketContext<Session>) {
        let msg = serde_json::from_str::<Procesar_pedi>(text.as_ref());

        match msg {
            Ok(Procesar_pedi::Cliente_pe(pe)) => {
            }
            Ok(Procesar_pedi::Raideri_pe(rai)) => (),
            Ok(Procesar_pedi::Faemacia_pe(far)) => (),

            Err(_) => (),
        }
    }
}
