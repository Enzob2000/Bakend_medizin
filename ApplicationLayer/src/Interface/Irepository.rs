pub trait Irepository {
    type Tinput;
    type Touput;
    type Error;

    async fn search(&mut self, list: Vec<Self::Tinput>) -> Result<Vec<Self::Touput>, Self::Error>;
}
