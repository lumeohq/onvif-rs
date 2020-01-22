// Based on types.xsd

// targetNamespace="http://www.onvif.org/ver10/pacs"

// xmlns:xs="http://www.w3.org/2001/XMLSchema"
// xmlns:pt="http://www.onvif.org/ver10/pacs"

use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

//<xs:simpleType name="Name">
//    <xs:restriction base="xs:string">
//        <xs:maxLength value="64"/>
//        <xs:minLength value="0"/>
//    </xs:restriction>
//</xs:simpleType>

#[derive(Default, PartialEq, Debug)]
pub struct Name(pub String);

impl YaDeserialize for Name {
    fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        if let Ok(xml::reader::XmlEvent::StartElement { .. }) = reader.peek() {
            reader.next_event()?;
        } else {
            return Err("Start element not found".to_string());
        }

        if let Ok(xml::reader::XmlEvent::Characters(ref text)) = reader.peek() {
            if text.len() > 64 {
                Err(format!("Max length exceeded: {}", text.len()))
            } else {
                Ok(Name(text.clone()))
            }
        } else {
            Err("Start element not found".to_string())
        }
    }
}

impl YaSerialize for Name {
    fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>) -> Result<(), String> {
        // TODO: this should be simplified since yaserde 0.3.11
        if let Some(override_name) = writer.get_start_event_name() {
            writer
                .write(xml::writer::XmlEvent::start_element(override_name.as_str()))
                .map_err(|_e| "Start element write failed".to_string())
        } else {
            if !writer.skip_start_end() {
                writer
                    .write(xml::writer::XmlEvent::start_element("tt:Name"))
                    .map_err(|_e| "Start element write failed".to_string())?;
            }

            writer
                .write(xml::writer::XmlEvent::characters(self.0.as_str()))
                .map_err(|_e| "Element value write failed".to_string())?;

            if !writer.skip_start_end() {
                writer
                    .write(xml::writer::XmlEvent::end_element())
                    .map_err(|_e| "End element write failed".to_string())?;
            }

            Ok(())
        }
    }
}
