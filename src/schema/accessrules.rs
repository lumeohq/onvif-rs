// Based on accessrules.wsdl.xml

// targetNamespace="http://www.onvif.org/ver10/accessrules/wsdl">

// xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:pt="http://www.onvif.org/ver10/pacs"
// xmlns:tar="http://www.onvif.org/ver10/accessrules/wsdl" elementFormDefault="qualified" version="18.12">

// <xs:import namespace="http://www.onvif.org/ver10/pacs" schemaLocation="../../pacs/types.xsd"/>

use crate::schema::types as tt;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};
