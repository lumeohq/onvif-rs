use crate::utils;
use bigdecimal::BigDecimal;
use std::io::{Read, Write};
use std::str::FromStr;
use yaserde::{YaDeserialize, YaSerialize};

#[derive(Default, PartialEq, Debug)]
pub struct Decimal {
    pub value: BigDecimal,
}

impl Decimal {
    pub fn from_bigdecimal(bigdecimal: BigDecimal) -> Self {
        Decimal { value: bigdecimal }
    }

    pub fn to_bigdecimal(&self) -> BigDecimal {
        self.value.clone()
    }
}

impl YaDeserialize for Decimal {
    fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        utils::yaserde::deserialize(reader, |s| {
            BigDecimal::from_str(s)
                .map(Decimal::from_bigdecimal)
                .map_err(|e| e.to_string())
        })
    }
}

impl YaSerialize for Decimal {
    fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>) -> Result<(), String> {
        utils::yaserde::serialize(self, "Decimal", writer, |s| Ok(s.value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::xml_eq::assert_xml_eq;
    use num_bigint::ToBigInt;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "t", namespace = "t: test")]
    pub struct DecimalPair {
        #[yaserde(prefix = "t", rename = "First")]
        pub first: Decimal,

        #[yaserde(prefix = "t", rename = "Second")]
        pub second: Decimal,
    }

    #[test]
    fn integer_serialize_test() {
        let expected = r#"
            <?xml version="1.0" encoding="utf-8"?>
            <t:DecimalPair xmlns:t="test">
                <t:First>0.01234</t:First>
                <t:Second>-12.34</t:Second>
            </t:DecimalPair>
            "#;
        let i = DecimalPair {
            first: Decimal::from_bigdecimal(BigDecimal::new(1234.to_bigint().unwrap(), 5)),
            second: Decimal::from_bigdecimal(BigDecimal::new(-1234.to_bigint().unwrap(), 2)),
        };
        let actual = yaserde::ser::to_string(&i).unwrap();
        assert_xml_eq(&actual, expected);
    }

    #[test]
    fn integer_deserialize_test() {
        // Value "+0.01234" is used to check optional plus sign deserialization.
        let s = r#"
            <?xml version="1.0" encoding="utf-8"?>
            <t:DecimalPair xmlns:t="test">
                <t:First>+0.01234</t:First>
                <t:Second>-12.34</t:Second>
            </t:DecimalPair>
            "#;
        let i: DecimalPair = yaserde::de::from_str(&s).unwrap();
        assert_eq!(
            i.first.to_bigdecimal(),
            BigDecimal::new(1234.to_bigint().unwrap(), 5)
        );
        assert_eq!(
            i.second.to_bigdecimal(),
            BigDecimal::new(-1234.to_bigint().unwrap(), 2)
        );
    }
}
