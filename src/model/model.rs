use reqwest::Client;
use serde::Deserialize;
use anyhow::{Result, Error};
use serde_json::Value;

pub struct Model {
    url: String,
    req: Client,
}

impl Model {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            req: Client::new()
        }
    }

    pub async fn generate(&self, prompt: &str) -> Result<String> {
        let mut arg = std::collections::HashMap::new();
        arg.insert("prompt", prompt);
        let response = self.req.post(&self.url).json(&arg).send().await?.json::<Value>().await?;

        let response = response.get("response");
        match response {
            Some(r) => {
                Ok(r.to_string())       
            }
            None => {
                Err(Error::msg("response parse error"))
            }
        }
        //Ok(rm.response)
        
    }
}
