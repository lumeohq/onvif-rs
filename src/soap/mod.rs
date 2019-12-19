const SOAP_URI: &str = "http://www.w3.org/2003/05/soap-envelope";

pub fn soap(xml: &str) -> Option<String> {
    let mut out = vec![];

    let mut writer = xml::EmitterConfig::new().create_writer(&mut out);

    let reader = xml::EventReader::new(xml.as_bytes());

    writer
        .write(xml::writer::XmlEvent::start_element("s:Envelope").ns("s", SOAP_URI))
        .unwrap();
    writer
        .write(xml::writer::XmlEvent::start_element("s:Body").ns("s", SOAP_URI))
        .unwrap();

    reader
        .into_iter()
        .filter(|x| match x {
            Ok(xml::reader::XmlEvent::StartDocument { .. }) => false,
            _ => true,
        })
        .for_each(|event| {
            if let Ok(event) = event {
                if let Some(event) = event.as_writer_event() {
                    writer.write(event).unwrap();
                }
            }
        });

    writer.write(xml::writer::XmlEvent::end_element()).unwrap();
    writer.write(xml::writer::XmlEvent::end_element()).unwrap();

    String::from_utf8(out).ok()
}

pub fn unsoap(xml: &str) -> Option<String> {
    let reader = xml::EventReader::new(xml.as_bytes());

    let mut out = vec![];

    let mut writer = xml::EmitterConfig::new().create_writer(&mut out);

    let is_soap = |name: &xml::name::OwnedName| {
        name.namespace == Some(SOAP_URI.to_string())
            && (name.local_name == "Envelope" || name.local_name == "Body")
    };

    reader
        .into_iter()
        .filter(|x| match x {
            Ok(xml::reader::XmlEvent::StartElement { name, .. }) => !is_soap(&name),
            Ok(xml::reader::XmlEvent::EndElement { name }) => !is_soap(&name),
            _ => true,
        })
        .for_each(|event| {
            if let Ok(event) = event {
                if let Some(event) = event.as_writer_event() {
                    writer.write(event).unwrap();
                }
            }
        });

    String::from_utf8(out).ok()
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
            <?xml version="1.0" encoding="utf-8"?>
            <s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope">
                <s:Body>
                    <my:Book xmlns:my="http://www.example.my/schema">
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

        println!("{}", actual);

        let parsed: Book = yaserde::de::from_str(&actual).unwrap();

        assert_eq!(parsed.title, "Such book");
        assert_eq!(parsed.pages, 42);
    }
}
