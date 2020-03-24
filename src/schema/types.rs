// Based on types.xsd

// targetNamespace="http://www.onvif.org/ver10/pacs"

// xmlns:xs="http://www.w3.org/2001/XMLSchema"
// xmlns:pt="http://www.onvif.org/ver10/pacs"

pub use crate::schema::common::*;
use crate::schema::validate::Validate;
use crate::utils;
use macro_utils::*;
use std::io::{Read, Write};
use std::str::FromStr;
use yaserde::{YaDeserialize, YaSerialize};

// Type used to reference logical and physical entities.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct ReferenceToken(pub String);

impl Validate for ReferenceToken {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > "64".parse().unwrap() {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 64 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        if self.0.len() < "0".parse().unwrap() {
            return Err(format!(
                "MinLength validation error. \nExpected: 0 length >= 0 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

// General datastructure referenced by a token.
// Should be used as extension base.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "pt", namespace = "pt: http://www.onvif.org/ver10/pacs")]
pub struct DataEntity {
    // A service-unique identifier of the item.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

impl Validate for DataEntity {}

// Type used for names of logical and physical entities.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Name(pub String);

impl Validate for Name {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > "64".parse().unwrap() {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 64 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        if self.0.len() < "0".parse().unwrap() {
            return Err(format!(
                "MinLength validation error. \nExpected: 0 length >= 0 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

// Description is optional and the maximum length is device specific.
// If the length is more than maximum length, it is silently chopped to the
// maximum length
// supported by the device/service (which may be 0).
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Description(pub String);

impl Validate for Description {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > "1024".parse().unwrap() {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 1024 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        if self.0.len() < "0".parse().unwrap() {
            return Err(format!(
                "MinLength validation error. \nExpected: 0 length >= 0 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

// Type used to represent the numbers from 1 ,2 , 3,...
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct PositiveInteger(pub u32);

impl Validate for PositiveInteger {
    fn validate(&self) -> Result<(), String> {
        if self.0 < 1 {
            return Err(format!("MinInclusive validation error: invalid value of 0! \nExpected: 0 >= 1.\nActual: 0 == {}", self.0));
        }
        Ok(())
    }
}

// Attributes contains a Name and an optional Value and type.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "pt", namespace = "pt: http://www.onvif.org/ver10/pacs")]
pub struct Attribute {
    // Name of attribute. Key names starting with "ONVIF" (any case) are
    // reserved for ONVIF
    // use.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,

    // Value of attribute
    #[yaserde(attribute, rename = "Value")]
    pub value: Option<String>,
}

impl Validate for Attribute {}
