// Based on onvif.xsd

// targetNamespace="http://www.onvif.org/ver10/schema"

// xmlns:tt="http://www.onvif.org/ver10/schema"
// xmlns:xs="http://www.w3.org/2001/XMLSchema"
// xmlns:xmime="http://www.w3.org/2005/05/xmlmime"
// xmlns:wsnt="http://docs.oasis-open.org/wsn/b-2"
// xmlns:xop="http://www.w3.org/2004/08/xop/include"
// xmlns:soapenv="http://www.w3.org/2003/05/soap-envelope"

// <xs:include schemaLocation="common.xsd"/>
// <xs:import namespace="http://www.w3.org/2005/05/xmlmime" schemaLocation="http://www.w3.org/2005/05/xmlmime"/>
// <xs:import namespace="http://www.w3.org/2003/05/soap-envelope" schemaLocation="http://www.w3.org/2003/05/soap-envelope"/>
// <xs:import namespace="http://docs.oasis-open.org/wsn/b-2" schemaLocation="http://docs.oasis-open.org/wsn/b-2.xsd"/>
// <xs:import namespace="http://www.w3.org/2004/08/xop/include" schemaLocation="http://www.w3.org/2004/08/xop/include"/>

use crate::schema::common::*;
use crate::utils;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

//<xs:simpleType name="Name">
//    <xs:annotation>
//        <xs:documentation>User readable name. Length up to 64 characters.</xs:documentation>
//    </xs:annotation>
//    <xs:restriction base="xs:string">
//        <xs:maxLength value="64"/>
//    </xs:restriction>
//</xs:simpleType>

