use std::{future::Future, pin::Pin, process::Output};
use async_trait::async_trait;

#[async_trait]
pub trait IrepositoryFa<Tinput>:Send+Sync {
    
    
   
   async  fn search(&self,busqueda:Tinput)->Result<Vec<String>,String>;
    
}