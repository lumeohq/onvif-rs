use super::*;

use itertools::izip;
use onvif as tt;

pub struct FakeTransport {
    pub response: String,
}

impl transport::Transport for FakeTransport {
    fn request(&mut self, _message: &str) -> Result<String, transport::Error> {
        Ok(self.response.clone())
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

    let response: devicemgmt::GetSystemDateAndTimeResponse =
        yaserde::de::from_str(&response).unwrap();
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
            Ok(xml::reader::XmlEvent::StartDocument { .. }) => false,
            _ => true,
        });

    let expected_iter = xml::EventReader::new(expected.as_bytes())
        .into_iter()
        .filter(|e| match e {
            Ok(xml::reader::XmlEvent::StartDocument { .. }) => false,
            Ok(xml::reader::XmlEvent::Whitespace(_)) => false, // Remove indents from expected string
            _ => true,
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
    assert_eq!(des.name, types::Name("V_SRC_CFG_000".to_string()));
    assert_eq!(des.use_count, 2);
    assert_eq!(des.source_token, "V_SRC_000");
    assert_eq!(des.bounds.x, 0);
    assert_eq!(des.bounds.y, 0);
    assert_eq!(des.bounds.width, 1280);
    assert_eq!(des.bounds.height, 720);
}

#[test]
fn extend_base_serialization() {
    let model = tt::VideoSourceConfiguration {
        token: "123abc".to_string(),
        name: types::Name("MyName".to_string()),
        use_count: 2,
        source_token: "456cde".to_string(),
        bounds: tt::IntRectangle {
            x: 1,
            y: 2,
            width: 3,
            height: 4,
        },
    };

    let expected = r#"
    <?xml version="1.0" encoding="utf-8"?>
    <tt:VideoSourceConfiguration xmlns:tt="http://www.onvif.org/ver10/schema" token="123abc">
        <tt:Name>MyName</tt:Name>
        <tt:UseCount>2</tt:UseCount>
        <tt:SourceToken>456cde</tt:SourceToken>
        <tt:Bounds x="1" y="2" width="3" height="4" />
    </tt:VideoSourceConfiguration>"#;

    let actual = yaserde::ser::to_string(&model).unwrap();

    println!("actual: {}", actual);
    println!("expected: {}", expected);

    assert_xml_eq(actual.as_str(), expected);
}

#[test]
fn choice_deserialization() {
    let ser = r#"
    <tt:ColorOptions attr_name="attr_value" xmlns:tt="http://www.onvif.org/ver10/schema">
        <tt:ColorspaceRange>
            <X>0.1</X>
            <Y>0.2</Y>
            <Z>0.3</Z>
            <Colorspace>http://my.color.space</Colorspace>
        </tt:ColorspaceRange>
        <tt:ColorspaceRange>
            <X>0.5</X>
            <Y>0.6</Y>
            <Z>0.7</Z>
            <Colorspace>http://my.color.space</Colorspace>
        </tt:ColorspaceRange>
    </tt:ColorOptions>
    "#;

    let des: tt::ColorOptions = yaserde::de::from_str(&ser).unwrap();

    match des.choice {
        tt::ColorOptionsChoice::ColorspaceRange(colors) => {
            assert_eq!(colors.len(), 2);

            assert_eq!(colors[0].x, 0.1);
            assert_eq!(colors[0].y, 0.2);
            assert_eq!(colors[0].z, 0.3);
            assert_eq!(colors[0].colorspace, String::from("http://my.color.space"));

            assert_eq!(colors[1].x, 0.5);
            assert_eq!(colors[1].y, 0.6);
            assert_eq!(colors[1].z, 0.7);
            assert_eq!(colors[1].colorspace, String::from("http://my.color.space"));
        },
        _ => panic!("Wrong variant")
    }

    assert_eq!(des.any_attribute, Some(("attr_name".to_string(), "attr_value".to_string())));
}

#[test]
fn choice_serialization() {
    let model = tt::ColorOptions {
        choice: tt::ColorOptionsChoice::ColorspaceRange(vec![
            tt::ColorspaceRange {
                x: 0.1,
                y: 0.2,
                z: 0.3,
                colorspace: "http://my.color.space".to_string()
            },
            tt::ColorspaceRange {
                x: 0.5,
                y: 0.6,
                z: 0.7,
                colorspace: "http://my.color.space".to_string()
            }
        ]),
        any_attribute: Some(("attr_name".to_string(), "attr_value".to_string()))
    };

    // TODO: "ColorspaceRange" must be "tt:ColorspaceRange", fixed in yaserde 0.3.11

    let expected = r#"
    <?xml version="1.0" encoding="utf-8"?>
    <tt:ColorOptions attr_name="attr_value" xmlns:tt="http://www.onvif.org/ver10/schema">
        <ColorspaceRange>
            <tt:X>0.1</tt:X>
            <tt:Y>0.2</tt:Y>
            <tt:Z>0.3</tt:Z>
            <tt:Colorspace>http://my.color.space</tt:Colorspace>
        </ColorspaceRange>
        <ColorspaceRange>
            <tt:X>0.5</tt:X>
            <tt:Y>0.6</tt:Y>
            <tt:Z>0.7</tt:Z>
            <tt:Colorspace>http://my.color.space</tt:Colorspace>
        </ColorspaceRange>
    </tt:ColorOptions>
    "#;

    let actual = yaserde::ser::to_string(&model).unwrap();

    println!("actual: {}", actual);
    println!("expected: {}", expected);

    assert_xml_eq(actual.as_str(), expected);
}

#[test]
fn duration_serialization() {
    let model = tt::MediaUri {
        uri: "http://a/b/c".to_string(),
        invalid_after_connect: false,
        invalid_after_reboot: true,
        timeout: crate::schema::duration::Duration {
            seconds: 60.0,
            .. Default::default()
        }
    };

    let expected = r#"
    <?xml version="1.0" encoding="utf-8"?>
    <tt:MediaUri xmlns:tt="http://www.onvif.org/ver10/schema">
        <tt:Uri>http://a/b/c</tt:Uri>
        <tt:InvalidAfterConnect>false</tt:InvalidAfterConnect>
        <tt:InvalidAfterReboot>true</tt:InvalidAfterReboot>
        <tt:Timeout>PT60S</tt:Timeout>
    </tt:MediaUri>
    "#;

    let actual = yaserde::ser::to_string(&model).unwrap();

    assert_xml_eq(actual.as_str(), expected);
}

#[test]
fn duration_deserialization() {
    let ser = r#"
    <tt:MediaUri xmlns:tt="http://www.onvif.org/ver10/schema">
        <tt:Uri>http://a/b/c</tt:Uri>
        <tt:InvalidAfterConnect>false</tt:InvalidAfterConnect>
        <tt:InvalidAfterReboot>true</tt:InvalidAfterReboot>
        <tt:Timeout>PT60S</tt:Timeout>
    </tt:MediaUri>
    "#;

    let des: tt::MediaUri = yaserde::de::from_str(&ser).unwrap();

    assert_eq!(des.uri, "http://a/b/c".to_string());
    assert_eq!(des.invalid_after_connect, false);
    assert_eq!(des.invalid_after_reboot, true);
    assert_eq!(des.timeout.seconds, 60.0);
}

#[test]
fn operation_get_system_date_and_time() {
    let req: devicemgmt::GetSystemDateAndTime = Default::default();

    let mut transport = FakeTransport {
        response: r#"
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
            </tds:GetSystemDateAndTimeResponse>"#
            .into(),
    };

    let resp = devicemgmt::get_system_date_and_time(&mut transport, &req).unwrap();

    assert_eq!(resp.system_date_and_time.utc_date_time.time.second, 40);
}

#[test]
fn operation_get_device_information() {
    let req: devicemgmt::GetDeviceInformation = Default::default();

    let mut transport = FakeTransport {
        response: r#"
        <tds:GetDeviceInformationResponse
                    xmlns:tds="http://www.onvif.org/ver10/device/wsdl"
                    xmlns:tt="http://www.onvif.org/ver10/schema">
            <tds:Manufacturer>Somebody</tds:Manufacturer>
            <tds:Model>IPCamera</tds:Model>
            <tds:FirmwareVersion>1.5</tds:FirmwareVersion>
            <tds:SerialNumber>a12b34</tds:SerialNumber>
            <tds:HardwareId>2.0</tds:HardwareId>
        </tds:GetDeviceInformationResponse>
    "#
        .into(),
    };

    let resp = devicemgmt::get_device_information(&mut transport, &req).unwrap();

    assert_eq!(resp.manufacturer, "Somebody");
}

fn assert_xml_eq(actual: &str, expected: &str) -> () {
    for (a, e) in izip!(without_whitespaces(actual), without_whitespaces(expected)) {
        assert_eq!(a, e);
    }
}

fn without_whitespaces<'a>(
    expected: &'a str,
) -> impl Iterator<Item = Result<xml::reader::XmlEvent, xml::reader::Error>> + 'a {
    xml::EventReader::new(expected.as_bytes())
        .into_iter()
        .filter(|e| match e {
            Ok(xml::reader::XmlEvent::Whitespace(_)) => false,
            _ => true,
        })
}
