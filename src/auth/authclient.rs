
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug)]
pub struct AuthClient {
    pub id: String,
    pub secret: String,
    pub auth_server_url: String,
    pub api_server_url: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
}

impl AuthClient{
    pub async fn new(
        id: &str, 
        secret: &str, 
        auth_server_url: &str,
        api_server_url: &str
     ) -> Result<Self,  Box<dyn Error>>{
        
        let mut auth = AuthClient {
            id: id.to_string(),
            secret: secret.to_string(),
            auth_server_url: auth_server_url.to_string(),
            api_server_url: api_server_url.to_string(),
            token: "".to_string()
        };

        let _ = auth.authenticate().await?;
        Ok(auth)
    }

  
    pub async fn authenticate(&mut self)->Result<(),  Box<dyn Error>> {
        let token_url = format!("{}{}",self.auth_server_url.to_string(),"/oauth/token");

        // Your client credentials
        let client_id = self.id.to_string();
        let client_secret = self.secret.to_string();

        // Request token using client credentials grant
        let client = Client::new();
        let params = [("grant_type", "client_credentials")];
        let response = client
            .post(token_url)
            .basic_auth(client_id, Some(client_secret))
            .form(&params)
            .send()
            .await?;

        // Parse and print the token
        let token_response: TokenResponse = response.json().await?;
        self.token=token_response.access_token.to_string();
    

        //Need to put some basic error handling here
        Ok(())

    }
 }