// Based on metadatastream.xsd

// targetNamespace="http://www.onvif.org/ver10/schema"

// xmlns:xs="http://www.w3.org/2001/XMLSchema"
// xmlns:tt="http://www.onvif.org/ver10/schema"
// xmlns:wsnt="http://docs.oasis-open.org/wsn/b-2"

// 	<xs:include schemaLocation="common.xsd"/>
// 	<xs:import namespace="http://docs.oasis-open.org/wsn/b-2" schemaLocation="http://docs.oasis-open.org/wsn/b-2.xsd"/>

use crate::schema::common as tt;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};
