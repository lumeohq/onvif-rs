pub use crate::common::*;
use crate::validate::Validate;
use std::str::FromStr;
use xsd_macro_utils::*;

pub type EndpointReference = EndpointReferenceType;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2005/08/addressing"
)]
pub struct EndpointReferenceType {
    #[yaserde(prefix = "tns", rename = "Address")]
    pub address: AttributedURIType,

    #[yaserde(prefix = "tns", rename = "ReferenceParameters")]
    pub reference_parameters: Option<ReferenceParameters>,

    #[yaserde(prefix = "tns", rename = "Metadata")]
    pub metadata: Option<Metadata>,
}

impl Validate for EndpointReferenceType {}

pub type ReferenceParameters = ReferenceParametersType;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2005/08/addressing"
)]
pub struct ReferenceParametersType {}

impl Validate for ReferenceParametersType {}

pub type Metadata = MetadataType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2005/08/addressing"
)]
pub struct MetadataType {}

impl Validate for MetadataType {}

pub type MessageID = AttributedURIType;
pub type RelatesTo = RelatesToType;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2005/08/addressing"
)]
pub struct RelatesToType {
    #[yaserde(attribute, rename = "RelationshipType")]
    pub relationship_type: Option<RelationshipTypeOpenEnum>,
}

impl Validate for RelatesToType {}

#[derive(PartialEq, Debug, UtilsUnionSerDe)]
pub enum RelationshipTypeOpenEnum {
    RelationshipType(RelationshipType),
    AnyURI(String),
    __Unknown__(String),
}

impl Default for RelationshipTypeOpenEnum {
    fn default() -> RelationshipTypeOpenEnum {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for RelationshipTypeOpenEnum {}

#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct RelationshipType(pub String);

impl Validate for RelationshipType {}
pub type ReplyTo = EndpointReferenceType;
pub type From = EndpointReferenceType;
pub type FaultTo = EndpointReferenceType;
pub type To = AttributedURIType;
pub type Action = AttributedURIType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2005/08/addressing"
)]
pub struct AttributedURIType {}

impl Validate for AttributedURIType {}

pub type IsReferenceParameter = bool;
#[derive(PartialEq, Debug, UtilsUnionSerDe)]
pub enum FaultCodesOpenEnumType {
    FaultCodesType(FaultCodesType),
    Qname(String),
    __Unknown__(String),
}

impl Default for FaultCodesOpenEnumType {
    fn default() -> FaultCodesOpenEnumType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for FaultCodesOpenEnumType {}

#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct FaultCodesType(pub String);

impl Validate for FaultCodesType {}
pub type RetryAfter = AttributedUnsignedLongType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2005/08/addressing"
)]
pub struct AttributedUnsignedLongType {}

impl Validate for AttributedUnsignedLongType {}

pub type ProblemHeaderQName = AttributedQNameType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2005/08/addressing"
)]
pub struct AttributedQNameType {}

impl Validate for AttributedQNameType {}

pub type ProblemIRI = AttributedURIType;
pub type ProblemAction = ProblemActionType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2005/08/addressing"
)]
pub struct ProblemActionType {
    #[yaserde(prefix = "tns", rename = "Action")]
    pub action: Option<Action>,

    #[yaserde(prefix = "tns", rename = "SoapAction")]
    pub soap_action: Option<String>,
}

impl Validate for ProblemActionType {}
