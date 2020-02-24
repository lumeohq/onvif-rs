use yaserde::{YaDeserialize, YaSerialize};

use crate::soap;
use async_trait::async_trait;
use reqwest;

#[derive(Debug)]
pub enum Error {
    Serialization(String),
    Deserialization(String),
    Http(reqwest::Error),
    Soap(soap::Error),
    Onvif(String),
}

#[async_trait]
pub trait Transport {
    async fn request(&self, message: &str) -> Result<String, Error>;
}

pub async fn request<T: Transport, R: YaSerialize, S: YaDeserialize>(
    transport: &T,
    request: &R,
) -> Result<S, Error> {
    let ser = |obj: &R| yaserde::ser::to_string(obj).map_err(Error::Serialization);

    let de = |s: &str| yaserde::de::from_str(s).map_err(Error::Deserialization);

    de(&transport
        .request(&crop_xml_declaration(&ser(&request)?))
        .await?)
}

fn crop_xml_declaration(xml: &str) -> String {
    xml.split("?>").skip(1).collect()
}

#[test]
fn test_crop_xml_declaration() {
    assert_eq!(
        crop_xml_declaration(r#"<?xml version="1.0" encoding="utf-8"?><element />"#),
        "<element />"
    );
}
