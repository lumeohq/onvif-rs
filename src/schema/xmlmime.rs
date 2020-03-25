use crate::utils;
use macro_utils::*;
use std::io::{Read, Write};
use std::str::FromStr;
use yaserde::{YaDeserialize, YaSerialize};

#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct ContentType(pub String);

//generated file
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xmime",
    namespace = "xmime: http://www.w3.org/2005/05/xmlmime"
)]
pub struct Base64Binary {
    #[yaserde(attribute, prefix = "xmime" rename = "contentType")]
    pub xmime_content_type: Option<ContentType>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xmime",
    namespace = "xmime: http://www.w3.org/2005/05/xmlmime"
)]
pub struct HexBinary {
    #[yaserde(attribute, prefix = "xmime" rename = "contentType")]
    pub xmime_content_type: Option<ContentType>,
}
