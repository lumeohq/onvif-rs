use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

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
    pub name0: String,
    #[yaserde(prefix = "tt", rename = "UseCount")]
    pub use_count: i32,
    #[yaserde(prefix = "tt", rename = "SourceToken")]
    pub source_token: String,
    #[yaserde(prefix = "tt", rename = "Bounds")]
    pub bounds: IntRectangle,
}
