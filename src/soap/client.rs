extern crate reqwest;

use crate::schema::transport::{Error, Transport};
use crate::soap;

pub struct Client {
    uri: String,
    client: reqwest::Client,
}

impl Transport for Client {
    fn request(&self, message: &str) -> Result<String, Error> {
        soap::soap(message)
            .map_err(Error::Soap)
            .and_then(|envelope| {
                self.client
                    .post(self.uri.as_str())
                    .header("Content-Type", "application/soap+xml; charset=utf-8;")
                    .body(envelope)
                    .send()
                    .map_err(Error::Http)
                    .and_then(|mut response| {
                        response.text().map_err(Error::Http).and_then(|text| {
                            soap::unsoap(&text)
                                .map_err(Error::Soap)
                                // TODO: process SOAP fault messages
                                // probably need to return something more than a String
                                .map(|response| response.response.unwrap())
                        })
                    })
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
