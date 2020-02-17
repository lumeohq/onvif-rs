use super::*;

use async_trait::async_trait;
use common;
use itertools::izip;
use onvif as tt;

pub struct FakeTransport {
    pub response: String,
}

#[async_trait]
impl transport::Transport for FakeTransport {
    async fn request(&self, _message: &str) -> Result<String, transport::Error> {
        Ok(self.response.clone())
    }
}

#[test]
#[ignore] // Need to deal with SystemDateAndTime namespace
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

    let de = response.system_date_and_time;

    println!("{:#?}", de);

    assert_eq!(de.date_time_type, tt::SetDateTimeType::Ntp);
    assert_eq!(de.daylight_savings, false);
    assert_eq!(de.time_zone.unwrap().tz, "PST7PDT");
    assert_eq!(de.utc_date_time.as_ref().unwrap().date.year, 2019);
    assert_eq!(de.utc_date_time.as_ref().unwrap().date.month, 11);
    assert_eq!(de.utc_date_time.as_ref().unwrap().date.day, 18);
    assert_eq!(de.utc_date_time.as_ref().unwrap().time.hour, 16);
    assert_eq!(de.utc_date_time.as_ref().unwrap().time.minute, 20);
    assert_eq!(de.utc_date_time.as_ref().unwrap().time.second, 9);
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

    let de: tt::VideoSourceConfiguration = yaserde::de::from_str(&ser).unwrap();

    assert_eq!(
        de.token,
        common::ReferenceToken("V_SRC_CFG_000".to_string())
    );
    assert_eq!(de.name, tt::Name("V_SRC_CFG_000".to_string()));
    assert_eq!(de.use_count, 2);
    assert_eq!(
        de.source_token,
        common::ReferenceToken("V_SRC_000".to_string())
    );
    assert_eq!(de.bounds.x, 0);
    assert_eq!(de.bounds.y, 0);
    assert_eq!(de.bounds.width, 1280);
    assert_eq!(de.bounds.height, 720);
}

#[test]
fn extend_base_serialization() {
    let model = tt::VideoSourceConfiguration {
        token: common::ReferenceToken("123abc".to_string()),
        name: tt::Name("MyName".to_string()),
        use_count: 2,
        source_token: common::ReferenceToken("456cde".to_string()),
        bounds: tt::IntRectangle {
            x: 1,
            y: 2,
            width: 3,
            height: 4,
        },
        ..Default::default()
    };

    let expected = r#"
    <?xml version="1.0" encoding="utf-8"?>
    <tt:VideoSourceConfiguration xmlns:tt="http://www.onvif.org/ver10/schema" token="123abc">
        <tt:SourceToken>456cde</tt:SourceToken>
        <tt:Bounds x="1" y="2" width="3" height="4" />
        <tt:Name>MyName</tt:Name>
        <tt:UseCount>2</tt:UseCount>
    </tt:VideoSourceConfiguration>"#;

    let actual = yaserde::ser::to_string(&model).unwrap();

    println!("actual: {}", actual);
    println!("expected: {}", expected);

    assert_xml_eq(actual.as_str(), expected);
}

#[test]
fn choice_deserialization() {
    let ser = r#"
    <tt:ColorOptions tt:any_attribute="attr_value" xmlns:tt="http://www.onvif.org/ver10/schema">
        <tt:ColorspaceRange>
            <tt:X><tt:Min>0.1</tt:Min><tt:Max>0.11</tt:Max></tt:X>
            <tt:Y><tt:Min>0.2</tt:Min><tt:Max>0.22</tt:Max></tt:Y>
            <tt:Z><tt:Min>0.3</tt:Min><tt:Max>0.33</tt:Max></tt:Z>
            <tt:Colorspace>http://my.color.space</tt:Colorspace>
        </tt:ColorspaceRange>
        <tt:ColorspaceRange>
            <tt:X><tt:Min>0.4</tt:Min><tt:Max>0.44</tt:Max></tt:X>
            <tt:Y><tt:Min>0.5</tt:Min><tt:Max>0.55</tt:Max></tt:Y>
            <tt:Z><tt:Min>0.6</tt:Min><tt:Max>0.66</tt:Max></tt:Z>
            <tt:Colorspace>http://my.color.space</tt:Colorspace>
        </tt:ColorspaceRange>
    </tt:ColorOptions>
    "#;

    let des: tt::ColorOptions = yaserde::de::from_str(&ser).unwrap();

    match des.color_options_choice {
        tt::ColorOptionsChoice::ColorspaceRange(colors) => {
            assert_eq!(colors.len(), 2);

            assert_eq!(
                colors[0].x,
                tt::FloatRange {
                    min: 0.1,
                    max: 0.11
                }
            );
            assert_eq!(
                colors[0].y,
                tt::FloatRange {
                    min: 0.2,
                    max: 0.22
                }
            );
            assert_eq!(
                colors[0].z,
                tt::FloatRange {
                    min: 0.3,
                    max: 0.33
                }
            );
            assert_eq!(colors[0].colorspace, String::from("http://my.color.space"));

            assert_eq!(
                colors[1].x,
                tt::FloatRange {
                    min: 0.4,
                    max: 0.44
                }
            );
            assert_eq!(
                colors[1].y,
                tt::FloatRange {
                    min: 0.5,
                    max: 0.55
                }
            );
            assert_eq!(
                colors[1].z,
                tt::FloatRange {
                    min: 0.6,
                    max: 0.66
                }
            );
            assert_eq!(colors[1].colorspace, String::from("http://my.color.space"));
        }
        _ => panic!("Wrong variant: {:?}", des.color_options_choice),
    }
}

