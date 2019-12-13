use yaserde::{YaDeserialize, YaSerialize};

pub trait Transport {
    fn request(&mut self, message: &str) -> Option<String>;
}

pub fn request<T: Transport, R: YaSerialize, S: YaDeserialize>(
    transport: &mut T,
    request: &R,
) -> Option<S> {
    // TODO: process all kinds of faults: HTTP, SOAP, ONVIF

    if let Ok(xml_text) = yaserde::ser::to_string(request) {
        if let Some(resp_text) = transport.request(&crop_xml_declaration(&xml_text)) {
            if let Ok(val) = yaserde::de::from_str(&resp_text) {
                return Some(val);
            }
        }
    }

    None
}

fn crop_xml_declaration(xml: &str) -> String {
    xml.split("?>").skip(1).collect()
}

#[test]
fn test_crop_xml_declaration() {
    assert_eq!(
        crop_xml_declaration(r#"<?xml version="1.0" encoding="utf-8"?><element />"#),
        "<element />"
    );
}
