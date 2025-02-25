

use mongodb::{ 
    bson::{doc, Document},
    Client,
    Collection, Database 
};


pub struct  Repositori_inv{
    database:Database
}


impl Repositori_inv {

    
    pub async fn new() ->Self{
        // Replace the placeholder with your Atlas connection string
        let uri = "mongodb://localhost:27017";
    
        // Create a new client and connect to the server
        let client = Client::with_uri_str(uri).await.unwrap();
    
        // Get a handle on the movies collection
        let database = client.database("pueba");
       
       Self{
        database
       }
    
        
        
    }

    pub async fn search(&mut self){

    let list=self.database.list_collection_names().await.unwrap();

    list.into_iter().for_each(|x|println!("{x}"));

    }
    
    


    
}