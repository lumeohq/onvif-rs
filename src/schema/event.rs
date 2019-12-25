// Based events.wsdl.xml

// targetNamespace="http://www.onvif.org/ver10/events/wsdl"

// xmlns:wsa="http://www.w3.org/2005/08/addressing"
// xmlns:wstop="http://docs.oasis-open.org/wsn/t-1"
// xmlns:wsnt="http://docs.oasis-open.org/wsn/b-2"

// <xs:import namespace="http://www.w3.org/2005/08/addressing" schemaLocation="http://www.w3.org/2005/08/addressing/ws-addr.xsd"/>
// <xs:import namespace="http://docs.oasis-open.org/wsn/t-1" schemaLocation="http://docs.oasis-open.org/wsn/t-1.xsd"/>
// <xs:import namespace="http://docs.oasis-open.org/wsn/b-2" schemaLocation="http://docs.oasis-open.org/wsn/b-2.xsd"/>

use crate::schema::onvif as tt;
// TODO: wsa (http://www.w3.org/2005/08/addressing) - ?
// TODO: wstop (http://docs.oasis-open.org/wsn/t-1) - ?
// TODO: wsnt (http://docs.oasis-open.org/wsn/b-2.xsd) - ?
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};
