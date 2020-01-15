// Based on credential.wsdl.xml

// targetNamespace="http://www.onvif.org/ver10/credential/wsdl"

// xmlns:xs="http://www.w3.org/2001/XMLSchema"
// xmlns:pt="http://www.onvif.org/ver10/pacs"
// xmlns:tcr="http://www.onvif.org/ver10/credential/wsdl"

// <xs:import namespace="http://www.onvif.org/ver10/pacs" schemaLocation="../../pacs/types.xsd"/>

use crate::schema::types as pt;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};
