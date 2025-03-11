use actix::prelude::*;
use super::super::Server::server::Server;
use InterfaceAdapters::DTO::websokect::Roles;

#[derive(Clone)]
pub struct Session {
    /// unique session id
    pub id: String,

    /// peer name
    pub name: String,

    pub rol:Roles,

    /// Chat server
    pub addr: Addr<Server>,
}

impl Session {

    pub fn new(id:String,name:String,addr:Addr<Server> ,rol:Roles  )->Self{

        Self{

            id,
            name,
            rol,
            addr
        }
    }
    
}





