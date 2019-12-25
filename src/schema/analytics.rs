// Based on analytics.wsdl.xml

// targetNamespace="http://www.onvif.org/ver20/analytics/wsdl"

// xmlns:tan="http://www.onvif.org/ver20/analytics/wsdl"
// xmlns:tt="http://www.onvif.org/ver10/schema"

// <xs:import namespace="http://www.onvif.org/ver10/schema" schemaLocation="../../../ver10/schema/onvif.xsd"/>

use crate::schema::onvif as tt;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};
