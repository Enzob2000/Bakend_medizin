use super::server::Server;
use actix::prelude::*;
use InterfaceAdapters::DTO::websokect::Roles;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: String,
    pub rol: Roles,
}

impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        match msg.rol {
            Roles::Cliente => {
                let mut cliente = self.clientes.lock().unwrap();

                cliente.remove(&msg.id);
            }
            Roles::Raideri => {
                let mut raider = self.raideris.lock().unwrap();

                raider.remove(&msg.id);
            }
            Roles::Farmacia => {
                let mut farmacia = self.farmacias.lock().unwrap();

                farmacia.remove(&msg.id);
            }
        }
    }
}
