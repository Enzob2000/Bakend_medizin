use std::{future::Future, pin::Pin, process::Output};



pub trait Irepository_pe<Tinput,Geo>:Send+Sync {
    
    
   
    fn search(&self,med:Vec<Tinput>,geo:Geo)->Pin<Box<dyn Future<Output=Result<Vec<String>,()>>+Send>>;
    
}