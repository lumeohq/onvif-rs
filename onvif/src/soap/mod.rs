pub mod auth;
pub mod client;
#[cfg(test)]
mod tests;

use auth::username_token::UsernameToken;
use schema::soap_envelope;
use xmltree::{Element, Namespace, XMLNode};

const SOAP_URI: &str = "http://www.w3.org/2003/05/soap-envelope";

#[derive(Debug)]
pub enum Error {
    ParseError,
    EnvelopeNotFound,
    BodyNotFound,
    BodyIsEmpty,
    Fault(soap_envelope::Fault),
    InternalError(String),
}

#[derive(Debug)]
pub struct Response {
    pub response: Option<String>,
}

pub fn soap(xml: &str, username_token: &Option<UsernameToken>) -> Result<String, Error> {
    let app_data = parse(xml)?;

    let mut namespaces = app_data.namespaces.clone().unwrap_or_else(Namespace::empty);
    namespaces.put("s", SOAP_URI);

    let mut body = Element::new("Body");
    body.prefix = Some("s".to_string());
    body.children.push(XMLNode::Element(app_data));

    let mut envelope = Element::new("Envelope");
    envelope.namespaces = Some(namespaces);
    envelope.prefix = Some("s".to_string());

    if let Some(username_token) = username_token {
        let mut header = Element::new("Header");
        header.prefix = Some("s".to_string());
        header
            .children
            .push(XMLNode::Element(parse(&username_token.to_xml())?));
        envelope.children.push(XMLNode::Element(header));
    }

    envelope.children.push(XMLNode::Element(body));

    xml_element_to_string(&envelope)
}

pub fn unsoap(xml: &str) -> Result<String, Error> {
    let root = parse(xml)?;

    if root.name != "Envelope" {
        return Err(Error::EnvelopeNotFound);
    }

    let body = root.get_child("Body").ok_or(Error::BodyNotFound)?;

    if let Some(fault) = body.get_child("Fault") {
        let fault = deserialize_fault(fault)?;
        return Err(Error::Fault(fault));
    }

    body.children
        .iter()
        .find_map(|node| match node {
            XMLNode::Element(app_data) => Some(xml_element_to_string(app_data)),
            _ => None,
        })
        .ok_or(Error::BodyIsEmpty)?
}

fn parse(xml: &str) -> Result<Element, Error> {
    Element::parse(xml.as_bytes()).map_err(|_| Error::ParseError)
}

fn xml_element_to_string(el: &Element) -> Result<String, Error> {
    let mut out = vec![];
    el.write(&mut out)
        .map_err(|_| Error::InternalError("Could not write XML element".to_string()))?;
    String::from_utf8(out).map_err(|e| Error::InternalError(e.to_string()))
}

fn deserialize_fault(envelope: &Element) -> Result<soap_envelope::Fault, Error> {
    let string = xml_element_to_string(envelope)?;
    yaserde::de::from_str(&string).map_err(Error::InternalError)
}
