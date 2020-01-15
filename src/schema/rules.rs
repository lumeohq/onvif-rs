// Based on rules.xsd

// targetNamespace="http://www.onvif.org/ver20/analytics"

// xmlns:axt="http://www.onvif.org/ver20/analytics"
// xmlns:xs="http://www.w3.org/2001/XMLSchema"
// xmlns:tt="http://www.onvif.org/ver10/schema"

// <xs:import namespace="http://www.onvif.org/ver10/schema" schemaLocation="../../ver10/schema/onvif.xsd"/>

use crate::schema::onvif as tt;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};
