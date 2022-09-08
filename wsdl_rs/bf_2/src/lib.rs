#![allow(clippy::derive_partial_eq_without_eq)]

use validate::Validate;
use ws_addr as wsa;
use xml_xsd as xml;
use xsd_types::types as xsd;
use yaserde_derive::{YaDeserialize, YaSerialize};

// pub type BaseFault = BaseFaultType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsrf-bf",
    namespace = "wsrf-bf: http://docs.oasis-open.org/wsrf/bf-2"
)]
pub struct BaseFaultType {
    #[yaserde(prefix = "wsrf-bf", rename = "Timestamp")]
    pub timestamp: xsd::DateTime,

    #[yaserde(prefix = "wsrf-bf", rename = "Originator")]
    pub originator: wsa::EndpointReferenceType,

    #[yaserde(prefix = "wsrf-bf", rename = "ErrorCode")]
    pub error_code: base_fault_type::ErrorCodeType,

    #[yaserde(prefix = "wsrf-bf", rename = "Description")]
    pub description: Vec<base_fault_type::DescriptionType>,

    #[yaserde(prefix = "wsrf-bf", rename = "FaultCause")]
    pub fault_cause: base_fault_type::FaultCauseType,
}

impl Validate for BaseFaultType {}

pub mod base_fault_type {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "wsrf-bf",
        namespace = "wsrf-bf: http://docs.oasis-open.org/wsrf/bf-2"
    )]
    pub struct ErrorCodeType {
        #[yaserde(attribute, rename = "dialect")]
        pub dialect: String,
    }

    impl Validate for ErrorCodeType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "wsrf-bf",
        namespace = "wsrf-bf: http://docs.oasis-open.org/wsrf/bf-2"
    )]
    pub struct DescriptionType {
        #[yaserde(attribute, prefix = "xml" rename = "lang")]
        pub lang: Option<xml::Lang>,
    }

    impl Validate for DescriptionType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "wsrf-bf",
        namespace = "wsrf-bf: http://docs.oasis-open.org/wsrf/bf-2"
    )]
    pub struct FaultCauseType {}

    impl Validate for FaultCauseType {}
}
