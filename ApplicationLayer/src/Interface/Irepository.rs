pub trait Irepository{
    
    type Tinput;
    type Touput;
    type Error;

    async  fn new() ->Self;
    async fn search(&self, list: Vec<Self::Tinput>) -> Result<Vec<Self::Touput>,Self::Error>;
}
