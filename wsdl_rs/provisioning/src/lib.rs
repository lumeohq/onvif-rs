#![allow(clippy::derive_partial_eq_without_eq)]

use onvif as tt;
use validate::Validate;
use xsd_types::types as xs;
use yaserde_derive::{YaDeserialize, YaSerialize};

// The direction for PanMove to move the device.
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum PanDirection {
    // Move left in relation to the video source image.
    Left,
    // Move right in relation to the video source image.
    Right,
    __Unknown__(String),
}

impl Default for PanDirection {
    fn default() -> PanDirection {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for PanDirection {}

// The direction for TiltMove to move the device.
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum TiltDirection {
    // Move up in relation to the video source image.
    Up,
    // Move down in relation to the video source image.
    Down,
    __Unknown__(String),
}

impl Default for TiltDirection {
    fn default() -> TiltDirection {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for TiltDirection {}

// The direction for ZoomMove to change the focal length in relation to the
// video source.
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum ZoomDirection {
    // Move video source lens toward a wider field of view.
    Wide,
    // Move video source lens toward a narrower field of view.
    Telephoto,
    __Unknown__(String),
}

impl Default for ZoomDirection {
    fn default() -> ZoomDirection {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for ZoomDirection {}

// The direction for RollMove to move the device.
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum RollDirection {
    // Move clockwise in relation to the video source image.
    Clockwise,
    // Move counterclockwise in relation to the video source image.
    Counterclockwise,
    // Automatically level the device in relation to the video source image.
    Auto,
    __Unknown__(String),
}

impl Default for RollDirection {
    fn default() -> RollDirection {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for RollDirection {}

// The direction for FocusMove to move the focal plane in relation to the video
// source.
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum FocusDirection {
    // Move to focus on close objects.
    Near,
    // Move to focus on distant objects.
    Far,
    // Automatically focus for the sharpest video source image.
    Auto,
    __Unknown__(String),
}

impl Default for FocusDirection {
    fn default() -> FocusDirection {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for FocusDirection {}

// The quantity of movement events that have occured over the lifetime of the
// device.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tpv",
    namespace = "tpv: http://www.onvif.org/ver10/provisioning/wsdl"
)]
pub struct Usage {
    // The quantity of pan movement events over the life of the device.
    #[yaserde(prefix = "tpv", rename = "Pan")]
    pub pan: Option<xs::Integer>,

    // The quantity of tilt movement events over the life of the device.
    #[yaserde(prefix = "tpv", rename = "Tilt")]
    pub tilt: Option<xs::Integer>,

    // The quantity of zoom movement events over the life of the device.
    #[yaserde(prefix = "tpv", rename = "Zoom")]
    pub zoom: Option<xs::Integer>,

    // The quantity of roll movement events over the life of the device.
    #[yaserde(prefix = "tpv", rename = "Roll")]
    pub roll: Option<xs::Integer>,

    // The quantity of focus movement events over the life of the device.
    #[yaserde(prefix = "tpv", rename = "Focus")]
    pub focus: Option<xs::Integer>,
}

impl Validate for Usage {}

// The provisioning capabilities of a video source on the device.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tpv",
    namespace = "tpv: http://www.onvif.org/ver10/provisioning/wsdl"
)]
pub struct SourceCapabilities {
    // Unique identifier of a video source.
    #[yaserde(attribute, rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,

    // Lifetime limit of pan moves for this video source. Presence of this
    // attribute indicates support of pan move.
    #[yaserde(attribute, rename = "MaximumPanMoves")]
    pub maximum_pan_moves: Option<xs::Integer>,

    // Lifetime limit of tilt moves for this video source. Presence of this
    // attribute indicates support of tilt move.
    #[yaserde(attribute, rename = "MaximumTiltMoves")]
    pub maximum_tilt_moves: Option<xs::Integer>,

    // Lifetime limit of zoom moves for this video source. Presence of this
    // attribute indicates support of zoom move.
    #[yaserde(attribute, rename = "MaximumZoomMoves")]
    pub maximum_zoom_moves: Option<xs::Integer>,

    // Lifetime limit of roll moves for this video source. Presence of this
    // attribute indicates support of roll move.
    #[yaserde(attribute, rename = "MaximumRollMoves")]
    pub maximum_roll_moves: Option<xs::Integer>,

    // Indicates "auto" as a valid enum for Direction in RollMove.
    #[yaserde(attribute, rename = "AutoLevel")]
    pub auto_level: Option<bool>,

    // Lifetime limit of focus moves for this video source. Presence of this
    // attribute indicates support of focus move.
    #[yaserde(attribute, rename = "MaximumFocusMoves")]
    pub maximum_focus_moves: Option<xs::Integer>,

    // Indicates "auto" as a valid enum for Direction in FocusMove.
    #[yaserde(attribute, rename = "AutoFocus")]
    pub auto_focus: Option<bool>,
}

impl Validate for SourceCapabilities {}

// The capabilities of Provisioning Service on the device.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tpv",
    namespace = "tpv: http://www.onvif.org/ver10/provisioning/wsdl"
)]
pub struct Capabilities {
    // Maximum time before stopping movement after a move operation.
    #[yaserde(prefix = "tpv", rename = "DefaultTimeout")]
    pub default_timeout: xs::Duration,

    // Capabilities per video source.
    #[yaserde(prefix = "tpv", rename = "Source")]
    pub source: Vec<SourceCapabilities>,
}

impl Validate for Capabilities {}

// pub type Capabilities = Capabilities;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tpv",
    namespace = "tpv: http://www.onvif.org/ver10/provisioning/wsdl"
)]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tpv",
    namespace = "tpv: http://www.onvif.org/ver10/provisioning/wsdl"
)]
pub struct GetServiceCapabilitiesResponse {
    // The capabilities for the provisioning service on this device.
    #[yaserde(prefix = "tpv", rename = "Capabilities")]
    pub capabilities: Capabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tpv",
    namespace = "tpv: http://www.onvif.org/ver10/provisioning/wsdl"
)]
pub struct PanMove {
    // The video source associated with the provisioning.
    #[yaserde(prefix = "tpv", rename = "VideoSource")]
    pub video_source: tt::ReferenceToken,

