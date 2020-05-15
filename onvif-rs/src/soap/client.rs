extern crate reqwest;
use crate::soap;
use async_trait::async_trait;
use schema::transport::{Error, Transport};

#[derive(Default, Debug, Clone)]
pub struct Client {
    uri: String,
    client: reqwest::Client,
    credentials: Option<Credentials>,
}

#[derive(Default, Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[async_trait]
impl Transport for Client {
    async fn request(&self, message: &str) -> Result<String, Error> {
        self.client
            .post(self.uri.as_str())
            .header("Content-Type", "application/soap+xml; charset=utf-8;")
            .body(
                soap::soap(message, &self.username_token())
                    .map_err(|e| Error::Transport(format!("{:?}", e)))?,
            )
            .send()
            .await
            .map_err(|e| Error::Transport(e.to_string()))?
            .text()
            .await
            .map_err(|e| Error::Transport(e.to_string()))
            .and_then(|text| soap::unsoap(&text).map_err(|e| Error::Transport(format!("{:?}", e))))
    }
}

impl Client {
    pub fn new(uri: &str, credentials: Option<Credentials>) -> Self {
        Client {
            uri: uri.into(),
            client: reqwest::Client::new(),
            credentials,
        }
    }

    pub fn username_token(&self) -> Option<soap::auth::UsernameToken> {
        self.credentials
            .as_ref()
            .map(|c| soap::auth::UsernameToken::new(&c.username, &c.password))
    }
}
