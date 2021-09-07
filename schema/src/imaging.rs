use crate::onvif as tt;
use crate::transport;
use crate::validate::Validate;
use std::str::FromStr;
use xsd_macro_utils::*;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "timg",
    namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl"
)]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "timg",
    namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl"
)]
pub struct GetServiceCapabilitiesResponse {
    // The capabilities for the imaging service is returned in the Capabilities
    // element.
    #[yaserde(prefix = "timg", rename = "Capabilities")]
    pub capabilities: Capabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "timg",
    namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl"
)]
pub struct Capabilities {
    // Indicates whether or not Image Stabilization feature is supported.
    // The use of this capability is deprecated, a client should use GetOption
    // to find out if image stabilization is supported.
    #[yaserde(attribute, rename = "ImageStabilization")]
    pub image_stabilization: Option<bool>,

    // Indicates whether or not Imaging Presets feature is supported.
    #[yaserde(attribute, rename = "Presets")]
    pub presets: Option<bool>,

    // Indicates whether or not imaging preset settings can be updated.
    #[yaserde(attribute, rename = "AdaptablePreset")]
    pub adaptable_preset: Option<bool>,
}

impl Validate for Capabilities {}

// pub type Capabilities = Capabilities;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "timg",
    namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl"
)]
pub struct GetImagingSettings {
    // Reference token to the VideoSource for which the ImagingSettings.
    #[yaserde(prefix = "timg", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,
}

impl Validate for GetImagingSettings {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "timg",
    namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl"
)]
pub struct GetImagingSettingsResponse {
    // ImagingSettings for the VideoSource that was requested.
    #[yaserde(prefix = "timg", rename = "ImagingSettings")]
    pub imaging_settings: tt::ImagingSettings20,
}

impl Validate for GetImagingSettingsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "timg",
    namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl"
)]
pub struct SetImagingSettings {
    #[yaserde(prefix = "timg", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,

    #[yaserde(prefix = "timg", rename = "ImagingSettings")]
    pub imaging_settings: tt::ImagingSettings20,

    #[yaserde(prefix = "timg", rename = "ForcePersistence")]
    pub force_persistence: bool,
}

impl Validate for SetImagingSettings {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "timg",
    namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl"
)]
pub struct SetImagingSettingsResponse {}

impl Validate for SetImagingSettingsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "timg",
    namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl"
)]
pub struct GetOptions {
    // Reference token to the VideoSource for which the imaging parameter
    // options are requested.
    #[yaserde(prefix = "timg", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,
}

impl Validate for GetOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "timg",
    namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl"
)]
pub struct GetOptionsResponse {
    // Valid ranges for the imaging parameters that are categorized as device
    // specific.
    #[yaserde(prefix = "timg", rename = "ImagingOptions")]
    pub imaging_options: tt::ImagingOptions20,
}

impl Validate for GetOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "timg",
    namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl"
)]
pub struct Move {
    // Reference to the VideoSource for the requested move (focus) operation.
    #[yaserde(prefix = "timg", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,

    // Content of the requested move (focus) operation.
    #[yaserde(prefix = "timg", rename = "Focus")]
    pub focus: Vec<tt::FocusMove>,
}

impl Validate for Move {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "timg",
    namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl"
)]
pub struct MoveResponse {}

impl Validate for MoveResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "timg",
    namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl"
)]
pub struct GetMoveOptions {
    // Reference token to the VideoSource for the requested move options.
    #[yaserde(prefix = "timg", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,
}

impl Validate for GetMoveOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "timg",
    namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl"
)]
pub struct GetMoveOptionsResponse {
    // Valid ranges for the focus lens move options.
    #[yaserde(prefix = "timg", rename = "MoveOptions")]
    pub move_options: tt::MoveOptions20,
}

impl Validate for GetMoveOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "timg",
    namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl"
)]
pub struct Stop {
    // Reference token to the VideoSource where the focus movement should be
    // stopped.
    #[yaserde(prefix = "timg", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,
}

impl Validate for Stop {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "timg",
    namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl"
)]
pub struct StopResponse {}

impl Validate for StopResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "timg",
    namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl"
)]
pub struct GetStatus {
    // Reference token to the VideoSource where the imaging status should be
    // requested.
    #[yaserde(prefix = "timg", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,
}

impl Validate for GetStatus {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "timg",
    namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl"
)]
pub struct GetStatusResponse {
    // Requested imaging status.
    #[yaserde(prefix = "timg", rename = "Status")]
    pub status: tt::ImagingStatus20,
}

impl Validate for GetStatusResponse {}

// Describes standard Imaging Preset types, used to facilitate Multi-language
// support and client display.
// "Custom" Type shall be used when Imaging Preset Name does not match any of
// the types included in the standard classification.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct ImagingPresetType(pub String);

impl Validate for ImagingPresetType {}
// Type describing the Imaging Preset settings.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "timg",
    namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl"
)]
pub struct ImagingPreset {
    // Human readable name of the Imaging Preset.
    #[yaserde(prefix = "timg", rename = "Name")]
    pub name: tt::Name,

    // Unique identifier of this Imaging Preset.
    #[yaserde(attribute, rename = "token")]
    pub token: tt::ReferenceToken,

    // Indicates Imaging Preset Type. Use timg:ImagingPresetType.
    // Used for multi-language support and display.
    #[yaserde(attribute, rename = "type")]
    pub _type: String,
}

impl Validate for ImagingPreset {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "timg",
    namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl"
)]
pub struct GetPresets {
    // A reference to the VideoSource where the operation should take place.
    #[yaserde(prefix = "timg", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,
}

