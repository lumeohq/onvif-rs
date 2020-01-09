// Based on bf-2.xsd

// targetNamespace="http://docs.oasis-open.org/wsrf/bf-2">

//  xmlns:xsd="http://www.w3.org/2001/XMLSchema"
//  xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
//  xmlns:wsa="http://www.w3.org/2005/08/addressing"
//  xmlns:wsrf-bf="http://docs.oasis-open.org/wsrf/bf-2"

//  <xsd:import
//     namespace="http://www.w3.org/2005/08/addressing"
//     schemaLocation=
//              "http://www.w3.org/2005/08/addressing/ws-addr.xsd"/>
//
//  <xsd:import namespace="http://www.w3.org/XML/1998/namespace"
//              schemaLocation="http://www.w3.org/2001/xml.xsd">

use crate::schema::ws_addr as wsa;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};