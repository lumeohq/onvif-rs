pub mod auth;
pub mod client;
pub mod fault;
#[cfg(test)]
mod tests;

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
