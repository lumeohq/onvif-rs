#![allow(clippy::derive_partial_eq_without_eq)]

use onvif as tt;
use validate::Validate;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trv",
    namespace = "trv: http://www.onvif.org/ver10/receiver/wsdl"
)]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trv",
    namespace = "trv: http://www.onvif.org/ver10/receiver/wsdl"
)]
pub struct GetServiceCapabilitiesResponse {
    // The capabilities for the receiver service is returned in the Capabilities
    // element.
    #[yaserde(prefix = "trv", rename = "Capabilities")]
    pub capabilities: Capabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trv",
    namespace = "trv: http://www.onvif.org/ver10/receiver/wsdl"
)]
pub struct Capabilities {
    // Indicates that the device can receive RTP multicast streams.
    #[yaserde(attribute, rename = "RTP_Multicast")]
    pub rtp_multicast: Option<bool>,

    // Indicates that the device can receive RTP/TCP streams
    #[yaserde(attribute, rename = "RTP_TCP")]
    pub rtp_tcp: Option<bool>,

    // Indicates that the device can receive RTP/RTSP/TCP streams.
    #[yaserde(attribute, rename = "RTP_RTSP_TCP")]
    pub rtp_rtsp_tcp: Option<bool>,

    // The maximum number of receivers supported by the device.
    #[yaserde(attribute, rename = "SupportedReceivers")]
    pub supported_receivers: i32,

    // The maximum allowed length for RTSP URIs (Minimum and default value is
    // 128 octet).
    #[yaserde(attribute, rename = "MaximumRTSPURILength")]
    pub maximum_rtspuri_length: Option<i32>,
}

impl Validate for Capabilities {}

// pub type Capabilities = Capabilities;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trv",
    namespace = "trv: http://www.onvif.org/ver10/receiver/wsdl"
)]
pub struct GetReceivers {}

impl Validate for GetReceivers {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trv",
    namespace = "trv: http://www.onvif.org/ver10/receiver/wsdl"
)]
pub struct GetReceiversResponse {
    // A list of all receivers that currently exist on the device.
    #[yaserde(prefix = "trv", rename = "Receivers")]
    pub receivers: Vec<tt::Receiver>,
}

impl Validate for GetReceiversResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trv",
    namespace = "trv: http://www.onvif.org/ver10/receiver/wsdl"
)]
pub struct GetReceiver {
    // The token of the receiver to be retrieved.
    #[yaserde(prefix = "trv", rename = "ReceiverToken")]
    pub receiver_token: tt::ReferenceToken,
}

impl Validate for GetReceiver {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trv",
    namespace = "trv: http://www.onvif.org/ver10/receiver/wsdl"
)]
pub struct GetReceiverResponse {
    // The details of the receiver.
    #[yaserde(prefix = "trv", rename = "Receiver")]
    pub receiver: tt::Receiver,
}

impl Validate for GetReceiverResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trv",
    namespace = "trv: http://www.onvif.org/ver10/receiver/wsdl"
)]
pub struct CreateReceiver {
    // The initial configuration for the new receiver.
    #[yaserde(prefix = "trv", rename = "Configuration")]
    pub configuration: tt::ReceiverConfiguration,
}

impl Validate for CreateReceiver {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trv",
    namespace = "trv: http://www.onvif.org/ver10/receiver/wsdl"
)]
pub struct CreateReceiverResponse {
    // The details of the receiver that was created.
    #[yaserde(prefix = "trv", rename = "Receiver")]
    pub receiver: tt::Receiver,
}

impl Validate for CreateReceiverResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trv",
    namespace = "trv: http://www.onvif.org/ver10/receiver/wsdl"
)]
pub struct DeleteReceiver {
    // The token of the receiver to be deleted.
    #[yaserde(prefix = "trv", rename = "ReceiverToken")]
    pub receiver_token: tt::ReferenceToken,
}

impl Validate for DeleteReceiver {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trv",
    namespace = "trv: http://www.onvif.org/ver10/receiver/wsdl"
)]
pub struct DeleteReceiverResponse {}

impl Validate for DeleteReceiverResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trv",
    namespace = "trv: http://www.onvif.org/ver10/receiver/wsdl"
)]
pub struct ConfigureReceiver {
    // The token of the receiver to be configured.
    #[yaserde(prefix = "trv", rename = "ReceiverToken")]
    pub receiver_token: tt::ReferenceToken,

