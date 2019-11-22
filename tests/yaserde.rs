#[macro_use]
extern crate log;
#[macro_use]
extern crate yaserde_derive;

use std::io::Read;
use std::io::Write;
use yaserde::YaSerialize;
use yaserde::YaDeserialize;
use itertools::izip;

// request types:

#[derive(Default, PartialEq, Debug, YaSerialize)]
#[yaserde(
prefix = "tds",
namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
struct GetSystemDateAndTime_GetSystemDateAndTime {
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
#[yaserde(
prefix = "s",
namespace = "s: http://www.w3.org/2003/05/soap-envelope",
)]
struct GetSystemDateAndTime_Body {
    #[yaserde(prefix = "tds", rename = "GetSystemDateAndTime")]
    get_system_date_and_time: GetSystemDateAndTime_GetSystemDateAndTime,
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
#[yaserde(
root = "Envelope",
prefix = "s",
namespace = "s: http://www.w3.org/2003/05/soap-envelope",
namespace = "tds: http://www.onvif.org/ver10/device/wsdl",
namespace = "tt: http://www.onvif.org/ver10/schema",
)]
struct GetSystemDateAndTime_Envelope {
    #[yaserde(prefix = "s", rename = "Body")]
    body: GetSystemDateAndTime_Body,
}


// response types:

#[derive(Default, PartialEq, Debug, YaDeserialize)]
#[yaserde(
prefix = "tt",
namespace = "tt: http://www.onvif.org/ver10/schema"
)]
struct TimeZone {
    #[yaserde(prefix = "tt", rename = "TZ")]
    tz: String,
}

#[derive(Default, PartialEq, Debug, YaDeserialize)]
#[yaserde(
prefix = "tt",
namespace = "tt: http://www.onvif.org/ver10/schema"
)]
struct Time {
    #[yaserde(prefix = "tt", rename = "Hour")]
    hour: i32,
    #[yaserde(prefix = "tt", rename = "Minute")]
    minute: i32,
    #[yaserde(prefix = "tt", rename = "Second")]
    second: i32,
}

#[derive(Default, PartialEq, Debug, YaDeserialize)]
#[yaserde(
prefix = "tt",
namespace = "tt: http://www.onvif.org/ver10/schema"
)]
struct Date {
    #[yaserde(prefix = "tt", rename = "Year")]
    year: i32,
    #[yaserde(prefix = "tt", rename = "Month")]
    month: i32,
    #[yaserde(prefix = "tt", rename = "Day")]
    day: i32,
}

#[derive(Default, PartialEq, Debug, YaDeserialize)]
#[yaserde(
prefix = "tt",
namespace = "tt: http://www.onvif.org/ver10/schema"
)]
struct DateTime {
    #[yaserde(rename = "Time")]
    time: Time,
    #[yaserde(rename = "Date")]
    date: Date,
}

#[derive(Default, PartialEq, Debug, YaDeserialize)]
#[yaserde(
prefix = "tds",
namespace = "tds: http://www.onvif.org/ver10/device/wsdl",
namespace = "tt: http://www.onvif.org/ver10/schema"
)]
struct SystemDateAndTime {
    #[yaserde(prefix = "tt", rename = "DateTimeType")]
    date_time_type: String,
    #[yaserde(prefix = "tt", rename = "DaylightSavings")]
    daylight_savings: bool,
    #[yaserde(rename = "TimeZone")]
    time_zone: TimeZone,
    #[yaserde(rename = "UTCDateTime")]
    utc_date_time: DateTime,
}

#[derive(Default, PartialEq, Debug, YaDeserialize)]
#[yaserde(
prefix = "tds",
namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
struct GetSystemDateAndTimeResponse {
    #[yaserde(rename = "SystemDateAndTime")]
    system_date_and_time: SystemDateAndTime,
}

#[derive(Default, PartialEq, Debug, YaDeserialize)]
#[yaserde(
prefix = "s",
namespace = "s: http://www.w3.org/2003/05/soap-envelope"
)]
struct Response_Body {
    #[yaserde(rename = "GetSystemDateAndTimeResponse")]
    get_system_date_and_time_response: GetSystemDateAndTimeResponse,
}

#[derive(Default, PartialEq, Debug, YaDeserialize)]
#[yaserde(
root = "Envelope",
prefix = "s",
namespace = "s: http://www.w3.org/2003/05/soap-envelope"
)]
struct Response_Envelope {
    #[yaserde(rename = "Body")]
    body: Response_Body,
}

