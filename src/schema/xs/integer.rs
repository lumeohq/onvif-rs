use crate::utils;
use num_bigint::{BigInt, ToBigInt};
use std::io::{Read, Write};
use std::str::FromStr;
use yaserde::{YaDeserialize, YaSerialize};

#[derive(Default, PartialEq, Debug)]
pub struct Integer {
    pub value: BigInt,
}

impl Integer {
    pub fn from_bigint(bigint: BigInt) -> Self {
        Integer { value: bigint }
    }
}

impl ToBigInt for Integer {
    fn to_bigint(&self) -> Option<BigInt> {
        Some(self.value.clone())
    }
}

impl YaDeserialize for Integer {
    fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        utils::yaserde::deserialize(reader, |s| {
            BigInt::from_str(s)
                .map(Integer::from_bigint)
                .map_err(|e| e.to_string())
        })
    }
}

impl YaSerialize for Integer {
    fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>) -> Result<(), String> {
        utils::yaserde::serialize(self, "Integer", writer, |s| Ok(s.value.to_str_radix(10)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::xml_eq::assert_xml_eq;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "t", namespace = "t: test")]
    pub struct IntegerPair {
        #[yaserde(prefix = "t", rename = "First")]
        pub first: Integer,

        #[yaserde(prefix = "t", rename = "Second")]
        pub second: Integer,
    }

    #[test]
    fn integer_serialize_test() {
        let expected = r#"
            <?xml version="1.0" encoding="utf-8"?>
            <t:IntegerPair xmlns:t="test">
                <t:First>1234</t:First>
                <t:Second>-1234</t:Second>
            </t:IntegerPair>
            "#;
        let i = IntegerPair {
            first: Integer::from_bigint(1234.to_bigint().unwrap()),
            second: Integer::from_bigint(-1234.to_bigint().unwrap()),
        };
        let actual = yaserde::ser::to_string(&i).unwrap();
        assert_xml_eq(&actual, expected);
    }

    #[test]
    fn integer_deserialize_test() {
        // Value "+1234" is used to check optional plus sign deserialization.
        let s = r#"
            <?xml version="1.0" encoding="utf-8"?>
            <t:IntegerPair xmlns:t="test">
                <t:First>+1234</t:First>
                <t:Second>-1234</t:Second>
            </t:IntegerPair>
            "#;
        let i: IntegerPair = yaserde::de::from_str(&s).unwrap();
        assert_eq!(i.first.to_bigint().unwrap(), 1234.to_bigint().unwrap());
        assert_eq!(i.second.to_bigint().unwrap(), -1234.to_bigint().unwrap());
    }
}
