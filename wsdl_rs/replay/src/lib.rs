#![allow(clippy::derive_partial_eq_without_eq)]

use onvif as tt;
use validate::Validate;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trp",
    namespace = "trp: http://www.onvif.org/ver10/replay/wsdl"
)]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trp",
    namespace = "trp: http://www.onvif.org/ver10/replay/wsdl"
)]
pub struct GetServiceCapabilitiesResponse {
    // The capabilities for the replay service is returned in the Capabilities
    // element.
    #[yaserde(prefix = "trp", rename = "Capabilities")]
    pub capabilities: Capabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trp",
    namespace = "trp: http://www.onvif.org/ver10/replay/wsdl"
)]
pub struct Capabilities {
    // Indicator that the Device supports reverse playback as defined in the
    // ONVIF Streaming Specification.
    #[yaserde(attribute, rename = "ReversePlayback")]
    pub reverse_playback: Option<bool>,

    // The list contains two elements defining the minimum and maximum valid
    // values supported as session timeout in seconds.
    #[yaserde(attribute, rename = "SessionTimeoutRange")]
    pub session_timeout_range: Option<tt::FloatAttrList>,

    // Indicates support for RTP/RTSP/TCP.
    #[yaserde(attribute, rename = "RTP_RTSP_TCP")]
    pub rtp_rtsp_tcp: Option<bool>,

    // If playback streaming over WebSocket is supported, this shall return the
    // RTSP WebSocket URI as described in Streaming Specification Section
    // 5.1.1.5.
    #[yaserde(attribute, rename = "RTSPWebSocketUri")]
    pub rtsp_web_socket_uri: Option<String>,
}

impl Validate for Capabilities {}

// pub type Capabilities = Capabilities;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trp",
    namespace = "trp: http://www.onvif.org/ver10/replay/wsdl"
)]
pub struct GetReplayUri {
    // Specifies the connection parameters to be used for the stream. The URI
    // that is returned may depend on these parameters.
    #[yaserde(prefix = "trp", rename = "StreamSetup")]
    pub stream_setup: tt::StreamSetup,

    // The identifier of the recording to be streamed.
    #[yaserde(prefix = "trp", rename = "RecordingToken")]
    pub recording_token: tt::ReferenceToken,
}

impl Validate for GetReplayUri {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trp",
    namespace = "trp: http://www.onvif.org/ver10/replay/wsdl"
)]
pub struct GetReplayUriResponse {
    // The URI to which the client should connect in order to stream the
    // recording.
    #[yaserde(prefix = "trp", rename = "Uri")]
    pub uri: String,
}

impl Validate for GetReplayUriResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trp",
    namespace = "trp: http://www.onvif.org/ver10/replay/wsdl"
)]
pub struct SetReplayConfiguration {
    // Description of the new replay configuration parameters.
    #[yaserde(prefix = "trp", rename = "Configuration")]
    pub configuration: tt::ReplayConfiguration,
}

impl Validate for SetReplayConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trp",
    namespace = "trp: http://www.onvif.org/ver10/replay/wsdl"
)]
pub struct SetReplayConfigurationResponse {}

impl Validate for SetReplayConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trp",
    namespace = "trp: http://www.onvif.org/ver10/replay/wsdl"
)]
pub struct GetReplayConfiguration {}

impl Validate for GetReplayConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trp",
    namespace = "trp: http://www.onvif.org/ver10/replay/wsdl"
)]
pub struct GetReplayConfigurationResponse {
    // The current replay configuration parameters.
    #[yaserde(prefix = "trp", rename = "Configuration")]
    pub configuration: tt::ReplayConfiguration,
}

impl Validate for GetReplayConfigurationResponse {}

// Returns the capabilities of the replay service. The result is returned in a
// typed answer.
pub async fn get_service_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetServiceCapabilities,
) -> Result<GetServiceCapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// Requests a URI that can be used to initiate playback of a recorded stream
// using RTSP as the control protocol. The URI is valid only as it is
// specified in the response.
// This operation is mandatory.
pub async fn get_replay_uri<T: transport::Transport>(
    transport: &T,
    request: &GetReplayUri,
) -> Result<GetReplayUriResponse, transport::Error> {
    transport::request(transport, request).await
}

// Returns the current configuration of the replay service.
// This operation is mandatory.
pub async fn get_replay_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetReplayConfiguration,
) -> Result<GetReplayConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// Changes the current configuration of the replay service.
// This operation is mandatory.
pub async fn set_replay_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetReplayConfiguration,
) -> Result<SetReplayConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}
