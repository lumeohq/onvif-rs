pub fn assert_xml_eq(actual: &str, expected: &str) {
    for (a, e) in without_whitespaces(actual).zip(without_whitespaces(expected)) {
        assert_eq!(a, e);
    }
}

fn without_whitespaces(
    expected: &str,
) -> impl Iterator<Item = Result<xml::reader::XmlEvent, xml::reader::Error>> + '_ {
    xml::EventReader::new(expected.as_bytes())
        .into_iter()
        .filter(|e| !matches!(e, Ok(xml::reader::XmlEvent::Whitespace(_))))
}
