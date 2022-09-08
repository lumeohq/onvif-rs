#![allow(clippy::derive_partial_eq_without_eq)]

use onvif as tt;
use validate::Validate;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tls",
    namespace = "tls: http://www.onvif.org/ver10/display/wsdl"
)]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tls",
    namespace = "tls: http://www.onvif.org/ver10/display/wsdl"
)]
pub struct GetServiceCapabilitiesResponse {
    // The capabilities for the display service is returned in the Capabilities
    // element.
    #[yaserde(prefix = "tls", rename = "Capabilities")]
    pub capabilities: Capabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tls",
    namespace = "tls: http://www.onvif.org/ver10/display/wsdl"
)]
pub struct Capabilities {
    // Indication that the SetLayout command supports only predefined layouts.
    #[yaserde(attribute, rename = "FixedLayout")]
    pub fixed_layout: Option<bool>,
}

impl Validate for Capabilities {}

// pub type Capabilities = Capabilities;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tls",
    namespace = "tls: http://www.onvif.org/ver10/display/wsdl"
)]
pub struct GetLayout {
    // Token of the Video Output whose Layout is requested
    #[yaserde(prefix = "tls", rename = "VideoOutput")]
    pub video_output: tt::ReferenceToken,
}

impl Validate for GetLayout {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tls",
    namespace = "tls: http://www.onvif.org/ver10/display/wsdl"
)]
pub struct GetLayoutResponse {
    // Current layout of the video output.
    #[yaserde(prefix = "tls", rename = "Layout")]
    pub layout: tt::Layout,
}

impl Validate for GetLayoutResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tls",
    namespace = "tls: http://www.onvif.org/ver10/display/wsdl"
)]
pub struct SetLayout {
    // Token of the Video Output whose Layout shall be changed.
    #[yaserde(prefix = "tls", rename = "VideoOutput")]
    pub video_output: tt::ReferenceToken,

    // Layout to be set
    #[yaserde(prefix = "tls", rename = "Layout")]
    pub layout: tt::Layout,
}

impl Validate for SetLayout {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tls",
    namespace = "tls: http://www.onvif.org/ver10/display/wsdl"
)]
pub struct SetLayoutResponse {}

impl Validate for SetLayoutResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tls",
    namespace = "tls: http://www.onvif.org/ver10/display/wsdl"
)]
pub struct GetDisplayOptions {
    // Token of the Video Output whose options are requested
    #[yaserde(prefix = "tls", rename = "VideoOutput")]
    pub video_output: tt::ReferenceToken,
}

impl Validate for GetDisplayOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tls",
    namespace = "tls: http://www.onvif.org/ver10/display/wsdl"
)]
pub struct GetDisplayOptionsResponse {
    // The LayoutOptions describe the fixed and predefined layouts of a device.
    // If the device does
    // not offer fixed layouts and allows setting the layout free this element
    // is empty.
    #[yaserde(prefix = "tls", rename = "LayoutOptions")]
    pub layout_options: Option<tt::LayoutOptions>,

    // decoding and encoding capabilities of the device
    #[yaserde(prefix = "tls", rename = "CodingCapabilities")]
    pub coding_capabilities: tt::CodingCapabilities,
}

impl Validate for GetDisplayOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tls",
    namespace = "tls: http://www.onvif.org/ver10/display/wsdl"
)]
pub struct GetPaneConfigurations {
    // Reference Token of the Video Output whose Pane Configurations are
    // requested
    #[yaserde(prefix = "tls", rename = "VideoOutput")]
    pub video_output: tt::ReferenceToken,
}

impl Validate for GetPaneConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tls",
    namespace = "tls: http://www.onvif.org/ver10/display/wsdl"
)]
pub struct GetPaneConfigurationsResponse {
    // Contains a list of defined Panes of the specified VideoOutput. Each
    // VideoOutput has at least one PaneConfiguration.
    #[yaserde(prefix = "tls", rename = "PaneConfiguration")]
    pub pane_configuration: Vec<tt::PaneConfiguration>,
}

impl Validate for GetPaneConfigurationsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tls",
    namespace = "tls: http://www.onvif.org/ver10/display/wsdl"
)]
pub struct GetPaneConfiguration {
    // Reference Token of the Video Output the requested pane belongs to
    #[yaserde(prefix = "tls", rename = "VideoOutput")]
    pub video_output: tt::ReferenceToken,

    // Reference Token of the Pane whose Configuration is requested
    #[yaserde(prefix = "tls", rename = "Pane")]
    pub pane: tt::ReferenceToken,
}

impl Validate for GetPaneConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tls",
    namespace = "tls: http://www.onvif.org/ver10/display/wsdl"
)]
pub struct GetPaneConfigurationResponse {
    // returns the configuration of the requested pane.
    #[yaserde(prefix = "tls", rename = "PaneConfiguration")]
    pub pane_configuration: tt::PaneConfiguration,
}

impl Validate for GetPaneConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tls",
    namespace = "tls: http://www.onvif.org/ver10/display/wsdl"
)]
pub struct SetPaneConfigurations {
    // Token of the video output whose panes to set.
    #[yaserde(prefix = "tls", rename = "VideoOutput")]
    pub video_output: tt::ReferenceToken,

    // Pane Configuration to be set.
    #[yaserde(prefix = "tls", rename = "PaneConfiguration")]
    pub pane_configuration: Vec<tt::PaneConfiguration>,
}

