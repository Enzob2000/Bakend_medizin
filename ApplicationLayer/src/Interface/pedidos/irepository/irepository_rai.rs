use async_trait::async_trait;

#[async_trait]
pub trait Irepository_rai<T>{


async fn search(&mut self,list_rai:T)->Result<Vec<String>,String>;



}