#![allow(clippy::derive_partial_eq_without_eq)]

use validate::Validate;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tup",
    namespace = "tup: http://www.onvif.org/ver10/uplink/wsdl"
)]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tup",
    namespace = "tup: http://www.onvif.org/ver10/uplink/wsdl"
)]
pub struct GetServiceCapabilitiesResponse {
    // The capabilities for the uplink service is returned in the Capabilities
    // element.
    #[yaserde(prefix = "tup", rename = "Capabilities")]
    pub capabilities: Capabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tup",
    namespace = "tup: http://www.onvif.org/ver10/uplink/wsdl"
)]
pub struct Capabilities {
    // Maximum number of uplink connections that can be configured.
    #[yaserde(attribute, rename = "MaxUplinks")]
    pub max_uplinks: Option<i32>,
}

impl Validate for Capabilities {}

// pub type Capabilities = Capabilities;
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum ConnectionStatus {
    Offline,
    Connecting,
    Connected,
    __Unknown__(String),
}

impl Default for ConnectionStatus {
    fn default() -> ConnectionStatus {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for ConnectionStatus {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tup",
    namespace = "tup: http://www.onvif.org/ver10/uplink/wsdl"
)]
pub struct Configuration {
    // Uniform resource locator by which the remote client can be reached.
    #[yaserde(prefix = "tup", rename = "RemoteAddress")]
    pub remote_address: String,

    // ID of the certificate to be used for client authentication.
    #[yaserde(prefix = "tup", rename = "CertificateID")]
    pub certificate_id: String,

    // Authorization level that will be assigned to the uplink connection.
    #[yaserde(prefix = "tup", rename = "UserLevel")]
    pub user_level: String,

    // Current connection status (see tup:ConnectionStatus for possible values).
    #[yaserde(prefix = "tup", rename = "Status")]
    pub status: Option<String>,
}

impl Validate for Configuration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tup",
    namespace = "tup: http://www.onvif.org/ver10/uplink/wsdl"
)]
pub struct GetUplinks {}

impl Validate for GetUplinks {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tup",
    namespace = "tup: http://www.onvif.org/ver10/uplink/wsdl"
)]
pub struct GetUplinksResponse {
    // List of configured uplinks.
    #[yaserde(prefix = "tup", rename = "Configuration")]
    pub configuration: Vec<Configuration>,
}

impl Validate for GetUplinksResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tup",
    namespace = "tup: http://www.onvif.org/ver10/uplink/wsdl"
)]
pub struct SetUplink {
    // SConfiguration to be added or modified.
    #[yaserde(prefix = "tup", rename = "Configuration")]
    pub configuration: Configuration,
}

impl Validate for SetUplink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tup",
    namespace = "tup: http://www.onvif.org/ver10/uplink/wsdl"
)]
pub struct SetUplinkResponse {}

impl Validate for SetUplinkResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tup",
    namespace = "tup: http://www.onvif.org/ver10/uplink/wsdl"
)]
pub struct DeleteUplink {
    // Uniform resource locator of the configuration to be deleted.
    #[yaserde(prefix = "tup", rename = "RemoteAddress")]
    pub remote_address: String,
}

impl Validate for DeleteUplink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tup",
    namespace = "tup: http://www.onvif.org/ver10/uplink/wsdl"
)]
pub struct DeleteUplinkResponse {}

impl Validate for DeleteUplinkResponse {}

// Returns the capabilities of the uplink service.
pub async fn get_service_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetServiceCapabilities,
) -> Result<GetServiceCapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// A device supporting uplinks shall support this command to retrieve the
// configured uplink configurations.
// The Status field shall signal whether a connection is Offline, Connecting or
// Online.
pub async fn get_uplinks<T: transport::Transport>(
    transport: &T,
    request: &GetUplinks,
) -> Result<GetUplinksResponse, transport::Error> {
    transport::request(transport, request).await
}

// A device supporting uplinks shall support this command to add or modify an
// uplink configuration.
// The Status property of the UplinkConfiguration shall be ignored by the
// device. A device shall
// use the field RemoteAddress to decide whether to update an existing entry or
// create a new entry.
pub async fn set_uplink<T: transport::Transport>(
    transport: &T,
    request: &SetUplink,
) -> Result<SetUplinkResponse, transport::Error> {
    transport::request(transport, request).await
}

// A device supporting uplinks shall support this command to remove an uplink
// configuration.
pub async fn delete_uplink<T: transport::Transport>(
    transport: &T,
    request: &DeleteUplink,
) -> Result<DeleteUplinkResponse, transport::Error> {
    transport::request(transport, request).await
}
