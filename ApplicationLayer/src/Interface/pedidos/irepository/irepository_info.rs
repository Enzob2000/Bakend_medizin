use async_trait::async_trait;
use serde::Serialize;

#[async_trait]
pub trait IrepositoryInfo <T:Serialize>{

    async fn read(&self,id:String)->Result<T,()>;

    
}