use yaserde::{YaDeserialize, YaSerialize};

#[derive(Debug)]
pub enum Error {
    Serialization(String),
    Http(String),
    Soap(String),
    Onvif(String),
}

pub trait Transport {
    fn request(&mut self, message: &str) -> Result<String, Error>;
}

pub fn request<T: Transport, R: YaSerialize, S: YaDeserialize>(
    transport: &mut T,
    request: &R,
) -> Result<S, Error> {
    let ser = |obj: &R| yaserde::ser::to_string(obj).map_err(|e| Error::Serialization(e));

    let de = |s: &str| yaserde::de::from_str(s).map_err(|e| Error::Serialization(e));

    ser(&request).and_then(|serialized| {
        transport
            .request(&crop_xml_declaration(&serialized))
            .and_then(|response| de(&response))
    })
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
