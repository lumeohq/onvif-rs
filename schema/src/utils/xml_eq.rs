use itertools::izip;

pub fn assert_xml_eq(actual: &str, expected: &str) {
    for (a, e) in izip!(without_whitespaces(actual), without_whitespaces(expected)) {
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
