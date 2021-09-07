pub use crate::common::*;
use crate::validate::Validate;
use std::str::FromStr;
use xsd_macro_utils::*;

// Type used to reference logical and physical entities.
// Token may be extended by intermediate terminal with adding prefix to make it
// global unique.
// The length should be within 36 characters for generating as a local token.
// See "Remote Token" section in Resource Query specification.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct ReferenceToken(pub String);

impl Validate for ReferenceToken {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > 64 {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 64 \nActual: 0 length == {}",
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
        if self.0.len() > 64 {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 64 \nActual: 0 length == {}",
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
        if self.0.len() > 1024 {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 1024 \nActual: 0 length == {}",
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
        if self.0 < "1".parse::<u32>().unwrap() {
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

// Recognition/identification types supported by ONVIF.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct RecognitionType(pub String);

impl Validate for RecognitionType {}
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct StringList(pub Vec<String>);

impl Validate for StringList {}
