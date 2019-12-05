pub mod get_system_date_and_time_request {
    use std::io::Write;
    use yaserde::YaSerialize;

    #[derive(Default, PartialEq, Debug, YaSerialize)]
    #[yaserde(
    root = "Envelope",
    prefix = "s",
    namespace = "s: http://www.w3.org/2003/05/soap-envelope",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl",
    namespace = "tt: http://www.onvif.org/ver10/schema",
    )]
    pub struct Envelope {
        #[yaserde(prefix = "s", rename = "Body")]
        body: Body,
    }

    #[derive(Default, PartialEq, Debug, YaSerialize)]
    #[yaserde(
    prefix = "s",
    namespace = "s: http://www.w3.org/2003/05/soap-envelope",
    )]
    pub struct Body {
        #[yaserde(prefix = "tds", rename = "GetSystemDateAndTime")]
        get_system_date_and_time: GetSystemDateAndTime,
    }

    #[derive(Default, PartialEq, Debug, YaSerialize)]
    #[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
    )]
    pub struct GetSystemDateAndTime {
    }
}


pub mod get_system_date_and_time_response {
    use std::io::Read;
    use yaserde::YaDeserialize;
    use crate::schema::onvif as tt;

    #[derive(Default, PartialEq, Debug, YaDeserialize)]
    #[yaserde(
    root = "Envelope",
    prefix = "s",
    namespace = "s: http://www.w3.org/2003/05/soap-envelope"
    )]
    pub struct Envelope {
        #[yaserde(rename = "Body")]
        pub body: Body,
    }

    #[derive(Default, PartialEq, Debug, YaDeserialize)]
    #[yaserde(
    prefix = "s",
    namespace = "s: http://www.w3.org/2003/05/soap-envelope"
    )]
    pub struct Body {
        #[yaserde(rename = "GetSystemDateAndTimeResponse")]
        pub get_system_date_and_time_response: GetSystemDateAndTimeResponse,
    }

    #[derive(Default, PartialEq, Debug, YaDeserialize)]
    #[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
    )]
    pub struct GetSystemDateAndTimeResponse {
        #[yaserde(rename = "SystemDateAndTime")]
        pub system_date_and_time: tt::SystemDateAndTime,
    }
}