    // "left" or "right".
    #[yaserde(prefix = "tpv", rename = "Direction")]
    pub direction: PanDirection,

    // "Operation timeout, if less than default timeout.
    #[yaserde(prefix = "tpv", rename = "Timeout")]
    pub timeout: Option<xs::Duration>,
}

impl Validate for PanMove {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tpv",
    namespace = "tpv: http://www.onvif.org/ver10/provisioning/wsdl"
)]
pub struct PanMoveResponse {}

impl Validate for PanMoveResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tpv",
    namespace = "tpv: http://www.onvif.org/ver10/provisioning/wsdl"
)]
pub struct TiltMove {
    // The video source associated with the provisioning.
    #[yaserde(prefix = "tpv", rename = "VideoSource")]
    pub video_source: tt::ReferenceToken,

    // "up" or "down".
    #[yaserde(prefix = "tpv", rename = "Direction")]
    pub direction: TiltDirection,

    // "Operation timeout, if less than default timeout.
    #[yaserde(prefix = "tpv", rename = "Timeout")]
    pub timeout: Option<xs::Duration>,
}

impl Validate for TiltMove {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tpv",
    namespace = "tpv: http://www.onvif.org/ver10/provisioning/wsdl"
)]
pub struct TiltMoveResponse {}

impl Validate for TiltMoveResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tpv",
    namespace = "tpv: http://www.onvif.org/ver10/provisioning/wsdl"
)]
pub struct ZoomMove {
    // The video source associated with the provisioning.
    #[yaserde(prefix = "tpv", rename = "VideoSource")]
    pub video_source: tt::ReferenceToken,

    // "wide" or "telephoto".
    #[yaserde(prefix = "tpv", rename = "Direction")]
    pub direction: ZoomDirection,

    // "Operation timeout, if less than default timeout.
    #[yaserde(prefix = "tpv", rename = "Timeout")]
    pub timeout: Option<xs::Duration>,
}

impl Validate for ZoomMove {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tpv",
    namespace = "tpv: http://www.onvif.org/ver10/provisioning/wsdl"
)]
pub struct ZoomMoveResponse {}

impl Validate for ZoomMoveResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tpv",
    namespace = "tpv: http://www.onvif.org/ver10/provisioning/wsdl"
)]
pub struct RollMove {
    // The video source associated with the provisioning.
    #[yaserde(prefix = "tpv", rename = "VideoSource")]
    pub video_source: tt::ReferenceToken,

