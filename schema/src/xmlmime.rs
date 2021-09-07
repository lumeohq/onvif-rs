use crate::validate::Validate;
use std::str::FromStr;
use xsd_macro_utils::*;

#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct ContentType(pub String);

impl Validate for ContentType {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() < "3".parse().unwrap() {
            return Err(format!(
                "MinLength validation error. \nExpected: 0 length >= 3 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

// pub type ExpectedContentTypes = String;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xmime",
    namespace = "xmime: http://www.w3.org/2005/05/xmlmime"
)]
pub struct Base64Binary {
    #[yaserde(attribute, prefix = "xmime" rename = "contentType")]
    pub content_type: Option<ContentType>,
}

impl Validate for Base64Binary {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xmime",
    namespace = "xmime: http://www.w3.org/2005/05/xmlmime"
)]
pub struct HexBinary {
    #[yaserde(attribute, prefix = "xmime" rename = "contentType")]
    pub content_type: Option<ContentType>,
}

impl Validate for HexBinary {}
