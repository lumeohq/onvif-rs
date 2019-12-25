// Based on authenticationbehavior.wsdl.xml

// targetNamespace="http://www.onvif.org/ver10/authenticationbehavior/wsdl"

// xmlns:xs="http://www.w3.org/2001/XMLSchema"
// xmlns:pt="http://www.onvif.org/ver10/pacs"
// xmlns:tt="http://www.onvif.org/ver10/schema"
// xmlns:tab="http://www.onvif.org/ver10/authenticationbehavior/wsdl"

// <xs:import namespace="http://www.onvif.org/ver10/pacs" schemaLocation="../../pacs/types.xsd"/>
// <xs:import namespace="http://www.onvif.org/ver10/schema" schemaLocation="../../schema/onvif.xsd"/>

use crate::schema::types as pt;
use crate::schema::onvif as tt;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};
