extern crate reqwest;
use crate::schema::transport::{Error, Transport};
use crate::soap;
use async_trait::async_trait;

pub struct Client {
    uri: String,
    client: reqwest::Client,
}

#[async_trait]
impl Transport for Client {
    async fn request(&self, message: &str) -> Result<String, Error> {
        self.client
            .post(self.uri.as_str())
            .header("Content-Type", "application/soap+xml; charset=utf-8;")
            .body(soap::soap(message).map_err(Error::Soap)?)
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
    pub fn new(uri: &str) -> Self {
        Client {
            uri: uri.into(),
            client: reqwest::Client::new(),
        }
    }
}
