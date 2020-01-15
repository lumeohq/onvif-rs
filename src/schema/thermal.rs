// Based on thermal.wsdl.xml

// targetNamespace="http://www.onvif.org/ver10/thermal/wsdl"

// xmlns:xs="http://www.w3.org/2001/XMLSchema"
// xmlns:tt="http://www.onvif.org/ver10/schema"
// xmlns:tth="http://www.onvif.org/ver10/thermal/wsdl"

// <xs:import namespace="http://www.onvif.org/ver10/schema" schemaLocation="../../schema/onvif.xsd"/>

use crate::schema::onvif as tt;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};
