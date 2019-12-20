use std::io::Read;
use yaserde::YaDeserialize;

#[derive(Default, PartialEq, Debug, YaDeserialize)]
#[yaserde(prefix = "s", namespace = "s: http://www.w3.org/2003/05/soap-envelope")]
pub struct Fault {
    #[yaserde(prefix = "s", rename = "Code")]
    pub code: Code,

    #[yaserde(prefix = "s", rename = "Reason")]
    pub reason: Reason,

    #[yaserde(prefix = "s", rename = "Node")]
    pub node: Option<String>,

    #[yaserde(prefix = "s", rename = "Role")]
    pub role: Option<String>,
    //  TODO: Detail is xs:any namespace="##any"
    //    #[yaserde(prefix = "s", rename = "Detail")]
    //    pub detail: Option<Detail>,
}

#[derive(Default, PartialEq, Debug, YaDeserialize)]
#[yaserde(prefix = "s", namespace = "s: http://www.w3.org/2003/05/soap-envelope")]
pub struct Code {
    #[yaserde(prefix = "s", rename = "Value")]
    pub value: String,

    #[yaserde(prefix = "s", rename = "Subcode")]
    pub subcode: Option<Subcode>,
}

#[derive(Default, PartialEq, Debug, YaDeserialize)]
#[yaserde(prefix = "s", namespace = "s: http://www.w3.org/2003/05/soap-envelope")]
pub struct Subcode {
    #[yaserde(prefix = "s", rename = "Value")]
    pub value: String,
    //  TODO: handle recursion
    //    #[yaserde(prefix = "s", rename = "Subcode")]
    //    pub subcode: Option<Subcode>,
}

#[derive(Default, PartialEq, Debug, YaDeserialize)]
#[yaserde(prefix = "s", namespace = "s: http://www.w3.org/2003/05/soap-envelope")]
pub struct Reason {
    #[yaserde(prefix = "s", rename = "Text")]
    pub text: Vec<String>,
}