#[test]
fn choice_serialization() {
    let model = tt::ColorOptions {
        color_options_choice: tt::ColorOptionsChoice::ColorspaceRange(vec![
            tt::ColorspaceRange {
                x: tt::FloatRange {
                    min: 0.1,
                    max: 0.11,
                },
                y: tt::FloatRange {
                    min: 0.2,
                    max: 0.22,
                },
                z: tt::FloatRange {
                    min: 0.3,
                    max: 0.33,
                },
                colorspace: "http://my.color.space".to_string(),
            },
            tt::ColorspaceRange {
                x: tt::FloatRange {
                    min: 0.4,
                    max: 0.44,
                },
                y: tt::FloatRange {
                    min: 0.5,
                    max: 0.55,
                },
                z: tt::FloatRange {
                    min: 0.6,
                    max: 0.66,
                },
                colorspace: "http://my.color.space".to_string(),
            },
        ]),
    };

    let expected = r#"
    <?xml version="1.0" encoding="utf-8"?>
    <tt:ColorOptions xmlns:tt="http://www.onvif.org/ver10/schema">
        <ColorspaceRange>
            <tt:X><tt:Min>0.1</tt:Min><tt:Max>0.11</tt:Max></tt:X>
            <tt:Y><tt:Min>0.2</tt:Min><tt:Max>0.22</tt:Max></tt:Y>
            <tt:Z><tt:Min>0.3</tt:Min><tt:Max>0.33</tt:Max></tt:Z>
            <tt:Colorspace>http://my.color.space</tt:Colorspace>
        </ColorspaceRange>
        <ColorspaceRange>
            <tt:X><tt:Min>0.4</tt:Min><tt:Max>0.44</tt:Max></tt:X>
            <tt:Y><tt:Min>0.5</tt:Min><tt:Max>0.55</tt:Max></tt:Y>
            <tt:Z><tt:Min>0.6</tt:Min><tt:Max>0.66</tt:Max></tt:Z>
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
        timeout: crate::schema::xs::Duration {
            seconds: 60.0,
            ..Default::default()
        },
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

#[tokio::test]
#[ignore] // Need to deal with SystemDateAndTime namespace
async fn operation_get_system_date_and_time() {
    let req: devicemgmt::GetSystemDateAndTime = Default::default();

    let transport = FakeTransport {
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

    let resp = devicemgmt::get_system_date_and_time(&transport, &req)
        .await
        .unwrap();

    assert_eq!(
        resp.system_date_and_time.utc_date_time.unwrap().time.second,
        40
    );
}

#[tokio::test]
async fn operation_get_device_information() {
    let req: devicemgmt::GetDeviceInformation = Default::default();

    let transport = FakeTransport {
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

    let resp = devicemgmt::get_device_information(&transport, &req)
        .await
        .unwrap();

    assert_eq!(resp.manufacturer, "Somebody");
}

#[test]
fn probe_serialization() {
    let expected = r#"
        <?xml version="1.0" encoding="utf-8"?>
        <s:Envelope
                xmlns:d="http://schemas.xmlsoap.org/ws/2005/04/discovery"
                xmlns:s="http://www.w3.org/2003/05/soap-envelope"
                xmlns:w="http://schemas.xmlsoap.org/ws/2004/08/addressing">
            <s:Header>
                <w:MessageID>uuid:84ede3de-7dec-11d0-c360-f01234567890</w:MessageID>
                <w:To>urn:schemas-xmlsoap-org:ws:2005:04:discovery</w:To>
                <w:Action>http://schemas.xmlsoap.org/ws/2005/04/discovery/Probe</w:Action>
            </s:Header>
            <s:Body>
                <d:Probe />
            </s:Body>
        </s:Envelope>
        "#;

    use ws_discovery::probe::*;

    let probe = Envelope {
        header: Header {
            message_id: "uuid:84ede3de-7dec-11d0-c360-f01234567890".into(),
            action: "http://schemas.xmlsoap.org/ws/2005/04/discovery/Probe".into(),
            to: "urn:schemas-xmlsoap-org:ws:2005:04:discovery".into(),
        },
        ..Default::default()
    };

    let actual = yaserde::ser::to_string(&probe).unwrap();

    assert_xml_eq(&actual, expected);
}

#[test]
fn probe_match_deserialization() {
    // Following XML was taken from ONVIF guide
    // https://www.onvif.org/wp-content/uploads/2016/12/ONVIF_WG-APG-Application_Programmers_Guide-1.pdf

    let ser = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <SOAP-ENV:Envelope
                    xmlns:SOAP-ENV="http://www.w3.org/2003/05/soap-envelope"
                    xmlns:wsa="http://schemas.xmlsoap.org/ws/2004/08/addressing"
                    xmlns:d="http://schemas.xmlsoap.org/ws/2005/04/discovery"
                    xmlns:dn="http://www.onvif.org/ver10/network/wsdl">
            <SOAP-ENV:Header>
                <wsa:MessageID>uuid:84ede3de-e374-11df-b259-00408c1836b2</wsa:MessageID>
                <wsa:RelatesTo>uuid:84ede3de-7dec-11d0-c360-F01234567890</wsa:RelatesTo>
                <wsa:To SOAP-ENV:mustUnderstand="true">
                    http://schemas.xmlsoap.org/ws/2004/08/addressing/role/anonymous
                </wsa:To>
                <wsa:Action SOAP-ENV:mustUnderstand="true">
                    http://schemas.xmlsoap.org/ws/2005/04/discovery/ProbeMatches
                </wsa:Action>
                <d:AppSequence SOAP-ENV:mustUnderstand="true" MessageNumber="3" InstanceId="1287607812">
                </d:AppSequence>
            </SOAP-ENV:Header>
            <SOAP-ENV:Body>
                <d:ProbeMatches>
                    <d:ProbeMatch>
                        <wsa:EndpointReference>
                            <wsa:Address>urn:uuid:a1f48ac2-dc8b-11df-b255-00408c1836b2</wsa:Address>
                        </wsa:EndpointReference>
                        <d:Types>dn:NetworkVideoTransmitter</d:Types>
                        <d:Scopes>
                            onvif://www.onvif.org/type/video_encoder
                            onvif://www.onvif.org/type/audio_encoder
                            onvif://www.onvif.org/hardware/MODEL
                            onvif://www.onvif.org/name/VENDOR%20MODEL
                            onvif://www.onvif.org/location/ANY
                        </d:Scopes>
                        <d:XAddrs>
                            http://169.254.76.145/onvif/services
                            http://192.168.1.24/onvif/services
                        </d:XAddrs>
                        <d:MetadataVersion>1</d:MetadataVersion>
                    </d:ProbeMatch>
                </d:ProbeMatches>
            </SOAP-ENV:Body>
        </SOAP-ENV:Envelope>
        "#;

    let des: ws_discovery::probe_matches::Envelope = yaserde::de::from_str(&ser).unwrap();

    assert_eq!(
        des.header.relates_to,
        "uuid:84ede3de-7dec-11d0-c360-F01234567890".to_string()
    );
    assert_eq!(
        des.body.probe_matches.probe_match[0]
            .x_addrs
            .split_whitespace()
            .collect::<Vec<&str>>(),
        vec![
            "http://169.254.76.145/onvif/services",
            "http://192.168.1.24/onvif/services"
        ]
    );
}

#[test]
fn list_serialization() {
    let model = tt::FocusOptions20Extension {
        af_modes: Some(tt::StringAttrList(vec![
            "Auto".to_string(),
            "Manual".to_string(),
        ])),
    };

    let expected = r#"
    <?xml version="1.0" encoding="utf-8"?>
    <tt:FocusOptions20Extension xmlns:tt="http://www.onvif.org/ver10/schema">
        <tt:AFModes>Auto Manual</tt:AFModes>
    </tt:FocusOptions20Extension>
    "#;

    let actual = yaserde::ser::to_string(&model).unwrap();

    assert_xml_eq(actual.as_str(), expected);
}

#[test]
fn list_deserialization() {
    let ser = r#"
    <tt:FocusOptions20Extension xmlns:tt="http://www.onvif.org/ver10/schema">
        <tt:AFModes>Auto Manual</tt:AFModes>
    </tt:FocusOptions20Extension>
    "#;

    let des: tt::FocusOptions20Extension = yaserde::de::from_str(&ser).unwrap();

    assert_eq!(
        des.af_modes,
        Some(tt::StringAttrList(vec![
            "Auto".to_string(),
            "Manual".to_string()
        ]))
    );
}

pub fn assert_xml_eq(actual: &str, expected: &str) -> () {
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
