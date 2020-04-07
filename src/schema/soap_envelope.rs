pub use crate::schema::common::*;
use crate::schema::validate::Validate;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2003/05/soap-envelope"
)]
pub struct Envelope {
    #[yaserde(prefix = "tns", rename = "Header")]
    pub header: Option<Header>,

    #[yaserde(prefix = "tns", rename = "Body")]
    pub body: Body,
}

impl Validate for Envelope {}

// pub type Header = Header;
// Elements replacing the wildcard MUST be namespace qualified, but can be in
// the targetNamespace
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2003/05/soap-envelope"
)]
pub struct Header {}

impl Validate for Header {}

// pub type Body = Body;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2003/05/soap-envelope"
)]
pub struct Body {}

impl Validate for Body {}

// pub type MustUnderstand = bool;
// pub type Relay = bool;
// pub type Role = String;
// pub type EncodingStyle = String;
// pub type Fault = Fault;
// Fault reporting structure
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2003/05/soap-envelope"
)]
pub struct Fault {
    #[yaserde(prefix = "tns", rename = "Code")]
    pub code: Faultcode,

    #[yaserde(prefix = "tns", rename = "Reason")]
    pub reason: Faultreason,

    #[yaserde(prefix = "tns", rename = "Node")]
    pub node: Option<String>,

    #[yaserde(prefix = "tns", rename = "Role")]
    pub role: Option<String>,

    #[yaserde(prefix = "tns", rename = "Detail")]
    pub detail: Option<Detail>,
}

impl Validate for Fault {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2003/05/soap-envelope"
)]
pub struct Faultreason {
    #[yaserde(prefix = "tns", rename = "Text")]
    pub text: Vec<String>,
}

impl Validate for Faultreason {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2003/05/soap-envelope"
)]
pub struct Faultcode {
    #[yaserde(prefix = "tns", rename = "Value")]
    pub value: FaultcodeEnum,

    #[yaserde(prefix = "tns", rename = "Subcode")]
    pub subcode: Vec<Subcode>,
}

impl Validate for Faultcode {}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum FaultcodeEnum {
    #[yaserde(rename = "tns:DataEncodingUnknown")]
    DataEncodingUnknown,
    #[yaserde(rename = "tns:MustUnderstand")]
    MustUnderstand,
    #[yaserde(rename = "tns:Receiver")]
    Receiver,
    #[yaserde(rename = "tns:Sender")]
    Sender,
    #[yaserde(rename = "tns:VersionMismatch")]
    VersionMismatch,
    __Unknown__(String),
}

impl Default for FaultcodeEnum {
    fn default() -> FaultcodeEnum {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for FaultcodeEnum {}
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2003/05/soap-envelope"
)]
pub struct Subcode {
    #[yaserde(prefix = "tns", rename = "Value")]
    pub value: String,
    // TODO: handle recursion
    // #[yaserde(prefix = "tns", rename = "Subcode")]
    // pub subcode: Vec<Subcode>,
}

impl Validate for Subcode {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2003/05/soap-envelope"
)]
pub struct Detail {}

impl Validate for Detail {}

// pub type NotUnderstood = NotUnderstoodType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2003/05/soap-envelope"
)]
pub struct NotUnderstoodType {
    #[yaserde(attribute, rename = "qname")]
    pub qname: String,
}

impl Validate for NotUnderstoodType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2003/05/soap-envelope"
)]
pub struct SupportedEnvType {
    #[yaserde(attribute, rename = "qname")]
    pub qname: String,
}

impl Validate for SupportedEnvType {}

// pub type Upgrade = UpgradeType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2003/05/soap-envelope"
)]
pub struct UpgradeType {
    #[yaserde(prefix = "tns", rename = "SupportedEnvelope")]
    pub supported_envelope: Vec<SupportedEnvType>,
}

impl Validate for UpgradeType {}
