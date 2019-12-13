use super::*;

use itertools::izip;
use onvif as tt;


pub struct FakeTransport {
    pub response: String
}


impl transport::Transport for FakeTransport {
    fn request(&mut self, _message: &str) -> Option<String> {
        Some(self.response.clone())
    }
}


#[test]
fn basic_deserialization() {
    let response = r#"
    <?xml version="1.0" encoding="UTF-8"?>
            <tds:GetSystemDateAndTimeResponse
                xmlns:tt="http://www.onvif.org/ver10/schema"
                xmlns:tds="http://www.onvif.org/ver10/device/wsdl">
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
    "#;


    let response: devicemgmt::GetSystemDateAndTimeResponse = yaserde::de::from_str(&response).unwrap();
    let system_date_and_time = response.system_date_and_time;

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
            <tds:GetSystemDateAndTime xmlns:tds="http://www.onvif.org/ver10/device/wsdl" />
    "#;

    let request: devicemgmt::GetSystemDateAndTime = Default::default();
    let actual = yaserde::ser::to_string(&request).unwrap();

    let actual_iter = xml::EventReader::new(actual.as_bytes())
        .into_iter()
        .filter(|e| match e {
            // TODO: test fails because "UTF-8" != "utf-8", need to think if it is crucial
            Ok(xml::reader::XmlEvent::StartDocument{..}) => false,
            _ => true
        });

    let expected_iter = xml::EventReader::new(expected.as_bytes())
        .into_iter()
        .filter(|e| match e {
            Ok(xml::reader::XmlEvent::StartDocument{..}) => false,
            Ok(xml::reader::XmlEvent::Whitespace(_)) => false, // Remove indents from expected string
            _ => true
        });

    for (a, e) in izip!(actual_iter, expected_iter) {
        println!("{:?}", a);
        println!("{:?}", e);

        assert_eq!(a, e);
    }
}


#[test]
fn extend_base_deserialization() {
    let ser = r#"
    <tt:VideoSourceConfiguration token="V_SRC_CFG_000" xmlns:tt="http://www.onvif.org/ver10/schema">
        <tt:Name>V_SRC_CFG_000</tt:Name>
        <tt:UseCount>2</tt:UseCount>
        <tt:SourceToken>V_SRC_000</tt:SourceToken>
        <tt:Bounds height="720" width="1280" y="0" x="0"/>
    </tt:VideoSourceConfiguration>
    "#;

    let des: tt::VideoSourceConfiguration = yaserde::de::from_str(&ser).unwrap();

    assert_eq!(des.token, "V_SRC_CFG_000");
    assert_eq!(des.name0, "V_SRC_CFG_000");
    assert_eq!(des.use_count, 2);
    assert_eq!(des.source_token, "V_SRC_000");
    assert_eq!(des.bounds.x, 0);
    assert_eq!(des.bounds.y, 0);
    assert_eq!(des.bounds.width, 1280);
    assert_eq!(des.bounds.height, 720);
}


#[test]
fn operation_get_system_date_and_time() {
    let req: devicemgmt::GetSystemDateAndTime = Default::default();

    let mut transport = FakeTransport { response: r#"
            <tds:GetSystemDateAndTimeResponse
                        xmlns:tds="http://www.onvif.org/ver10/device/wsdl"
                        xmlns:tt="http://www.onvif.org/ver10/schema">
                <tds:SystemDateAndTime>
                    <tt:DateTimeType>NTP</tt:DateTimeType>
                    <tt:DaylightSavings>false</tt:DaylightSavings>
                    <tt:TimeZone>
                        <tt:TZ>PST7PDT</tt:TZ>
                    </tt:TimeZone>
                    <tt:UTCDateTime>
                        <tt:Time>
                            <tt:Hour>8</tt:Hour>
                            <tt:Minute>5</tt:Minute>
                            <tt:Second>40</tt:Second>
                        </tt:Time>
                        <tt:Date>
                            <tt:Year>2019</tt:Year>
                            <tt:Month>11</tt:Month>
                            <tt:Day>21</tt:Day>
                        </tt:Date>
                    </tt:UTCDateTime>
                </tds:SystemDateAndTime>
            </tds:GetSystemDateAndTimeResponse>"#.into() };

    let resp = devicemgmt::get_system_date_and_time(&mut transport, &req).unwrap();

    assert_eq!(resp.system_date_and_time.utc_date_time.time.second, 40);
}


#[test]
fn operation_get_device_information(){
    let req: devicemgmt::GetDeviceInformation = Default::default();

    let mut transport = FakeTransport { response: r#"
        <tds:GetDeviceInformationResponse
                    xmlns:tds="http://www.onvif.org/ver10/device/wsdl"
                    xmlns:tt="http://www.onvif.org/ver10/schema">
            <tds:Manufacturer>Somebody</tds:Manufacturer>
            <tds:Model>IPCamera</tds:Model>
            <tds:FirmwareVersion>1.5</tds:FirmwareVersion>
            <tds:SerialNumber>a12b34</tds:SerialNumber>
            <tds:HardwareId>2.0</tds:HardwareId>
        </tds:GetDeviceInformationResponse>
    "#.into() };

    let resp = devicemgmt::get_device_information(&mut transport, &req).unwrap();

    assert_eq!(resp.manufacturer, "Somebody");
}
