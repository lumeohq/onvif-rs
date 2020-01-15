// Based on accesscontrol.wsdl.rs

// targetNamespace="http://www.onvif.org/ver10/accesscontrol/wsdl"

// xmlns:xs="http://www.w3.org/2001/XMLSchema"
// xmlns:pt="http://www.onvif.org/ver10/pacs"
// xmlns:tac="http://www.onvif.org/ver10/accesscontrol/wsdl"

// <xs:import namespace="http://www.onvif.org/ver10/pacs" schemaLocation="types.xsd"/>

use crate::schema::types as tt;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};
