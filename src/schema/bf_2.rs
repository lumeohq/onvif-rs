// Based on bf-2.xsd

// targetNamespace="http://docs.oasis-open.org/wsrf/bf-2">

//  xmlns:xsd="http://www.w3.org/2001/XMLSchema"
//  xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
//  xmlns:wsa="http://www.w3.org/2005/08/addressing"
//  xmlns:wsrf-bf="http://docs.oasis-open.org/wsrf/bf-2"

//  <xsd:import
//     namespace="http://www.w3.org/2005/08/addressing"
//     schemaLocation=
//              "http://www.w3.org/2005/08/addressing/ws-addr.xsd"/>
//
//  <xsd:import namespace="http://www.w3.org/XML/1998/namespace"
//              schemaLocation="http://www.w3.org/2001/xml.xsd">

//use http://www.w3.org/2005/08/addressing/ws-addr.xsd  http://www.w3.org/2005/08/addressing;
//use http://www.w3.org/2001/xml.xsd  http://www.w3.org/XML/1998/namespace;

pub use crate::schema::common::*;
use crate::schema::validate::Validate;
use crate::schema::ws_addr as wsa;
use crate::schema::xml;
use crate::transport;
use crate::utils;
use macro_utils::*;
use std::io::{Read, Write};
use std::str::FromStr;
use xsd_types::types as xsd;
use yaserde::{YaDeserialize, YaSerialize};

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
