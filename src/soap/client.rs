extern crate reqwest;
use crate::schema::transport::{Error, Transport};
use crate::soap;
use async_trait::async_trait;

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
            .body(soap::soap(message, &self.username_token()).map_err(Error::Soap)?)
            .send()
            .await
            .map_err(Error::Http)?
            .text()
            .await
            .map_err(Error::Http)
            .and_then(|text| {
                soap::unsoap(&text)
                    .map_err(Error::Soap)
                    // TODO: process SOAP fault messages
                    // probably need to return something more than a String
                    .map(|response| response.response.unwrap())
            })
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