#[test]
fn basic_deserialization() {
    let response = r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope" xmlns:e="http://www.w3.org/2003/05/soap-encoding"
                xmlns:wsa="http://www.w3.org/2005/08/addressing" xmlns:xs="http://www.w3.org/2001/XMLSchema"
                xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:wsaw="http://www.w3.org/2006/05/addressing/wsdl"
                xmlns:wsnt="http://docs.oasis-open.org/wsn/b-2" xmlns:wstop="http://docs.oasis-open.org/wsn/t-1"
                xmlns:wsntw="http://docs.oasis-open.org/wsn/bw-2" xmlns:wsrf-rw="http://docs.oasis-open.org/wsrf/rw-2"
                xmlns:wsrf-r="http://docs.oasis-open.org/wsrf/r-2" xmlns:wsrf-bf="http://docs.oasis-open.org/wsrf/bf-2"
                xmlns:wsdl="http://schemas.xmlsoap.org/wsdl" xmlns:wsoap12="http://schemas.xmlsoap.org/wsdl/soap12"
                xmlns:http="http://schemas.xmlsoap.org/wsdl/http" xmlns:d="http://schemas.xmlsoap.org/ws/2005/04/discovery"
                xmlns:wsadis="http://schemas.xmlsoap.org/ws/2004/08/addressing" xmlns:tt="http://www.onvif.org/ver10/schema"
                xmlns:tns1="http://www.onvif.org/ver10/topics" xmlns:tds="http://www.onvif.org/ver10/device/wsdl"
                xmlns:trt="http://www.onvif.org/ver10/media/wsdl" xmlns:tev="http://www.onvif.org/ver10/events/wsdl"
                xmlns:timg="http://www.onvif.org/ver20/imaging/wsdl" xmlns:tst="http://www.onvif.org/ver10/storage/wsdl"
                xmlns:dn="http://www.onvif.org/ver10/network/wsdl" xmlns:tr2="http://www.onvif.org/ver20/media/wsdl"
                xmlns:tptz="http://www.onvif.org/ver20/ptz/wsdl" xmlns:tan="http://www.onvif.org/ver20/analytics/wsdl"
                xmlns:axt="http://www.onvif.org/ver20/analytics" xmlns:trp="http://www.onvif.org/ver10/replay/wsdl"
                xmlns:tse="http://www.onvif.org/ver10/search/wsdl" xmlns:trc="http://www.onvif.org/ver10/recording/wsdl"
                xmlns:tac="http://www.onvif.org/ver10/accesscontrol/wsdl"
                xmlns:tdc="http://www.onvif.org/ver10/doorcontrol/wsdl" xmlns:pt="http://www.onvif.org/ver10/pacs"
                xmlns:tmd="http://www.onvif.org/ver10/deviceIO/wsdl" xmlns:tth="http://www.onvif.org/ver10/thermal/wsdl"
                xmlns:tcr="http://www.onvif.org/ver10/credential/wsdl"
                xmlns:tar="http://www.onvif.org/ver10/accessrules/wsdl" xmlns:tsc="http://www.onvif.org/ver10/schedule/wsdl"
                xmlns:trv="http://www.onvif.org/ver10/receiver/wsdl"
                xmlns:tpv="http://www.onvif.org/ver10/provisioning/wsdl" xmlns:ter="http://www.onvif.org/ver10/error">
        <s:Body>
            <tds:GetSystemDateAndTimeResponse>
                <tds:SystemDateAndTime>
                    <tt:DateTimeType>NTP</tt:DateTimeType>
                    <tt:DaylightSavings>false</tt:DaylightSavings>
                    <tt:TimeZone>
                        <tt:TZ>PST7PDT</tt:TZ>
                    </tt:TimeZone>
                    <tt:UTCDateTime>
                        <tt:Time>
                            <tt:Hour>16</tt:Hour>
                            <tt:Minute>20</tt:Minute>
                            <tt:Second>9</tt:Second>
                        </tt:Time>
                        <tt:Date>
                            <tt:Year>2019</tt:Year>
                            <tt:Month>11</tt:Month>
                            <tt:Day>18</tt:Day>
                        </tt:Date>
                    </tt:UTCDateTime>
                </tds:SystemDateAndTime>
            </tds:GetSystemDateAndTimeResponse>
        </s:Body>
    </s:Envelope>
    "#;


    let envelope: Response_Envelope = yaserde::de::from_str(&response).unwrap();
    let system_date_and_time = envelope.body
        .get_system_date_and_time_response
        .system_date_and_time;

    println!("{:#?}", system_date_and_time);

    assert_eq!(system_date_and_time.date_time_type, "NTP");
    assert_eq!(system_date_and_time.daylight_savings, false);
    assert_eq!(system_date_and_time.time_zone.tz, "PST7PDT");
    assert_eq!(system_date_and_time.utc_date_time.date.year, 2019);
    assert_eq!(system_date_and_time.utc_date_time.date.month, 11);
    assert_eq!(system_date_and_time.utc_date_time.date.day, 18);
    assert_eq!(system_date_and_time.utc_date_time.time.hour, 16);
    assert_eq!(system_date_and_time.utc_date_time.time.minute, 20);
    assert_eq!(system_date_and_time.utc_date_time.time.second, 9);
}


#[test]
fn basic_serialization() {
    let expected = r#"<?xml version="1.0" encoding="UTF-8"?>
    <s:Envelope
        xmlns:s="http://www.w3.org/2003/05/soap-envelope"
        xmlns:tds="http://www.onvif.org/ver10/device/wsdl"
        xmlns:tt="http://www.onvif.org/ver10/schema">
        <s:Body>
            <tds:GetSystemDateAndTime/>
        </s:Body>
    </s:Envelope>
    "#;

    let envelope: GetSystemDateAndTime_Envelope = Default::default();
    let actual = yaserde::ser::to_string(&envelope).unwrap();

    let actual_iter = xml::EventReader::new(actual.as_bytes())
        .into_iter()
        .filter(|e| match e {
            // TODO: test fails because "UTF-8" != "utf-8", need to think if it is crucial
            Ok(xml::reader::XmlEvent::StartDocument{encoding, ..}) => false,
            _ => true
        });

    let expected_iter = xml::EventReader::new(expected.as_bytes())
        .into_iter()
        .filter(|e| match e {
            Ok(xml::reader::XmlEvent::StartDocument{version, ..}) => false,
            Ok(xml::reader::XmlEvent::Whitespace(_)) => false, // Remove indents from expected string
            _ => true
        });

    for (a, e) in izip!(actual_iter, expected_iter) {
        println!("{:?}", a);
        println!("{:?}", e);

        assert_eq!(a, e);
    }
}
