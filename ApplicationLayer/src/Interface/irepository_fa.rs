

pub trait Irepository_pe<Tinput,Geo>:Send+Sync {
    
    
   
    fn search(&self,med:Vec<Tinput>,geo:Geo)->Result<Vec<String>,String>;
    
}