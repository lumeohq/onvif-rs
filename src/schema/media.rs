// Based on media.wsdl.xml

// targetNamespace="http://www.onvif.org/ver10/media/wsdl"

// xmlns:tt="http://www.onvif.org/ver10/schema"
// xmlns:xs="http://www.w3.org/2001/XMLSchema"

// <xs:import namespace="http://www.onvif.org/ver10/schema" schemaLocation="../../../ver10/schema/onvif.xsd"/>

use crate::schema::onvif as tt;
use crate::transport;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

#[derive(Default, PartialEq, Debug, YaSerialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetProfiles {}

#[derive(Default, PartialEq, Debug, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetProfilesResponse {
    #[yaserde(prefix = "trt", rename = "Profiles")]
    pub profiles: Vec<tt::Profile>,
}

pub async fn get_profiles<T: transport::Transport>(
    transport: &T,
    request: &GetProfiles,
) -> Result<GetProfilesResponse, transport::Error> {
    transport::request(transport, request).await
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetStreamUri {
    #[yaserde(prefix = "trt", rename = "StreamSetup")]
    pub stream_setup: tt::StreamSetup,

    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

#[derive(Default, PartialEq, Debug, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetStreamUriResponse {
    #[yaserde(prefix = "trt", rename = "MediaUri")]
    pub media_uri: tt::MediaUri,
}

pub async fn get_stream_uri<T: transport::Transport>(
    transport: &T,
    request: &GetStreamUri,
) -> Result<GetStreamUriResponse, transport::Error> {
    transport::request(transport, request).await
}
