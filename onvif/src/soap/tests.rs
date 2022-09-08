use super::*;
use crate::utils::xml_eq::assert_xml_eq;
use schema::soap_envelope::{FaultcodeEnum, Reasontext, Subcode};

#[test]
fn test_soap() {
    let app_data = r#"
        <my:Book xmlns:my="http://www.example.my/schema">
            <my:Title>Such book</my:Title>
            <my:Pages>42</my:Pages>
        </my:Book>
        "#;

    let expected = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope"
                    xmlns:my="http://www.example.my/schema">
            <s:Body>
                <my:Book>
                    <my:Title>Such book</my:Title>
                    <my:Pages>42</my:Pages>
                </my:Book>
            </s:Body>
        </s:Envelope>
        "#;

    let actual = soap(app_data, &None).unwrap();

    println!("{}", actual);
    println!("{}", expected);

    assert_xml_eq(actual.as_str(), expected);
}

#[test]
fn test_unsoap() {
    #[derive(Default, Eq, PartialEq, Debug, YaDeserialize)]
    #[yaserde(prefix = "my", namespace = "my: http://www.example.my/schema")]
    pub struct Book {
        #[yaserde(prefix = "my", rename = "Title")]
        pub title: String,

        #[yaserde(prefix = "my", rename = "Pages")]
        pub pages: i32,
    }

    let input = r#"
        <?xml version="1.0" encoding="utf-8"?>
        <s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope"
                    xmlns:my="http://www.example.my/schema">
            <s:Body>
                <my:Book>
                    <my:Title>Such book</my:Title>
                    <my:Pages>42</my:Pages>
                </my:Book>
            </s:Body>
        </s:Envelope>
        "#;

    let actual = unsoap(input).unwrap();

    println!("{:?}", actual);

    let parsed: Book = yaserde::de::from_str(&actual).unwrap();

    assert_eq!(parsed.title, "Such book");
    assert_eq!(parsed.pages, 42);
}

#[test]
fn test_get_fault() {
    let response = r#"
        <?xml version="1.0" ?>
        <soapenv:Fault
            xmlns:soapenv="http://www.w3.org/2003/05/soap-envelope"
            xmlns:ter="http://www.onvif.org/ver10/error"
            xmlns:xs="http://www.w3.org/2000/10/XMLSchema">
            <soapenv:Code>
                <soapenv:Value>tns:DataEncodingUnknown</soapenv:Value>
                <soapenv:Subcode>
                    <soapenv:Value>ter:fault subcode</soapenv:Value>
                    <soapenv:Subcode>
                        <soapenv:Value>ter:fault subcode</soapenv:Value>
                    </soapenv:Subcode>
                </soapenv:Subcode>
            </soapenv:Code>
            <soapenv:Reason>
                <soapenv:Text xml:lang="en">fault reason 1</soapenv:Text>
                <soapenv:Text xml:lang="en">fault reason 2</soapenv:Text>
            </soapenv:Reason>
            <soapenv:Node>http://www.w3.org/2003/05/soap-envelope/node/ultimateReceiver</soapenv:Node>
            <soapenv:Role>http://www.w3.org/2003/05/soap-envelope/role/ultimateReceiver</soapenv:Role>
            <soapenv:Detail>
                <soapenv:Text>fault detail</soapenv:Text>
            </soapenv:Detail>
        </soapenv:Fault>
        "#;

    let envelope = xmltree::Element::parse(response.as_bytes()).unwrap();

    let fault = deserialize_fault(&envelope).unwrap();

    assert_eq!(
        fault.code.value,
        FaultcodeEnum("tns:DataEncodingUnknown".to_string())
    );
    assert_eq!(
        fault.code.subcode,
        Some(Subcode {
            value: "ter:fault subcode".to_string(),
            // subcode: Vec::new()
        })
    );
    assert_eq!(
        fault.reason.text,
        vec![
            Reasontext {
                lang: "en".to_string()
            },
            Reasontext {
                lang: "en".to_string()
            }
        ]
    );
}
