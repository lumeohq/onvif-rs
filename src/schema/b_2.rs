// Based on b-2.xsd

// targetNamespace="http://docs.oasis-open.org/wsn/b-2"

// xmlns:wsnt="http://docs.oasis-open.org/wsn/b-2"
// xmlns:wsa="http://www.w3.org/2005/08/addressing"
// xmlns:wsrf-bf="http://docs.oasis-open.org/wsrf/bf-2"
// xmlns:wstop="http://docs.oasis-open.org/wsn/t-1"

//  <xsd:import namespace="http://www.w3.org/2005/08/addressing"
//              schemaLocation="http://www.w3.org/2005/08/addressing/ws-addr.xsd"
//  />
//  <xsd:import namespace="http://docs.oasis-open.org/wsrf/bf-2"
//              schemaLocation="http://docs.oasis-open.org/wsrf/bf-2.xsd"
//  />
//  <xsd:import namespace="http://docs.oasis-open.org/wsn/t-1"
//              schemaLocation="http://docs.oasis-open.org/wsn/t-1.xsd"
//  />

use crate::schema::bf_2 as wsrf_bf;
use crate::schema::t_1 as wstop;
use crate::schema::ws_addr as wsa;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};
