// Based on actionengine.wsdl.xml

// targetNamespace="http://www.onvif.org/ver10/actionengine/wsdl"

// xmlns:tt="http://www.onvif.org/ver10/schema"
// xmlns:wsnt="http://docs.oasis-open.org/wsn/b-2"

// <xs:import namespace="http://www.onvif.org/ver10/schema" schemaLocation="../ver10/schema/onvif.xsd"/>
// <xs:import namespace="http://docs.oasis-open.org/wsn/b-2" schemaLocation="http://docs.oasis-open.org/wsn/b-2.xsd"/>

use crate::schema::onvif as tt;
// TODO: wsnt (http://docs.oasis-open.org/wsn/b-2.xsd) - ?
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};
