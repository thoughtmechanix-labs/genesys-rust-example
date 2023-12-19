use std::env;
use crate::auth::authclient;
use crate::routing::queue;
use std::error::Error;

mod auth;
mod routing;


#[tokio::main]
async fn main()  -> Result<(), Box<dyn Error>>{
    let genesyscloud_client_id = env::var("GENESYSCLOUD_CLIENT_ID").unwrap();
    let genesyscloud_client_secret = env::var("GENESYSCLOUD_CLIENT_SECRET").unwrap();
    let genesyscloud_authserver_url = env::var("GENESYSCLOUD_AUTHSERVER_URL").unwrap();
    let genesyscloud_apiserver_url= env::var("GENESYSCLOUD_APISERVER_URL").unwrap();


    let auth_client = authclient::AuthClient::new(
        genesyscloud_client_id.as_str(), 
    genesyscloud_client_secret.as_str(), 
            genesyscloud_authserver_url.as_str(), 
    genesyscloud_apiserver_url.as_str()).await?;


    let queue= queue::retrieve_by_id(auth_client,"5311c59a-4c4b-4f47-a8d0-1344c9368800".to_string()).await?;

    println!("Queue Name: {}. {}", queue.name, queue.description);
    Ok(())
}
