use std::{future::Future, pin::Pin, process::Output};
use async_trait::async_trait;

#[async_trait]
pub trait Irepository_pe<Tinput,Geo>:Send+Sync {
    
    
   
   async  fn search(&self,med:Vec<Tinput>,geo:Geo)->Result<Vec<String>,()>;
    
}