    // "clockwise", "counterclockwise", or "auto".
    #[yaserde(prefix = "tpv", rename = "Direction")]
    pub direction: RollDirection,

    // "Operation timeout, if less than default timeout.
    #[yaserde(prefix = "tpv", rename = "Timeout")]
    pub timeout: Option<xs::Duration>,
}

impl Validate for RollMove {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tpv",
    namespace = "tpv: http://www.onvif.org/ver10/provisioning/wsdl"
)]
pub struct RollMoveResponse {}

impl Validate for RollMoveResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tpv",
    namespace = "tpv: http://www.onvif.org/ver10/provisioning/wsdl"
)]
pub struct FocusMove {
    // The video source associated with the provisioning.
    #[yaserde(prefix = "tpv", rename = "VideoSource")]
    pub video_source: tt::ReferenceToken,

    // "near", "far", or "auto".
    #[yaserde(prefix = "tpv", rename = "Direction")]
    pub direction: FocusDirection,

    // "Operation timeout, if less than default timeout.
    #[yaserde(prefix = "tpv", rename = "Timeout")]
    pub timeout: Option<xs::Duration>,
}

impl Validate for FocusMove {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tpv",
    namespace = "tpv: http://www.onvif.org/ver10/provisioning/wsdl"
)]
pub struct FocusMoveResponse {}

impl Validate for FocusMoveResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tpv",
    namespace = "tpv: http://www.onvif.org/ver10/provisioning/wsdl"
)]
pub struct Stop {
    // The video source associated with the provisioning.
    #[yaserde(prefix = "tpv", rename = "VideoSource")]
    pub video_source: tt::ReferenceToken,
}

impl Validate for Stop {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tpv",
    namespace = "tpv: http://www.onvif.org/ver10/provisioning/wsdl"
)]
pub struct StopResponse {}

impl Validate for StopResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tpv",
    namespace = "tpv: http://www.onvif.org/ver10/provisioning/wsdl"
)]
pub struct GetUsage {
    // The video source associated with the provisioning.
    #[yaserde(prefix = "tpv", rename = "VideoSource")]
    pub video_source: tt::ReferenceToken,
}

impl Validate for GetUsage {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tpv",
    namespace = "tpv: http://www.onvif.org/ver10/provisioning/wsdl"
)]
pub struct GetUsageResponse {
    // The set of lifetime usage values for the provisioning associated with the
    // video source.
    #[yaserde(prefix = "tpv", rename = "Usage")]
    pub usage: Usage,
}

impl Validate for GetUsageResponse {}

// Returns the capabilities of the provisioning service.
pub async fn get_service_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetServiceCapabilities,
) -> Result<GetServiceCapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// Moves device on the pan axis.
pub async fn pan_move<T: transport::Transport>(
    transport: &T,
    request: &PanMove,
) -> Result<PanMoveResponse, transport::Error> {
    transport::request(transport, request).await
}

// Moves device on the tilt axis.
pub async fn tilt_move<T: transport::Transport>(
    transport: &T,
    request: &TiltMove,
) -> Result<TiltMoveResponse, transport::Error> {
    transport::request(transport, request).await
}

// Moves device on the zoom axis.
pub async fn zoom_move<T: transport::Transport>(
    transport: &T,
    request: &ZoomMove,
) -> Result<ZoomMoveResponse, transport::Error> {
    transport::request(transport, request).await
}

// Moves device on the roll axis.
pub async fn roll_move<T: transport::Transport>(
    transport: &T,
    request: &RollMove,
) -> Result<RollMoveResponse, transport::Error> {
    transport::request(transport, request).await
}

// Moves device on the focus axis.
pub async fn focus_move<T: transport::Transport>(
    transport: &T,
    request: &FocusMove,
) -> Result<FocusMoveResponse, transport::Error> {
    transport::request(transport, request).await
}

// Stops device motion on all axes.
pub async fn stop<T: transport::Transport>(
    transport: &T,
    request: &Stop,
) -> Result<StopResponse, transport::Error> {
    transport::request(transport, request).await
}

// Returns the lifetime move counts.
pub async fn get_usage<T: transport::Transport>(
    transport: &T,
    request: &GetUsage,
) -> Result<GetUsageResponse, transport::Error> {
    transport::request(transport, request).await
}
