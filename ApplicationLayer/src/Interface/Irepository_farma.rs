pub trait Irepository_farma<Tinput,Touput,Error,Geo> {

    async fn search(&mut self, list: Vec<Tinput>,geo:Geo) -> Result<Vec<Touput>, Error>;
}