    // The new configuration for the receiver.
    #[yaserde(prefix = "trv", rename = "Configuration")]
    pub configuration: tt::ReceiverConfiguration,
}

impl Validate for ConfigureReceiver {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trv",
    namespace = "trv: http://www.onvif.org/ver10/receiver/wsdl"
)]
pub struct ConfigureReceiverResponse {}

impl Validate for ConfigureReceiverResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trv",
    namespace = "trv: http://www.onvif.org/ver10/receiver/wsdl"
)]
pub struct SetReceiverMode {
    // The token of the receiver to be changed.
    #[yaserde(prefix = "trv", rename = "ReceiverToken")]
    pub receiver_token: tt::ReferenceToken,

    // The new receiver mode. Options available are:
    #[yaserde(prefix = "trv", rename = "Mode")]
    pub mode: tt::ReceiverMode,
}

impl Validate for SetReceiverMode {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trv",
    namespace = "trv: http://www.onvif.org/ver10/receiver/wsdl"
)]
pub struct SetReceiverModeResponse {}

impl Validate for SetReceiverModeResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trv",
    namespace = "trv: http://www.onvif.org/ver10/receiver/wsdl"
)]
pub struct GetReceiverState {
    // The token of the receiver to be queried.
    #[yaserde(prefix = "trv", rename = "ReceiverToken")]
    pub receiver_token: tt::ReferenceToken,
}

impl Validate for GetReceiverState {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trv",
    namespace = "trv: http://www.onvif.org/ver10/receiver/wsdl"
)]
pub struct GetReceiverStateResponse {
    // Description of the current receiver state.
    #[yaserde(prefix = "trv", rename = "ReceiverState")]
    pub receiver_state: tt::ReceiverStateInformation,
}

impl Validate for GetReceiverStateResponse {}

// Returns the capabilities of the receiver service. The result is returned in a
// typed answer.
pub async fn get_service_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetServiceCapabilities,
) -> Result<GetServiceCapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// Lists all receivers currently present on a device. This operation is
// mandatory.
pub async fn get_receivers<T: transport::Transport>(
    transport: &T,
    request: &GetReceivers,
) -> Result<GetReceiversResponse, transport::Error> {
    transport::request(transport, request).await
}

// Retrieves the details of a specific receiver. This operation is mandatory.
pub async fn get_receiver<T: transport::Transport>(
    transport: &T,
    request: &GetReceiver,
) -> Result<GetReceiverResponse, transport::Error> {
    transport::request(transport, request).await
}

// Creates a new receiver. This operation is mandatory, although the service may
// raise a fault if the receiver cannot be created.
pub async fn create_receiver<T: transport::Transport>(
    transport: &T,
    request: &CreateReceiver,
) -> Result<CreateReceiverResponse, transport::Error> {
    transport::request(transport, request).await
}

// Deletes an existing receiver. A receiver may be deleted only if it is not
// currently in use; otherwise a fault shall be raised.
// This operation is mandatory.
pub async fn delete_receiver<T: transport::Transport>(
    transport: &T,
    request: &DeleteReceiver,
) -> Result<DeleteReceiverResponse, transport::Error> {
    transport::request(transport, request).await
}

// Configures an existing receiver. This operation is mandatory.
pub async fn configure_receiver<T: transport::Transport>(
    transport: &T,
    request: &ConfigureReceiver,
) -> Result<ConfigureReceiverResponse, transport::Error> {
    transport::request(transport, request).await
}

// Sets the mode of the receiver without affecting the rest of its
// configuration.
// This operation is mandatory.
pub async fn set_receiver_mode<T: transport::Transport>(
    transport: &T,
    request: &SetReceiverMode,
) -> Result<SetReceiverModeResponse, transport::Error> {
    transport::request(transport, request).await
}

// Determines whether the receiver is currently disconnected, connected or
// attempting to connect.
// This operation is mandatory.
pub async fn get_receiver_state<T: transport::Transport>(
    transport: &T,
    request: &GetReceiverState,
) -> Result<GetReceiverStateResponse, transport::Error> {
    transport::request(transport, request).await
}
