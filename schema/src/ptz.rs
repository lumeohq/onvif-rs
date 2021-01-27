use crate::onvif as tt;
use crate::transport;
use crate::validate::Validate;
use xsd_types::types as xs;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GetServiceCapabilitiesResponse {
    // The capabilities for the PTZ service is returned in the Capabilities
    // element.
    #[yaserde(prefix = "tptz", rename = "Capabilities")]
    pub capabilities: Capabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct Capabilities {
    // Indicates whether or not EFlip is supported.
    #[yaserde(attribute, rename = "EFlip")]
    pub e_flip: Option<bool>,

    // Indicates whether or not reversing of PT control direction is supported.
    #[yaserde(attribute, rename = "Reverse")]
    pub reverse: Option<bool>,

    // Indicates support for the GetCompatibleConfigurations command.
    #[yaserde(attribute, rename = "GetCompatibleConfigurations")]
    pub get_compatible_configurations: Option<bool>,

    // Indicates that the PTZStatus includes MoveStatus information.
    #[yaserde(attribute, rename = "MoveStatus")]
    pub move_status: Option<bool>,

    // Indicates that the PTZStatus includes Position information.
    #[yaserde(attribute, rename = "StatusPosition")]
    pub status_position: Option<bool>,
}

impl Validate for Capabilities {}

// pub type Capabilities = Capabilities;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GetNodes {}

impl Validate for GetNodes {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GetNodesResponse {
    // A list of the existing PTZ Nodes on the device.
    #[yaserde(prefix = "tptz", rename = "PTZNode")]
    pub ptz_node: Vec<tt::Ptznode>,
}

impl Validate for GetNodesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GetNode {
    // Token of the requested PTZNode.
    #[yaserde(prefix = "tptz", rename = "NodeToken")]
    pub node_token: tt::ReferenceToken,
}

impl Validate for GetNode {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GetNodeResponse {
    // A requested PTZNode.
    #[yaserde(prefix = "tptz", rename = "PTZNode")]
    pub ptz_node: tt::Ptznode,
}

impl Validate for GetNodeResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GetConfigurations {}

impl Validate for GetConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GetConfigurationsResponse {
    // A list of all existing PTZConfigurations on the device.
    #[yaserde(prefix = "tptz", rename = "PTZConfiguration")]
    pub ptz_configuration: Vec<tt::Ptzconfiguration>,
}

impl Validate for GetConfigurationsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GetConfiguration {
    // Token of the requested PTZConfiguration.
    #[yaserde(prefix = "tptz", rename = "PTZConfigurationToken")]
    pub ptz_configuration_token: tt::ReferenceToken,
}

impl Validate for GetConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GetConfigurationResponse {
    // A requested PTZConfiguration.
    #[yaserde(prefix = "tptz", rename = "PTZConfiguration")]
    pub ptz_configuration: tt::Ptzconfiguration,
}

impl Validate for GetConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct SetConfiguration {
    #[yaserde(prefix = "tptz", rename = "PTZConfiguration")]
    pub ptz_configuration: tt::Ptzconfiguration,

    // Flag that makes configuration persistent. Example: User wants the
    // configuration to exist after reboot.
    #[yaserde(prefix = "tptz", rename = "ForcePersistence")]
    pub force_persistence: bool,
}

impl Validate for SetConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct SetConfigurationResponse {}

impl Validate for SetConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GetConfigurationOptions {
    // Token of an existing configuration that the options are intended for.
    #[yaserde(prefix = "tptz", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetConfigurationOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GetConfigurationOptionsResponse {
    // The requested PTZ configuration options.
    #[yaserde(prefix = "tptz", rename = "PTZConfigurationOptions")]
    pub ptz_configuration_options: tt::PtzconfigurationOptions,
}

impl Validate for GetConfigurationOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct SendAuxiliaryCommand {
    // A reference to the MediaProfile where the operation should take place.
    #[yaserde(prefix = "tptz", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // The Auxiliary request data.
    #[yaserde(prefix = "tptz", rename = "AuxiliaryData")]
    pub auxiliary_data: tt::AuxiliaryData,
}

impl Validate for SendAuxiliaryCommand {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct SendAuxiliaryCommandResponse {
    // The response contains the auxiliary response.
    #[yaserde(prefix = "tptz", rename = "AuxiliaryResponse")]
    pub auxiliary_response: tt::AuxiliaryData,
}

impl Validate for SendAuxiliaryCommandResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GetPresets {
    // A reference to the MediaProfile where the operation should take place.
    #[yaserde(prefix = "tptz", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetPresets {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GetPresetsResponse {
    // A list of presets which are available for the requested MediaProfile.
    #[yaserde(prefix = "tptz", rename = "Preset")]
    pub preset: Vec<tt::Ptzpreset>,
}

impl Validate for GetPresetsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct SetPreset {
    // A reference to the MediaProfile where the operation should take place.
    #[yaserde(prefix = "tptz", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // A requested preset name.
    #[yaserde(prefix = "tptz", rename = "PresetName")]
    pub preset_name: Option<String>,

    // A requested preset token.
    #[yaserde(prefix = "tptz", rename = "PresetToken")]
    pub preset_token: Option<tt::ReferenceToken>,
}

impl Validate for SetPreset {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct SetPresetResponse {
    // A token to the Preset which has been set.
    #[yaserde(prefix = "tptz", rename = "PresetToken")]
    pub preset_token: tt::ReferenceToken,
}

impl Validate for SetPresetResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct RemovePreset {
    // A reference to the MediaProfile where the operation should take place.
    #[yaserde(prefix = "tptz", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // A requested preset token.
    #[yaserde(prefix = "tptz", rename = "PresetToken")]
    pub preset_token: tt::ReferenceToken,
}

impl Validate for RemovePreset {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct RemovePresetResponse {}

impl Validate for RemovePresetResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GotoPreset {
    // A reference to the MediaProfile where the operation should take place.
    #[yaserde(prefix = "tptz", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // A requested preset token.
    #[yaserde(prefix = "tptz", rename = "PresetToken")]
    pub preset_token: tt::ReferenceToken,

    // A requested speed.The speed parameter can only be specified when Speed
    // Spaces are available for the PTZ Node.
    #[yaserde(prefix = "tptz", rename = "Speed")]
    pub speed: Option<tt::Ptzspeed>,
}

impl Validate for GotoPreset {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GotoPresetResponse {}

impl Validate for GotoPresetResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GetStatus {
    // A reference to the MediaProfile where the PTZStatus should be requested.
    #[yaserde(prefix = "tptz", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetStatus {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GetStatusResponse {
    // The PTZStatus for the requested MediaProfile.
    #[yaserde(prefix = "tptz", rename = "PTZStatus")]
    pub ptz_status: tt::Ptzstatus,
}

impl Validate for GetStatusResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GotoHomePosition {
    // A reference to the MediaProfile where the operation should take place.
    #[yaserde(prefix = "tptz", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // A requested speed.The speed parameter can only be specified when Speed
    // Spaces are available for the PTZ Node.
    #[yaserde(prefix = "tptz", rename = "Speed")]
    pub speed: Option<tt::Ptzspeed>,
}

impl Validate for GotoHomePosition {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GotoHomePositionResponse {}

impl Validate for GotoHomePositionResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct SetHomePosition {
    // A reference to the MediaProfile where the home position should be set.
    #[yaserde(prefix = "tptz", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for SetHomePosition {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct SetHomePositionResponse {}

impl Validate for SetHomePositionResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct ContinuousMove {
    // A reference to the MediaProfile.
    #[yaserde(prefix = "tptz", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // A Velocity vector specifying the velocity of pan, tilt and zoom.
    #[yaserde(prefix = "tptz", rename = "Velocity")]
    pub velocity: tt::Ptzspeed,

    // An optional Timeout parameter.
    #[yaserde(prefix = "tptz", rename = "Timeout")]
    pub timeout: Option<xs::Duration>,
}

impl Validate for ContinuousMove {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct ContinuousMoveResponse {}

impl Validate for ContinuousMoveResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct RelativeMove {
    // A reference to the MediaProfile.
    #[yaserde(prefix = "tptz", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // A positional Translation relative to the current position
    #[yaserde(prefix = "tptz", rename = "Translation")]
    pub translation: tt::Ptzvector,

    // An optional Speed parameter.
    #[yaserde(prefix = "tptz", rename = "Speed")]
    pub speed: Option<tt::Ptzspeed>,
}

impl Validate for RelativeMove {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct RelativeMoveResponse {}

impl Validate for RelativeMoveResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct AbsoluteMove {
    // A reference to the MediaProfile.
    #[yaserde(prefix = "tptz", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // A Position vector specifying the absolute target position.
    #[yaserde(prefix = "tptz", rename = "Position")]
    pub position: tt::Ptzvector,

    // An optional Speed.
    #[yaserde(prefix = "tptz", rename = "Speed")]
    pub speed: Option<tt::Ptzspeed>,
}

impl Validate for AbsoluteMove {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct AbsoluteMoveResponse {}

impl Validate for AbsoluteMoveResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GeoMove {
    // A reference to the MediaProfile.
    #[yaserde(prefix = "tptz", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // The geolocation of the target position.
    #[yaserde(prefix = "tptz", rename = "Target")]
    pub target: tt::GeoLocation,

    // An optional Speed.
    #[yaserde(prefix = "tptz", rename = "Speed")]
    pub speed: Option<tt::Ptzspeed>,

    // An optional indication of the height of the target/area.
    #[yaserde(prefix = "tptz", rename = "AreaHeight")]
    pub area_height: Option<f64>,

    // An optional indication of the width of the target/area.
    #[yaserde(prefix = "tptz", rename = "AreaWidth")]
    pub area_width: Option<f64>,
}

impl Validate for GeoMove {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GeoMoveResponse {}

impl Validate for GeoMoveResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct Stop {
    // A reference to the MediaProfile that indicate what should be stopped.
    #[yaserde(prefix = "tptz", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // Set true when we want to stop ongoing pan and tilt movements.If PanTilt
    // arguments are not present, this command stops these movements.
    #[yaserde(prefix = "tptz", rename = "PanTilt")]
    pub pan_tilt: Option<bool>,

    // Set true when we want to stop ongoing zoom movement.If Zoom arguments are
    // not present, this command stops ongoing zoom movement.
    #[yaserde(prefix = "tptz", rename = "Zoom")]
    pub zoom: Option<bool>,
}

impl Validate for Stop {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct StopResponse {}

impl Validate for StopResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GetPresetTours {
    #[yaserde(prefix = "tptz", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetPresetTours {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GetPresetToursResponse {
    #[yaserde(prefix = "tptz", rename = "PresetTour")]
    pub preset_tour: Vec<tt::PresetTour>,
}

impl Validate for GetPresetToursResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GetPresetTour {
    #[yaserde(prefix = "tptz", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    #[yaserde(prefix = "tptz", rename = "PresetTourToken")]
    pub preset_tour_token: tt::ReferenceToken,
}

impl Validate for GetPresetTour {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GetPresetTourResponse {
    #[yaserde(prefix = "tptz", rename = "PresetTour")]
    pub preset_tour: tt::PresetTour,
}

impl Validate for GetPresetTourResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GetPresetTourOptions {
    #[yaserde(prefix = "tptz", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    #[yaserde(prefix = "tptz", rename = "PresetTourToken")]
    pub preset_tour_token: Option<tt::ReferenceToken>,
}

impl Validate for GetPresetTourOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GetPresetTourOptionsResponse {
    #[yaserde(prefix = "tptz", rename = "Options")]
    pub options: tt::PtzpresetTourOptions,
}

impl Validate for GetPresetTourOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct CreatePresetTour {
    #[yaserde(prefix = "tptz", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for CreatePresetTour {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct CreatePresetTourResponse {
    #[yaserde(prefix = "tptz", rename = "PresetTourToken")]
    pub preset_tour_token: tt::ReferenceToken,
}

impl Validate for CreatePresetTourResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct ModifyPresetTour {
    #[yaserde(prefix = "tptz", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    #[yaserde(prefix = "tptz", rename = "PresetTour")]
    pub preset_tour: tt::PresetTour,
}

impl Validate for ModifyPresetTour {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct ModifyPresetTourResponse {}

impl Validate for ModifyPresetTourResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct OperatePresetTour {
    #[yaserde(prefix = "tptz", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    #[yaserde(prefix = "tptz", rename = "PresetTourToken")]
    pub preset_tour_token: tt::ReferenceToken,

    #[yaserde(prefix = "tptz", rename = "Operation")]
    pub operation: tt::PtzpresetTourOperation,
}

impl Validate for OperatePresetTour {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct OperatePresetTourResponse {}

impl Validate for OperatePresetTourResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct RemovePresetTour {
    #[yaserde(prefix = "tptz", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    #[yaserde(prefix = "tptz", rename = "PresetTourToken")]
    pub preset_tour_token: tt::ReferenceToken,
}

impl Validate for RemovePresetTour {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct RemovePresetTourResponse {}

impl Validate for RemovePresetTourResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GetCompatibleConfigurations {
    // Contains the token of an existing media profile the configurations shall
    // be compatible with.
    #[yaserde(prefix = "tptz", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetCompatibleConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tptz",
    namespace = "tptz: http://www.onvif.org/ver20/ptz/wsdl"
)]
pub struct GetCompatibleConfigurationsResponse {
    // A list of all existing PTZConfigurations on the NVT that is suitable to
    // be added to the addressed media profile.
    #[yaserde(prefix = "tptz", rename = "PTZConfiguration")]
    pub ptz_configuration: Vec<tt::Ptzconfiguration>,
}

impl Validate for GetCompatibleConfigurationsResponse {}

// Returns the capabilities of the PTZ service. The result is returned in a
// typed answer.
pub async fn get_service_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetServiceCapabilities,
) -> Result<GetServiceCapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// Get the descriptions of the available PTZ Nodes.
pub async fn get_nodes<T: transport::Transport>(
    transport: &T,
    request: &GetNodes,
) -> Result<GetNodesResponse, transport::Error> {
    transport::request(transport, request).await
}

// Get a specific PTZ Node identified by a reference
// token or a name.
pub async fn get_node<T: transport::Transport>(
    transport: &T,
    request: &GetNode,
) -> Result<GetNodeResponse, transport::Error> {
    transport::request(transport, request).await
}

// Get a specific PTZconfiguration from the device, identified by its reference
// token or name.
pub async fn get_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetConfiguration,
) -> Result<GetConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// Get all the existing PTZConfigurations from the device.
pub async fn get_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetConfigurations,
) -> Result<GetConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// Set/update a existing PTZConfiguration on the device.
pub async fn set_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetConfiguration,
) -> Result<SetConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// List supported coordinate systems including their range limitations.
// Therefore, the options
// MAY differ depending on whether the PTZ Configuration is assigned to a
// Profile containing a
// Video Source Configuration. In that case, the options may additionally
// contain coordinate
// systems referring to the image coordinate system described by the Video
// Source
// Configuration. If the PTZ Node supports continuous movements, it shall return
// a Timeout Range within
// which Timeouts are accepted by the PTZ Node.
pub async fn get_configuration_options<T: transport::Transport>(
    transport: &T,
    request: &GetConfigurationOptions,
) -> Result<GetConfigurationOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// Operation to send auxiliary commands to the PTZ device
// mapped by the PTZNode in the selected profile. The
// operation is supported
// if the AuxiliarySupported element of the PTZNode is true
pub async fn send_auxiliary_command<T: transport::Transport>(
    transport: &T,
    request: &SendAuxiliaryCommand,
) -> Result<SendAuxiliaryCommandResponse, transport::Error> {
    transport::request(transport, request).await
}

// Operation to request all PTZ presets for the PTZNode
// in the selected profile. The operation is supported if there is support
// for at least on PTZ preset by the PTZNode.
pub async fn get_presets<T: transport::Transport>(
    transport: &T,
    request: &GetPresets,
) -> Result<GetPresetsResponse, transport::Error> {
    transport::request(transport, request).await
}

// The SetPreset command saves the current device position parameters so that
// the device can
// move to the saved preset position through the GotoPreset operation.
// In order to create a new preset, the SetPresetRequest contains no
// PresetToken. If creation is
// successful, the Response contains the PresetToken which uniquely identifies
// the Preset. An
// existing Preset can be overwritten by specifying the PresetToken of the
// corresponding Preset.
// In both cases (overwriting or creation) an optional PresetName can be
// specified. The
// operation fails if the PTZ device is moving during the SetPreset operation.
// The device MAY internally save additional states such as imaging properties
// in the PTZ
// Preset which then should be recalled in the GotoPreset operation.
pub async fn set_preset<T: transport::Transport>(
    transport: &T,
    request: &SetPreset,
) -> Result<SetPresetResponse, transport::Error> {
    transport::request(transport, request).await
}

// Operation to remove a PTZ preset for the Node in
// the
// selected profile. The operation is supported if the
// PresetPosition
// capability exists for teh Node in the
// selected profile.
pub async fn remove_preset<T: transport::Transport>(
    transport: &T,
    request: &RemovePreset,
) -> Result<RemovePresetResponse, transport::Error> {
    transport::request(transport, request).await
}

// Operation to go to a saved preset position for the
// PTZNode in the selected profile. The operation is supported if there is
// support for at least on PTZ preset by the PTZNode.
pub async fn goto_preset<T: transport::Transport>(
    transport: &T,
    request: &GotoPreset,
) -> Result<GotoPresetResponse, transport::Error> {
    transport::request(transport, request).await
}

// Operation to move the PTZ device to it's "home" position. The operation is
// supported if the HomeSupported element in the PTZNode is true.
pub async fn goto_home_position<T: transport::Transport>(
    transport: &T,
    request: &GotoHomePosition,
) -> Result<GotoHomePositionResponse, transport::Error> {
    transport::request(transport, request).await
}

// Operation to save current position as the home position.
// The SetHomePosition command returns with a failure if the “home” position
// is fixed and
// cannot be overwritten. If the SetHomePosition is successful, it is possible
// to recall the
// Home Position with the GotoHomePosition command.
pub async fn set_home_position<T: transport::Transport>(
    transport: &T,
    request: &SetHomePosition,
) -> Result<SetHomePositionResponse, transport::Error> {
    transport::request(transport, request).await
}

// Operation for continuous Pan/Tilt and Zoom movements. The operation is
// supported if the PTZNode supports at least one continuous Pan/Tilt or Zoom
// space. If the space argument is omitted, the default space set by the
// PTZConfiguration will be used.
pub async fn continuous_move<T: transport::Transport>(
    transport: &T,
    request: &ContinuousMove,
) -> Result<ContinuousMoveResponse, transport::Error> {
    transport::request(transport, request).await
}

// Operation for Relative Pan/Tilt and Zoom Move. The operation is supported if
// the PTZNode supports at least one relative Pan/Tilt or Zoom space.
pub async fn relative_move<T: transport::Transport>(
    transport: &T,
    request: &RelativeMove,
) -> Result<RelativeMoveResponse, transport::Error> {
    transport::request(transport, request).await
}

// Operation to request PTZ status for the Node in the
// selected profile.
pub async fn get_status<T: transport::Transport>(
    transport: &T,
    request: &GetStatus,
) -> Result<GetStatusResponse, transport::Error> {
    transport::request(transport, request).await
}

// Operation to move pan,tilt or zoom to a absolute destination.
pub async fn absolute_move<T: transport::Transport>(
    transport: &T,
    request: &AbsoluteMove,
) -> Result<AbsoluteMoveResponse, transport::Error> {
    transport::request(transport, request).await
}

// Operation to move pan,tilt or zoom to point to a destination based on the
// geolocation of the target.
pub async fn geo_move<T: transport::Transport>(
    transport: &T,
    request: &GeoMove,
) -> Result<GeoMoveResponse, transport::Error> {
    transport::request(transport, request).await
}

// Operation to stop ongoing pan, tilt and zoom movements of absolute relative
// and continuous type.
// If no stop argument for pan, tilt or zoom is set, the device will stop all
// ongoing pan, tilt and zoom movements.
pub async fn stop<T: transport::Transport>(
    transport: &T,
    request: &Stop,
) -> Result<StopResponse, transport::Error> {
    transport::request(transport, request).await
}

// Operation to request PTZ preset tours in the selected media profiles.
pub async fn get_preset_tours<T: transport::Transport>(
    transport: &T,
    request: &GetPresetTours,
) -> Result<GetPresetToursResponse, transport::Error> {
    transport::request(transport, request).await
}

// Operation to request a specific PTZ preset tour in the selected media
// profile.
pub async fn get_preset_tour<T: transport::Transport>(
    transport: &T,
    request: &GetPresetTour,
) -> Result<GetPresetTourResponse, transport::Error> {
    transport::request(transport, request).await
}

// Operation to request available options to configure PTZ preset tour.
pub async fn get_preset_tour_options<T: transport::Transport>(
    transport: &T,
    request: &GetPresetTourOptions,
) -> Result<GetPresetTourOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// Operation to create a preset tour for the selected media profile.
pub async fn create_preset_tour<T: transport::Transport>(
    transport: &T,
    request: &CreatePresetTour,
) -> Result<CreatePresetTourResponse, transport::Error> {
    transport::request(transport, request).await
}

// Operation to modify a preset tour for the selected media profile.
pub async fn modify_preset_tour<T: transport::Transport>(
    transport: &T,
    request: &ModifyPresetTour,
) -> Result<ModifyPresetTourResponse, transport::Error> {
    transport::request(transport, request).await
}

// Operation to perform specific operation on the preset tour in selected media
// profile.
pub async fn operate_preset_tour<T: transport::Transport>(
    transport: &T,
    request: &OperatePresetTour,
) -> Result<OperatePresetTourResponse, transport::Error> {
    transport::request(transport, request).await
}

// Operation to delete a specific preset tour from the media profile.
pub async fn remove_preset_tour<T: transport::Transport>(
    transport: &T,
    request: &RemovePresetTour,
) -> Result<RemovePresetTourResponse, transport::Error> {
    transport::request(transport, request).await
}

// Operation to get all available PTZConfigurations that can be added to the
// referenced media profile.
pub async fn get_compatible_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetCompatibleConfigurations,
) -> Result<GetCompatibleConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}
