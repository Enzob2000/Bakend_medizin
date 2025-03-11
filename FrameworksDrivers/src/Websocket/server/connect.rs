use actix::prelude::*;
use super::message::message;
use super::server::Server;
use InterfaceAdapters::DTO::websokect::Roles;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<message>,
    pub rol:Roles,
    pub id:String
}



impl Handler<Connect> for Server {
    type Result = ();

    fn handle(&mut self, conect: Connect, _: &mut Context<Self>) -> Self::Result {
        
      match conect.rol {

          Roles::Cliente=>{

            {
           let mut cliente= self.clientes.lock().unwrap();

           cliente.insert(conect.id, conect.addr);
          
          }


          },
          Roles::Raideri=>{

            {
           let mut raider= self.raideris.lock().unwrap();

           raider.insert(conect.id, conect.addr);
          
          }


          },
          Roles::Farmacia=>{

            {
           let mut farmacia= self.farmacias.lock().unwrap();

           farmacia.insert(conect.id, conect.addr);
          
          }


          },

      }

      
    }
}