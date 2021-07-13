use super::*;

use crate::transport;
use crate::utils::xml_eq::assert_xml_eq;
use assert_approx_eq::assert_approx_eq;
use async_trait::async_trait;
use onvif as tt;
use xsd_types::types as xs;

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
        yaserde::de::from_str(response).unwrap();

    let de = response.system_date_and_time;

    println!("{:#?}", de);

    assert_eq!(de.date_time_type, tt::SetDateTimeType::Ntp);
    assert!(!de.daylight_savings);
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
    let expected = r#"
        <?xml version="1.0" encoding="utf-8"?>
        <tds:GetSystemDateAndTime xmlns:tds="http://www.onvif.org/ver10/device/wsdl" />
        "#;

    let request: devicemgmt::GetSystemDateAndTime = Default::default();
    let actual = yaserde::ser::to_string(&request).unwrap();

    assert_xml_eq(actual.as_str(), expected);
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

    let de: tt::VideoSourceConfiguration = yaserde::de::from_str(ser).unwrap();

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
        </tt:VideoSourceConfiguration>
        "#;

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

    let des: tt::ColorOptions = yaserde::de::from_str(ser).unwrap();

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
        timeout: xs::Duration {
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

    let des: tt::MediaUri = yaserde::de::from_str(ser).unwrap();

    assert_eq!(des.uri, "http://a/b/c".to_string());
    assert!(!des.invalid_after_connect);
    assert!(des.invalid_after_reboot);
    assert_approx_eq!(des.timeout.seconds, 60.0);
}

#[tokio::test]
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
            </tds:GetDeviceInformationResponse>"#
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
                <d:Probe>
                    <d:Types></d:Types>
                </d:Probe>
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

    let des: ws_discovery::probe_matches::Envelope = yaserde::de::from_str(ser).unwrap();

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
fn string_list_serialization() {
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
fn string_list_deserialization() {
    let ser = r#"
        <tt:FocusOptions20Extension xmlns:tt="http://www.onvif.org/ver10/schema">
            <tt:AFModes>Auto Manual</tt:AFModes>
        </tt:FocusOptions20Extension>
        "#;

    let des: tt::FocusOptions20Extension = yaserde::de::from_str(ser).unwrap();

    assert_eq!(
        des.af_modes,
        Some(tt::StringAttrList(vec![
            "Auto".to_string(),
            "Manual".to_string()
        ]))
    );
}

#[test]
fn float_list_serialization() {
    let model = tt::FloatAttrList(vec![1.0, 2.3, 3.99]);

    let expected = r#"
        <?xml version="1.0" encoding="utf-8"?>
        <FloatAttrList>1 2.3 3.99</FloatAttrList>
        "#;

    let actual = yaserde::ser::to_string(&model).unwrap();

    assert_xml_eq(actual.as_str(), expected);
}

#[test]
fn float_list_deserialization() {
    let ser = r#"
        <?xml version="1.0" encoding="utf-8"?>
        <FloatAttrList>1 2.3 3.99</FloatAttrList>
        "#;

    let des: tt::FloatAttrList = yaserde::de::from_str(ser).unwrap();

    assert_eq!(des, tt::FloatAttrList(vec![1.0, 2.3, 3.99]));
}

#[test]
fn nested_structs_with_same_named_attributes() {
    // https://github.com/media-io/yaserde/issues/12#issuecomment-601235031
    let ser = r#"
        <?xml version="1.0" encoding="utf-8"?>
        <tt:Profile token="a" xmlns:tt="http://www.onvif.org/ver10/schema">
            <tt:PTZConfiguration token="b" />
        </tt:Profile>
        "#;

    let des: tt::Profile = yaserde::de::from_str(ser).unwrap();

    assert_eq!(des.token.0.as_str(), "a");
    assert_eq!(des.ptz_configuration.unwrap().token.0.as_str(), "b");
}

#[test]
fn nested_structs_with_same_named_fields() {
    // There was an issue in yaserde which is now fixed
    // https://github.com/media-io/yaserde/issues/51
    let ser = r#"
        <?xml version="1.0" encoding="utf-8"?>
        <tt:Profile xmlns:tt="http://www.onvif.org/ver10/schema">
            <tt:Extension />
        </tt:Profile>
        "#;

    let des: tt::Profile = yaserde::de::from_str(ser).unwrap();
    assert_eq!(
        des,
        tt::Profile {
            extension: Some(tt::ProfileExtension::default()),
            ..Default::default()
        }
    );
}

#[test]
fn extension_inside_extension() {
    // `Extension` inside `Extension` causes infinite loop at deserialization
    // https://github.com/media-io/yaserde/issues/76
    // If field `extension` in `SecurityCapabilitiesExtension` is uncommented accidentally
    // then this test will fail. Also note that there's a bunch of such cases in `onvif.rs`.
    let ser = r#"
    <?xml version="1.0" encoding="utf-8"?>
    <tt:SecurityCapabilities xmlns:tt="http://www.onvif.org/ver10/schema">
        <tt:Extension>
            <tt:TLS1.0>false</tt:TLS1.0>
            <tt:Extension>
                <tt:RemoteUserHandling>false</tt:RemoteUserHandling>
            </tt:Extension>
        </tt:Extension>
    </tt:SecurityCapabilities>
    "#;

    let _ = yaserde::de::from_str::<tt::SecurityCapabilities>(ser).unwrap();
}