impl Validate for GetPresets {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "timg",
    namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl"
)]
pub struct GetPresetsResponse {
    // List of Imaging Presets which are available for the requested
    // VideoSource.
    #[yaserde(prefix = "timg", rename = "Preset")]
    pub preset: Vec<ImagingPreset>,
}

impl Validate for GetPresetsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "timg",
    namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl"
)]
pub struct GetCurrentPreset {
    // Reference token to the VideoSource where the current Imaging Preset
    // should be requested.
    #[yaserde(prefix = "timg", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,
}

impl Validate for GetCurrentPreset {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "timg",
    namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl"
)]
pub struct GetCurrentPresetResponse {
    // Current Imaging Preset in use for the specified Video Source.
    #[yaserde(prefix = "timg", rename = "Preset")]
    pub preset: Option<ImagingPreset>,
}

impl Validate for GetCurrentPresetResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "timg",
    namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl"
)]
pub struct SetCurrentPreset {
    // Reference token to the VideoSource to which the specified Imaging Preset
    // should be applied.
    #[yaserde(prefix = "timg", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,

    // Reference token to the Imaging Preset to be applied to the specified
    // Video Source.
    #[yaserde(prefix = "timg", rename = "PresetToken")]
    pub preset_token: tt::ReferenceToken,
}

impl Validate for SetCurrentPreset {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "timg",
    namespace = "timg: http://www.onvif.org/ver20/imaging/wsdl"
)]
pub struct SetCurrentPresetResponse {}

impl Validate for SetCurrentPresetResponse {}

// Returns the capabilities of the imaging service. The result is returned in a
// typed answer.
pub async fn get_service_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetServiceCapabilities,
) -> Result<GetServiceCapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// Get the ImagingConfiguration for the requested VideoSource.
pub async fn get_imaging_settings<T: transport::Transport>(
    transport: &T,
    request: &GetImagingSettings,
) -> Result<GetImagingSettingsResponse, transport::Error> {
    transport::request(transport, request).await
}

// Set the ImagingConfiguration for the requested VideoSource.
pub async fn set_imaging_settings<T: transport::Transport>(
    transport: &T,
    request: &SetImagingSettings,
) -> Result<SetImagingSettingsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation gets the valid ranges for the imaging parameters that have
// device specific ranges.
// This command is mandatory for all device implementing the imaging service.
// The command returns all supported parameters and their ranges
// such that these can be applied to the SetImagingSettings command.
pub async fn get_options<T: transport::Transport>(
    transport: &T,
    request: &GetOptions,
) -> Result<GetOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// The Move command moves the focus lens in an absolute, a relative or in a
// continuous manner from its current position.
// The speed argument is optional for absolute and relative control, but
// required for continuous. If no speed argument is used, the default speed is
// used.
// Focus adjustments through this operation will turn off the autofocus. A
// device with support for remote focus control should support absolute,
// relative or continuous control through the Move operation. The supported
// MoveOpions are signalled via the GetMoveOptions command.
// At least one focus control capability is required for this operation to be
// functional.
pub async fn _move<T: transport::Transport>(
    transport: &T,
    request: &Move,
) -> Result<MoveResponse, transport::Error> {
    transport::request(transport, request).await
}

// Imaging move operation options supported for the Video source.
pub async fn get_move_options<T: transport::Transport>(
    transport: &T,
    request: &GetMoveOptions,
) -> Result<GetMoveOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// The Stop command stops all ongoing focus movements of the lense. A device
// with support for remote focus control as signalled via
// the GetMoveOptions supports this command.
pub async fn stop<T: transport::Transport>(
    transport: &T,
    request: &Stop,
) -> Result<StopResponse, transport::Error> {
    transport::request(transport, request).await
}

// Via this command the current status of the Move operation can be requested.
// Supported for this command is available if the support for the Move operation
// is signalled via GetMoveOptions.
pub async fn get_status<T: transport::Transport>(
    transport: &T,
    request: &GetStatus,
) -> Result<GetStatusResponse, transport::Error> {
    transport::request(transport, request).await
}

// Via this command the list of available Imaging Presets can be requested.
pub async fn get_presets<T: transport::Transport>(
    transport: &T,
    request: &GetPresets,
) -> Result<GetPresetsResponse, transport::Error> {
    transport::request(transport, request).await
}

// Via this command the last Imaging Preset applied can be requested.
// If the camera configuration does not match any of the existing Imaging
// Presets, the output of GetCurrentPreset shall be Empty.
// GetCurrentPreset shall return 0 if Imaging Presets are not supported by the
// Video Source.
pub async fn get_current_preset<T: transport::Transport>(
    transport: &T,
    request: &GetCurrentPreset,
) -> Result<GetCurrentPresetResponse, transport::Error> {
    transport::request(transport, request).await
}

// The SetCurrentPreset command shall request a given Imaging Preset to be
// applied to the specified Video Source.
// SetCurrentPreset shall only be available for Video Sources with Imaging
// Presets Capability.
// Imaging Presets are defined by the Manufacturer, and offered as a tool to
// simplify Imaging Settings adjustments for specific scene content.
// When the new Imaging Preset is applied by SetCurrentPreset, the Device shall
// adjust the Video Source settings to match those defined by the specified
// Imaging Preset.
pub async fn set_current_preset<T: transport::Transport>(
    transport: &T,
    request: &SetCurrentPreset,
) -> Result<SetCurrentPresetResponse, transport::Error> {
    transport::request(transport, request).await
}
