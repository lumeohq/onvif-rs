// Based on types.xsd

// targetNamespace="http://www.onvif.org/ver10/pacs"

// xmlns:xs="http://www.w3.org/2001/XMLSchema"
// xmlns:pt="http://www.onvif.org/ver10/pacs"

use crate::utils;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

//<xs:simpleType name="Name">
//    <xs:restriction base="xs:string">
//        <xs:maxLength value="64"/>
//        <xs:minLength value="0"/>
//    </xs:restriction>
//</xs:simpleType>

#[derive(Default, PartialEq, Debug)]
pub struct Name(pub String);

impl YaDeserialize for Name {
    fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        utils::yaserde::deserialize(reader, |s| Ok(Name(s.to_owned())))
    }
}

impl YaSerialize for Name {
    fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>) -> Result<(), String> {
        utils::yaserde::serialize(self, "Name", writer, |s| Ok(s.0.to_string()))
    }
}
