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

    #[yaserde(attribute, rename = "any_attribute")]
    pub any_attribute: Option<String>,
}

// Elements replacing the wildcard MUST be namespace qualified, but can be in
// the targetNamespace
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2003/05/soap-envelope"
)]
pub struct Header {
    #[yaserde(prefix = "tns", rename = "any")]
    pub any: Option<String>,

    #[yaserde(attribute, rename = "any_attribute")]
    pub any_attribute: Option<String>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2003/05/soap-envelope"
)]
pub struct Body {
    #[yaserde(prefix = "tns", rename = "any")]
    pub any: Option<String>,

    #[yaserde(attribute, rename = "any_attribute")]
    pub any_attribute: Option<String>,
}

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

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2003/05/soap-envelope"
)]
pub struct Faultreason {
    #[yaserde(prefix = "tns", rename = "Text")]
    pub text: Vec<String>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2003/05/soap-envelope"
)]
pub struct Faultcode {
    #[yaserde(prefix = "tns", rename = "Value")]
    pub value: String,

    #[yaserde(prefix = "tns", rename = "Subcode")]
    pub subcode: Option<Subcode>,
}

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

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2003/05/soap-envelope"
)]
pub struct Detail {
    #[yaserde(prefix = "tns", rename = "any")]
    pub any: Option<String>,

    #[yaserde(attribute, rename = "any_attribute")]
    pub any_attribute: Option<String>,
}

pub type NotUnderstood = NotUnderstoodType;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2003/05/soap-envelope"
)]
pub struct NotUnderstoodType {
    #[yaserde(attribute, rename = "qname")]
    pub qname: String,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2003/05/soap-envelope"
)]
pub struct SupportedEnvType {
    #[yaserde(attribute, rename = "qname")]
    pub qname: String,
}

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
