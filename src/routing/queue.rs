use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::Deserialize;
use std::error::Error;
use crate::authclient::AuthClient;


#[derive(Debug, Deserialize)]
pub struct QueueResponse {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl QueueResponse{
    pub fn new(
        id: &str, 
        name: &str, 
        description: &str,
     ) -> Self {
        
        QueueResponse {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string()
        }
    }
}

pub async fn retrieve_by_id(auth_client: AuthClient, id: String) -> Result<QueueResponse,  Box<dyn Error> >{
   
    // Replace these values with your actual OAuth token and microservice URL
    let oauth_token =  auth_client.token.to_string();
    let api_url = format!("{}{}{}",auth_client.api_server_url.to_string(),"/api/v2/routing/queues/", id);

    // Build the authorization header with the OAuth token
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", oauth_token)).unwrap(),
    );


    // Create a reqwest client with authorization header
    let client = reqwest::Client::new();
    let client = client
        .get(api_url)
        .headers(headers);

    // Make the GET request
    let response = client.send().await?;
    let api_response: QueueResponse = response.json().await?;
    Ok(api_response)

}