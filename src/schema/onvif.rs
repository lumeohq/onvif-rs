use std::io::{Read, Write};
use yaserde::{YaSerialize, YaDeserialize};


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
prefix = "tt",
namespace = "tt: http://www.onvif.org/ver10/schema"
)]
pub struct TimeZone {
    #[yaserde(prefix = "tt", rename = "TZ")]
    pub tz: String,
}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
prefix = "tt",
namespace = "tt: http://www.onvif.org/ver10/schema"
)]
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
#[yaserde(
prefix = "tt",
namespace = "tt: http://www.onvif.org/ver10/schema"
)]
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
#[yaserde(
prefix = "tt",
namespace = "tt: http://www.onvif.org/ver10/schema"
)]
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
