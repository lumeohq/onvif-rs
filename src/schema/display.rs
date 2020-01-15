// Based on display.wsdl.xml

// targetNamespace="http://www.onvif.org/ver10/display/wsdl"

// xmlns:tt="http://www.onvif.org/ver10/schema"
// xmlns:xs="http://www.w3.org/2001/XMLSchema"

// <xs:import namespace="http://www.onvif.org/ver10/schema" schemaLocation="http://www.onvif.org/onvif/ver10/schema/onvif.xsd"/>

use crate::schema::onvif as tt;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};
