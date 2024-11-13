use xml::reader::XmlEvent;

pub fn assert_xml_eq(actual: &str, expected: &str) {
    for (a, e) in without_whitespaces(actual).zip(without_whitespaces(expected)) {
        match (a, e) {
            (
                Ok(XmlEvent::StartDocument {
                    version,
                    encoding,
                    standalone,
                }),
                Ok(XmlEvent::StartDocument {
                    version: version_expected,
                    encoding: encoding_expected,
                    standalone: standalone_expected,
                }),
            ) => {
                assert_eq!(version, version_expected);
                assert_eq!(encoding.to_lowercase(), encoding_expected.to_lowercase());
                assert_eq!(standalone, standalone_expected);
            }
            (a, e) => assert_eq!(a, e),
        }
    }
}

fn without_whitespaces(
    expected: &str,
) -> impl Iterator<Item = Result<xml::reader::XmlEvent, xml::reader::Error>> + '_ {
    xml::EventReader::new(expected.as_bytes())
        .into_iter()
        .filter(|e| !matches!(e, Ok(xml::reader::XmlEvent::Whitespace(_))))
}
