use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

//generated file
// type Include = Include;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://www.w3.org/2004/08/xop/include"
)]
pub struct Include {
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,
    #[yaserde(attribute, rename = "href")]
    pub href: String,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}
