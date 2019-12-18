pub fn unsoap(xml: &str) -> Option<String> {
    let reader = xml::EventReader::new(xml.as_bytes());

    let mut out = vec![];

    let mut writer = xml::EmitterConfig::new().create_writer(&mut out);

    let is_soap = |name: &xml::name::OwnedName| {
        name.namespace == Some("http://www.w3.org/2003/05/soap-envelope".to_string())
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
                    writer.write(event);
                }
            }
        });

    String::from_utf8(out).ok()
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
