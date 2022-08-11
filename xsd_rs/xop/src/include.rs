use validate::Validate;
use yaserde_derive::{YaDeserialize, YaSerialize};

// pub type Include = Include;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2004/08/xop/include"
)]
pub struct Include {
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for Include {}
