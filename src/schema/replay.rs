// Based on replay.wsdl.xml

// targetNamespace="http://www.onvif.org/ver10/replay/wsdl"

// xmlns:tt="http://www.onvif.org/ver10/schema"
// xmlns:xs="http://www.w3.org/2001/XMLSchema"

// <xs:import namespace="http://www.onvif.org/ver10/schema" schemaLocation="../ver10/schema/onvif.xsd"/>

use crate::schema::onvif as tt;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};