#[derive(Default, PartialEq, Debug)]
pub struct Name(pub String);

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct TimeZone {
    #[yaserde(prefix = "tt", rename = "TZ")]
    pub tz: String,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Time {
    // TODO: use more specific types that allow range validation.
    #[yaserde(prefix = "tt", rename = "Hour")]
    pub hour: i32,
    #[yaserde(prefix = "tt", rename = "Minute")]
    pub minute: i32,
    #[yaserde(prefix = "tt", rename = "Second")]
    pub second: i32,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Date {
    // TODO: use more specific types that allow range validation.
    #[yaserde(prefix = "tt", rename = "Year")]
    pub year: i32,
    #[yaserde(prefix = "tt", rename = "Month")]
    pub month: i32,
    #[yaserde(prefix = "tt", rename = "Day")]
    pub day: i32,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct DateTime {
    #[yaserde(rename = "Time")]
    pub time: Time,
    #[yaserde(rename = "Date")]
    pub date: Date,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl",
    namespace = "tt: http://www.onvif.org/ver10/schema"
)]
pub struct SystemDateAndTime {
    #[yaserde(prefix = "tt", rename = "DateTimeType")]
    pub date_time_type: String,
    #[yaserde(prefix = "tt", rename = "DaylightSavings")]
    pub daylight_savings: bool,
    #[yaserde(rename = "TimeZone")]
    pub time_zone: TimeZone,
    #[yaserde(rename = "UTCDateTime")]
    pub utc_date_time: DateTime,
}

//<xs:complexType name="IntRectangle">
//    <xs:attribute name="x" type="xs:int" use="required"/>
//    <xs:attribute name="y" type="xs:int" use="required"/>
//    <xs:attribute name="width" type="xs:int" use="required"/>
//    <xs:attribute name="height" type="xs:int" use="required"/>
//</xs:complexType>

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct IntRectangle {
    #[yaserde(attribute)]
    pub x: i32,
    #[yaserde(attribute)]
    pub y: i32,
    #[yaserde(attribute)]
    pub width: i32,
    #[yaserde(attribute)]
    pub height: i32,
}

// Type VideoSourceConfiguration extends ConfigurationEntity

//<xs:complexType name="ConfigurationEntity">
//    <xs:sequence>
//        <xs:element name="Name" type="tt:Name"></xs:element>
//        <xs:element name="UseCount" type="xs:int"></xs:element>
//    </xs:sequence>
//    <xs:attribute name="token" type="tt:ReferenceToken" use="required"></xs:attribute>
//</xs:complexType>

//<xs:complexType name="VideoSourceConfiguration">
//    <xs:complexContent>
//        <xs:extension base="tt:ConfigurationEntity">
//            <xs:sequence>
//                <xs:element name="SourceToken" type="tt:ReferenceToken"></xs:element>
//                <xs:element name="Bounds" type="tt:IntRectangle"></xs:element>
//            </xs:sequence>
//        </xs:extension>
//    </xs:complexContent>
//</xs:complexType>

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoSourceConfiguration {
    #[yaserde(attribute)]
    pub token: String,
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: Name,
    #[yaserde(prefix = "tt", rename = "UseCount")]
    pub use_count: i32,
    #[yaserde(prefix = "tt", rename = "SourceToken")]
    pub source_token: String,
    #[yaserde(prefix = "tt", rename = "Bounds")]
    pub bounds: IntRectangle,
}

// Range of values greater equal Min value and less equal Max value.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct FloatRange {
    #[yaserde(prefix = "tt", rename = "Min")]
    pub min: f64,

    #[yaserde(prefix = "tt", rename = "Max")]
    pub max: f64,
}

// "ColorspaceRange" type is defined in onvif.xsd
// <xs:complexType name="ColorspaceRange">
//     <xs:sequence>
//         <xs:element name="X" type="tt:FloatRange"/>
//         <xs:element name="Y" type="tt:FloatRange"/>
//         <xs:element name="Z" type="tt:FloatRange"/>
//         <xs:element name="Colorspace" type="xs:anyURI" />
//     </xs:sequence>
// </xs:complexType>

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ColorspaceRange {
    #[yaserde(prefix = "tt", rename = "X")]
    pub x: FloatRange,
    #[yaserde(prefix = "tt", rename = "Y")]
    pub y: FloatRange,
    #[yaserde(prefix = "tt", rename = "Z")]
    pub z: FloatRange,
    #[yaserde(prefix = "tt", rename = "Colorspace")]
    pub colorspace: String,
}

// <xs:complexType name="ColorOptions">
//     <xs:choice>
//         <xs:element name="ColorList" type="tt:Color" maxOccurs="unbounded" />
//         <xs:element name="ColorspaceRange" type="tt:ColorspaceRange" maxOccurs="unbounded" />
//     </xs:choice>
//     <xs:anyAttribute processContents="lax"/>
// </xs:complexType>

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum ColorOptionsChoice {
    ColorList(Vec<Color>),
    ColorspaceRange(Vec<ColorspaceRange>),
}

impl Default for ColorOptionsChoice {
    fn default() -> ColorOptionsChoice {
        ColorOptionsChoice::ColorList(vec![])
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ColorOptions {
    #[yaserde(prefix = "tt", flatten)]
    pub choice: ColorOptionsChoice,
    #[yaserde(prefix = "tt", attribute)]
    pub any_attribute: Option<String>,
}

// A type that uses xs:duration (annotations removed)
//
//<xs:complexType name="MediaUri">
//    <xs:sequence>
//        <xs:element name="Uri" type="xs:anyURI" />
//        <xs:element name="InvalidAfterConnect" type="xs:boolean" />
//        <xs:element name="InvalidAfterReboot" type="xs:boolean" />
//        <xs:element name="Timeout" type="xs:duration" />
//        <xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>
//    </xs:sequence>
//    <xs:anyAttribute processContents="lax"/>
//</xs:complexType>

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MediaUri {
    #[yaserde(prefix = "tt", rename = "Uri")]
    pub uri: String,
    #[yaserde(prefix = "tt", rename = "InvalidAfterConnect")]
    pub invalid_after_connect: bool,
    #[yaserde(prefix = "tt", rename = "InvalidAfterReboot")]
    pub invalid_after_reboot: bool,
    #[yaserde(prefix = "tt", rename = "Timeout")]
    pub timeout: crate::schema::duration::Duration,
}

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

#[derive(Default, PartialEq, Debug)]
pub struct StringAttrList(pub Vec<String>);

impl YaDeserialize for StringAttrList {
    fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        utils::yaserde::deserialize(reader, |s| {
            Ok(StringAttrList(
                s.split_whitespace().map(|s| s.to_string()).collect(),
            ))
        })
    }
}

impl YaSerialize for StringAttrList {
    fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>) -> Result<(), String> {
        utils::yaserde::serialize(self, "StringAttrList", writer, |s| Ok(s.0.join(" ")))
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct FocusOptions20Extension {
    // TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // Supported options for auto focus. Options shall be chosen from tt:AFModes.
    #[yaserde(prefix = "tt", rename = "AFModes")]
    pub af_modes: Option<StringAttrList>,
}
