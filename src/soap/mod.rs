pub mod fault;

const SOAP_URI: &str = "http://www.w3.org/2003/05/soap-envelope";

#[derive(Debug)]
pub enum Error {
    ParseError,
    EnvelopeNotFound,
    BodyNotFound,
    BodyIsEmpty,
}

#[derive(Debug)]
pub struct Response {
    response: Option<String>,
    fault: Option<fault::Fault>,
}

pub fn soap(xml: &str) -> Option<String> {
    let app_data = match xmltree::Element::parse(xml.as_bytes()) {
        Ok(v) => v,
        Err(_) => return None,
    };

    let mut namespaces = app_data
        .namespaces
        .clone()
        .unwrap_or(xmltree::Namespace::empty());
    namespaces.put("s", SOAP_URI);

    let mut body = xmltree::Element::new("Body");
    body.prefix = Some("s".to_string());
    body.children.push(app_data);

    let mut envelope = xmltree::Element::new("Envelope");
    envelope.namespaces = Some(namespaces);
    envelope.prefix = Some("s".to_string());
    envelope.children.push(body);

    Some(xml_element_to_string(&envelope))
}

pub fn unsoap(xml: &str) -> Result<Response, Error> {
    let envelope = match xmltree::Element::parse(xml.as_bytes()) {
        Ok(envelope) => match envelope.name.as_ref() {
            "Envelope" => Ok(envelope),
            _ => Err(Error::EnvelopeNotFound),
        },
        _ => Err(Error::ParseError),
    }?;

    let body = match envelope.get_child("Body") {
        Some(body) => Ok(body),
        None => Err(Error::BodyNotFound),
    }?;

    let response = match body.children.first() {
        Some(app_data) => Ok(xml_element_to_string(&app_data)),
        _ => Err(Error::BodyIsEmpty),
    }?;

    let fault = body.get_child("Fault").and_then(|elem| get_fault(elem));

    Ok(Response {
        response: Some(response),
        fault,
    })
}

fn xml_element_to_string(el: &xmltree::Element) -> String {
    // TODO: process errors
    let mut out = vec![];
    el.write(&mut out).unwrap();
    String::from_utf8(out).unwrap()
}

fn get_fault(envelope: &xmltree::Element) -> Option<fault::Fault> {
    yaserde::de::from_str(&xml_element_to_string(envelope)).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::izip;

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

        let actual = soap(app_data).unwrap();

        println!("{}", actual);
        println!("{}", expected);

        assert_xml_eq(actual.as_str(), expected);
    }

    fn assert_xml_eq(actual: &str, expected: &str) -> () {
        for (a, e) in izip!(without_whitespaces(actual), without_whitespaces(expected)) {
            println!("{:?}", a);
            println!("{:?}", e);

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

    #[test]
    fn test_unsoap() {
        use std::io::Read;
        use yaserde::YaDeserialize;

        #[derive(Default, PartialEq, Debug, YaDeserialize)]
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

        let parsed: Book = yaserde::de::from_str(&actual.response.unwrap()).unwrap();

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
                    <soapenv:Value>fault code</soapenv:Value>
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

        let fault = get_fault(&envelope).unwrap();

        assert_eq!(fault.code.value, "fault code");
        assert_eq!(fault.code.subcode.unwrap().value, "ter:fault subcode");
        assert_eq!(fault.reason.text, vec!["fault reason 1", "fault reason 2"]);
    }
}
