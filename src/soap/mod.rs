pub mod auth;
pub mod client;
pub mod fault;

const SOAP_URI: &str = "http://www.w3.org/2003/05/soap-envelope";

#[derive(Debug)]
pub enum Error {
    ParseError,
    EnvelopeNotFound,
    BodyNotFound,
    BodyIsEmpty,
    Fault(fault::Fault),
    InternalError(String),
}

#[derive(Debug)]
pub struct Response {
    pub response: Option<String>,
}

pub fn soap(xml: &str, username_token: &Option<auth::UsernameToken>) -> Result<String, Error> {
    let app_data = parse(xml)?;

    let mut namespaces = app_data
        .namespaces
        .clone()
        .unwrap_or_else(xmltree::Namespace::empty);
    namespaces.put("s", SOAP_URI);

    let mut body = xmltree::Element::new("Body");
    body.prefix = Some("s".to_string());
    body.children.push(app_data);

    let mut envelope = xmltree::Element::new("Envelope");
    envelope.namespaces = Some(namespaces);
    envelope.prefix = Some("s".to_string());

    if let Some(username_token) = username_token {
        let mut header = xmltree::Element::new("Header");
        header.prefix = Some("s".to_string());
        header.children.push(parse(&username_token.to_xml())?);
        envelope.children.push(header);
    }

    envelope.children.push(body);

    xml_element_to_string(&envelope)
}

pub fn unsoap(xml: &str) -> Result<String, Error> {
    let root = parse(xml)?;

    let envelope = match root.name.as_ref() {
        "Envelope" => Ok(root),
        _ => Err(Error::EnvelopeNotFound),
    }?;

    let body = match envelope.get_child("Body") {
        Some(body) => Ok(body),
        None => Err(Error::BodyNotFound),
    }?;

    match body.get_child("Fault") {
        Some(fault) => deserialize_fault(fault).and_then(|fault| Err(Error::Fault(fault))),
        None => Ok(()),
    }?;

    let response = match body.children.first() {
        Some(app_data) => xml_element_to_string(&app_data),
        _ => Err(Error::BodyIsEmpty),
    }?;

    Ok(response)
}

fn parse(xml: &str) -> Result<xmltree::Element, Error> {
    xmltree::Element::parse(xml.as_bytes()).map_err(|_| Error::ParseError)
}

fn xml_element_to_string(el: &xmltree::Element) -> Result<String, Error> {
    let mut out = vec![];
    el.write(&mut out)
        .map_err(|_| Error::InternalError("Could not write XML element".to_string()))?;
    String::from_utf8(out).map_err(|e| Error::InternalError(e.to_string()))
}

fn deserialize_fault(envelope: &xmltree::Element) -> Result<fault::Fault, Error> {
    let string = xml_element_to_string(envelope)?;
    yaserde::de::from_str(&string).map_err(Error::InternalError)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::xml_eq::assert_xml_eq;

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

        let fault = deserialize_fault(&envelope).unwrap();

        assert_eq!(fault.code.value, "fault code");
        assert_eq!(fault.code.subcode.unwrap().value, "ter:fault subcode");
        assert_eq!(fault.reason.text, vec!["fault reason 1", "fault reason 2"]);
    }
}