impl Validate for SetPaneConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tls",
    namespace = "tls: http://www.onvif.org/ver10/display/wsdl"
)]
pub struct SetPaneConfigurationsResponse {}

impl Validate for SetPaneConfigurationsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tls",
    namespace = "tls: http://www.onvif.org/ver10/display/wsdl"
)]
pub struct SetPaneConfiguration {
    // Token of the video output whose panes to set.
    #[yaserde(prefix = "tls", rename = "VideoOutput")]
    pub video_output: tt::ReferenceToken,

    // Pane Configuration to be set.
    #[yaserde(prefix = "tls", rename = "PaneConfiguration")]
    pub pane_configuration: tt::PaneConfiguration,
}

impl Validate for SetPaneConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tls",
    namespace = "tls: http://www.onvif.org/ver10/display/wsdl"
)]
pub struct SetPaneConfigurationResponse {}

impl Validate for SetPaneConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tls",
    namespace = "tls: http://www.onvif.org/ver10/display/wsdl"
)]
pub struct CreatePaneConfiguration {
    // Token of the video output where the pane shall be created.
    #[yaserde(prefix = "tls", rename = "VideoOutput")]
    pub video_output: tt::ReferenceToken,

    // Configuration of the pane to be created.
    #[yaserde(prefix = "tls", rename = "PaneConfiguration")]
    pub pane_configuration: tt::PaneConfiguration,
}

impl Validate for CreatePaneConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tls",
    namespace = "tls: http://www.onvif.org/ver10/display/wsdl"
)]
pub struct CreatePaneConfigurationResponse {
    // Token of the new pane configuration.
    #[yaserde(prefix = "tls", rename = "PaneToken")]
    pub pane_token: tt::ReferenceToken,
}

impl Validate for CreatePaneConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tls",
    namespace = "tls: http://www.onvif.org/ver10/display/wsdl"
)]
pub struct DeletePaneConfiguration {
    // Token of the video output where the pane shall be deleted.
    #[yaserde(prefix = "tls", rename = "VideoOutput")]
    pub video_output: tt::ReferenceToken,

    // Token of the pane to be deleted.
    #[yaserde(prefix = "tls", rename = "PaneToken")]
    pub pane_token: tt::ReferenceToken,
}

impl Validate for DeletePaneConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tls",
    namespace = "tls: http://www.onvif.org/ver10/display/wsdl"
)]
pub struct DeletePaneConfigurationResponse {}

impl Validate for DeletePaneConfigurationResponse {}

// Returns the capabilities of the display service. The result is returned in a
// typed answer.
pub async fn get_service_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetServiceCapabilities,
) -> Result<GetServiceCapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// Return the current layout of a video output. The Layout assigns a pane
// configuration to a certain area of the display. The layout settings
// directly affect a specific video output. The layout consists of a list of
// PaneConfigurations and
// their associated display areas.
pub async fn get_layout<T: transport::Transport>(
    transport: &T,
    request: &GetLayout,
) -> Result<GetLayoutResponse, transport::Error> {
    transport::request(transport, request).await
}

// Change the layout of a display (e.g. change from
// single view to split screen view).The Layout assigns a pane configuration to
// a certain area of the display. The layout settings
// directly affect a specific video output. The layout consists of a list of
// PaneConfigurations and
// their associated display areas.
pub async fn set_layout<T: transport::Transport>(
    transport: &T,
    request: &SetLayout,
) -> Result<SetLayoutResponse, transport::Error> {
    transport::request(transport, request).await
}

// The Display Options contain the supported layouts (LayoutOptions) and the
// decoding and
// encoding capabilities (CodingCapabilities) of the device. The
// GetDisplayOptions command
// returns both, Layout and Coding Capabilities, of a VideoOutput.
pub async fn get_display_options<T: transport::Transport>(
    transport: &T,
    request: &GetDisplayOptions,
) -> Result<GetDisplayOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// List all currently defined panes of a device for a specified video output
// (regardless if this pane is visible at a moment). A Pane is a display area on
// the monitor that is attached to a video output. A pane has a
// PaneConfiguration that describes which entities are associated with the pane.
// A client has to configure the pane according to the connection to be
// established by setting the
// AudioOutput and/or AudioSourceToken. If a Token is not set, the corresponding
// session will
// not be established.
pub async fn get_pane_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetPaneConfigurations,
) -> Result<GetPaneConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// Retrieve the pane configuration for a pane token.
pub async fn get_pane_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetPaneConfiguration,
) -> Result<GetPaneConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// Modify one or more configurations of the specified video output.
// This method will only modify the provided configurations and leave the others
// unchanged.
// Use
pub async fn set_pane_configurations<T: transport::Transport>(
    transport: &T,
    request: &SetPaneConfigurations,
) -> Result<SetPaneConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This command changes the configuration of the specified pane (tbd)
pub async fn set_pane_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetPaneConfiguration,
) -> Result<SetPaneConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// Create a new pane configuration describing the streaming and coding settings
// for a display area.
pub async fn create_pane_configuration<T: transport::Transport>(
    transport: &T,
    request: &CreatePaneConfiguration,
) -> Result<CreatePaneConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// Delete a pane configuration. A service must respond with an error if the pane
// configuration
// is in use by the current layout.
pub async fn delete_pane_configuration<T: transport::Transport>(
    transport: &T,
    request: &DeletePaneConfiguration,
) -> Result<DeletePaneConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}
