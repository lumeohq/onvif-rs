// Based on doorcontrol.wsdl.xml

// targetNamespace="http://www.onvif.org/ver10/doorcontrol/wsdl"

// xmlns:xs="http://www.w3.org/2001/XMLSchema"
// xmlns:pt="http://www.onvif.org/ver10/pacs"
// xmlns:tdc="http://www.onvif.org/ver10/doorcontrol/wsdl"

// <xs:import namespace="http://www.onvif.org/ver10/pacs" schemaLocation="types.xsd"/>

use crate::schema::types as pt;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};
