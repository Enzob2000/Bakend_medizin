use async_trait::async_trait;

#[async_trait]
pub trait IrepositoryRai<T>{


async fn search(&mut self,list_rai:T)->Result<Vec<String>,String>;



}