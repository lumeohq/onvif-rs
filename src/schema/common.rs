// Based on common.xsd

// targetNamespace="http://www.onvif.org/ver10/schema"

// xmlns:xs="http://www.w3.org/2001/XMLSchema"
// xmlns:tt="http://www.onvif.org/ver10/schema"

use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

// <xs:complexType name="Color">
//     <xs:sequence>
//         <xs:element name="X" type="tt:FloatRange"/>
//         <xs:element name="Y" type="tt:FloatRange"/>
//         <xs:element name="Z" type="tt:FloatRange"/>
//         <xs:element name="Colorspace" type="xs:anyURI" />
//     </xs:sequence>
// </xs:complexType>

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Color {
    #[yaserde(prefix = "tt", rename = "X")]
    pub x: f32,
    #[yaserde(prefix = "tt", rename = "Y")]
    pub y: f32,
    #[yaserde(prefix = "tt", rename = "Z")]
    pub z: f32,
    #[yaserde(prefix = "tt", rename = "Colorspace")]
    pub colorspace: String,
}
