use std::{future::Future, pin::Pin, process::Output};
use async_trait::async_trait;

#[async_trait]
pub trait Irepository_pe<Tinput>:Send+Sync {
    
    
   
   async  fn search(&self,busqueda:Tinput)->Result<Vec<String>,String>;
    
}