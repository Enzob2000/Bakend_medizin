
pub trait Imapper<Tinput,Touput> {

    fn mapper(&self,entrada:Tinput)->Touput;
    
}