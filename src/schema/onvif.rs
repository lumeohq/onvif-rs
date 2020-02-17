// Based on onvif.xsd

// targetNamespace="http://www.onvif.org/ver10/schema"

// xmlns:tt="http://www.onvif.org/ver10/schema"
// xmlns:xs="http://www.w3.org/2001/XMLSchema"
// xmlns:xmime="http://www.w3.org/2005/05/xmlmime"
// xmlns:wsnt="http://docs.oasis-open.org/wsn/b-2"
// xmlns:xop="http://www.w3.org/2004/08/xop/include"
// xmlns:soapenv="http://www.w3.org/2003/05/soap-envelope"

// <xs:include schemaLocation="common.xsd"/>
// <xs:import namespace="http://www.w3.org/2005/05/xmlmime" schemaLocation="http://www.w3.org/2005/05/xmlmime"/>
// <xs:import namespace="http://www.w3.org/2003/05/soap-envelope" schemaLocation="http://www.w3.org/2003/05/soap-envelope"/>
// <xs:import namespace="http://docs.oasis-open.org/wsn/b-2" schemaLocation="http://docs.oasis-open.org/wsn/b-2.xsd"/>
// <xs:import namespace="http://www.w3.org/2004/08/xop/include" schemaLocation="http://www.w3.org/2004/08/xop/include"/>

use crate::schema::common::*;
use crate::schema::{b_2 as wsnt, soap_envelope as soapenv, xmlmime as xmime, xop, xs};
use crate::utils;
use macro_utils::*;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

//generated file
//use common.xsd  ;

//use http://www.w3.org/2005/05/xmlmime  http://www.w3.org/2005/05/xmlmime;

//use http://www.w3.org/2003/05/soap-envelope  http://www.w3.org/2003/05/soap-envelope;

//use http://docs.oasis-open.org/wsn/b-2.xsd  http://docs.oasis-open.org/wsn/b-2;

//use http://www.w3.org/2004/08/xop/include  http://www.w3.org/2004/08/xop/include;

// Base class for physical entities like inputs and outputs.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct DeviceEntity {
    // Unique identifier referencing the physical entity.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

// User readable name. Length up to 64 characters.
#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct Name(pub String);

// Rectangle defined by lower left corner position and size. Units are pixel.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct IntRectangle {
    #[yaserde(attribute, rename = "x")]
    pub x: i32,

    #[yaserde(attribute, rename = "y")]
    pub y: i32,

    #[yaserde(attribute, rename = "width")]
    pub width: i32,

    #[yaserde(attribute, rename = "height")]
    pub height: i32,
}

// Range of a rectangle. The rectangle itself is defined by lower left corner
// position and size. Units are pixel.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct IntRectangleRange {
    // Range of X-axis.
    #[yaserde(prefix = "tt", rename = "XRange")]
    pub x_range: IntRange,

    // Range of Y-axis.
    #[yaserde(prefix = "tt", rename = "YRange")]
    pub y_range: IntRange,

    // Range of width.
    #[yaserde(prefix = "tt", rename = "WidthRange")]
    pub width_range: IntRange,

    // Range of height.
    #[yaserde(prefix = "tt", rename = "HeightRange")]
    pub height_range: IntRange,
}

// Range of values greater equal Min value and less equal Max value.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct FloatRange {
    #[yaserde(prefix = "tt", rename = "Min")]
    pub min: f64,

    #[yaserde(prefix = "tt", rename = "Max")]
    pub max: f64,
}

// Range of duration greater equal Min duration and less equal Max duration.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct DurationRange {
    #[yaserde(prefix = "tt", rename = "Min")]
    pub min: xs::Duration,

    #[yaserde(prefix = "tt", rename = "Max")]
    pub max: xs::Duration,
}

// List of values.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct IntList {
    #[yaserde(prefix = "tt", rename = "Items")]
    pub items: Vec<i32>,
}

#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct IntAttrList(pub Vec<i32>);

#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct FloatAttrList(pub Vec<f64>);

#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct StringAttrList(pub Vec<String>);

#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct ReferenceTokenList(pub Vec<ReferenceToken>);

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct FloatList {
    #[yaserde(prefix = "tt", rename = "Items")]
    pub items: Vec<f64>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct StringItems {
    #[yaserde(prefix = "tt", rename = "Item")]
    pub item: Vec<String>,
}

// pub type StringList = StringAttrList;

// type IntRange = IntRange;

// pub type IntList = IntAttrList;

// type FloatRange = FloatRange;

// pub type FloatList = FloatAttrList;

// type DurationRange = DurationRange;

// type IntRectangleRange = IntRectangleRange;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AnyHolder {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,

// //
//TODO: any_attribute macros
//  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoSource {
    // Frame rate in frames per second.
    #[yaserde(prefix = "tt", rename = "Framerate")]
    pub framerate: f64,

    // Horizontal and vertical resolution
    #[yaserde(prefix = "tt", rename = "Resolution")]
    pub resolution: VideoResolution,

    // Optional configuration of the image sensor.
    #[yaserde(prefix = "tt", rename = "Imaging")]
    pub imaging: Option<ImagingSettings>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<VideoSourceExtension>,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // Unique identifier referencing the physical entity.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoSourceExtension {
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // Optional configuration of the image sensor. To be used if imaging service
    // 2.00 is supported.
    #[yaserde(prefix = "tt", rename = "Imaging")]
    pub imaging: Option<ImagingSettings20>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<VideoSourceExtension2>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoSourceExtension2 {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AudioSource {
    // number of available audio channels. (1: mono, 2: stereo)
    #[yaserde(prefix = "tt", rename = "Channels")]
    pub channels: i32,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // Unique identifier referencing the physical entity.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

// A media profile consists of a set of media configurations. Media profiles are
// used by a client
// to configure properties of a media stream from an NVT.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Profile {
    // User readable name of the profile.
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: Name,

    // Optional configuration of the Video input.
    #[yaserde(prefix = "tt", rename = "VideoSourceConfiguration")]
    pub video_source_configuration: Option<VideoSourceConfiguration>,

    // Optional configuration of the Audio input.
    #[yaserde(prefix = "tt", rename = "AudioSourceConfiguration")]
    pub audio_source_configuration: Option<AudioSourceConfiguration>,

    // Optional configuration of the Video encoder.
    #[yaserde(prefix = "tt", rename = "VideoEncoderConfiguration")]
    pub video_encoder_configuration: Option<VideoEncoderConfiguration>,

    // Optional configuration of the Audio encoder.
    #[yaserde(prefix = "tt", rename = "AudioEncoderConfiguration")]
    pub audio_encoder_configuration: Option<AudioEncoderConfiguration>,

    // Optional configuration of the video analytics module and rule engine.
    #[yaserde(prefix = "tt", rename = "VideoAnalyticsConfiguration")]
    pub video_analytics_configuration: Option<VideoAnalyticsConfiguration>,

    // Optional configuration of the pan tilt zoom unit.
    #[yaserde(prefix = "tt", rename = "PTZConfiguration")]
    pub ptz_configuration: Option<Ptzconfiguration>,

    // Optional configuration of the metadata stream.
    #[yaserde(prefix = "tt", rename = "MetadataConfiguration")]
    pub metadata_configuration: Option<MetadataConfiguration>,

    // Extensions defined in ONVIF 2.0
    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ProfileExtension>,

    // Unique identifier of the profile.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,

    // A value of true signals that the profile cannot be deleted. Default is false.
    #[yaserde(attribute, rename = "fixed")]
    pub fixed: Option<bool>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ProfileExtension {
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // Optional configuration of the Audio output.
    #[yaserde(prefix = "tt", rename = "AudioOutputConfiguration")]
    pub audio_output_configuration: Option<AudioOutputConfiguration>,

    // Optional configuration of the Audio decoder.
    #[yaserde(prefix = "tt", rename = "AudioDecoderConfiguration")]
    pub audio_decoder_configuration: Option<AudioDecoderConfiguration>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ProfileExtension2>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ProfileExtension2 {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

// type VideoSourceConfiguration = VideoSourceConfiguration;

// type AudioSourceConfiguration = AudioSourceConfiguration;

// type VideoEncoderConfiguration = VideoEncoderConfiguration;

// type AudioEncoderConfiguration = AudioEncoderConfiguration;

// type VideoAnalyticsConfiguration = VideoAnalyticsConfiguration;

// type Ptzconfiguration = Ptzconfiguration;

// type MetadataConfiguration = MetadataConfiguration;

// type AudioOutputConfiguration = AudioOutputConfiguration;

// type AudioDecoderConfiguration = AudioDecoderConfiguration;

// Base type defining the common properties of a configuration.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ConfigurationEntity {
    // User readable name. Length up to 64 characters.
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: Name,

    // Number of internal references currently using this configuration.
    #[yaserde(prefix = "tt", rename = "UseCount")]
    pub use_count: i32,

    // Token that uniquely references this configuration. Length up to 64
    // characters.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoSourceConfiguration {
    // Reference to the physical input.
    #[yaserde(prefix = "tt", rename = "SourceToken")]
    pub source_token: ReferenceToken,

    // Rectangle specifying the Video capturing area. The capturing area shall not
    // be larger than the whole Video source area.
    #[yaserde(prefix = "tt", rename = "Bounds")]
    pub bounds: IntRectangle,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,
    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<VideoSourceConfigurationExtension>,

    // Readonly parameter signalling Source configuration's view mode, for devices
    // supporting different view modes as defined in tt:viewModes.
    #[yaserde(attribute, rename = "ViewMode")]
    pub view_mode: Option<String>,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // User readable name. Length up to 64 characters.
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: Name,

    // Number of internal references currently using this configuration.
    #[yaserde(prefix = "tt", rename = "UseCount")]
    pub use_count: i32,

    // Token that uniquely references this configuration. Length up to 64
    // characters.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoSourceConfigurationExtension {
    // Optional element to configure rotation of captured image.
    // What resolutions a device supports shall be unaffected by the Rotate
    // parameters.
    #[yaserde(prefix = "tt", rename = "Rotate")]
    pub rotate: Option<Rotate>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<VideoSourceConfigurationExtension2>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoSourceConfigurationExtension2 {
    // Optional element describing the geometric lens distortion. Multiple instances
    // for future variable lens support.
    #[yaserde(prefix = "tt", rename = "LensDescription")]
    pub lens_description: Vec<LensDescription>,

    // Optional element describing the scene orientation in the camera’s field of
    // view.
    #[yaserde(prefix = "tt", rename = "SceneOrientation")]
    pub scene_orientation: SceneOrientation,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Rotate {
    // Parameter to enable/disable Rotation feature.
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: RotateMode,

    // Optional parameter to configure how much degree of clockwise rotation of
    // image for On mode. Omitting this parameter for On mode means 180 degree
    // rotation.
    #[yaserde(prefix = "tt", rename = "Degree")]
    pub degree: Option<i32>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<RotateExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RotateExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum RotateMode {
    // Enable the Rotate feature. Degree of rotation is specified Degree parameter.
    Off,
    // Disable the Rotate feature.
    On,
    // Rotate feature is automatically activated by the device.
    Auto,

    __Unknown__(String),
}

impl Default for RotateMode {
    fn default() -> RotateMode {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct LensProjection {
    // Angle of incidence.
    #[yaserde(prefix = "tt", rename = "Angle")]
    pub angle: f64,

    // Mapping radius as a consequence of the emergent angle.
    #[yaserde(prefix = "tt", rename = "Radius")]
    pub radius: f64,

    // Optional ray absorption at the given angle due to vignetting. A value of one
    // means no absorption.
    #[yaserde(prefix = "tt", rename = "Transmittance")]
    pub transmittance: Option<f64>,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct LensOffset {
    // Optional horizontal offset of the lens center in normalized coordinates.
    #[yaserde(attribute, rename = "x")]
    pub x: Option<f64>,

    // Optional vertical offset of the lens center in normalized coordinates.
    #[yaserde(attribute, rename = "y")]
    pub y: Option<f64>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct LensDescription {
    // Offset of the lens center to the imager center in normalized coordinates.
    #[yaserde(prefix = "tt", rename = "Offset")]
    pub offset: LensOffset,

    // Radial description of the projection characteristics. The resulting curve is
    // defined by the B-Spline interpolation
    // over the given elements. The element for Radius zero shall not be provided.
    // The projection points shall be ordered with ascending Radius.
    // Items outside the last projection Radius shall be assumed to be invisible
    // (black).
    #[yaserde(prefix = "tt", rename = "Projection")]
    pub projection: Vec<LensProjection>,

    // Compensation of the x coordinate needed for the ONVIF normalized coordinate
    // system.
    #[yaserde(prefix = "tt", rename = "XFactor")]
    pub x_factor: f64,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // Optional focal length of the optical system.
    #[yaserde(attribute, rename = "FocalLength")]
    pub focal_length: Option<f64>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoSourceConfigurationOptions {
    // Supported range for the capturing area.
    // Device that does not support cropped streaming shall express BoundsRange
    // option as mentioned below
    // BoundsRange->XRange and BoundsRange->YRange with same Min/Max values
    // HeightRange and WidthRange Min/Max values same as VideoSource Height and
    // Width Limits.
    #[yaserde(prefix = "tt", rename = "BoundsRange")]
    pub bounds_range: IntRectangleRange,

    // List of physical inputs.
    #[yaserde(prefix = "tt", rename = "VideoSourceTokensAvailable")]
    pub video_source_tokens_available: Vec<ReferenceToken>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<VideoSourceConfigurationOptionsExtension>,

    // Maximum number of profiles.
    #[yaserde(attribute, rename = "MaximumNumberOfProfiles")]
    pub maximum_number_of_profiles: Option<i32>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoSourceConfigurationOptionsExtension {
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // Options of parameters for Rotation feature.
    #[yaserde(prefix = "tt", rename = "Rotate")]
    pub rotate: Option<RotateOptions>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<VideoSourceConfigurationOptionsExtension2>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoSourceConfigurationOptionsExtension2 {
    // Scene orientation modes supported by the device for this configuration.
    #[yaserde(prefix = "tt", rename = "SceneOrientationMode")]
    pub scene_orientation_mode: Vec<SceneOrientationMode>,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RotateOptions {
    // Supported options of Rotate mode parameter.
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: Vec<RotateMode>,

    // List of supported degree value for rotation.
    #[yaserde(prefix = "tt", rename = "DegreeList")]
    pub degree_list: Option<IntList>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<RotateOptionsExtension>,

    // After setting the rotation, if a device starts to reboot this value is true.
    // If a device can handle rotation setting without rebooting this value is
    // false.
    #[yaserde(attribute, rename = "Reboot")]
    pub reboot: Option<bool>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RotateOptionsExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum SceneOrientationMode {
    Manual,
    Auto,

    __Unknown__(String),
}

impl Default for SceneOrientationMode {
    fn default() -> SceneOrientationMode {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum SceneOrientationOption {
    Below,
    Horizon,
    Above,

    __Unknown__(String),
}

impl Default for SceneOrientationOption {
    fn default() -> SceneOrientationOption {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SceneOrientation {
    // Parameter to assign the way the camera determines the scene orientation.
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: SceneOrientationMode,

    // Assigned or determined scene orientation based on the Mode. When assigning
    // the Mode to AUTO, this field
    // is optional and will be ignored by the device. When assigning the Mode to
    // MANUAL, this field is required
    // and the device will return an InvalidArgs fault if missing.
    #[yaserde(prefix = "tt", rename = "Orientation")]
    pub orientation: Option<String>,
}

// Source view modes supported by device.
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum ViewModes {
    // Undewarped viewmode from device supporting fisheye lens.
    Fisheye,
    // 360 degree panoramic view.
    _360Panorama,
    // 180 degree panoramic view.
    _180Panorama,
    // View mode combining four streams in single Quad, eg., applicable for devices
    // supporting four heads.
    Quad,
    // Unaltered view from the sensor.
    Original,
    // Viewmode combining the left side sensors, applicable for devices supporting
    // multiple sensors.
    LeftHalf,
    // Viewmode combining the right side sensors, applicable for devices supporting
    // multiple sensors.
    RightHalf,
    // Dewarped view mode for device supporting fisheye lens.
    Dewarp,

    __Unknown__(String),
}

impl Default for ViewModes {
    fn default() -> ViewModes {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoEncoderConfiguration {
    // Used video codec, either Jpeg, H.264 or Mpeg4
    #[yaserde(prefix = "tt", rename = "Encoding")]
    pub encoding: VideoEncoding,

    // Configured video resolution
    #[yaserde(prefix = "tt", rename = "Resolution")]
    pub resolution: VideoResolution,

    // Relative value for the video quantizers and the quality of the video. A high
    // value within supported quality range means higher quality
    #[yaserde(prefix = "tt", rename = "Quality")]
    pub quality: f64,

    // Optional element to configure rate control related parameters.
    #[yaserde(prefix = "tt", rename = "RateControl")]
    pub rate_control: Option<VideoRateControl>,

    // Optional element to configure Mpeg4 related parameters.
    #[yaserde(prefix = "tt", rename = "MPEG4")]
    pub mpeg4: Option<Mpeg4Configuration>,

    // Optional element to configure H.264 related parameters.
    #[yaserde(prefix = "tt", rename = "H264")]
    pub h264: Option<H264Configuration>,

    // Defines the multicast settings that could be used for video streaming.
    #[yaserde(prefix = "tt", rename = "Multicast")]
    pub multicast: MulticastConfiguration,

    // The rtsp session timeout for the related video stream
    #[yaserde(prefix = "tt", rename = "SessionTimeout")]
    pub session_timeout: xs::Duration,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // A value of true indicates that frame rate is a fixed value rather than an
    // upper limit,
    // and that the video encoder shall prioritize frame rate over all other
    // adaptable
    // configuration values such as bitrate. Default is false.
    #[yaserde(attribute, rename = "GuaranteedFrameRate")]
    pub guaranteed_frame_rate: Option<bool>,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // User readable name. Length up to 64 characters.
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: Name,

    // Number of internal references currently using this configuration.
    #[yaserde(prefix = "tt", rename = "UseCount")]
    pub use_count: i32,

    // Token that uniquely references this configuration. Length up to 64
    // characters.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum VideoEncoding {
    Jpeg,
    Mpeg4,
    H264,

    __Unknown__(String),
}

impl Default for VideoEncoding {
    fn default() -> VideoEncoding {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum Mpeg4Profile {
    Sp,
    Asp,

    __Unknown__(String),
}

impl Default for Mpeg4Profile {
    fn default() -> Mpeg4Profile {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum H264Profile {
    Baseline,
    Main,
    Extended,
    High,

    __Unknown__(String),
}

impl Default for H264Profile {
    fn default() -> H264Profile {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoResolution {
    // Number of the columns of the Video image.
    #[yaserde(prefix = "tt", rename = "Width")]
    pub width: i32,

    // Number of the lines of the Video image.
    #[yaserde(prefix = "tt", rename = "Height")]
    pub height: i32,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoRateControl {
    // Maximum output framerate in fps. If an EncodingInterval is provided the
    // resulting encoded framerate will be reduced by the given factor.
    #[yaserde(prefix = "tt", rename = "FrameRateLimit")]
    pub frame_rate_limit: i32,

    // Interval at which images are encoded and transmitted. (A value of 1 means
    // that every frame is encoded, a value of 2 means that every 2nd frame is
    // encoded ...)
    #[yaserde(prefix = "tt", rename = "EncodingInterval")]
    pub encoding_interval: i32,

    // the maximum output bitrate in kbps
    #[yaserde(prefix = "tt", rename = "BitrateLimit")]
    pub bitrate_limit: i32,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Mpeg4Configuration {
    // Determines the interval in which the I-Frames will be coded. An entry of 1
    // indicates I-Frames are continuously generated. An entry of 2 indicates that
    // every 2nd image is an I-Frame, and 3 only every 3rd frame, etc. The frames in
    // between are coded as P or B Frames.
    #[yaserde(prefix = "tt", rename = "GovLength")]
    pub gov_length: i32,

    // the Mpeg4 profile, either simple profile (SP) or advanced simple profile
    // (ASP)
    #[yaserde(prefix = "tt", rename = "Mpeg4Profile")]
    pub mpeg_4_profile: Mpeg4Profile,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct H264Configuration {
    // Group of Video frames length. Determines typically the interval in which the
    // I-Frames will be coded. An entry of 1 indicates I-Frames are continuously
    // generated. An entry of 2 indicates that every 2nd image is an I-Frame, and 3
    // only every 3rd frame, etc. The frames in between are coded as P or B Frames.
    #[yaserde(prefix = "tt", rename = "GovLength")]
    pub gov_length: i32,

    // the H.264 profile, either baseline, main, extended or high
    #[yaserde(prefix = "tt", rename = "H264Profile")]
    pub h264_profile: H264Profile,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoEncoderConfigurationOptions {
    // Range of the quality values. A high value means higher quality.
    #[yaserde(prefix = "tt", rename = "QualityRange")]
    pub quality_range: IntRange,

    // Optional JPEG encoder settings ranges (See also Extension element).
    #[yaserde(prefix = "tt", rename = "JPEG")]
    pub jpeg: Option<JpegOptions>,

    // Optional MPEG-4 encoder settings ranges (See also Extension element).
    #[yaserde(prefix = "tt", rename = "MPEG4")]
    pub mpeg4: Option<Mpeg4Options>,

    // Optional H.264 encoder settings ranges (See also Extension element).
    #[yaserde(prefix = "tt", rename = "H264")]
    pub h264: Option<H264Options>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<VideoEncoderOptionsExtension>,

    // Indicates the support for the GuaranteedFrameRate attribute on the
    // VideoEncoderConfiguration element.
    #[yaserde(attribute, rename = "GuaranteedFrameRateSupported")]
    pub guaranteed_frame_rate_supported: Option<bool>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoEncoderOptionsExtension {
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // Optional JPEG encoder settings ranges.
    #[yaserde(prefix = "tt", rename = "JPEG")]
    pub jpeg: Option<JpegOptions2>,

    // Optional MPEG-4 encoder settings ranges.
    #[yaserde(prefix = "tt", rename = "MPEG4")]
    pub mpeg4: Option<Mpeg4Options2>,

    // Optional H.264 encoder settings ranges.
    #[yaserde(prefix = "tt", rename = "H264")]
    pub h264: Option<H264Options2>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<VideoEncoderOptionsExtension2>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoEncoderOptionsExtension2 {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct JpegOptions {
    // List of supported image sizes.
    #[yaserde(prefix = "tt", rename = "ResolutionsAvailable")]
    pub resolutions_available: Vec<VideoResolution>,

    // Supported frame rate in fps (frames per second).
    #[yaserde(prefix = "tt", rename = "FrameRateRange")]
    pub frame_rate_range: IntRange,

    // Supported encoding interval range. The encoding interval corresponds to the
    // number of frames devided by the encoded frames. An encoding interval value of
    // "1" means that all frames are encoded.
    #[yaserde(prefix = "tt", rename = "EncodingIntervalRange")]
    pub encoding_interval_range: IntRange,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct JpegOptions2 {
    // Supported range of encoded bitrate in kbps.
    #[yaserde(prefix = "tt", rename = "BitrateRange")]
    pub bitrate_range: IntRange,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // List of supported image sizes.
    #[yaserde(prefix = "tt", rename = "ResolutionsAvailable")]
    pub resolutions_available: Vec<VideoResolution>,

    // Supported frame rate in fps (frames per second).
    #[yaserde(prefix = "tt", rename = "FrameRateRange")]
    pub frame_rate_range: IntRange,

    // Supported encoding interval range. The encoding interval corresponds to the
    // number of frames devided by the encoded frames. An encoding interval value of
    // "1" means that all frames are encoded.
    #[yaserde(prefix = "tt", rename = "EncodingIntervalRange")]
    pub encoding_interval_range: IntRange,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Mpeg4Options {
    // List of supported image sizes.
    #[yaserde(prefix = "tt", rename = "ResolutionsAvailable")]
    pub resolutions_available: Vec<VideoResolution>,

    // Supported group of Video frames length. This value typically corresponds to
    // the I-Frame distance.
    #[yaserde(prefix = "tt", rename = "GovLengthRange")]
    pub gov_length_range: IntRange,

    // Supported frame rate in fps (frames per second).
    #[yaserde(prefix = "tt", rename = "FrameRateRange")]
    pub frame_rate_range: IntRange,

    // Supported encoding interval range. The encoding interval corresponds to the
    // number of frames devided by the encoded frames. An encoding interval value of
    // "1" means that all frames are encoded.
    #[yaserde(prefix = "tt", rename = "EncodingIntervalRange")]
    pub encoding_interval_range: IntRange,

    // List of supported MPEG-4 profiles.
    #[yaserde(prefix = "tt", rename = "Mpeg4ProfilesSupported")]
    pub mpeg_4_profiles_supported: Vec<Mpeg4Profile>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Mpeg4Options2 {
    // Supported range of encoded bitrate in kbps.
    #[yaserde(prefix = "tt", rename = "BitrateRange")]
    pub bitrate_range: IntRange,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // List of supported image sizes.
    #[yaserde(prefix = "tt", rename = "ResolutionsAvailable")]
    pub resolutions_available: Vec<VideoResolution>,

    // Supported group of Video frames length. This value typically corresponds to
    // the I-Frame distance.
    #[yaserde(prefix = "tt", rename = "GovLengthRange")]
    pub gov_length_range: IntRange,

    // Supported frame rate in fps (frames per second).
    #[yaserde(prefix = "tt", rename = "FrameRateRange")]
    pub frame_rate_range: IntRange,

    // Supported encoding interval range. The encoding interval corresponds to the
    // number of frames devided by the encoded frames. An encoding interval value of
    // "1" means that all frames are encoded.
    #[yaserde(prefix = "tt", rename = "EncodingIntervalRange")]
    pub encoding_interval_range: IntRange,

    // List of supported MPEG-4 profiles.
    #[yaserde(prefix = "tt", rename = "Mpeg4ProfilesSupported")]
    pub mpeg_4_profiles_supported: Vec<Mpeg4Profile>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct H264Options {
    // List of supported image sizes.
    #[yaserde(prefix = "tt", rename = "ResolutionsAvailable")]
    pub resolutions_available: Vec<VideoResolution>,

    // Supported group of Video frames length. This value typically corresponds to
    // the I-Frame distance.
    #[yaserde(prefix = "tt", rename = "GovLengthRange")]
    pub gov_length_range: IntRange,

    // Supported frame rate in fps (frames per second).
    #[yaserde(prefix = "tt", rename = "FrameRateRange")]
    pub frame_rate_range: IntRange,

    // Supported encoding interval range. The encoding interval corresponds to the
    // number of frames devided by the encoded frames. An encoding interval value of
    // "1" means that all frames are encoded.
    #[yaserde(prefix = "tt", rename = "EncodingIntervalRange")]
    pub encoding_interval_range: IntRange,

    // List of supported H.264 profiles.
    #[yaserde(prefix = "tt", rename = "H264ProfilesSupported")]
    pub h264_profiles_supported: Vec<H264Profile>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct H264Options2 {
    // Supported range of encoded bitrate in kbps.
    #[yaserde(prefix = "tt", rename = "BitrateRange")]
    pub bitrate_range: IntRange,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // List of supported image sizes.
    #[yaserde(prefix = "tt", rename = "ResolutionsAvailable")]
    pub resolutions_available: Vec<VideoResolution>,

    // Supported group of Video frames length. This value typically corresponds to
    // the I-Frame distance.
    #[yaserde(prefix = "tt", rename = "GovLengthRange")]
    pub gov_length_range: IntRange,

    // Supported frame rate in fps (frames per second).
    #[yaserde(prefix = "tt", rename = "FrameRateRange")]
    pub frame_rate_range: IntRange,

    // Supported encoding interval range. The encoding interval corresponds to the
    // number of frames devided by the encoded frames. An encoding interval value of
    // "1" means that all frames are encoded.
    #[yaserde(prefix = "tt", rename = "EncodingIntervalRange")]
    pub encoding_interval_range: IntRange,

    // List of supported H.264 profiles.
    #[yaserde(prefix = "tt", rename = "H264ProfilesSupported")]
    pub h264_profiles_supported: Vec<H264Profile>,
}

// Video Media Subtypes as referenced by IANA (without the leading "video/"
// Video Media Type). See also
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum VideoEncodingMimeNames {
    Jpeg,
    Mpv4Es,
    H264,
    H265,

    __Unknown__(String),
}

impl Default for VideoEncodingMimeNames {
    fn default() -> VideoEncodingMimeNames {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum VideoEncodingProfiles {
    Simple,
    AdvancedSimple,
    Baseline,
    Main,
    Main10,
    Extended,
    High,

    __Unknown__(String),
}

impl Default for VideoEncodingProfiles {
    fn default() -> VideoEncodingProfiles {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoEncoder2Configuration {
    // Video Media Subtype for the video format. For definitions see
    // tt:VideoEncodingMimeNames and
    #[yaserde(prefix = "tt", rename = "Encoding")]
    pub encoding: String,

    // Configured video resolution
    #[yaserde(prefix = "tt", rename = "Resolution")]
    pub resolution: VideoResolution2,

    // Optional element to configure rate control related parameters.
    #[yaserde(prefix = "tt", rename = "RateControl")]
    pub rate_control: Option<VideoRateControl2>,

    // Defines the multicast settings that could be used for video streaming.
    #[yaserde(prefix = "tt", rename = "Multicast")]
    pub multicast: Option<MulticastConfiguration>,

    // Relative value for the video quantizers and the quality of the video. A high
    // value within supported quality range means higher quality
    #[yaserde(prefix = "tt", rename = "Quality")]
    pub quality: f64,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // Group of Video frames length. Determines typically the interval in which the
    // I-Frames will be coded. An entry of 1 indicates I-Frames are continuously
    // generated. An entry of 2 indicates that every 2nd image is an I-Frame, and 3
    // only every 3rd frame, etc. The frames in between are coded as P or B Frames.
    #[yaserde(attribute, rename = "GovLength")]
    pub gov_length: Option<i32>,

    // The encoder profile as defined in tt:VideoEncodingProfiles.
    #[yaserde(attribute, rename = "Profile")]
    pub profile: Option<String>,

    // A value of true indicates that frame rate is a fixed value rather than an
    // upper limit,
    // and that the video encoder shall prioritize frame rate over all other
    // adaptable
    // configuration values such as bitrate. Default is false.
    #[yaserde(attribute, rename = "GuaranteedFrameRate")]
    pub guaranteed_frame_rate: Option<bool>,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // User readable name. Length up to 64 characters.
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: Name,

    // Number of internal references currently using this configuration.
    #[yaserde(prefix = "tt", rename = "UseCount")]
    pub use_count: i32,

    // Token that uniquely references this configuration. Length up to 64
    // characters.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoResolution2 {
    // Number of the columns of the Video image.
    #[yaserde(prefix = "tt", rename = "Width")]
    pub width: i32,

    // Number of the lines of the Video image.
    #[yaserde(prefix = "tt", rename = "Height")]
    pub height: i32,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoRateControl2 {
    // Desired frame rate in fps. The actual rate may be lower due to e.g.
    // performance limitations.
    #[yaserde(prefix = "tt", rename = "FrameRateLimit")]
    pub frame_rate_limit: f64,

    // the maximum output bitrate in kbps
    #[yaserde(prefix = "tt", rename = "BitrateLimit")]
    pub bitrate_limit: i32,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // Enforce constant bitrate.
    #[yaserde(attribute, rename = "ConstantBitRate")]
    pub constant_bit_rate: Option<bool>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoEncoder2ConfigurationOptions {
    // Video Media Subtype for the video format. For definitions see
    // tt:VideoEncodingMimeNames and
    #[yaserde(prefix = "tt", rename = "Encoding")]
    pub encoding: String,

    // Range of the quality values. A high value means higher quality.
    #[yaserde(prefix = "tt", rename = "QualityRange")]
    pub quality_range: FloatRange,

    // List of supported image sizes.
    #[yaserde(prefix = "tt", rename = "ResolutionsAvailable")]
    pub resolutions_available: Vec<VideoResolution2>,

    // Supported range of encoded bitrate in kbps.
    #[yaserde(prefix = "tt", rename = "BitrateRange")]
    pub bitrate_range: IntRange,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // Exactly two values, which define the Lower and Upper bounds for the supported
    // group of Video frames length. These values typically correspond to the
    // I-Frame distance.
    #[yaserde(attribute, rename = "GovLengthRange")]
    pub gov_length_range: Option<IntAttrList>,

    // List of supported target frame rates in fps (frames per second). The list
    // shall be sorted with highest values first.
    #[yaserde(attribute, rename = "FrameRatesSupported")]
    pub frame_rates_supported: Option<FloatAttrList>,

    // List of supported encoder profiles as defined in tt::VideoEncodingProfiles.
    #[yaserde(attribute, rename = "ProfilesSupported")]
    pub profiles_supported: Option<StringAttrList>,

    // Signal whether enforcing constant bitrate is supported.
    #[yaserde(attribute, rename = "ConstantBitRateSupported")]
    pub constant_bit_rate_supported: Option<bool>,

    // Indicates the support for the GuaranteedFrameRate attribute on the
    // VideoEncoder2Configuration element.
    #[yaserde(attribute, rename = "GuaranteedFrameRateSupported")]
    pub guaranteed_frame_rate_supported: Option<bool>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AudioSourceConfiguration {
    // Token of the Audio Source the configuration applies to
    #[yaserde(prefix = "tt", rename = "SourceToken")]
    pub source_token: ReferenceToken,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // User readable name. Length up to 64 characters.
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: Name,

    // Number of internal references currently using this configuration.
    #[yaserde(prefix = "tt", rename = "UseCount")]
    pub use_count: i32,

    // Token that uniquely references this configuration. Length up to 64
    // characters.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AudioSourceConfigurationOptions {
    // Tokens of the audio source the configuration can be used for.
    #[yaserde(prefix = "tt", rename = "InputTokensAvailable")]
    pub input_tokens_available: Vec<ReferenceToken>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<AudioSourceOptionsExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AudioSourceOptionsExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AudioEncoderConfiguration {
    // Audio codec used for encoding the audio input (either G.711, G.726 or AAC)
    #[yaserde(prefix = "tt", rename = "Encoding")]
    pub encoding: AudioEncoding,

    // The output bitrate in kbps.
    #[yaserde(prefix = "tt", rename = "Bitrate")]
    pub bitrate: i32,

    // The output sample rate in kHz.
    #[yaserde(prefix = "tt", rename = "SampleRate")]
    pub sample_rate: i32,

    // Defines the multicast settings that could be used for video streaming.
    #[yaserde(prefix = "tt", rename = "Multicast")]
    pub multicast: MulticastConfiguration,

    // The rtsp session timeout for the related audio stream
    #[yaserde(prefix = "tt", rename = "SessionTimeout")]
    pub session_timeout: xs::Duration,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // User readable name. Length up to 64 characters.
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: Name,

    // Number of internal references currently using this configuration.
    #[yaserde(prefix = "tt", rename = "UseCount")]
    pub use_count: i32,

    // Token that uniquely references this configuration. Length up to 64
    // characters.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum AudioEncoding {
    G711,
    G726,
    Aac,

    __Unknown__(String),
}

impl Default for AudioEncoding {
    fn default() -> AudioEncoding {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AudioEncoderConfigurationOptions {
    // list of supported AudioEncoderConfigurations
    #[yaserde(prefix = "tt", rename = "Options")]
    pub options: Vec<AudioEncoderConfigurationOption>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AudioEncoderConfigurationOption {
    // The enoding used for audio data (either G.711, G.726 or AAC)
    #[yaserde(prefix = "tt", rename = "Encoding")]
    pub encoding: AudioEncoding,

    // List of supported bitrates in kbps for the specified Encoding
    #[yaserde(prefix = "tt", rename = "BitrateList")]
    pub bitrate_list: IntList,

    // List of supported Sample Rates in kHz for the specified Encoding
    #[yaserde(prefix = "tt", rename = "SampleRateList")]
    pub sample_rate_list: IntList,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

// Audio Media Subtypes as referenced by IANA (without the leading "audio/"
// Audio Media Type). See also
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum AudioEncodingMimeNames {
    Pcmu,
    G726,
    Mp4ALatm,
    Mpeg4Generic,

    __Unknown__(String),
}

impl Default for AudioEncodingMimeNames {
    fn default() -> AudioEncodingMimeNames {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AudioEncoder2Configuration {
    // Audio Media Subtype for the audio format. For definitions see
    // tt:AudioEncodingMimeNames and
    #[yaserde(prefix = "tt", rename = "Encoding")]
    pub encoding: String,

    // Optional multicast configuration of the audio stream.
    #[yaserde(prefix = "tt", rename = "Multicast")]
    pub multicast: Option<MulticastConfiguration>,

    // The output bitrate in kbps.
    #[yaserde(prefix = "tt", rename = "Bitrate")]
    pub bitrate: i32,

    // The output sample rate in kHz.
    #[yaserde(prefix = "tt", rename = "SampleRate")]
    pub sample_rate: i32,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // User readable name. Length up to 64 characters.
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: Name,

    // Number of internal references currently using this configuration.
    #[yaserde(prefix = "tt", rename = "UseCount")]
    pub use_count: i32,

    // Token that uniquely references this configuration. Length up to 64
    // characters.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AudioEncoder2ConfigurationOptions {
    // Audio Media Subtype for the audio format. For definitions see
    // tt:AudioEncodingMimeNames and
    #[yaserde(prefix = "tt", rename = "Encoding")]
    pub encoding: String,

    // List of supported bitrates in kbps for the specified Encoding
    #[yaserde(prefix = "tt", rename = "BitrateList")]
    pub bitrate_list: IntList,

    // List of supported Sample Rates in kHz for the specified Encoding
    #[yaserde(prefix = "tt", rename = "SampleRateList")]
    pub sample_rate_list: IntList,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoAnalyticsConfiguration {
    #[yaserde(prefix = "tt", rename = "AnalyticsEngineConfiguration")]
    pub analytics_engine_configuration: AnalyticsEngineConfiguration,

    #[yaserde(prefix = "tt", rename = "RuleEngineConfiguration")]
    pub rule_engine_configuration: RuleEngineConfiguration,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // User readable name. Length up to 64 characters.
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: Name,

    // Number of internal references currently using this configuration.
    #[yaserde(prefix = "tt", rename = "UseCount")]
    pub use_count: i32,

    // Token that uniquely references this configuration. Length up to 64
    // characters.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MetadataConfiguration {
    // optional element to configure which PTZ related data is to include in the
    // metadata stream
    #[yaserde(prefix = "tt", rename = "PTZStatus")]
    pub ptz_status: Option<Ptzfilter>,

    // Optional element to configure the streaming of events. A client might be
    // interested in receiving all,
    // none or some of the events produced by the device:
    #[yaserde(prefix = "tt", rename = "Events")]
    pub events: Option<EventSubscription>,

    // Defines whether the streamed metadata will include metadata from the
    // analytics engines (video, cell motion, audio etc.)
    #[yaserde(prefix = "tt", rename = "Analytics")]
    pub analytics: Option<bool>,

    // Defines the multicast settings that could be used for video streaming.
    #[yaserde(prefix = "tt", rename = "Multicast")]
    pub multicast: MulticastConfiguration,

    // The rtsp session timeout for the related audio stream (when using Media2
    // Service, this value is deprecated and ignored)
    #[yaserde(prefix = "tt", rename = "SessionTimeout")]
    pub session_timeout: xs::Duration,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,
    #[yaserde(prefix = "tt", rename = "AnalyticsEngineConfiguration")]
    pub analytics_engine_configuration: Option<AnalyticsEngineConfiguration>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<MetadataConfigurationExtension>,

    // Optional parameter to configure compression type of Metadata payload. Use
    // values from enumeration MetadataCompressionType.
    #[yaserde(attribute, rename = "CompressionType")]
    pub compression_type: Option<String>,

    // Optional parameter to configure if the metadata stream shall contain the Geo
    // Location coordinates of each target.
    #[yaserde(attribute, rename = "GeoLocation")]
    pub geo_location: Option<bool>,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // User readable name. Length up to 64 characters.
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: Name,

    // Number of internal references currently using this configuration.
    #[yaserde(prefix = "tt", rename = "UseCount")]
    pub use_count: i32,

    // Token that uniquely references this configuration. Length up to 64
    // characters.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MetadataConfigurationExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Ptzfilter {
    // True if the metadata stream shall contain the PTZ status (IDLE, MOVING or
    // UNKNOWN)
    #[yaserde(prefix = "tt", rename = "Status")]
    pub status: bool,

    // True if the metadata stream shall contain the PTZ position
    #[yaserde(prefix = "tt", rename = "Position")]
    pub position: bool,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

// Subcription handling in the same way as base notification subscription.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct EventSubscription {
    #[yaserde(prefix = "tt", rename = "Filter")]
    pub filter: Option<wsnt::FilterType>,

    #[yaserde(prefix = "tt", rename = "SubscriptionPolicy")]
    pub subscription_policy: Option<SubscriptionPolicyType>,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SubscriptionPolicyType {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MetadataConfigurationOptions {
    #[yaserde(prefix = "tt", rename = "PTZStatusFilterOptions")]
    pub ptz_status_filter_options: PtzstatusFilterOptions,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,
    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<MetadataConfigurationOptionsExtension>,

    // True if the device is able to stream the Geo Located positions of each
    // target.
    #[yaserde(attribute, rename = "GeoLocation")]
    pub geo_location: Option<bool>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MetadataConfigurationOptionsExtension {
    // List of supported metadata compression type. Its options shall be chosen from
    // tt:MetadataCompressionType.
    #[yaserde(prefix = "tt", rename = "CompressionType")]
    pub compression_type: Vec<String>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<MetadataConfigurationOptionsExtension2>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MetadataConfigurationOptionsExtension2 {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum MetadataCompressionType {
    None,
    Gzip,
    Exi,

    __Unknown__(String),
}

impl Default for MetadataCompressionType {
    fn default() -> MetadataCompressionType {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzstatusFilterOptions {
    // True if the device is able to stream pan or tilt status information.
    #[yaserde(prefix = "tt", rename = "PanTiltStatusSupported")]
    pub pan_tilt_status_supported: bool,

    // True if the device is able to stream zoom status inforamtion.
    #[yaserde(prefix = "tt", rename = "ZoomStatusSupported")]
    pub zoom_status_supported: bool,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // True if the device is able to stream the pan or tilt position.
    #[yaserde(prefix = "tt", rename = "PanTiltPositionSupported")]
    pub pan_tilt_position_supported: Option<bool>,

    // True if the device is able to stream zoom position information.
    #[yaserde(prefix = "tt", rename = "ZoomPositionSupported")]
    pub zoom_position_supported: Option<bool>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<PtzstatusFilterOptionsExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzstatusFilterOptionsExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoOutput {
    #[yaserde(prefix = "tt", rename = "Layout")]
    pub layout: Layout,

    // Resolution of the display in Pixel.
    #[yaserde(prefix = "tt", rename = "Resolution")]
    pub resolution: Option<VideoResolution>,

    // Refresh rate of the display in Hertz.
    #[yaserde(prefix = "tt", rename = "RefreshRate")]
    pub refresh_rate: Option<f64>,

    // Aspect ratio of the display as physical extent of width divided by height.
    #[yaserde(prefix = "tt", rename = "AspectRatio")]
    pub aspect_ratio: Option<f64>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<VideoOutputExtension>,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // Unique identifier referencing the physical entity.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoOutputExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoOutputConfiguration {
    // Token of the Video Output the configuration applies to
    #[yaserde(prefix = "tt", rename = "OutputToken")]
    pub output_token: ReferenceToken,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // User readable name. Length up to 64 characters.
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: Name,

    // Number of internal references currently using this configuration.
    #[yaserde(prefix = "tt", rename = "UseCount")]
    pub use_count: i32,

    // Token that uniquely references this configuration. Length up to 64
    // characters.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoOutputConfigurationOptions {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,

// //
//TODO: any_attribute macros
//  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoDecoderConfigurationOptions {
    // If the device is able to decode Jpeg streams this element describes the
    // supported codecs and configurations
    #[yaserde(prefix = "tt", rename = "JpegDecOptions")]
    pub jpeg_dec_options: Option<JpegDecOptions>,

    // If the device is able to decode H.264 streams this element describes the
    // supported codecs and configurations
    #[yaserde(prefix = "tt", rename = "H264DecOptions")]
    pub h264_dec_options: Option<H264DecOptions>,

    // If the device is able to decode Mpeg4 streams this element describes the
    // supported codecs and configurations
    #[yaserde(prefix = "tt", rename = "Mpeg4DecOptions")]
    pub mpeg_4_dec_options: Option<Mpeg4DecOptions>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<VideoDecoderConfigurationOptionsExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct H264DecOptions {
    // List of supported H.264 Video Resolutions
    #[yaserde(prefix = "tt", rename = "ResolutionsAvailable")]
    pub resolutions_available: Vec<VideoResolution>,

    // List of supported H264 Profiles (either baseline, main, extended or high)
    #[yaserde(prefix = "tt", rename = "SupportedH264Profiles")]
    pub supported_h264_profiles: Vec<H264Profile>,

    // Supported H.264 bitrate range in kbps
    #[yaserde(prefix = "tt", rename = "SupportedInputBitrate")]
    pub supported_input_bitrate: IntRange,

    // Supported H.264 framerate range in fps
    #[yaserde(prefix = "tt", rename = "SupportedFrameRate")]
    pub supported_frame_rate: IntRange,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct JpegDecOptions {
    // List of supported Jpeg Video Resolutions
    #[yaserde(prefix = "tt", rename = "ResolutionsAvailable")]
    pub resolutions_available: Vec<VideoResolution>,

    // Supported Jpeg bitrate range in kbps
    #[yaserde(prefix = "tt", rename = "SupportedInputBitrate")]
    pub supported_input_bitrate: IntRange,

    // Supported Jpeg framerate range in fps
    #[yaserde(prefix = "tt", rename = "SupportedFrameRate")]
    pub supported_frame_rate: IntRange,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Mpeg4DecOptions {
    // List of supported Mpeg4 Video Resolutions
    #[yaserde(prefix = "tt", rename = "ResolutionsAvailable")]
    pub resolutions_available: Vec<VideoResolution>,

    // List of supported Mpeg4 Profiles (either SP or ASP)
    #[yaserde(prefix = "tt", rename = "SupportedMpeg4Profiles")]
    pub supported_mpeg_4_profiles: Vec<Mpeg4Profile>,

    // Supported Mpeg4 bitrate range in kbps
    #[yaserde(prefix = "tt", rename = "SupportedInputBitrate")]
    pub supported_input_bitrate: IntRange,

    // Supported Mpeg4 framerate range in fps
    #[yaserde(prefix = "tt", rename = "SupportedFrameRate")]
    pub supported_frame_rate: IntRange,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoDecoderConfigurationOptionsExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AudioOutput {
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // Unique identifier referencing the physical entity.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AudioOutputConfiguration {
    // Token of the phsycial Audio output.
    #[yaserde(prefix = "tt", rename = "OutputToken")]
    pub output_token: ReferenceToken,

    // An audio channel MAY support different types of audio transmission. While for
    // full duplex
    // operation no special handling is required, in half duplex operation the
    // transmission direction
    // needs to be switched.
    // The optional SendPrimacy parameter inside the AudioOutputConfiguration
    // indicates which
    // direction is currently active. An NVC can switch between different modes by
    // setting the
    // AudioOutputConfiguration.
    #[yaserde(prefix = "tt", rename = "SendPrimacy")]
    pub send_primacy: Option<String>,

    // Volume setting of the output. The applicable range is defined via the option
    // AudioOutputOptions.OutputLevelRange.
    #[yaserde(prefix = "tt", rename = "OutputLevel")]
    pub output_level: i32,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // User readable name. Length up to 64 characters.
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: Name,

    // Number of internal references currently using this configuration.
    #[yaserde(prefix = "tt", rename = "UseCount")]
    pub use_count: i32,

    // Token that uniquely references this configuration. Length up to 64
    // characters.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AudioOutputConfigurationOptions {
    // Tokens of the physical Audio outputs (typically one).
    #[yaserde(prefix = "tt", rename = "OutputTokensAvailable")]
    pub output_tokens_available: Vec<ReferenceToken>,

    // An
    #[yaserde(prefix = "tt", rename = "SendPrimacyOptions")]
    pub send_primacy_options: Vec<String>,

    // Minimum and maximum level range supported for this Output.
    #[yaserde(prefix = "tt", rename = "OutputLevelRange")]
    pub output_level_range: IntRange,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AudioDecoderConfiguration {
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // User readable name. Length up to 64 characters.
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: Name,

    // Number of internal references currently using this configuration.
    #[yaserde(prefix = "tt", rename = "UseCount")]
    pub use_count: i32,

    // Token that uniquely references this configuration. Length up to 64
    // characters.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AudioDecoderConfigurationOptions {
    // If the device is able to decode AAC encoded audio this section describes the
    // supported configurations
    #[yaserde(prefix = "tt", rename = "AACDecOptions")]
    pub aac_dec_options: Option<AacdecOptions>,

    // If the device is able to decode G711 encoded audio this section describes the
    // supported configurations
    #[yaserde(prefix = "tt", rename = "G711DecOptions")]
    pub g711_dec_options: Option<G711DecOptions>,

    // If the device is able to decode G726 encoded audio this section describes the
    // supported configurations
    #[yaserde(prefix = "tt", rename = "G726DecOptions")]
    pub g726_dec_options: Option<G726DecOptions>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<AudioDecoderConfigurationOptionsExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct G711DecOptions {
    // List of supported bitrates in kbps
    #[yaserde(prefix = "tt", rename = "Bitrate")]
    pub bitrate: IntList,

    // List of supported sample rates in kHz
    #[yaserde(prefix = "tt", rename = "SampleRateRange")]
    pub sample_rate_range: IntList,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AacdecOptions {
    // List of supported bitrates in kbps
    #[yaserde(prefix = "tt", rename = "Bitrate")]
    pub bitrate: IntList,

    // List of supported sample rates in kHz
    #[yaserde(prefix = "tt", rename = "SampleRateRange")]
    pub sample_rate_range: IntList,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct G726DecOptions {
    // List of supported bitrates in kbps
    #[yaserde(prefix = "tt", rename = "Bitrate")]
    pub bitrate: IntList,

    // List of supported sample rates in kHz
    #[yaserde(prefix = "tt", rename = "SampleRateRange")]
    pub sample_rate_range: IntList,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AudioDecoderConfigurationOptionsExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MulticastConfiguration {
    // The multicast address (if this address is set to 0 no multicast streaming is
    // enaled)
    #[yaserde(prefix = "tt", rename = "Address")]
    pub address: Ipaddress,

    // The RTP mutlicast destination port. A device may support RTCP. In this case
    // the port value shall be even to allow the corresponding RTCP stream to be
    // mapped to the next higher (odd) destination port number as defined in the
    // RTSP specification.
    #[yaserde(prefix = "tt", rename = "Port")]
    pub port: i32,

    // In case of IPv6 the TTL value is assumed as the hop limit. Note that for IPV6
    // and administratively scoped IPv4 multicast the primary use for hop limit /
    // TTL is to prevent packets from (endlessly) circulating and not limiting
    // scope. In these cases the address contains the scope.
    #[yaserde(prefix = "tt", rename = "TTL")]
    pub ttl: i32,

    // Read only property signalling that streaming is persistant. Use the methods
    // StartMulticastStreaming and StopMulticastStreaming to switch its state.
    #[yaserde(prefix = "tt", rename = "AutoStart")]
    pub auto_start: bool,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct StreamSetup {
    // Defines if a multicast or unicast stream is requested
    #[yaserde(prefix = "tt", rename = "Stream")]
    pub stream: StreamType,

    #[yaserde(prefix = "tt", rename = "Transport")]
    pub transport: Transport,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum StreamType {
    RtpUnicast,
    RtpMulticast,

    __Unknown__(String),
}

impl Default for StreamType {
    fn default() -> StreamType {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Transport {
    // Defines the network protocol for streaming, either UDP=RTP/UDP,
    // RTSP=RTP/RTSP/TCP or HTTP=RTP/RTSP/HTTP/TCP
    #[yaserde(prefix = "tt", rename = "Protocol")]
    pub protocol: TransportProtocol,
    //    // Optional element to describe further tunnel options. This element is normally
    //// not needed
    //    #[yaserde(prefix = "tt", rename = "Tunnel")]
    //    pub tunnel: Option<Transport>,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum TransportProtocol {
    Udp,
    // This value is deprecated.
    Tcp,
    Rtsp,
    Http,

    __Unknown__(String),
}

impl Default for TransportProtocol {
    fn default() -> TransportProtocol {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MediaUri {
    // Stable Uri to be used for requesting the media stream
    #[yaserde(prefix = "tt", rename = "Uri")]
    pub uri: String,

    // Indicates if the Uri is only valid until the connection is established. The
    // value shall be set to "false".
    #[yaserde(prefix = "tt", rename = "InvalidAfterConnect")]
    pub invalid_after_connect: bool,

    // Indicates if the Uri is invalid after a reboot of the device. The value shall
    // be set to "false".
    #[yaserde(prefix = "tt", rename = "InvalidAfterReboot")]
    pub invalid_after_reboot: bool,

    // Duration how long the Uri is valid. This parameter shall be set to PT0S to
    // indicate that this stream URI is indefinitely valid even if the profile
    // changes
    #[yaserde(prefix = "tt", rename = "Timeout")]
    pub timeout: xs::Duration,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum ScopeDefinition {
    Fixed,
    Configurable,

    __Unknown__(String),
}

impl Default for ScopeDefinition {
    fn default() -> ScopeDefinition {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Scope {
    // Indicates if the scope is fixed or configurable.
    #[yaserde(prefix = "tt", rename = "ScopeDef")]
    pub scope_def: ScopeDefinition,

    // Scope item URI.
    #[yaserde(prefix = "tt", rename = "ScopeItem")]
    pub scope_item: String,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum DiscoveryMode {
    Discoverable,
    NonDiscoverable,

    __Unknown__(String),
}

impl Default for DiscoveryMode {
    fn default() -> DiscoveryMode {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct NetworkInterface {
    // Indicates whether or not an interface is enabled.
    #[yaserde(prefix = "tt", rename = "Enabled")]
    pub enabled: bool,

    // Network interface information
    #[yaserde(prefix = "tt", rename = "Info")]
    pub info: Option<NetworkInterfaceInfo>,

    // Link configuration.
    #[yaserde(prefix = "tt", rename = "Link")]
    pub link: Option<NetworkInterfaceLink>,

    // IPv4 network interface configuration.
    #[yaserde(prefix = "tt", rename = "IPv4")]
    pub i_pv_4: Option<Ipv4NetworkInterface>,

    // IPv6 network interface configuration.
    #[yaserde(prefix = "tt", rename = "IPv6")]
    pub i_pv_6: Option<Ipv6NetworkInterface>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<NetworkInterfaceExtension>,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // Unique identifier referencing the physical entity.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct NetworkInterfaceExtension {
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,
    #[yaserde(prefix = "tt", rename = "InterfaceType")]
    pub interface_type: IanaIfTypes,

    // Extension point prepared for future 802.3 configuration.
    #[yaserde(prefix = "tt", rename = "Dot3")]
    pub dot_3: Vec<Dot3Configuration>,

    #[yaserde(prefix = "tt", rename = "Dot11")]
    pub dot_11: Vec<Dot11Configuration>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<NetworkInterfaceExtension2>,
}

#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct NetworkInterfaceConfigPriority(pub i64);

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Dot3Configuration {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,

// //
//TODO: any_attribute macros
//  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct NetworkInterfaceExtension2 {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct NetworkInterfaceLink {
    // Configured link settings.
    #[yaserde(prefix = "tt", rename = "AdminSettings")]
    pub admin_settings: NetworkInterfaceConnectionSetting,

    // Current active link settings.
    #[yaserde(prefix = "tt", rename = "OperSettings")]
    pub oper_settings: NetworkInterfaceConnectionSetting,

    // Integer indicating interface type, for example: 6 is ethernet.
    #[yaserde(prefix = "tt", rename = "InterfaceType")]
    pub interface_type: IanaIfTypes,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct NetworkInterfaceConnectionSetting {
    // Auto negotiation on/off.
    #[yaserde(prefix = "tt", rename = "AutoNegotiation")]
    pub auto_negotiation: bool,

    // Speed.
    #[yaserde(prefix = "tt", rename = "Speed")]
    pub speed: i32,

    // Duplex type, Half or Full.
    #[yaserde(prefix = "tt", rename = "Duplex")]
    pub duplex: Duplex,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum Duplex {
    Full,
    Half,

    __Unknown__(String),
}

impl Default for Duplex {
    fn default() -> Duplex {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct IanaIfTypes(pub i32);

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct NetworkInterfaceInfo {
    // Network interface name, for example eth0.
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: Option<String>,

    // Network interface MAC address.
    #[yaserde(prefix = "tt", rename = "HwAddress")]
    pub hw_address: HwAddress,

    // Maximum transmission unit.
    #[yaserde(prefix = "tt", rename = "MTU")]
    pub mtu: Option<i32>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Ipv6NetworkInterface {
    // Indicates whether or not IPv6 is enabled.
    #[yaserde(prefix = "tt", rename = "Enabled")]
    pub enabled: bool,

    // IPv6 configuration.
    #[yaserde(prefix = "tt", rename = "Config")]
    pub config: Option<Ipv6Configuration>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Ipv4NetworkInterface {
    // Indicates whether or not IPv4 is enabled.
    #[yaserde(prefix = "tt", rename = "Enabled")]
    pub enabled: bool,

    // IPv4 configuration.
    #[yaserde(prefix = "tt", rename = "Config")]
    pub config: Ipv4Configuration,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Ipv4Configuration {
    // List of manually added IPv4 addresses.
    #[yaserde(prefix = "tt", rename = "Manual")]
    pub manual: Vec<PrefixedIPv4Address>,

    // Link local address.
    #[yaserde(prefix = "tt", rename = "LinkLocal")]
    pub link_local: Option<PrefixedIPv4Address>,

    // IPv4 address configured by using DHCP.
    #[yaserde(prefix = "tt", rename = "FromDHCP")]
    pub from_dhcp: Option<PrefixedIPv4Address>,

    // Indicates whether or not DHCP is used.
    #[yaserde(prefix = "tt", rename = "DHCP")]
    pub dhcp: bool,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Ipv6Configuration {
    // Indicates whether router advertisment is used.
    #[yaserde(prefix = "tt", rename = "AcceptRouterAdvert")]
    pub accept_router_advert: Option<bool>,

    // DHCP configuration.
    #[yaserde(prefix = "tt", rename = "DHCP")]
    pub dhcp: Ipv6DHCPConfiguration,

    // List of manually entered IPv6 addresses.
    #[yaserde(prefix = "tt", rename = "Manual")]
    pub manual: Vec<PrefixedIPv6Address>,

    // List of link local IPv6 addresses.
    #[yaserde(prefix = "tt", rename = "LinkLocal")]
    pub link_local: Vec<PrefixedIPv6Address>,

    // List of IPv6 addresses configured by using DHCP.
    #[yaserde(prefix = "tt", rename = "FromDHCP")]
    pub from_dhcp: Vec<PrefixedIPv6Address>,

    // List of IPv6 addresses configured by using router advertisment.
    #[yaserde(prefix = "tt", rename = "FromRA")]
    pub from_ra: Vec<PrefixedIPv6Address>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<Ipv6ConfigurationExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Ipv6ConfigurationExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum Ipv6DHCPConfiguration {
    Auto,
    Stateful,
    Stateless,
    Off,

    __Unknown__(String),
}

impl Default for Ipv6DHCPConfiguration {
    fn default() -> Ipv6DHCPConfiguration {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct NetworkProtocol {
    // Network protocol type string.
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: NetworkProtocolType,

    // Indicates if the protocol is enabled or not.
    #[yaserde(prefix = "tt", rename = "Enabled")]
    pub enabled: bool,

    // The port that is used by the protocol.
    #[yaserde(prefix = "tt", rename = "Port")]
    pub port: Vec<i32>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<NetworkProtocolExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct NetworkProtocolExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum NetworkProtocolType {
    Http,
    Https,
    Rtsp,

    __Unknown__(String),
}

impl Default for NetworkProtocolType {
    fn default() -> NetworkProtocolType {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum NetworkHostType {
    Ipv4,
    Ipv6,
    Dns,

    __Unknown__(String),
}

impl Default for NetworkHostType {
    fn default() -> NetworkHostType {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct NetworkHost {
    // Network host type: IPv4, IPv6 or DNS.
    #[yaserde(prefix = "tt", rename = "Type")]
    pub _type: NetworkHostType,

    // IPv4 address.
    #[yaserde(prefix = "tt", rename = "IPv4Address")]
    pub i_pv_4_address: Option<Ipv4Address>,

    // IPv6 address.
    #[yaserde(prefix = "tt", rename = "IPv6Address")]
    pub i_pv_6_address: Option<Ipv6Address>,

    // DNS name.
    #[yaserde(prefix = "tt", rename = "DNSname")]
    pub dn_sname: Option<Dnsname>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<NetworkHostExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct NetworkHostExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Ipaddress {
    // Indicates if the address is an IPv4 or IPv6 address.
    #[yaserde(prefix = "tt", rename = "Type")]
    pub _type: Iptype,

    // IPv4 address.
    #[yaserde(prefix = "tt", rename = "IPv4Address")]
    pub i_pv_4_address: Option<Ipv4Address>,

    // IPv6 address
    #[yaserde(prefix = "tt", rename = "IPv6Address")]
    pub i_pv_6_address: Option<Ipv6Address>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PrefixedIPv4Address {
    // IPv4 address
    #[yaserde(prefix = "tt", rename = "Address")]
    pub address: Ipv4Address,

    // Prefix/submask length
    #[yaserde(prefix = "tt", rename = "PrefixLength")]
    pub prefix_length: i32,
}

#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct Ipv4Address(pub String);

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PrefixedIPv6Address {
    // IPv6 address
    #[yaserde(prefix = "tt", rename = "Address")]
    pub address: Ipv6Address,

    // Prefix/submask length
    #[yaserde(prefix = "tt", rename = "PrefixLength")]
    pub prefix_length: i32,
}

#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct Ipv6Address(pub String);

#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct HwAddress(pub String);

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum Iptype {
    Ipv4,
    Ipv6,

    __Unknown__(String),
}

impl Default for Iptype {
    fn default() -> Iptype {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct Dnsname(pub String);

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct HostnameInformation {
    // Indicates whether the hostname is obtained from DHCP or not.
    #[yaserde(prefix = "tt", rename = "FromDHCP")]
    pub from_dhcp: bool,

    // Indicates the hostname.
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: Option<String>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<HostnameInformationExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct HostnameInformationExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Dnsinformation {
    // Indicates whether or not DNS information is retrieved from DHCP.
    #[yaserde(prefix = "tt", rename = "FromDHCP")]
    pub from_dhcp: bool,

    // Search domain.
    #[yaserde(prefix = "tt", rename = "SearchDomain")]
    pub search_domain: Vec<String>,

    // List of DNS addresses received from DHCP.
    #[yaserde(prefix = "tt", rename = "DNSFromDHCP")]
    pub dns_from_dhcp: Vec<Ipaddress>,

    // List of manually entered DNS addresses.
    #[yaserde(prefix = "tt", rename = "DNSManual")]
    pub dns_manual: Vec<Ipaddress>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<DnsinformationExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct DnsinformationExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Ntpinformation {
    // Indicates if NTP information is to be retrieved by using DHCP.
    #[yaserde(prefix = "tt", rename = "FromDHCP")]
    pub from_dhcp: bool,

    // List of NTP addresses retrieved by using DHCP.
    #[yaserde(prefix = "tt", rename = "NTPFromDHCP")]
    pub ntp_from_dhcp: Vec<NetworkHost>,

    // List of manually entered NTP addresses.
    #[yaserde(prefix = "tt", rename = "NTPManual")]
    pub ntp_manual: Vec<NetworkHost>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<NtpinformationExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct NtpinformationExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct Domain(pub String);

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum IpaddressFilterType {
    Allow,
    Deny,

    __Unknown__(String),
}

impl Default for IpaddressFilterType {
    fn default() -> IpaddressFilterType {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct DynamicDNSInformation {
    // Dynamic DNS type.
    #[yaserde(prefix = "tt", rename = "Type")]
    pub _type: DynamicDNSType,

    // DNS name.
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: Option<Dnsname>,

    // Time to live.
    #[yaserde(prefix = "tt", rename = "TTL")]
    pub ttl: Option<xs::Duration>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<DynamicDNSInformationExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct DynamicDNSInformationExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum DynamicDNSType {
    NoUpdate,
    ClientUpdates,
    ServerUpdates,

    __Unknown__(String),
}

impl Default for DynamicDNSType {
    fn default() -> DynamicDNSType {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct NetworkInterfaceSetConfiguration {
    // Indicates whether or not an interface is enabled.
    #[yaserde(prefix = "tt", rename = "Enabled")]
    pub enabled: Option<bool>,

    // Link configuration.
    #[yaserde(prefix = "tt", rename = "Link")]
    pub link: Option<NetworkInterfaceConnectionSetting>,

    // Maximum transmission unit.
    #[yaserde(prefix = "tt", rename = "MTU")]
    pub mtu: Option<i32>,

    // IPv4 network interface configuration.
    #[yaserde(prefix = "tt", rename = "IPv4")]
    pub i_pv_4: Option<Ipv4NetworkInterfaceSetConfiguration>,

    // IPv6 network interface configuration.
    #[yaserde(prefix = "tt", rename = "IPv6")]
    pub i_pv_6: Option<Ipv6NetworkInterfaceSetConfiguration>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<NetworkInterfaceSetConfigurationExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct NetworkInterfaceSetConfigurationExtension {
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,
    #[yaserde(prefix = "tt", rename = "Dot3")]
    pub dot_3: Vec<Dot3Configuration>,

    #[yaserde(prefix = "tt", rename = "Dot11")]
    pub dot_11: Vec<Dot11Configuration>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<NetworkInterfaceSetConfigurationExtension2>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Ipv6NetworkInterfaceSetConfiguration {
    // Indicates whether or not IPv6 is enabled.
    #[yaserde(prefix = "tt", rename = "Enabled")]
    pub enabled: Option<bool>,

    // Indicates whether router advertisment is used.
    #[yaserde(prefix = "tt", rename = "AcceptRouterAdvert")]
    pub accept_router_advert: Option<bool>,

    // List of manually added IPv6 addresses.
    #[yaserde(prefix = "tt", rename = "Manual")]
    pub manual: Vec<PrefixedIPv6Address>,

    // DHCP configuration.
    #[yaserde(prefix = "tt", rename = "DHCP")]
    pub dhcp: Option<Ipv6DHCPConfiguration>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Ipv4NetworkInterfaceSetConfiguration {
    // Indicates whether or not IPv4 is enabled.
    #[yaserde(prefix = "tt", rename = "Enabled")]
    pub enabled: Option<bool>,

    // List of manually added IPv4 addresses.
    #[yaserde(prefix = "tt", rename = "Manual")]
    pub manual: Vec<PrefixedIPv4Address>,

    // Indicates whether or not DHCP is used.
    #[yaserde(prefix = "tt", rename = "DHCP")]
    pub dhcp: Option<bool>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct NetworkGateway {
    // IPv4 address string.
    #[yaserde(prefix = "tt", rename = "IPv4Address")]
    pub i_pv_4_address: Vec<Ipv4Address>,

    // IPv6 address string.
    #[yaserde(prefix = "tt", rename = "IPv6Address")]
    pub i_pv_6_address: Vec<Ipv6Address>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct NetworkZeroConfiguration {
    // Unique identifier of network interface.
    #[yaserde(prefix = "tt", rename = "InterfaceToken")]
    pub interface_token: ReferenceToken,

    // Indicates whether the zero-configuration is enabled or not.
    #[yaserde(prefix = "tt", rename = "Enabled")]
    pub enabled: bool,

    // The zero-configuration IPv4 address(es)
    #[yaserde(prefix = "tt", rename = "Addresses")]
    pub addresses: Vec<Ipv4Address>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<NetworkZeroConfigurationExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct NetworkZeroConfigurationExtension {
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // Optional array holding the configuration for the second and possibly further
    // interfaces.
    #[yaserde(prefix = "tt", rename = "Additional")]
    pub additional: Vec<NetworkZeroConfiguration>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<NetworkZeroConfigurationExtension2>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct NetworkZeroConfigurationExtension2 {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct IpaddressFilter {
    #[yaserde(prefix = "tt", rename = "Type")]
    pub _type: IpaddressFilterType,

    #[yaserde(prefix = "tt", rename = "IPv4Address")]
    pub i_pv_4_address: Vec<PrefixedIPv4Address>,

    #[yaserde(prefix = "tt", rename = "IPv6Address")]
    pub i_pv_6_address: Vec<PrefixedIPv6Address>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<IpaddressFilterExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct IpaddressFilterExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Dot11Configuration {
    #[yaserde(prefix = "tt", rename = "SSID")]
    pub ssid: Dot11SSIDType,

    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: Dot11StationMode,

    #[yaserde(prefix = "tt", rename = "Alias")]
    pub alias: Name,

    #[yaserde(prefix = "tt", rename = "Priority")]
    pub priority: NetworkInterfaceConfigPriority,

    #[yaserde(prefix = "tt", rename = "Security")]
    pub security: Dot11SecurityConfiguration,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct Dot11SSIDType(pub String);

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum Dot11StationMode {
    AdHoc,
    Infrastructure,
    Extended,

    __Unknown__(String),
}

impl Default for Dot11StationMode {
    fn default() -> Dot11StationMode {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Dot11SecurityConfiguration {
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: Dot11SecurityMode,

    #[yaserde(prefix = "tt", rename = "Algorithm")]
    pub algorithm: Option<Dot11Cipher>,

    #[yaserde(prefix = "tt", rename = "PSK")]
    pub psk: Option<Dot11PSKSet>,

    #[yaserde(prefix = "tt", rename = "Dot1X")]
    pub dot_1x: Option<ReferenceToken>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<Dot11SecurityConfigurationExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Dot11SecurityConfigurationExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,

// //
//TODO: any_attribute macros
//  pub any_attribute: AnyAttribute,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum Dot11SecurityMode {
    None,
    Wep,
    Psk,
    Dot1X,
    Extended,

    __Unknown__(String),
}

impl Default for Dot11SecurityMode {
    fn default() -> Dot11SecurityMode {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum Dot11Cipher {
    Ccmp,
    Tkip,
    Any,
    Extended,

    __Unknown__(String),
}

impl Default for Dot11Cipher {
    fn default() -> Dot11Cipher {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct Dot11PSK(pub String);

#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct Dot11PSKPassphrase(pub String);

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Dot11PSKSet {
    // According to IEEE802.11-2007 H.4.1 the RSNA PSK consists of 256 bits, or 64
    // octets when represented in hex
    #[yaserde(prefix = "tt", rename = "Key")]
    pub key: Option<Dot11PSK>,

    // According to IEEE802.11-2007 H.4.1 a pass-phrase is a sequence of between 8
    // and 63 ASCII-encoded characters and
    // each character in the pass-phrase must have an encoding in the range of 32 to
    // 126 (decimal),inclusive.
    #[yaserde(prefix = "tt", rename = "Passphrase")]
    pub passphrase: Option<Dot11PSKPassphrase>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<Dot11PSKSetExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Dot11PSKSetExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct NetworkInterfaceSetConfigurationExtension2 {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Dot11Capabilities {
    #[yaserde(prefix = "tt", rename = "TKIP")]
    pub tkip: bool,

    #[yaserde(prefix = "tt", rename = "ScanAvailableNetworks")]
    pub scan_available_networks: bool,

    #[yaserde(prefix = "tt", rename = "MultipleConfiguration")]
    pub multiple_configuration: bool,

    #[yaserde(prefix = "tt", rename = "AdHocStationMode")]
    pub ad_hoc_station_mode: bool,

    #[yaserde(prefix = "tt", rename = "WEP")]
    pub wep: bool,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum Dot11SignalStrength {
    None,
    VeryBad,
    Bad,
    Good,
    VeryGood,
    Extended,

    __Unknown__(String),
}

impl Default for Dot11SignalStrength {
    fn default() -> Dot11SignalStrength {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Dot11Status {
    #[yaserde(prefix = "tt", rename = "SSID")]
    pub ssid: Dot11SSIDType,

    #[yaserde(prefix = "tt", rename = "BSSID")]
    pub bssid: Option<String>,

    #[yaserde(prefix = "tt", rename = "PairCipher")]
    pub pair_cipher: Option<Dot11Cipher>,

    #[yaserde(prefix = "tt", rename = "GroupCipher")]
    pub group_cipher: Option<Dot11Cipher>,

    #[yaserde(prefix = "tt", rename = "SignalStrength")]
    pub signal_strength: Option<Dot11SignalStrength>,

    #[yaserde(prefix = "tt", rename = "ActiveConfigAlias")]
    pub active_config_alias: ReferenceToken,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum Dot11AuthAndMangementSuite {
    None,
    Dot1X,
    Psk,
    Extended,

    __Unknown__(String),
}

impl Default for Dot11AuthAndMangementSuite {
    fn default() -> Dot11AuthAndMangementSuite {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Dot11AvailableNetworks {
    #[yaserde(prefix = "tt", rename = "SSID")]
    pub ssid: Dot11SSIDType,

    #[yaserde(prefix = "tt", rename = "BSSID")]
    pub bssid: Option<String>,

    // See IEEE802.11 7.3.2.25.2 for details.
    #[yaserde(prefix = "tt", rename = "AuthAndMangementSuite")]
    pub auth_and_mangement_suite: Vec<Dot11AuthAndMangementSuite>,

    #[yaserde(prefix = "tt", rename = "PairCipher")]
    pub pair_cipher: Vec<Dot11Cipher>,

    #[yaserde(prefix = "tt", rename = "GroupCipher")]
    pub group_cipher: Vec<Dot11Cipher>,

    #[yaserde(prefix = "tt", rename = "SignalStrength")]
    pub signal_strength: Option<Dot11SignalStrength>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<Dot11AvailableNetworksExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Dot11AvailableNetworksExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum CapabilityCategory {
    All,
    Analytics,
    Device,
    Events,
    Imaging,
    Media,
    Ptz,

    __Unknown__(String),
}

impl Default for CapabilityCategory {
    fn default() -> CapabilityCategory {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Capabilities {
    // Analytics capabilities
    #[yaserde(prefix = "tt", rename = "Analytics")]
    pub analytics: Option<AnalyticsCapabilities>,

    // Device capabilities
    #[yaserde(prefix = "tt", rename = "Device")]
    pub device: Option<DeviceCapabilities>,

    // Event capabilities
    #[yaserde(prefix = "tt", rename = "Events")]
    pub events: Option<EventCapabilities>,

    // Imaging capabilities
    #[yaserde(prefix = "tt", rename = "Imaging")]
    pub imaging: Option<ImagingCapabilities>,

    // Media capabilities
    #[yaserde(prefix = "tt", rename = "Media")]
    pub media: Option<MediaCapabilities>,

    // PTZ capabilities
    #[yaserde(prefix = "tt", rename = "PTZ")]
    pub ptz: Option<Ptzcapabilities>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<CapabilitiesExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct CapabilitiesExtension {
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,
    #[yaserde(prefix = "tt", rename = "DeviceIO")]
    pub device_io: Option<DeviceIOCapabilities>,

    #[yaserde(prefix = "tt", rename = "Display")]
    pub display: Option<DisplayCapabilities>,

    #[yaserde(prefix = "tt", rename = "Recording")]
    pub recording: Option<RecordingCapabilities>,

    #[yaserde(prefix = "tt", rename = "Search")]
    pub search: Option<SearchCapabilities>,

    #[yaserde(prefix = "tt", rename = "Replay")]
    pub replay: Option<ReplayCapabilities>,

    #[yaserde(prefix = "tt", rename = "Receiver")]
    pub receiver: Option<ReceiverCapabilities>,

    #[yaserde(prefix = "tt", rename = "AnalyticsDevice")]
    pub analytics_device: Option<AnalyticsDeviceCapabilities>,

    #[yaserde(prefix = "tt", rename = "Extensions")]
    pub extensions: Option<CapabilitiesExtension2>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct CapabilitiesExtension2 {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AnalyticsCapabilities {
    // Analytics service URI.
    #[yaserde(prefix = "tt", rename = "XAddr")]
    pub x_addr: String,

    // Indicates whether or not rules are supported.
    #[yaserde(prefix = "tt", rename = "RuleSupport")]
    pub rule_support: bool,

    // Indicates whether or not modules are supported.
    #[yaserde(prefix = "tt", rename = "AnalyticsModuleSupport")]
    pub analytics_module_support: bool,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct DeviceCapabilities {
    // Device service URI.
    #[yaserde(prefix = "tt", rename = "XAddr")]
    pub x_addr: String,

    // Network capabilities.
    #[yaserde(prefix = "tt", rename = "Network")]
    pub network: Option<NetworkCapabilities>,

    // System capabilities.
    #[yaserde(prefix = "tt", rename = "System")]
    pub system: Option<SystemCapabilities>,

    // I/O capabilities.
    #[yaserde(prefix = "tt", rename = "IO")]
    pub io: Option<Iocapabilities>,

    // Security capabilities.
    #[yaserde(prefix = "tt", rename = "Security")]
    pub security: Option<SecurityCapabilities>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<DeviceCapabilitiesExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct DeviceCapabilitiesExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct EventCapabilities {
    // Event service URI.
    #[yaserde(prefix = "tt", rename = "XAddr")]
    pub x_addr: String,

    // Indicates whether or not WS Subscription policy is supported.
    #[yaserde(prefix = "tt", rename = "WSSubscriptionPolicySupport")]
    pub ws_subscription_policy_support: bool,

    // Indicates whether or not WS Pull Point is supported.
    #[yaserde(prefix = "tt", rename = "WSPullPointSupport")]
    pub ws_pull_point_support: bool,

    // Indicates whether or not WS Pausable Subscription Manager Interface is
    // supported.
    #[yaserde(
        prefix = "tt",
        rename = "WSPausableSubscriptionManagerInterfaceSupport"
    )]
    pub ws_pausable_subscription_manager_interface_support: bool,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Iocapabilities {
    // Number of input connectors.
    #[yaserde(prefix = "tt", rename = "InputConnectors")]
    pub input_connectors: Option<i32>,

    // Number of relay outputs.
    #[yaserde(prefix = "tt", rename = "RelayOutputs")]
    pub relay_outputs: Option<i32>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<IocapabilitiesExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct IocapabilitiesExtension {
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,
    #[yaserde(prefix = "tt", rename = "Auxiliary")]
    pub auxiliary: Option<bool>,

    #[yaserde(prefix = "tt", rename = "AuxiliaryCommands")]
    pub auxiliary_commands: Vec<AuxiliaryData>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: IocapabilitiesExtension2,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct IocapabilitiesExtension2 {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MediaCapabilities {
    // Media service URI.
    #[yaserde(prefix = "tt", rename = "XAddr")]
    pub x_addr: String,

    // Streaming capabilities.
    #[yaserde(prefix = "tt", rename = "StreamingCapabilities")]
    pub streaming_capabilities: RealTimeStreamingCapabilities,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,
    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<MediaCapabilitiesExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MediaCapabilitiesExtension {
    #[yaserde(prefix = "tt", rename = "ProfileCapabilities")]
    pub profile_capabilities: ProfileCapabilities,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RealTimeStreamingCapabilities {
    // Indicates whether or not RTP multicast is supported.
    #[yaserde(prefix = "tt", rename = "RTPMulticast")]
    pub rtp_multicast: Option<bool>,

    // Indicates whether or not RTP over TCP is supported.
    #[yaserde(prefix = "tt", rename = "RTP_TCP")]
    pub rtp_tcp: Option<bool>,

    // Indicates whether or not RTP/RTSP/TCP is supported.
    #[yaserde(prefix = "tt", rename = "RTP_RTSP_TCP")]
    pub rtp_rtsp_tcp: Option<bool>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<RealTimeStreamingCapabilitiesExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RealTimeStreamingCapabilitiesExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ProfileCapabilities {
    // Maximum number of profiles.
    #[yaserde(prefix = "tt", rename = "MaximumNumberOfProfiles")]
    pub maximum_number_of_profiles: i32,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct NetworkCapabilities {
    // Indicates whether or not IP filtering is supported.
    #[yaserde(prefix = "tt", rename = "IPFilter")]
    pub ip_filter: Option<bool>,

    // Indicates whether or not zeroconf is supported.
    #[yaserde(prefix = "tt", rename = "ZeroConfiguration")]
    pub zero_configuration: Option<bool>,

    // Indicates whether or not IPv6 is supported.
    #[yaserde(prefix = "tt", rename = "IPVersion6")]
    pub ip_version_6: Option<bool>,

    // Indicates whether or not is supported.
    #[yaserde(prefix = "tt", rename = "DynDNS")]
    pub dyn_dns: Option<bool>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<NetworkCapabilitiesExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct NetworkCapabilitiesExtension {
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,
    #[yaserde(prefix = "tt", rename = "Dot11Configuration")]
    pub dot_11_configuration: Option<bool>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<NetworkCapabilitiesExtension2>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct NetworkCapabilitiesExtension2 {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SecurityCapabilities {
    // Indicates whether or not TLS 1.1 is supported.
    #[yaserde(prefix = "tt", rename = "TLS1.1")]
    pub tls1_1: bool,

    // Indicates whether or not TLS 1.2 is supported.
    #[yaserde(prefix = "tt", rename = "TLS1.2")]
    pub tls1_2: bool,

    // Indicates whether or not onboard key generation is supported.
    #[yaserde(prefix = "tt", rename = "OnboardKeyGeneration")]
    pub onboard_key_generation: bool,

    // Indicates whether or not access policy configuration is supported.
    #[yaserde(prefix = "tt", rename = "AccessPolicyConfig")]
    pub access_policy_config: bool,

    // Indicates whether or not WS-Security X.509 token is supported.
    #[yaserde(prefix = "tt", rename = "X.509Token")]
    pub x_509_token: bool,

    // Indicates whether or not WS-Security SAML token is supported.
    #[yaserde(prefix = "tt", rename = "SAMLToken")]
    pub saml_token: bool,

    // Indicates whether or not WS-Security Kerberos token is supported.
    #[yaserde(prefix = "tt", rename = "KerberosToken")]
    pub kerberos_token: bool,

    // Indicates whether or not WS-Security REL token is supported.
    #[yaserde(prefix = "tt", rename = "RELToken")]
    pub rel_token: bool,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,
    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<SecurityCapabilitiesExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SecurityCapabilitiesExtension {
    #[yaserde(prefix = "tt", rename = "TLS1.0")]
    pub tls1_0: bool,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<SecurityCapabilitiesExtension2>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SecurityCapabilitiesExtension2 {
    #[yaserde(prefix = "tt", rename = "Dot1X")]
    pub dot_1x: bool,

    // EAP Methods supported by the device. The int values refer to the
    #[yaserde(prefix = "tt", rename = "SupportedEAPMethod")]
    pub supported_eap_method: Vec<i32>,

    #[yaserde(prefix = "tt", rename = "RemoteUserHandling")]
    pub remote_user_handling: bool,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SystemCapabilities {
    // Indicates whether or not WS Discovery resolve requests are supported.
    #[yaserde(prefix = "tt", rename = "DiscoveryResolve")]
    pub discovery_resolve: bool,

    // Indicates whether or not WS-Discovery Bye is supported.
    #[yaserde(prefix = "tt", rename = "DiscoveryBye")]
    pub discovery_bye: bool,

    // Indicates whether or not remote discovery is supported.
    #[yaserde(prefix = "tt", rename = "RemoteDiscovery")]
    pub remote_discovery: bool,

    // Indicates whether or not system backup is supported.
    #[yaserde(prefix = "tt", rename = "SystemBackup")]
    pub system_backup: bool,

    // Indicates whether or not system logging is supported.
    #[yaserde(prefix = "tt", rename = "SystemLogging")]
    pub system_logging: bool,

    // Indicates whether or not firmware upgrade is supported.
    #[yaserde(prefix = "tt", rename = "FirmwareUpgrade")]
    pub firmware_upgrade: bool,

    // Indicates supported ONVIF version(s).
    #[yaserde(prefix = "tt", rename = "SupportedVersions")]
    pub supported_versions: Vec<OnvifVersion>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<SystemCapabilitiesExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SystemCapabilitiesExtension {
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,
    #[yaserde(prefix = "tt", rename = "HttpFirmwareUpgrade")]
    pub http_firmware_upgrade: Option<bool>,

    #[yaserde(prefix = "tt", rename = "HttpSystemBackup")]
    pub http_system_backup: Option<bool>,

    #[yaserde(prefix = "tt", rename = "HttpSystemLogging")]
    pub http_system_logging: Option<bool>,

    #[yaserde(prefix = "tt", rename = "HttpSupportInformation")]
    pub http_support_information: Option<bool>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<SystemCapabilitiesExtension2>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SystemCapabilitiesExtension2 {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct OnvifVersion {
    // Major version number.
    #[yaserde(prefix = "tt", rename = "Major")]
    pub major: i32,

    // Two digit minor version number.
    // If major version number is less than "16", X.0.1 maps to "01" and X.2.1 maps
    // to "21" where X stands for Major version number.
    // Otherwise, minor number is month of release, such as "06" for June.
    #[yaserde(prefix = "tt", rename = "Minor")]
    pub minor: i32,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ImagingCapabilities {
    // Imaging service URI.
    #[yaserde(prefix = "tt", rename = "XAddr")]
    pub x_addr: String,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Ptzcapabilities {
    // PTZ service URI.
    #[yaserde(prefix = "tt", rename = "XAddr")]
    pub x_addr: String,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct DeviceIOCapabilities {
    #[yaserde(prefix = "tt", rename = "XAddr")]
    pub x_addr: String,

    #[yaserde(prefix = "tt", rename = "VideoSources")]
    pub video_sources: i32,

    #[yaserde(prefix = "tt", rename = "VideoOutputs")]
    pub video_outputs: i32,

    #[yaserde(prefix = "tt", rename = "AudioSources")]
    pub audio_sources: i32,

    #[yaserde(prefix = "tt", rename = "AudioOutputs")]
    pub audio_outputs: i32,

    #[yaserde(prefix = "tt", rename = "RelayOutputs")]
    pub relay_outputs: i32,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct DisplayCapabilities {
    #[yaserde(prefix = "tt", rename = "XAddr")]
    pub x_addr: String,

    // Indication that the SetLayout command supports only predefined layouts.
    #[yaserde(prefix = "tt", rename = "FixedLayout")]
    pub fixed_layout: bool,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RecordingCapabilities {
    #[yaserde(prefix = "tt", rename = "XAddr")]
    pub x_addr: String,

    #[yaserde(prefix = "tt", rename = "ReceiverSource")]
    pub receiver_source: bool,

    #[yaserde(prefix = "tt", rename = "MediaProfileSource")]
    pub media_profile_source: bool,

    #[yaserde(prefix = "tt", rename = "DynamicRecordings")]
    pub dynamic_recordings: bool,

    #[yaserde(prefix = "tt", rename = "DynamicTracks")]
    pub dynamic_tracks: bool,

    #[yaserde(prefix = "tt", rename = "MaxStringLength")]
    pub max_string_length: i32,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SearchCapabilities {
    #[yaserde(prefix = "tt", rename = "XAddr")]
    pub x_addr: String,

    #[yaserde(prefix = "tt", rename = "MetadataSearch")]
    pub metadata_search: bool,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ReplayCapabilities {
    // The address of the replay service.
    #[yaserde(prefix = "tt", rename = "XAddr")]
    pub x_addr: String,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ReceiverCapabilities {
    // The address of the receiver service.
    #[yaserde(prefix = "tt", rename = "XAddr")]
    pub x_addr: String,

    // Indicates whether the device can receive RTP multicast streams.
    #[yaserde(prefix = "tt", rename = "RTP_Multicast")]
    pub rtp_multicast: bool,

    // Indicates whether the device can receive RTP/TCP streams
    #[yaserde(prefix = "tt", rename = "RTP_TCP")]
    pub rtp_tcp: bool,

    // Indicates whether the device can receive RTP/RTSP/TCP streams.
    #[yaserde(prefix = "tt", rename = "RTP_RTSP_TCP")]
    pub rtp_rtsp_tcp: bool,

    // The maximum number of receivers supported by the device.
    #[yaserde(prefix = "tt", rename = "SupportedReceivers")]
    pub supported_receivers: i32,

    // The maximum allowed length for RTSP URIs.
    #[yaserde(prefix = "tt", rename = "MaximumRTSPURILength")]
    pub maximum_rtspuri_length: i32,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AnalyticsDeviceCapabilities {
    #[yaserde(prefix = "tt", rename = "XAddr")]
    pub x_addr: String,

    // Obsolete property.
    #[yaserde(prefix = "tt", rename = "RuleSupport")]
    pub rule_support: Option<bool>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<AnalyticsDeviceExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AnalyticsDeviceExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

// Enumeration describing the available system log modes.
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum SystemLogType {
    // Indicates that a system log is requested.
    System,
    // Indicates that a access log is requested.
    Access,

    __Unknown__(String),
}

impl Default for SystemLogType {
    fn default() -> SystemLogType {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SystemLog {
    // The log information as attachment data.
    #[yaserde(prefix = "tt", rename = "Binary")]
    pub binary: Option<AttachmentData>,

    // The log information as character data.
    #[yaserde(prefix = "tt", rename = "String")]
    pub string: Option<String>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SupportInformation {
    // The support information as attachment data.
    #[yaserde(prefix = "tt", rename = "Binary")]
    pub binary: Option<AttachmentData>,

    // The support information as character data.
    #[yaserde(prefix = "tt", rename = "String")]
    pub string: Option<String>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct BinaryData {
    // base64 encoded binary data.
    #[yaserde(prefix = "tt", rename = "Data")]
    pub data: String,

    #[yaserde(attribute, prefix = "xmime" rename = "contentType")]
    pub xmime_content_type: Option<xmime::ContentType>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AttachmentData {
    #[yaserde(prefix = "tt", rename = "Include")]
    pub xop_include: xop::Include,

    #[yaserde(attribute, prefix = "xmime" rename = "contentType")]
    pub xmime_content_type: Option<xmime::ContentType>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct BackupFile {
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: String,

    #[yaserde(prefix = "tt", rename = "Data")]
    pub data: AttachmentData,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SystemLogUriList {
    #[yaserde(prefix = "tt", rename = "SystemLog")]
    pub system_log: Vec<SystemLogUri>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SystemLogUri {
    #[yaserde(prefix = "tt", rename = "Type")]
    pub _type: SystemLogType,

    #[yaserde(prefix = "tt", rename = "Uri")]
    pub uri: String,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

// Enumeration describing the available factory default modes.
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum FactoryDefaultType {
    // Indicates that a hard factory default is requested.
    Hard,
    // Indicates that a soft factory default is requested.
    Soft,

    __Unknown__(String),
}

impl Default for FactoryDefaultType {
    fn default() -> FactoryDefaultType {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum SetDateTimeType {
    // Indicates that the date and time are set manually.
    Manual,
    // Indicates that the date and time are set through NTP
    #[yaserde(rename = "NTP")]
    Ntp,

    __Unknown__(String),
}

impl Default for SetDateTimeType {
    fn default() -> SetDateTimeType {
        Self::__Unknown__("No valid variants".into())
    }
}

// General date time inforamtion returned by the GetSystemDateTime method.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SystemDateTime {
    // Indicates if the time is set manully or through NTP.
    #[yaserde(prefix = "tt", rename = "DateTimeType")]
    pub date_time_type: SetDateTimeType,

    // Informative indicator whether daylight savings is currently on/off.
    #[yaserde(prefix = "tt", rename = "DaylightSavings")]
    pub daylight_savings: bool,

    // Timezone information in Posix format.
    #[yaserde(prefix = "tt", rename = "TimeZone")]
    pub time_zone: Option<TimeZone>,

    // Current system date and time in UTC format. This field is mandatory since
    // version 2.0.
    #[yaserde(prefix = "tt", rename = "UTCDateTime")]
    pub utc_date_time: Option<DateTime>,

    // Date and time in local format.
    #[yaserde(prefix = "tt", rename = "LocalDateTime")]
    pub local_date_time: Option<DateTime>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<SystemDateTimeExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SystemDateTimeExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct DateTime {
    #[yaserde(prefix = "tt", rename = "Time")]
    pub time: Time,

    #[yaserde(prefix = "tt", rename = "Date")]
    pub date: Date,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Date {
    #[yaserde(prefix = "tt", rename = "Year")]
    pub year: i32,

    // Range is 1 to 12.
    #[yaserde(prefix = "tt", rename = "Month")]
    pub month: i32,

    // Range is 1 to 31.
    #[yaserde(prefix = "tt", rename = "Day")]
    pub day: i32,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Time {
    // Range is 0 to 23.
    #[yaserde(prefix = "tt", rename = "Hour")]
    pub hour: i32,

    // Range is 0 to 59.
    #[yaserde(prefix = "tt", rename = "Minute")]
    pub minute: i32,

    // Range is 0 to 61 (typically 59).
    #[yaserde(prefix = "tt", rename = "Second")]
    pub second: i32,
}

// The TZ format is specified by POSIX, please refer to POSIX 1003.1 section 8.3
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct TimeZone {
    // Posix timezone string.
    #[yaserde(prefix = "tt", rename = "TZ")]
    pub tz: String,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RemoteUser {
    #[yaserde(prefix = "tt", rename = "Username")]
    pub username: String,

    #[yaserde(prefix = "tt", rename = "Password")]
    pub password: Option<String>,

    #[yaserde(prefix = "tt", rename = "UseDerivedPassword")]
    pub use_derived_password: bool,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum UserLevel {
    Administrator,
    Operator,
    User,
    Anonymous,
    Extended,

    __Unknown__(String),
}

impl Default for UserLevel {
    fn default() -> UserLevel {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct User {
    // Username string.
    #[yaserde(prefix = "tt", rename = "Username")]
    pub username: String,

    // Password string.
    #[yaserde(prefix = "tt", rename = "Password")]
    pub password: Option<String>,

    // User level string.
    #[yaserde(prefix = "tt", rename = "UserLevel")]
    pub user_level: UserLevel,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<UserExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct UserExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct CertificateGenerationParameters {
    #[yaserde(prefix = "tt", rename = "CertificateID")]
    pub certificate_id: Option<String>,

    #[yaserde(prefix = "tt", rename = "Subject")]
    pub subject: Option<String>,

    #[yaserde(prefix = "tt", rename = "ValidNotBefore")]
    pub valid_not_before: Option<String>,

    #[yaserde(prefix = "tt", rename = "ValidNotAfter")]
    pub valid_not_after: Option<String>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<CertificateGenerationParametersExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct CertificateGenerationParametersExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Certificate {
    // Certificate id.
    #[yaserde(prefix = "tt", rename = "CertificateID")]
    pub certificate_id: String,

    // base64 encoded DER representation of certificate.
    #[yaserde(prefix = "tt", rename = "Certificate")]
    pub certificate: BinaryData,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct CertificateStatus {
    // Certificate id.
    #[yaserde(prefix = "tt", rename = "CertificateID")]
    pub certificate_id: String,

    // Indicates whether or not a certificate is used in a HTTPS configuration.
    #[yaserde(prefix = "tt", rename = "Status")]
    pub status: bool,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct CertificateWithPrivateKey {
    #[yaserde(prefix = "tt", rename = "CertificateID")]
    pub certificate_id: Option<String>,

    #[yaserde(prefix = "tt", rename = "Certificate")]
    pub certificate: BinaryData,

    #[yaserde(prefix = "tt", rename = "PrivateKey")]
    pub private_key: BinaryData,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct CertificateInformation {
    #[yaserde(prefix = "tt", rename = "CertificateID")]
    pub certificate_id: String,

    #[yaserde(prefix = "tt", rename = "IssuerDN")]
    pub issuer_dn: Option<String>,

    #[yaserde(prefix = "tt", rename = "SubjectDN")]
    pub subject_dn: Option<String>,

    #[yaserde(prefix = "tt", rename = "KeyUsage")]
    pub key_usage: Option<CertificateUsage>,

    #[yaserde(prefix = "tt", rename = "ExtendedKeyUsage")]
    pub extended_key_usage: Option<CertificateUsage>,

    #[yaserde(prefix = "tt", rename = "KeyLength")]
    pub key_length: Option<i32>,

    #[yaserde(prefix = "tt", rename = "Version")]
    pub version: Option<String>,

    #[yaserde(prefix = "tt", rename = "SerialNum")]
    pub serial_num: Option<String>,

    // Validity Range is from "NotBefore" to "NotAfter"; the corresponding
    // DateTimeRange is from "From" to "Until"
    #[yaserde(prefix = "tt", rename = "SignatureAlgorithm")]
    pub signature_algorithm: Option<String>,

    #[yaserde(prefix = "tt", rename = "Validity")]
    pub validity: Option<DateTimeRange>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<CertificateInformationExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct CertificateUsage {
    #[yaserde(attribute, rename = "Critical")]
    pub critical: bool,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct CertificateInformationExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Dot1XConfiguration {
    #[yaserde(prefix = "tt", rename = "Dot1XConfigurationToken")]
    pub dot_1x_configuration_token: ReferenceToken,

    #[yaserde(prefix = "tt", rename = "Identity")]
    pub identity: String,

    #[yaserde(prefix = "tt", rename = "AnonymousID")]
    pub anonymous_id: Option<String>,

    // EAP Method type as defined in
    #[yaserde(prefix = "tt", rename = "EAPMethod")]
    pub eap_method: i32,

    #[yaserde(prefix = "tt", rename = "CACertificateID")]
    pub ca_certificate_id: Vec<String>,

    #[yaserde(prefix = "tt", rename = "EAPMethodConfiguration")]
    pub eap_method_configuration: Option<EapmethodConfiguration>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<Dot1XConfigurationExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Dot1XConfigurationExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct EapmethodConfiguration {
    // Confgiuration information for TLS Method.
    #[yaserde(prefix = "tt", rename = "TLSConfiguration")]
    pub tls_configuration: Option<Tlsconfiguration>,

    // Password for those EAP Methods that require a password. The password shall
    // never be returned on a get method.
    #[yaserde(prefix = "tt", rename = "Password")]
    pub password: Option<String>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<EapMethodExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct EapMethodExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Tlsconfiguration {
    #[yaserde(prefix = "tt", rename = "CertificateID")]
    pub certificate_id: String,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct GenericEapPwdConfigurationExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum RelayLogicalState {
    Active,
    Inactive,

    __Unknown__(String),
}

impl Default for RelayLogicalState {
    fn default() -> RelayLogicalState {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum RelayIdleState {
    Closed,
    Open,

    __Unknown__(String),
}

impl Default for RelayIdleState {
    fn default() -> RelayIdleState {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RelayOutputSettings {
    // 'Bistable' or 'Monostable'
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: RelayMode,

    // Time after which the relay returns to its idle state if it is in monostable
    // mode. If the Mode field is set to bistable mode the value of the parameter
    // can be ignored.
    #[yaserde(prefix = "tt", rename = "DelayTime")]
    pub delay_time: xs::Duration,

    // 'open' or 'closed'
    #[yaserde(prefix = "tt", rename = "IdleState")]
    pub idle_state: RelayIdleState,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum RelayMode {
    Monostable,
    Bistable,

    __Unknown__(String),
}

impl Default for RelayMode {
    fn default() -> RelayMode {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RelayOutput {
    #[yaserde(prefix = "tt", rename = "Properties")]
    pub properties: RelayOutputSettings,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // Unique identifier referencing the physical entity.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum DigitalIdleState {
    Closed,
    Open,

    __Unknown__(String),
}

impl Default for DigitalIdleState {
    fn default() -> DigitalIdleState {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct DigitalInput {
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // Indicate the Digital IdleState status.
    #[yaserde(attribute, rename = "IdleState")]
    pub idle_state: Option<DigitalIdleState>,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // Unique identifier referencing the physical entity.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Ptznode {
    // A unique identifier that is used to reference PTZ Nodes.
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: Option<Name>,

    // A list of Coordinate Systems available for the PTZ Node. For each Coordinate
    // System, the PTZ Node MUST specify its allowed range.
    #[yaserde(prefix = "tt", rename = "SupportedPTZSpaces")]
    pub supported_ptz_spaces: Ptzspaces,

    // All preset operations MUST be available for this PTZ Node if one preset is
    // supported.
    #[yaserde(prefix = "tt", rename = "MaximumNumberOfPresets")]
    pub maximum_number_of_presets: i32,

    // A boolean operator specifying the availability of a home position. If set to
    // true, the Home Position Operations MUST be available for this PTZ Node.
    #[yaserde(prefix = "tt", rename = "HomeSupported")]
    pub home_supported: bool,

    // A list of supported Auxiliary commands. If the list is not empty, the
    // Auxiliary Operations MUST be available for this PTZ Node.
    #[yaserde(prefix = "tt", rename = "AuxiliaryCommands")]
    pub auxiliary_commands: Vec<AuxiliaryData>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<PtznodeExtension>,

    // Indication whether the HomePosition of a Node is fixed or it can be changed
    // via the SetHomePosition command.
    #[yaserde(attribute, rename = "FixedHomePosition")]
    pub fixed_home_position: Option<bool>,

    // Indication whether the Node supports the geo-referenced move command.
    #[yaserde(attribute, rename = "GeoMove")]
    pub geo_move: Option<bool>,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // Unique identifier referencing the physical entity.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtznodeExtension {
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // Detail of supported Preset Tour feature.
    #[yaserde(prefix = "tt", rename = "SupportedPresetTour")]
    pub supported_preset_tour: Option<PtzpresetTourSupported>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<PtznodeExtension2>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtznodeExtension2 {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzpresetTourSupported {
    // Indicates number of preset tours that can be created. Required preset tour
    // operations shall be available for this PTZ Node if one or more preset tour is
    // supported.
    #[yaserde(prefix = "tt", rename = "MaximumNumberOfPresetTours")]
    pub maximum_number_of_preset_tours: i32,

    // Indicates which preset tour operations are available for this PTZ Node.
    #[yaserde(prefix = "tt", rename = "PTZPresetTourOperation")]
    pub ptz_preset_tour_operation: Vec<PtzpresetTourOperation>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<PtzpresetTourSupportedExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzpresetTourSupportedExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Ptzconfiguration {
    // A mandatory reference to the PTZ Node that the PTZ Configuration belongs to.
    #[yaserde(prefix = "tt", rename = "NodeToken")]
    pub node_token: ReferenceToken,

    // If the PTZ Node supports absolute Pan/Tilt movements, it shall specify one
    // Absolute Pan/Tilt Position Space as default.
    #[yaserde(prefix = "tt", rename = "DefaultAbsolutePantTiltPositionSpace")]
    pub default_absolute_pant_tilt_position_space: Option<String>,

    // If the PTZ Node supports absolute zoom movements, it shall specify one
    // Absolute Zoom Position Space as default.
    #[yaserde(prefix = "tt", rename = "DefaultAbsoluteZoomPositionSpace")]
    pub default_absolute_zoom_position_space: Option<String>,

    // If the PTZ Node supports relative Pan/Tilt movements, it shall specify one
    // RelativePan/Tilt Translation Space as default.
    #[yaserde(prefix = "tt", rename = "DefaultRelativePanTiltTranslationSpace")]
    pub default_relative_pan_tilt_translation_space: Option<String>,

    // If the PTZ Node supports relative zoom movements, it shall specify one
    // Relative Zoom Translation Space as default.
    #[yaserde(prefix = "tt", rename = "DefaultRelativeZoomTranslationSpace")]
    pub default_relative_zoom_translation_space: Option<String>,

    // If the PTZ Node supports continuous Pan/Tilt movements, it shall specify one
    // Continuous Pan/Tilt Velocity Space as default.
    #[yaserde(prefix = "tt", rename = "DefaultContinuousPanTiltVelocitySpace")]
    pub default_continuous_pan_tilt_velocity_space: Option<String>,

    // If the PTZ Node supports continuous zoom movements, it shall specify one
    // Continuous Zoom Velocity Space as default.
    #[yaserde(prefix = "tt", rename = "DefaultContinuousZoomVelocitySpace")]
    pub default_continuous_zoom_velocity_space: Option<String>,

    // If the PTZ Node supports absolute or relative PTZ movements, it shall specify
    // corresponding default Pan/Tilt and Zoom speeds.
    #[yaserde(prefix = "tt", rename = "DefaultPTZSpeed")]
    pub default_ptz_speed: Option<Ptzspeed>,

    // If the PTZ Node supports continuous movements, it shall specify a default
    // timeout, after which the movement stops.
    #[yaserde(prefix = "tt", rename = "DefaultPTZTimeout")]
    pub default_ptz_timeout: Option<xs::Duration>,

    // The Pan/Tilt limits element should be present for a PTZ Node that supports an
    // absolute Pan/Tilt. If the element is present it signals the support for
    // configurable Pan/Tilt limits. If limits are enabled, the Pan/Tilt movements
    // shall always stay within the specified range. The Pan/Tilt limits are
    // disabled by setting the limits to –INF or +INF.
    #[yaserde(prefix = "tt", rename = "PanTiltLimits")]
    pub pan_tilt_limits: Option<PanTiltLimits>,

    // The Zoom limits element should be present for a PTZ Node that supports
    // absolute zoom. If the element is present it signals the supports for
    // configurable Zoom limits. If limits are enabled the zoom movements shall
    // always stay within the specified range. The Zoom limits are disabled by
    // settings the limits to -INF and +INF.
    #[yaserde(prefix = "tt", rename = "ZoomLimits")]
    pub zoom_limits: Option<ZoomLimits>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<PtzconfigurationExtension>,

    // The optional acceleration ramp used by the device when moving.
    #[yaserde(attribute, rename = "MoveRamp")]
    pub move_ramp: Option<i32>,

    // The optional acceleration ramp used by the device when recalling presets.
    #[yaserde(attribute, rename = "PresetRamp")]
    pub preset_ramp: Option<i32>,

    // The optional acceleration ramp used by the device when executing PresetTours.
    #[yaserde(attribute, rename = "PresetTourRamp")]
    pub preset_tour_ramp: Option<i32>,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // User readable name. Length up to 64 characters.
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: Name,

    // Number of internal references currently using this configuration.
    #[yaserde(prefix = "tt", rename = "UseCount")]
    pub use_count: i32,

    // Token that uniquely references this configuration. Length up to 64
    // characters.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzconfigurationExtension {
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // Optional element to configure PT Control Direction related features.
    #[yaserde(prefix = "tt", rename = "PTControlDirection")]
    pub pt_control_direction: Option<PtcontrolDirection>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<PtzconfigurationExtension2>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzconfigurationExtension2 {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtcontrolDirection {
    // Optional element to configure related parameters for E-Flip.
    #[yaserde(prefix = "tt", rename = "EFlip")]
    pub e_flip: Option<Eflip>,

    // Optional element to configure related parameters for reversing of PT Control
    // Direction.
    #[yaserde(prefix = "tt", rename = "Reverse")]
    pub reverse: Option<Reverse>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<PtcontrolDirectionExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtcontrolDirectionExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,

// //
//TODO: any_attribute macros
//  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Eflip {
    // Parameter to enable/disable E-Flip feature.
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: EflipMode,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Reverse {
    // Parameter to enable/disable Reverse feature.
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: ReverseMode,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum EflipMode {
    Off,
    On,
    Extended,

    __Unknown__(String),
}

impl Default for EflipMode {
    fn default() -> EflipMode {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum ReverseMode {
    Off,
    On,
    Auto,
    Extended,

    __Unknown__(String),
}

impl Default for ReverseMode {
    fn default() -> ReverseMode {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzconfigurationOptions {
    // A list of supported coordinate systems including their range limitations.
    #[yaserde(prefix = "tt", rename = "Spaces")]
    pub spaces: Ptzspaces,

    // A timeout Range within which Timeouts are accepted by the PTZ Node.
    #[yaserde(prefix = "tt", rename = "PTZTimeout")]
    pub ptz_timeout: DurationRange,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // Supported options for PT Direction Control.
    #[yaserde(prefix = "tt", rename = "PTControlDirection")]
    pub pt_control_direction: Option<PtcontrolDirectionOptions>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<PtzconfigurationOptions2>,

    // The list of acceleration ramps supported by the device. The
    // smallest acceleration value corresponds to the minimal index, the
    // highest acceleration corresponds to the maximum index.
    #[yaserde(attribute, rename = "PTZRamps")]
    pub ptz_ramps: Option<IntAttrList>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzconfigurationOptions2 {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtcontrolDirectionOptions {
    // Supported options for EFlip feature.
    #[yaserde(prefix = "tt", rename = "EFlip")]
    pub e_flip: Option<EflipOptions>,

    // Supported options for Reverse feature.
    #[yaserde(prefix = "tt", rename = "Reverse")]
    pub reverse: Option<ReverseOptions>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<PtcontrolDirectionOptionsExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtcontrolDirectionOptionsExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct EflipOptions {
    // Options of EFlip mode parameter.
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: Vec<EflipMode>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<EflipOptionsExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct EflipOptionsExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ReverseOptions {
    // Options of Reverse mode parameter.
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: Vec<ReverseMode>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ReverseOptionsExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ReverseOptionsExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PanTiltLimits {
    // A range of pan tilt limits.
    #[yaserde(prefix = "tt", rename = "Range")]
    pub range: Space2DDescription,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ZoomLimits {
    // A range of zoom limit
    #[yaserde(prefix = "tt", rename = "Range")]
    pub range: Space1DDescription,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Ptzspaces {
    // The Generic Pan/Tilt Position space is provided by every PTZ node that
    // supports absolute Pan/Tilt, since it does not relate to a specific physical
    // range.
    // Instead, the range should be defined as the full range of the PTZ unit
    // normalized to the range -1 to 1 resulting in the following space description.
    #[yaserde(prefix = "tt", rename = "AbsolutePanTiltPositionSpace")]
    pub absolute_pan_tilt_position_space: Vec<Space2DDescription>,

    // The Generic Zoom Position Space is provided by every PTZ node that supports
    // absolute Zoom, since it does not relate to a specific physical range.
    // Instead, the range should be defined as the full range of the Zoom normalized
    // to the range 0 (wide) to 1 (tele).
    // There is no assumption about how the generic zoom range is mapped to
    // magnification, FOV or other physical zoom dimension.
    #[yaserde(prefix = "tt", rename = "AbsoluteZoomPositionSpace")]
    pub absolute_zoom_position_space: Vec<Space1DDescription>,

    // The Generic Pan/Tilt translation space is provided by every PTZ node that
    // supports relative Pan/Tilt, since it does not relate to a specific physical
    // range.
    // Instead, the range should be defined as the full positive and negative
    // translation range of the PTZ unit normalized to the range -1 to 1,
    // where positive translation would mean clockwise rotation or movement in
    // right/up direction resulting in the following space description.
    #[yaserde(prefix = "tt", rename = "RelativePanTiltTranslationSpace")]
    pub relative_pan_tilt_translation_space: Vec<Space2DDescription>,

    // The Generic Zoom Translation Space is provided by every PTZ node that
    // supports relative Zoom, since it does not relate to a specific physical
    // range.
    // Instead, the corresponding absolute range should be defined as the full
    // positive and negative translation range of the Zoom normalized to the range
    // -1 to1,
    // where a positive translation maps to a movement in TELE direction. The
    // translation is signed to indicate direction (negative is to wide, positive is
    // to tele).
    // There is no assumption about how the generic zoom range is mapped to
    // magnification, FOV or other physical zoom dimension. This results in the
    // following space description.
    #[yaserde(prefix = "tt", rename = "RelativeZoomTranslationSpace")]
    pub relative_zoom_translation_space: Vec<Space1DDescription>,

    // The generic Pan/Tilt velocity space shall be provided by every PTZ node,
    // since it does not relate to a specific physical range.
    // Instead, the range should be defined as a range of the PTZ unit’s speed
    // normalized to the range -1 to 1, where a positive velocity would map to
    // clockwise
    // rotation or movement in the right/up direction. A signed speed can be
    // independently specified for the pan and tilt component resulting in the
    // following space description.
    #[yaserde(prefix = "tt", rename = "ContinuousPanTiltVelocitySpace")]
    pub continuous_pan_tilt_velocity_space: Vec<Space2DDescription>,

    // The generic zoom velocity space specifies a zoom factor velocity without
    // knowing the underlying physical model. The range should be normalized from -1
    // to 1,
    // where a positive velocity would map to TELE direction. A generic zoom
    // velocity space description resembles the following.
    #[yaserde(prefix = "tt", rename = "ContinuousZoomVelocitySpace")]
    pub continuous_zoom_velocity_space: Vec<Space1DDescription>,

    // The speed space specifies the speed for a Pan/Tilt movement when moving to an
    // absolute position or to a relative translation.
    // In contrast to the velocity spaces, speed spaces do not contain any
    // directional information. The speed of a combined Pan/Tilt
    // movement is represented by a single non-negative scalar value.
    #[yaserde(prefix = "tt", rename = "PanTiltSpeedSpace")]
    pub pan_tilt_speed_space: Vec<Space1DDescription>,

    // The speed space specifies the speed for a Zoom movement when moving to an
    // absolute position or to a relative translation.
    // In contrast to the velocity spaces, speed spaces do not contain any
    // directional information.
    #[yaserde(prefix = "tt", rename = "ZoomSpeedSpace")]
    pub zoom_speed_space: Vec<Space1DDescription>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<PtzspacesExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzspacesExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Space2DDescription {
    // A URI of coordinate systems.
    #[yaserde(prefix = "tt", rename = "URI")]
    pub uri: String,

    // A range of x-axis.
    #[yaserde(prefix = "tt", rename = "XRange")]
    pub x_range: FloatRange,

    // A range of y-axis.
    #[yaserde(prefix = "tt", rename = "YRange")]
    pub y_range: FloatRange,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Space1DDescription {
    // A URI of coordinate systems.
    #[yaserde(prefix = "tt", rename = "URI")]
    pub uri: String,

    // A range of x-axis.
    #[yaserde(prefix = "tt", rename = "XRange")]
    pub x_range: FloatRange,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Ptzspeed {
    // Pan and tilt speed. The x component corresponds to pan and the y component to
    // tilt. If omitted in a request, the current (if any) PanTilt movement should
    // not be affected.
    #[yaserde(prefix = "tt", rename = "PanTilt")]
    pub pan_tilt: Option<Vector2D>,

    // A zoom speed. If omitted in a request, the current (if any) Zoom movement
    // should not be affected.
    #[yaserde(prefix = "tt", rename = "Zoom")]
    pub zoom: Option<Vector1D>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Ptzpreset {
    // A list of preset position name.
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: Option<Name>,

    // A list of preset position.
    #[yaserde(prefix = "tt", rename = "PTZPosition")]
    pub ptz_position: Option<Ptzvector>,

    #[yaserde(attribute, rename = "token")]
    pub token: Option<ReferenceToken>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct AuxiliaryData(pub String);

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum PtzpresetTourState {
    Idle,
    Touring,
    Paused,
    Extended,

    __Unknown__(String),
}

impl Default for PtzpresetTourState {
    fn default() -> PtzpresetTourState {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum PtzpresetTourDirection {
    Forward,
    Backward,
    Extended,

    __Unknown__(String),
}

impl Default for PtzpresetTourDirection {
    fn default() -> PtzpresetTourDirection {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum PtzpresetTourOperation {
    Start,
    Stop,
    Pause,
    Extended,

    __Unknown__(String),
}

impl Default for PtzpresetTourOperation {
    fn default() -> PtzpresetTourOperation {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PresetTour {
    // Readable name of the preset tour.
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: Option<Name>,

    // Read only parameters to indicate the status of the preset tour.
    #[yaserde(prefix = "tt", rename = "Status")]
    pub status: PtzpresetTourStatus,

    // Auto Start flag of the preset tour. True allows the preset tour to be
    // activated always.
    #[yaserde(prefix = "tt", rename = "AutoStart")]
    pub auto_start: bool,

    // Parameters to specify the detail behavior of the preset tour.
    #[yaserde(prefix = "tt", rename = "StartingCondition")]
    pub starting_condition: PtzpresetTourStartingCondition,

    // A list of detail of touring spots including preset positions.
    #[yaserde(prefix = "tt", rename = "TourSpot")]
    pub tour_spot: Vec<PtzpresetTourSpot>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<PtzpresetTourExtension>,

    // Unique identifier of this preset tour.
    #[yaserde(attribute, rename = "token")]
    pub token: Option<ReferenceToken>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzpresetTourExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzpresetTourSpot {
    // Detail definition of preset position of the tour spot.
    #[yaserde(prefix = "tt", rename = "PresetDetail")]
    pub preset_detail: PtzpresetTourPresetDetail,

    // Optional parameter to specify Pan/Tilt and Zoom speed on moving toward this
    // tour spot.
    #[yaserde(prefix = "tt", rename = "Speed")]
    pub speed: Option<Ptzspeed>,

    // Optional parameter to specify time duration of staying on this tour sport.
    #[yaserde(prefix = "tt", rename = "StayTime")]
    pub stay_time: Option<xs::Duration>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<PtzpresetTourSpotExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzpresetTourSpotExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzpresetTourPresetDetail {
    #[yaserde(prefix = "tt", rename = "PtzpresetTourPresetDetailChoice")]
    pub ptzpreset_tour_preset_detail_choice: PtzpresetTourPresetDetailChoice,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum PtzpresetTourPresetDetailChoice {
    // Option to specify the preset position with Preset Token defined in advance.
    PresetToken(ReferenceToken),
    // Option to specify the preset position with the home position of this PTZ
    // Node. "False" to this parameter shall be treated as an invalid argument.
    Home(bool),
    // Option to specify the preset position with vector of PTZ node directly.
    Ptzposition(Ptzvector),
    TypeExtension(PtzpresetTourTypeExtension),

    __Unknown__(String),
}

impl Default for PtzpresetTourPresetDetailChoice {
    fn default() -> PtzpresetTourPresetDetailChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzpresetTourTypeExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzpresetTourStatus {
    // Indicates state of this preset tour by Idle/Touring/Paused.
    #[yaserde(prefix = "tt", rename = "State")]
    pub state: PtzpresetTourState,

    // Indicates a tour spot currently staying.
    #[yaserde(prefix = "tt", rename = "CurrentTourSpot")]
    pub current_tour_spot: Option<PtzpresetTourSpot>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<PtzpresetTourStatusExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzpresetTourStatusExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzpresetTourStartingCondition {
    // Optional parameter to specify how many times the preset tour is recurred.
    #[yaserde(prefix = "tt", rename = "RecurringTime")]
    pub recurring_time: Option<i32>,

    // Optional parameter to specify how long time duration the preset tour is
    // recurred.
    #[yaserde(prefix = "tt", rename = "RecurringDuration")]
    pub recurring_duration: Option<xs::Duration>,

    // Optional parameter to choose which direction the preset tour goes. Forward
    // shall be chosen in case it is omitted.
    #[yaserde(prefix = "tt", rename = "Direction")]
    pub direction: Option<PtzpresetTourDirection>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<PtzpresetTourStartingConditionExtension>,

    // Execute presets in random order. If set to true and Direction is also
    // present, Direction will be ignored and presets of the Tour will be recalled
    // randomly.
    #[yaserde(attribute, rename = "RandomPresetOrder")]
    pub random_preset_order: Option<bool>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzpresetTourStartingConditionExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzpresetTourOptions {
    // Indicates whether or not the AutoStart is supported.
    #[yaserde(prefix = "tt", rename = "AutoStart")]
    pub auto_start: bool,

    // Supported options for Preset Tour Starting Condition.
    #[yaserde(prefix = "tt", rename = "StartingCondition")]
    pub starting_condition: PtzpresetTourStartingConditionOptions,

    // Supported options for Preset Tour Spot.
    #[yaserde(prefix = "tt", rename = "TourSpot")]
    pub tour_spot: PtzpresetTourSpotOptions,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzpresetTourSpotOptions {
    // Supported options for detail definition of preset position of the tour spot.
    #[yaserde(prefix = "tt", rename = "PresetDetail")]
    pub preset_detail: PtzpresetTourPresetDetailOptions,

    // Supported range of stay time for a tour spot.
    #[yaserde(prefix = "tt", rename = "StayTime")]
    pub stay_time: DurationRange,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzpresetTourPresetDetailOptions {
    // A list of available Preset Tokens for tour spots.
    #[yaserde(prefix = "tt", rename = "PresetToken")]
    pub preset_token: Vec<ReferenceToken>,

    // An option to indicate Home postion for tour spots.
    #[yaserde(prefix = "tt", rename = "Home")]
    pub home: Option<bool>,

    // Supported range of Pan and Tilt for tour spots.
    #[yaserde(prefix = "tt", rename = "PanTiltPositionSpace")]
    pub pan_tilt_position_space: Option<Space2DDescription>,

    // Supported range of Zoom for a tour spot.
    #[yaserde(prefix = "tt", rename = "ZoomPositionSpace")]
    pub zoom_position_space: Option<Space1DDescription>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<PtzpresetTourPresetDetailOptionsExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzpresetTourPresetDetailOptionsExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzpresetTourStartingConditionOptions {
    // Supported range of Recurring Time.
    #[yaserde(prefix = "tt", rename = "RecurringTime")]
    pub recurring_time: Option<IntRange>,

    // Supported range of Recurring Duration.
    #[yaserde(prefix = "tt", rename = "RecurringDuration")]
    pub recurring_duration: Option<DurationRange>,

    // Supported options for Direction of Preset Tour.
    #[yaserde(prefix = "tt", rename = "Direction")]
    pub direction: Vec<PtzpresetTourDirection>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<PtzpresetTourStartingConditionOptionsExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzpresetTourStartingConditionOptionsExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ImagingStatus {
    #[yaserde(prefix = "tt", rename = "FocusStatus")]
    pub focus_status: FocusStatus,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct FocusStatus {
    // Status of focus position.
    #[yaserde(prefix = "tt", rename = "Position")]
    pub position: f64,

    // Status of focus MoveStatus.
    #[yaserde(prefix = "tt", rename = "MoveStatus")]
    pub move_status: MoveStatus,

    // Error status of focus.
    #[yaserde(prefix = "tt", rename = "Error")]
    pub error: String,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct FocusConfiguration {
    #[yaserde(prefix = "tt", rename = "AutoFocusMode")]
    pub auto_focus_mode: AutoFocusMode,

    #[yaserde(prefix = "tt", rename = "DefaultSpeed")]
    pub default_speed: f64,

    // Parameter to set autofocus near limit (unit: meter).
    #[yaserde(prefix = "tt", rename = "NearLimit")]
    pub near_limit: f64,

    // Parameter to set autofocus far limit (unit: meter).
    // If set to 0.0, infinity will be used.
    #[yaserde(prefix = "tt", rename = "FarLimit")]
    pub far_limit: f64,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum AutoFocusMode {
    Auto,
    Manual,

    __Unknown__(String),
}

impl Default for AutoFocusMode {
    fn default() -> AutoFocusMode {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum Afmodes {
    // Focus of a moving camera is updated only once after stopping a pan, tilt or
    // zoom movement.
    OnceAfterMove,

    __Unknown__(String),
}

impl Default for Afmodes {
    fn default() -> Afmodes {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ImagingSettings {
    // Enabled/disabled BLC mode (on/off).
    #[yaserde(prefix = "tt", rename = "BacklightCompensation")]
    pub backlight_compensation: Option<BacklightCompensation>,

    // Image brightness (unit unspecified).
    #[yaserde(prefix = "tt", rename = "Brightness")]
    pub brightness: Option<f64>,

    // Color saturation of the image (unit unspecified).
    #[yaserde(prefix = "tt", rename = "ColorSaturation")]
    pub color_saturation: Option<f64>,

    // Contrast of the image (unit unspecified).
    #[yaserde(prefix = "tt", rename = "Contrast")]
    pub contrast: Option<f64>,

    // Exposure mode of the device.
    #[yaserde(prefix = "tt", rename = "Exposure")]
    pub exposure: Option<Exposure>,

    // Focus configuration.
    #[yaserde(prefix = "tt", rename = "Focus")]
    pub focus: Option<FocusConfiguration>,

    // Infrared Cutoff Filter settings.
    #[yaserde(prefix = "tt", rename = "IrCutFilter")]
    pub ir_cut_filter: Option<IrCutFilterMode>,

    // Sharpness of the Video image.
    #[yaserde(prefix = "tt", rename = "Sharpness")]
    pub sharpness: Option<f64>,

    // WDR settings.
    #[yaserde(prefix = "tt", rename = "WideDynamicRange")]
    pub wide_dynamic_range: Option<WideDynamicRange>,

    // White balance settings.
    #[yaserde(prefix = "tt", rename = "WhiteBalance")]
    pub white_balance: Option<WhiteBalance>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ImagingSettingsExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ImagingSettingsExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Exposure {
    // Exposure Mode
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: ExposureMode,

    // The exposure priority mode (low noise/framerate).
    #[yaserde(prefix = "tt", rename = "Priority")]
    pub priority: ExposurePriority,

    // Rectangular exposure mask.
    #[yaserde(prefix = "tt", rename = "Window")]
    pub window: Rectangle,

    // Minimum value of exposure time range allowed to be used by the algorithm.
    #[yaserde(prefix = "tt", rename = "MinExposureTime")]
    pub min_exposure_time: f64,

    // Maximum value of exposure time range allowed to be used by the algorithm.
    #[yaserde(prefix = "tt", rename = "MaxExposureTime")]
    pub max_exposure_time: f64,

    // Minimum value of the sensor gain range that is allowed to be used by the
    // algorithm.
    #[yaserde(prefix = "tt", rename = "MinGain")]
    pub min_gain: f64,

    // Maximum value of the sensor gain range that is allowed to be used by the
    // algorithm.
    #[yaserde(prefix = "tt", rename = "MaxGain")]
    pub max_gain: f64,

    // Minimum value of the iris range allowed to be used by the algorithm.
    #[yaserde(prefix = "tt", rename = "MinIris")]
    pub min_iris: f64,

    // Maximum value of the iris range allowed to be used by the algorithm.
    #[yaserde(prefix = "tt", rename = "MaxIris")]
    pub max_iris: f64,

    // The fixed exposure time used by the image sensor (μs).
    #[yaserde(prefix = "tt", rename = "ExposureTime")]
    pub exposure_time: f64,

    // The fixed gain used by the image sensor (dB).
    #[yaserde(prefix = "tt", rename = "Gain")]
    pub gain: f64,

    // The fixed attenuation of input light affected by the iris (dB). 0dB maps to a
    // fully opened iris.
    #[yaserde(prefix = "tt", rename = "Iris")]
    pub iris: f64,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum WideDynamicMode {
    Off,
    On,

    __Unknown__(String),
}

impl Default for WideDynamicMode {
    fn default() -> WideDynamicMode {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct WideDynamicRange {
    // White dynamic range (on/off)
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: WideDynamicMode,

    // Optional level parameter (unitless)
    #[yaserde(prefix = "tt", rename = "Level")]
    pub level: f64,
}

// Enumeration describing the available backlight compenstation modes.
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum BacklightCompensationMode {
    // Backlight compensation is disabled.
    Off,
    // Backlight compensation is enabled.
    On,

    __Unknown__(String),
}

impl Default for BacklightCompensationMode {
    fn default() -> BacklightCompensationMode {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct BacklightCompensation {
    // Backlight compensation mode (on/off).
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: BacklightCompensationMode,

    // Optional level parameter (unit unspecified).
    #[yaserde(prefix = "tt", rename = "Level")]
    pub level: f64,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum ExposurePriority {
    LowNoise,
    FrameRate,

    __Unknown__(String),
}

impl Default for ExposurePriority {
    fn default() -> ExposurePriority {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ImagingOptions {
    #[yaserde(prefix = "tt", rename = "BacklightCompensation")]
    pub backlight_compensation: BacklightCompensationOptions,

    #[yaserde(prefix = "tt", rename = "Brightness")]
    pub brightness: FloatRange,

    #[yaserde(prefix = "tt", rename = "ColorSaturation")]
    pub color_saturation: FloatRange,

    #[yaserde(prefix = "tt", rename = "Contrast")]
    pub contrast: FloatRange,

    #[yaserde(prefix = "tt", rename = "Exposure")]
    pub exposure: ExposureOptions,

    #[yaserde(prefix = "tt", rename = "Focus")]
    pub focus: FocusOptions,

    #[yaserde(prefix = "tt", rename = "IrCutFilterModes")]
    pub ir_cut_filter_modes: Vec<IrCutFilterMode>,

    #[yaserde(prefix = "tt", rename = "Sharpness")]
    pub sharpness: FloatRange,

    #[yaserde(prefix = "tt", rename = "WideDynamicRange")]
    pub wide_dynamic_range: WideDynamicRangeOptions,

    #[yaserde(prefix = "tt", rename = "WhiteBalance")]
    pub white_balance: WhiteBalanceOptions,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct WideDynamicRangeOptions {
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: Vec<WideDynamicMode>,

    #[yaserde(prefix = "tt", rename = "Level")]
    pub level: FloatRange,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct BacklightCompensationOptions {
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: Vec<WideDynamicMode>,

    #[yaserde(prefix = "tt", rename = "Level")]
    pub level: FloatRange,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct FocusOptions {
    #[yaserde(prefix = "tt", rename = "AutoFocusModes")]
    pub auto_focus_modes: Vec<AutoFocusMode>,

    #[yaserde(prefix = "tt", rename = "DefaultSpeed")]
    pub default_speed: FloatRange,

    #[yaserde(prefix = "tt", rename = "NearLimit")]
    pub near_limit: FloatRange,

    #[yaserde(prefix = "tt", rename = "FarLimit")]
    pub far_limit: FloatRange,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ExposureOptions {
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: Vec<ExposureMode>,

    #[yaserde(prefix = "tt", rename = "Priority")]
    pub priority: Vec<ExposurePriority>,

    #[yaserde(prefix = "tt", rename = "MinExposureTime")]
    pub min_exposure_time: FloatRange,

    #[yaserde(prefix = "tt", rename = "MaxExposureTime")]
    pub max_exposure_time: FloatRange,

    #[yaserde(prefix = "tt", rename = "MinGain")]
    pub min_gain: FloatRange,

    #[yaserde(prefix = "tt", rename = "MaxGain")]
    pub max_gain: FloatRange,

    #[yaserde(prefix = "tt", rename = "MinIris")]
    pub min_iris: FloatRange,

    #[yaserde(prefix = "tt", rename = "MaxIris")]
    pub max_iris: FloatRange,

    #[yaserde(prefix = "tt", rename = "ExposureTime")]
    pub exposure_time: FloatRange,

    #[yaserde(prefix = "tt", rename = "Gain")]
    pub gain: FloatRange,

    #[yaserde(prefix = "tt", rename = "Iris")]
    pub iris: FloatRange,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct WhiteBalanceOptions {
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: Vec<WhiteBalanceMode>,

    #[yaserde(prefix = "tt", rename = "YrGain")]
    pub yr_gain: FloatRange,

    #[yaserde(prefix = "tt", rename = "YbGain")]
    pub yb_gain: FloatRange,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct FocusMove {
    // Parameters for the absolute focus control.
    #[yaserde(prefix = "tt", rename = "Absolute")]
    pub absolute: Option<AbsoluteFocus>,

    // Parameters for the relative focus control.
    #[yaserde(prefix = "tt", rename = "Relative")]
    pub relative: Option<RelativeFocus>,

    // Parameter for the continuous focus control.
    #[yaserde(prefix = "tt", rename = "Continuous")]
    pub continuous: Option<ContinuousFocus>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AbsoluteFocus {
    // Position parameter for the absolute focus control.
    #[yaserde(prefix = "tt", rename = "Position")]
    pub position: f64,

    // Speed parameter for the absolute focus control.
    #[yaserde(prefix = "tt", rename = "Speed")]
    pub speed: Option<f64>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RelativeFocus {
    // Distance parameter for the relative focus control.
    #[yaserde(prefix = "tt", rename = "Distance")]
    pub distance: f64,

    // Speed parameter for the relative focus control.
    #[yaserde(prefix = "tt", rename = "Speed")]
    pub speed: Option<f64>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ContinuousFocus {
    // Speed parameter for the Continuous focus control.
    #[yaserde(prefix = "tt", rename = "Speed")]
    pub speed: f64,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MoveOptions {
    #[yaserde(prefix = "tt", rename = "Absolute")]
    pub absolute: Option<AbsoluteFocusOptions>,

    #[yaserde(prefix = "tt", rename = "Relative")]
    pub relative: Option<RelativeFocusOptions>,

    #[yaserde(prefix = "tt", rename = "Continuous")]
    pub continuous: Option<ContinuousFocusOptions>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AbsoluteFocusOptions {
    // Valid ranges of the position.
    #[yaserde(prefix = "tt", rename = "Position")]
    pub position: FloatRange,

    // Valid ranges of the speed.
    #[yaserde(prefix = "tt", rename = "Speed")]
    pub speed: Option<FloatRange>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RelativeFocusOptions {
    // Valid ranges of the distance.
    #[yaserde(prefix = "tt", rename = "Distance")]
    pub distance: FloatRange,

    // Valid ranges of the speed.
    #[yaserde(prefix = "tt", rename = "Speed")]
    pub speed: FloatRange,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ContinuousFocusOptions {
    // Valid ranges of the speed.
    #[yaserde(prefix = "tt", rename = "Speed")]
    pub speed: FloatRange,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum ExposureMode {
    Auto,
    Manual,

    __Unknown__(String),
}

impl Default for ExposureMode {
    fn default() -> ExposureMode {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum Enabled {
    Enabled,
    Disabled,

    __Unknown__(String),
}

impl Default for Enabled {
    fn default() -> Enabled {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum WhiteBalanceMode {
    Auto,
    Manual,

    __Unknown__(String),
}

impl Default for WhiteBalanceMode {
    fn default() -> WhiteBalanceMode {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum IrCutFilterMode {
    On,
    Off,
    Auto,

    __Unknown__(String),
}

impl Default for IrCutFilterMode {
    fn default() -> IrCutFilterMode {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct WhiteBalance {
    // Auto whitebalancing mode (auto/manual).
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: WhiteBalanceMode,

    // Rgain (unitless).
    #[yaserde(prefix = "tt", rename = "CrGain")]
    pub cr_gain: f64,

    // Bgain (unitless).
    #[yaserde(prefix = "tt", rename = "CbGain")]
    pub cb_gain: f64,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ImagingStatus20 {
    // Status of focus.
    #[yaserde(prefix = "tt", rename = "FocusStatus20")]
    pub focus_status_20: Option<FocusStatus20>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ImagingStatus20Extension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ImagingStatus20Extension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct FocusStatus20 {
    // Status of focus position.
    #[yaserde(prefix = "tt", rename = "Position")]
    pub position: f64,

    // Status of focus MoveStatus.
    #[yaserde(prefix = "tt", rename = "MoveStatus")]
    pub move_status: MoveStatus,

    // Error status of focus.
    #[yaserde(prefix = "tt", rename = "Error")]
    pub error: Option<String>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<FocusStatus20Extension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct FocusStatus20Extension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

// Type describing the ImagingSettings of a VideoSource. The supported options
// and ranges can be obtained via the GetOptions command.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ImagingSettings20 {
    // Enabled/disabled BLC mode (on/off).
    #[yaserde(prefix = "tt", rename = "BacklightCompensation")]
    pub backlight_compensation: Option<BacklightCompensation20>,

    // Image brightness (unit unspecified).
    #[yaserde(prefix = "tt", rename = "Brightness")]
    pub brightness: Option<f64>,

    // Color saturation of the image (unit unspecified).
    #[yaserde(prefix = "tt", rename = "ColorSaturation")]
    pub color_saturation: Option<f64>,

    // Contrast of the image (unit unspecified).
    #[yaserde(prefix = "tt", rename = "Contrast")]
    pub contrast: Option<f64>,

    // Exposure mode of the device.
    #[yaserde(prefix = "tt", rename = "Exposure")]
    pub exposure: Option<Exposure20>,

    // Focus configuration.
    #[yaserde(prefix = "tt", rename = "Focus")]
    pub focus: Option<FocusConfiguration20>,

    // Infrared Cutoff Filter settings.
    #[yaserde(prefix = "tt", rename = "IrCutFilter")]
    pub ir_cut_filter: Option<IrCutFilterMode>,

    // Sharpness of the Video image.
    #[yaserde(prefix = "tt", rename = "Sharpness")]
    pub sharpness: Option<f64>,

    // WDR settings.
    #[yaserde(prefix = "tt", rename = "WideDynamicRange")]
    pub wide_dynamic_range: Option<WideDynamicRange20>,

    // White balance settings.
    #[yaserde(prefix = "tt", rename = "WhiteBalance")]
    pub white_balance: Option<WhiteBalance20>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ImagingSettingsExtension20>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ImagingSettingsExtension20 {
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // Optional element to configure Image Stabilization feature.
    #[yaserde(prefix = "tt", rename = "ImageStabilization")]
    pub image_stabilization: Option<ImageStabilization>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ImagingSettingsExtension202>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ImagingSettingsExtension202 {
    // An optional parameter applied to only auto mode to adjust timing of toggling
    // Ir cut filter.
    #[yaserde(prefix = "tt", rename = "IrCutFilterAutoAdjustment")]
    pub ir_cut_filter_auto_adjustment: Vec<IrCutFilterAutoAdjustment>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ImagingSettingsExtension203>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ImagingSettingsExtension203 {
    // Optional element to configure Image Contrast Compensation.
    #[yaserde(prefix = "tt", rename = "ToneCompensation")]
    pub tone_compensation: Option<ToneCompensation>,

    // Optional element to configure Image Defogging.
    #[yaserde(prefix = "tt", rename = "Defogging")]
    pub defogging: Option<Defogging>,

    // Optional element to configure Image Noise Reduction.
    #[yaserde(prefix = "tt", rename = "NoiseReduction")]
    pub noise_reduction: Option<NoiseReduction>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ImagingSettingsExtension204>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ImagingSettingsExtension204 {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ImageStabilization {
    // Parameter to enable/disable Image Stabilization feature.
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: ImageStabilizationMode,

    // Optional level parameter (unit unspecified)
    #[yaserde(prefix = "tt", rename = "Level")]
    pub level: Option<f64>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ImageStabilizationExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ImageStabilizationExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum ImageStabilizationMode {
    Off,
    On,
    Auto,
    Extended,

    __Unknown__(String),
}

impl Default for ImageStabilizationMode {
    fn default() -> ImageStabilizationMode {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct IrCutFilterAutoAdjustment {
    // Specifies which boundaries to automatically toggle Ir cut filter following
    // parameters are applied to. Its options shall be chosen from
    // tt:IrCutFilterAutoBoundaryType.
    #[yaserde(prefix = "tt", rename = "BoundaryType")]
    pub boundary_type: String,

    // Adjusts boundary exposure level for toggling Ir cut filter to on/off
    // specified with unitless normalized value from +1.0 to -1.0. Zero is default
    // and -1.0 is the darkest adjustment (Unitless).
    #[yaserde(prefix = "tt", rename = "BoundaryOffset")]
    pub boundary_offset: Option<f64>,

    // Delay time of toggling Ir cut filter to on/off after crossing of the boundary
    // exposure levels.
    #[yaserde(prefix = "tt", rename = "ResponseTime")]
    pub response_time: Option<xs::Duration>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<IrCutFilterAutoAdjustmentExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct IrCutFilterAutoAdjustmentExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum IrCutFilterAutoBoundaryType {
    Common,
    ToOn,
    ToOff,
    Extended,

    __Unknown__(String),
}

impl Default for IrCutFilterAutoBoundaryType {
    fn default() -> IrCutFilterAutoBoundaryType {
        Self::__Unknown__("No valid variants".into())
    }
}

// Type describing whether WDR mode is enabled or disabled (on/off).
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct WideDynamicRange20 {
    // Wide dynamic range mode (on/off).
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: WideDynamicMode,

    // Optional level parameter (unit unspecified).
    #[yaserde(prefix = "tt", rename = "Level")]
    pub level: Option<f64>,
}

// Type describing whether BLC mode is enabled or disabled (on/off).
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct BacklightCompensation20 {
    // Backlight compensation mode (on/off).
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: BacklightCompensationMode,

    // Optional level parameter (unit unspecified).
    #[yaserde(prefix = "tt", rename = "Level")]
    pub level: Option<f64>,
}

// Type describing the exposure settings.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Exposure20 {
    // Exposure Mode
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: ExposureMode,

    // The exposure priority mode (low noise/framerate).
    #[yaserde(prefix = "tt", rename = "Priority")]
    pub priority: Option<ExposurePriority>,

    // Rectangular exposure mask.
    #[yaserde(prefix = "tt", rename = "Window")]
    pub window: Option<Rectangle>,

    // Minimum value of exposure time range allowed to be used by the algorithm.
    #[yaserde(prefix = "tt", rename = "MinExposureTime")]
    pub min_exposure_time: Option<f64>,

    // Maximum value of exposure time range allowed to be used by the algorithm.
    #[yaserde(prefix = "tt", rename = "MaxExposureTime")]
    pub max_exposure_time: Option<f64>,

    // Minimum value of the sensor gain range that is allowed to be used by the
    // algorithm.
    #[yaserde(prefix = "tt", rename = "MinGain")]
    pub min_gain: Option<f64>,

    // Maximum value of the sensor gain range that is allowed to be used by the
    // algorithm.
    #[yaserde(prefix = "tt", rename = "MaxGain")]
    pub max_gain: Option<f64>,

    // Minimum value of the iris range allowed to be used by the algorithm. 0dB maps
    // to a fully opened iris and positive values map to higher attenuation.
    #[yaserde(prefix = "tt", rename = "MinIris")]
    pub min_iris: Option<f64>,

    // Maximum value of the iris range allowed to be used by the algorithm. 0dB maps
    // to a fully opened iris and positive values map to higher attenuation.
    #[yaserde(prefix = "tt", rename = "MaxIris")]
    pub max_iris: Option<f64>,

    // The fixed exposure time used by the image sensor (μs).
    #[yaserde(prefix = "tt", rename = "ExposureTime")]
    pub exposure_time: Option<f64>,

    // The fixed gain used by the image sensor (dB).
    #[yaserde(prefix = "tt", rename = "Gain")]
    pub gain: Option<f64>,

    // The fixed attenuation of input light affected by the iris (dB). 0dB maps to a
    // fully opened iris and positive values map to higher attenuation.
    #[yaserde(prefix = "tt", rename = "Iris")]
    pub iris: Option<f64>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ToneCompensation {
    // Parameter to enable/disable or automatic ToneCompensation feature. Its
    // options shall be chosen from tt:ToneCompensationMode Type.
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: String,

    // Optional level parameter specified with unitless normalized value from 0.0 to
    // +1.0.
    #[yaserde(prefix = "tt", rename = "Level")]
    pub level: Option<f64>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ToneCompensationExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ToneCompensationExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum ToneCompensationMode {
    Off,
    On,
    Auto,

    __Unknown__(String),
}

impl Default for ToneCompensationMode {
    fn default() -> ToneCompensationMode {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Defogging {
    // Parameter to enable/disable or automatic Defogging feature. Its options shall
    // be chosen from tt:DefoggingMode Type.
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: String,

    // Optional level parameter specified with unitless normalized value from 0.0 to
    // +1.0.
    #[yaserde(prefix = "tt", rename = "Level")]
    pub level: Option<f64>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<DefoggingExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct DefoggingExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum DefoggingMode {
    Off,
    On,
    Auto,

    __Unknown__(String),
}

impl Default for DefoggingMode {
    fn default() -> DefoggingMode {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct NoiseReduction {
    // Level parameter specified with unitless normalized value from 0.0 to +1.0.
    // Level=0 means no noise reduction or minimal noise reduction.
    #[yaserde(prefix = "tt", rename = "Level")]
    pub level: f64,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ImagingOptions20 {
    // Valid range of Backlight Compensation.
    #[yaserde(prefix = "tt", rename = "BacklightCompensation")]
    pub backlight_compensation: Option<BacklightCompensationOptions20>,

    // Valid range of Brightness.
    #[yaserde(prefix = "tt", rename = "Brightness")]
    pub brightness: Option<FloatRange>,

    // Valid range of Color Saturation.
    #[yaserde(prefix = "tt", rename = "ColorSaturation")]
    pub color_saturation: Option<FloatRange>,

    // Valid range of Contrast.
    #[yaserde(prefix = "tt", rename = "Contrast")]
    pub contrast: Option<FloatRange>,

    // Valid range of Exposure.
    #[yaserde(prefix = "tt", rename = "Exposure")]
    pub exposure: Option<ExposureOptions20>,

    // Valid range of Focus.
    #[yaserde(prefix = "tt", rename = "Focus")]
    pub focus: Option<FocusOptions20>,

    // Valid range of IrCutFilterModes.
    #[yaserde(prefix = "tt", rename = "IrCutFilterModes")]
    pub ir_cut_filter_modes: Vec<IrCutFilterMode>,

    // Valid range of Sharpness.
    #[yaserde(prefix = "tt", rename = "Sharpness")]
    pub sharpness: Option<FloatRange>,

    // Valid range of WideDynamicRange.
    #[yaserde(prefix = "tt", rename = "WideDynamicRange")]
    pub wide_dynamic_range: Option<WideDynamicRangeOptions20>,

    // Valid range of WhiteBalance.
    #[yaserde(prefix = "tt", rename = "WhiteBalance")]
    pub white_balance: Option<WhiteBalanceOptions20>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ImagingOptions20Extension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ImagingOptions20Extension {
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // Options of parameters for Image Stabilization feature.
    #[yaserde(prefix = "tt", rename = "ImageStabilization")]
    pub image_stabilization: Option<ImageStabilizationOptions>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ImagingOptions20Extension2>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ImagingOptions20Extension2 {
    // Options of parameters for adjustment of Ir cut filter auto mode.
    #[yaserde(prefix = "tt", rename = "IrCutFilterAutoAdjustment")]
    pub ir_cut_filter_auto_adjustment: Option<IrCutFilterAutoAdjustmentOptions>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ImagingOptions20Extension3>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ImagingOptions20Extension3 {
    // Options of parameters for Tone Compensation feature.
    #[yaserde(prefix = "tt", rename = "ToneCompensationOptions")]
    pub tone_compensation_options: Option<ToneCompensationOptions>,

    // Options of parameters for Defogging feature.
    #[yaserde(prefix = "tt", rename = "DefoggingOptions")]
    pub defogging_options: Option<DefoggingOptions>,

    // Options of parameter for Noise Reduction feature.
    #[yaserde(prefix = "tt", rename = "NoiseReductionOptions")]
    pub noise_reduction_options: Option<NoiseReductionOptions>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ImagingOptions20Extension4>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ImagingOptions20Extension4 {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ImageStabilizationOptions {
    // Supported options of Image Stabilization mode parameter.
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: Vec<ImageStabilizationMode>,

    // Valid range of the Image Stabilization.
    #[yaserde(prefix = "tt", rename = "Level")]
    pub level: Option<FloatRange>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ImageStabilizationOptionsExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ImageStabilizationOptionsExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct IrCutFilterAutoAdjustmentOptions {
    // Supported options of boundary types for adjustment of Ir cut filter auto
    // mode. The opptions shall be chosen from tt:IrCutFilterAutoBoundaryType.
    #[yaserde(prefix = "tt", rename = "BoundaryType")]
    pub boundary_type: Vec<String>,

    // Indicates whether or not boundary offset for toggling Ir cut filter is
    // supported.
    #[yaserde(prefix = "tt", rename = "BoundaryOffset")]
    pub boundary_offset: Option<bool>,

    // Supported range of delay time for toggling Ir cut filter.
    #[yaserde(prefix = "tt", rename = "ResponseTimeRange")]
    pub response_time_range: Option<DurationRange>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<IrCutFilterAutoAdjustmentOptionsExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct IrCutFilterAutoAdjustmentOptionsExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct WideDynamicRangeOptions20 {
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: Vec<WideDynamicMode>,

    #[yaserde(prefix = "tt", rename = "Level")]
    pub level: Option<FloatRange>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct BacklightCompensationOptions20 {
    // 'ON' or 'OFF'
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: Vec<BacklightCompensationMode>,

    // Level range of BacklightCompensation.
    #[yaserde(prefix = "tt", rename = "Level")]
    pub level: Option<FloatRange>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ExposureOptions20 {
    // Exposure Mode
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: Vec<ExposureMode>,

    // The exposure priority mode (low noise/framerate).
    #[yaserde(prefix = "tt", rename = "Priority")]
    pub priority: Vec<ExposurePriority>,

    // Valid range of the Minimum ExposureTime.
    #[yaserde(prefix = "tt", rename = "MinExposureTime")]
    pub min_exposure_time: Option<FloatRange>,

    // Valid range of the Maximum ExposureTime.
    #[yaserde(prefix = "tt", rename = "MaxExposureTime")]
    pub max_exposure_time: Option<FloatRange>,

    // Valid range of the Minimum Gain.
    #[yaserde(prefix = "tt", rename = "MinGain")]
    pub min_gain: Option<FloatRange>,

    // Valid range of the Maximum Gain.
    #[yaserde(prefix = "tt", rename = "MaxGain")]
    pub max_gain: Option<FloatRange>,

    // Valid range of the Minimum Iris.
    #[yaserde(prefix = "tt", rename = "MinIris")]
    pub min_iris: Option<FloatRange>,

    // Valid range of the Maximum Iris.
    #[yaserde(prefix = "tt", rename = "MaxIris")]
    pub max_iris: Option<FloatRange>,

    // Valid range of the ExposureTime.
    #[yaserde(prefix = "tt", rename = "ExposureTime")]
    pub exposure_time: Option<FloatRange>,

    // Valid range of the Gain.
    #[yaserde(prefix = "tt", rename = "Gain")]
    pub gain: Option<FloatRange>,

    // Valid range of the Iris.
    #[yaserde(prefix = "tt", rename = "Iris")]
    pub iris: Option<FloatRange>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MoveOptions20 {
    // Valid ranges for the absolute control.
    #[yaserde(prefix = "tt", rename = "Absolute")]
    pub absolute: Option<AbsoluteFocusOptions>,

    // Valid ranges for the relative control.
    #[yaserde(prefix = "tt", rename = "Relative")]
    pub relative: Option<RelativeFocusOptions20>,

    // Valid ranges for the continuous control.
    #[yaserde(prefix = "tt", rename = "Continuous")]
    pub continuous: Option<ContinuousFocusOptions>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RelativeFocusOptions20 {
    // Valid ranges of the distance.
    #[yaserde(prefix = "tt", rename = "Distance")]
    pub distance: FloatRange,

    // Valid ranges of the speed.
    #[yaserde(prefix = "tt", rename = "Speed")]
    pub speed: Option<FloatRange>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct WhiteBalance20 {
    // 'AUTO' or 'MANUAL'
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: WhiteBalanceMode,

    // Rgain (unitless).
    #[yaserde(prefix = "tt", rename = "CrGain")]
    pub cr_gain: Option<f64>,

    // Bgain (unitless).
    #[yaserde(prefix = "tt", rename = "CbGain")]
    pub cb_gain: Option<f64>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<WhiteBalance20Extension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct WhiteBalance20Extension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct FocusConfiguration20 {
    // Mode of auto focus.
    #[yaserde(prefix = "tt", rename = "AutoFocusMode")]
    pub auto_focus_mode: AutoFocusMode,

    #[yaserde(prefix = "tt", rename = "DefaultSpeed")]
    pub default_speed: Option<f64>,

    // Parameter to set autofocus near limit (unit: meter).
    #[yaserde(prefix = "tt", rename = "NearLimit")]
    pub near_limit: Option<f64>,

    // Parameter to set autofocus far limit (unit: meter).
    #[yaserde(prefix = "tt", rename = "FarLimit")]
    pub far_limit: Option<f64>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<FocusConfiguration20Extension>,

    // Zero or more modes as defined in enumeration tt:AFModes.
    #[yaserde(attribute, rename = "AFMode")]
    pub af_mode: Option<StringAttrList>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct FocusConfiguration20Extension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct WhiteBalanceOptions20 {
    // Mode of WhiteBalance.
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: Vec<WhiteBalanceMode>,

    #[yaserde(prefix = "tt", rename = "YrGain")]
    pub yr_gain: Option<FloatRange>,

    #[yaserde(prefix = "tt", rename = "YbGain")]
    pub yb_gain: Option<FloatRange>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<WhiteBalanceOptions20Extension>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct WhiteBalanceOptions20Extension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct FocusOptions20 {
    // Supported modes for auto focus.
    #[yaserde(prefix = "tt", rename = "AutoFocusModes")]
    pub auto_focus_modes: Vec<AutoFocusMode>,

    // Valid range of DefaultSpeed.
    #[yaserde(prefix = "tt", rename = "DefaultSpeed")]
    pub default_speed: Option<FloatRange>,

    // Valid range of NearLimit.
    #[yaserde(prefix = "tt", rename = "NearLimit")]
    pub near_limit: Option<FloatRange>,

    // Valid range of FarLimit.
    #[yaserde(prefix = "tt", rename = "FarLimit")]
    pub far_limit: Option<FloatRange>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<FocusOptions20Extension>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct FocusOptions20Extension {
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // Supported options for auto focus. Options shall be chosen from tt:AFModes.
    #[yaserde(prefix = "tt", rename = "AFModes")]
    pub af_modes: Option<StringAttrList>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ToneCompensationOptions {
    // Supported options for Tone Compensation mode. Its options shall be chosen
    // from tt:ToneCompensationMode Type.
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: Vec<String>,

    // Indicates whether or not support Level parameter for Tone Compensation.
    #[yaserde(prefix = "tt", rename = "Level")]
    pub level: bool,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct DefoggingOptions {
    // Supported options for Defogging mode. Its options shall be chosen from
    // tt:DefoggingMode Type.
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: Vec<String>,

    // Indicates whether or not support Level parameter for Defogging.
    #[yaserde(prefix = "tt", rename = "Level")]
    pub level: bool,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct NoiseReductionOptions {
    // Indicates whether or not support Level parameter for NoiseReduction.
    #[yaserde(prefix = "tt", rename = "Level")]
    pub level: bool,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct TopicNamespaceLocation(pub String);

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum PropertyOperation {
    Initialized,
    Deleted,
    Changed,

    __Unknown__(String),
}

impl Default for PropertyOperation {
    fn default() -> PropertyOperation {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Message {
    // Token value pairs that triggered this message. Typically only one item is
    // present.
    #[yaserde(prefix = "tt", rename = "Source")]
    pub source: Option<ItemList>,

    #[yaserde(prefix = "tt", rename = "Key")]
    pub key: Option<ItemList>,

    #[yaserde(prefix = "tt", rename = "Data")]
    pub data: Option<ItemList>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<MessageExtension>,

    #[yaserde(attribute, rename = "UtcTime")]
    pub utc_time: String,

    #[yaserde(attribute, rename = "PropertyOperation")]
    pub property_operation: Option<PropertyOperation>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MessageExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

// List of parameters according to the corresponding ItemListDescription.
// Each item in the list shall have a unique name.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ItemList {
    // Value name pair as defined by the corresponding description.
    #[yaserde(prefix = "tt", rename = "SimpleItem")]
    pub simple_item: Vec<SimpleItemType>,

    // Complex value structure.
    #[yaserde(prefix = "tt", rename = "ElementItem")]
    pub element_item: Vec<ElementItemType>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ItemListExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SimpleItemType {
    // Item name.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,

    // Item value. The type is defined in the corresponding description.
    #[yaserde(attribute, rename = "Value")]
    pub value: String,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ElementItemType {
    // XML tree contiaing the element value as defined in the corresponding
    // description.
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // Item name.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ItemListExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MessageDescription {
    // Set of tokens producing this message. The list may only contain
    // SimpleItemDescription items.
    // The set of tokens identify the component within the WS-Endpoint, which is
    // responsible for the producing the message.
    #[yaserde(prefix = "tt", rename = "Source")]
    pub source: Option<ItemListDescription>,

    // Describes optional message payload parameters that may be used as key. E.g.
    // object IDs of tracked objects are conveyed as key.
    #[yaserde(prefix = "tt", rename = "Key")]
    pub key: Option<ItemListDescription>,

    // Describes the payload of the message.
    #[yaserde(prefix = "tt", rename = "Data")]
    pub data: Option<ItemListDescription>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<MessageDescriptionExtension>,

    // Must be set to true when the described Message relates to a property. An
    // alternative term of "property" is a "state" in contrast to a pure event,
    // which contains relevant information for only a single point in time.
    #[yaserde(attribute, rename = "IsProperty")]
    pub is_property: Option<bool>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MessageDescriptionExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

// Describes a list of items. Each item in the list shall have a unique name.
// The list is designed as linear structure without optional or unbounded
// elements.
// Use ElementItems only when complex structures are inevitable.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ItemListDescription {
    // Description of a simple item. The type must be of cathegory simpleType
    // (xs:string, xs:integer, xs:float, ...).
    #[yaserde(prefix = "tt", rename = "SimpleItemDescription")]
    pub simple_item_description: Vec<SimpleItemDescriptionType>,

    // Description of a complex type. The Type must reference a defined type.
    #[yaserde(prefix = "tt", rename = "ElementItemDescription")]
    pub element_item_description: Vec<ElementItemDescriptionType>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ItemListDescriptionExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SimpleItemDescriptionType {
    // Item name. Must be unique within a list.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,

    #[yaserde(attribute, rename = "Type")]
    pub _type: String,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ElementItemDescriptionType {
    // Item name. Must be unique within a list.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,

    // The type of the item. The Type must reference a defined type.
    #[yaserde(attribute, rename = "Type")]
    pub _type: String,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ItemListDescriptionExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Polyline {
    #[yaserde(prefix = "tt", rename = "Point")]
    pub point: Vec<Vector>,
}

// type Polyline = Polyline;

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum Direction {
    Left,
    Right,
    Any,

    __Unknown__(String),
}

impl Default for Direction {
    fn default() -> Direction {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AnalyticsEngineConfiguration {
    #[yaserde(prefix = "tt", rename = "AnalyticsModule")]
    pub analytics_module: Vec<Config>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<AnalyticsEngineConfigurationExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AnalyticsEngineConfigurationExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RuleEngineConfiguration {
    #[yaserde(prefix = "tt", rename = "Rule")]
    pub rule: Vec<Config>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<RuleEngineConfigurationExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RuleEngineConfigurationExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Config {
    // List of configuration parameters as defined in the correspding description.
    #[yaserde(prefix = "tt", rename = "Parameters")]
    pub parameters: ItemList,

    // Name of the configuration.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,

    // The Type attribute specifies the type of rule and shall be equal to value of
    // one of Name attributes of ConfigDescription elements returned by
    // GetSupportedRules and GetSupportedAnalyticsModules command.
    #[yaserde(attribute, rename = "Type")]
    pub _type: String,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ConfigDescription {
    // List describing the configuration parameters. The names of the parameters
    // must be unique. If possible SimpleItems
    // should be used to transport the information to ease parsing of dynamically
    // defined messages by a client
    // application.
    #[yaserde(prefix = "tt", rename = "Parameters")]
    pub parameters: ItemListDescription,

    // The analytics modules and rule engine produce Events, which must be listed
    // within the Analytics Module Description. In order to do so
    // the structure of the Message is defined and consists of three groups: Source,
    // Key, and Data. It is recommended to use SimpleItemDescriptions wherever
    // applicable.
    // The name of all Items must be unique within all Items contained in any group
    // of this Message.
    // Depending on the component multiple parameters or none may be needed to
    // identify the component uniquely.
    #[yaserde(prefix = "tt", rename = "Messages")]
    pub messages: Vec<MessagesType>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ConfigDescriptionExtension>,

    // The Name attribute (e.g. "tt::LineDetector") uniquely identifies the type of
    // rule, not a type definition in a schema.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,

    // The fixed attribute signals that it is not allowed to add or remove this type
    // of configuration.
    #[yaserde(attribute, rename = "fixed")]
    pub fixed: Option<bool>,

    // The maxInstances attribute signals the maximum number of instances per
    // configuration.
    #[yaserde(attribute, rename = "maxInstances")]
    pub max_instances: Option<i64>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MessagesType {
    // The ParentTopic labels the message (e.g. "nn:RuleEngine/LineCrossing"). The
    // real message can extend the ParentTopic
    // by for example the name of the instaniated rule (e.g.
    // "nn:RuleEngine/LineCrossing/corssMyFirstLine").
    // Even without knowing the complete topic name, the subscriber will be able to
    // distiguish the
    // messages produced by different rule instances of the same type via the Source
    // fields of the message.
    // There the name of the rule instance, which produced the message, must be
    // listed.
    #[yaserde(prefix = "tt", rename = "ParentTopic")]
    pub parent_topic: String,

    pub base: MessageDescription,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ConfigDescriptionExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SupportedRules {
    // Lists the location of all schemas that are referenced in the rules.
    #[yaserde(prefix = "tt", rename = "RuleContentSchemaLocation")]
    pub rule_content_schema_location: Vec<String>,

    // List of rules supported by the Video Analytics configuration..
    #[yaserde(prefix = "tt", rename = "RuleDescription")]
    pub rule_description: Vec<ConfigDescription>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<SupportedRulesExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SupportedRulesExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SupportedAnalyticsModules {
    // It optionally contains a list of URLs that provide the location of schema
    // files.
    // These schema files describe the types and elements used in the analytics
    // module descriptions.
    // Analytics module descriptions that reference types or elements imported from
    // any ONVIF defined schema files
    // need not explicitly list those schema files.
    #[yaserde(prefix = "tt", rename = "AnalyticsModuleContentSchemaLocation")]
    pub analytics_module_content_schema_location: Vec<String>,

    #[yaserde(prefix = "tt", rename = "AnalyticsModuleDescription")]
    pub analytics_module_description: Vec<ConfigDescription>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<SupportedAnalyticsModulesExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SupportedAnalyticsModulesExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PolygonConfiguration {
    // Contains Polygon configuration for rule parameters
    #[yaserde(prefix = "tt", rename = "Polygon")]
    pub polygon: Polygon,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PolylineArray {
    // Contains array of Polyline
    #[yaserde(prefix = "tt", rename = "Segment")]
    pub segment: Vec<Polyline>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<PolylineArrayExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PolylineArrayExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PolylineArrayConfiguration {
    // Contains PolylineArray configuration data
    #[yaserde(prefix = "tt", rename = "PolylineArray")]
    pub polyline_array: PolylineArray,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MotionExpression {
    // Motion Expression data structure contains motion expression which is based on
    // Scene Descriptor schema with XPATH syntax. The Type argument could allow
    // introduction of different dialects
    #[yaserde(prefix = "tt", rename = "Expression")]
    pub expression: String,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,
    #[yaserde(attribute, rename = "Type")]
    pub _type: Option<String>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MotionExpressionConfiguration {
    // Contains Rule MotionExpression configuration
    #[yaserde(prefix = "tt", rename = "MotionExpression")]
    pub motion_expression: MotionExpression,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct CellLayout {
    // Mapping of the cell grid to the Video frame. The cell grid is starting from
    // the upper left corner and x dimension is going from left to right and the y
    // dimension from up to down.
    #[yaserde(prefix = "tt", rename = "Transformation")]
    pub transformation: Transformation,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // Number of columns of the cell grid (x dimension)
    #[yaserde(attribute, rename = "Columns")]
    pub columns: i64,

    // Number of rows of the cell grid (y dimension)
    #[yaserde(attribute, rename = "Rows")]
    pub rows: i64,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

// Configuration of the streaming and coding settings of a Video window.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PaneConfiguration {
    // Optional name of the pane configuration.
    #[yaserde(prefix = "tt", rename = "PaneName")]
    pub pane_name: Option<String>,

    // If the device has audio outputs, this element contains a pointer to the audio
    // output that is associated with the pane. A client
    // can retrieve the available audio outputs of a device using the
    // GetAudioOutputs command of the DeviceIO service.
    #[yaserde(prefix = "tt", rename = "AudioOutputToken")]
    pub audio_output_token: Option<ReferenceToken>,

    // If the device has audio sources, this element contains a pointer to the audio
    // source that is associated with this pane.
    // The audio connection from a decoder device to the NVT is established using
    // the backchannel mechanism. A client can retrieve the available audio sources
    // of a device using the GetAudioSources command of the
    // DeviceIO service.
    #[yaserde(prefix = "tt", rename = "AudioSourceToken")]
    pub audio_source_token: Option<ReferenceToken>,

    // The configuration of the audio encoder including codec, bitrate
    // and sample rate.
    #[yaserde(prefix = "tt", rename = "AudioEncoderConfiguration")]
    pub audio_encoder_configuration: Option<AudioEncoderConfiguration>,

    // A pointer to a Receiver that has the necessary information to receive
    // data from a Transmitter. This Receiver can be connected and the network video
    // decoder displays the received data on the specified outputs. A client can
    // retrieve the available Receivers using the
    // GetReceivers command of the Receiver Service.
    #[yaserde(prefix = "tt", rename = "ReceiverToken")]
    pub receiver_token: Option<ReferenceToken>,

    // A unique identifier in the display device.
    #[yaserde(prefix = "tt", rename = "Token")]
    pub token: ReferenceToken,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

// A pane layout describes one Video window of a display. It links a pane
// configuration to a region of the screen.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PaneLayout {
    // Reference to the configuration of the streaming and coding parameters.
    #[yaserde(prefix = "tt", rename = "Pane")]
    pub pane: ReferenceToken,

    // Describes the location and size of the area on the monitor. The area
    // coordinate values are espressed in normalized units [-1.0, 1.0].
    #[yaserde(prefix = "tt", rename = "Area")]
    pub area: Rectangle,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

// A layout describes a set of Video windows that are displayed simultaniously
// on a display.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Layout {
    // List of panes assembling the display layout.
    #[yaserde(prefix = "tt", rename = "PaneLayout")]
    pub pane_layout: Vec<PaneLayout>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<LayoutExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct LayoutExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

// This type contains the Audio and Video coding capabilities of a display
// service.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct CodingCapabilities {
    // If the device supports audio encoding this section describes the supported
    // codecs and their configuration.
    #[yaserde(prefix = "tt", rename = "AudioEncodingCapabilities")]
    pub audio_encoding_capabilities: Option<AudioEncoderConfigurationOptions>,

    // If the device supports audio decoding this section describes the supported
    // codecs and their settings.
    #[yaserde(prefix = "tt", rename = "AudioDecodingCapabilities")]
    pub audio_decoding_capabilities: Option<AudioDecoderConfigurationOptions>,

    // This section describes the supported video codesc and their configuration.
    #[yaserde(prefix = "tt", rename = "VideoDecodingCapabilities")]
    pub video_decoding_capabilities: VideoDecoderConfigurationOptions,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

// The options supported for a display layout.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct LayoutOptions {
    // Lists the possible Pane Layouts of the Video Output
    #[yaserde(prefix = "tt", rename = "PaneLayoutOptions")]
    pub pane_layout_options: Vec<PaneLayoutOptions>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<LayoutOptionsExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct LayoutOptionsExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

// Description of a pane layout describing a complete display layout.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PaneLayoutOptions {
    // List of areas assembling a layout. Coordinate values are in the range [-1.0,
    // 1.0].
    #[yaserde(prefix = "tt", rename = "Area")]
    pub area: Vec<Rectangle>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<PaneOptionExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PaneOptionExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

// Description of a receiver, including its token and configuration.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Receiver {
    // Unique identifier of the receiver.
    #[yaserde(prefix = "tt", rename = "Token")]
    pub token: ReferenceToken,

    // Describes the configuration of the receiver.
    #[yaserde(prefix = "tt", rename = "Configuration")]
    pub configuration: ReceiverConfiguration,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

// Describes the configuration of a receiver.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ReceiverConfiguration {
    // The following connection modes are defined:
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: ReceiverMode,

    // Details of the URI to which the receiver should connect.
    #[yaserde(prefix = "tt", rename = "MediaUri")]
    pub media_uri: String,

    // Stream connection parameters.
    #[yaserde(prefix = "tt", rename = "StreamSetup")]
    pub stream_setup: StreamSetup,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

// Specifies a receiver connection mode.
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum ReceiverMode {
    // The receiver connects on demand, as required by consumers of the media
    // streams.
    AutoConnect,
    // The receiver attempts to maintain a persistent connection to the configured
    // endpoint.
    AlwaysConnect,
    // The receiver does not attempt to connect.
    NeverConnect,
    // This case should never happen.
    Unknown,

    __Unknown__(String),
}

impl Default for ReceiverMode {
    fn default() -> ReceiverMode {
        Self::__Unknown__("No valid variants".into())
    }
}

// Specifies the current connection state of the receiver.
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum ReceiverState {
    // The receiver is not connected.
    NotConnected,
    // The receiver is attempting to connect.
    Connecting,
    // The receiver is connected.
    Connected,
    // This case should never happen.
    Unknown,

    __Unknown__(String),
}

impl Default for ReceiverState {
    fn default() -> ReceiverState {
        Self::__Unknown__("No valid variants".into())
    }
}

// Contains information about a receiver's current state.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ReceiverStateInformation {
    // The connection state of the receiver may have one of the following states:
    #[yaserde(prefix = "tt", rename = "State")]
    pub state: ReceiverState,

    // Indicates whether or not the receiver was created automatically.
    #[yaserde(prefix = "tt", rename = "AutoCreated")]
    pub auto_created: bool,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct ReceiverReference(pub ReferenceToken);

#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct RecordingReference(pub ReferenceToken);

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SourceReference {
    #[yaserde(prefix = "tt", rename = "Token")]
    pub token: ReferenceToken,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,
    #[yaserde(attribute, rename = "Type")]
    pub _type: Option<String>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct TrackReference(pub ReferenceToken);

#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct Description(pub String);

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct DateTimeRange {
    #[yaserde(prefix = "tt", rename = "From")]
    pub from: String,

    #[yaserde(prefix = "tt", rename = "Until")]
    pub until: String,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RecordingSummary {
    // The earliest point in time where there is recorded data on the device.
    #[yaserde(prefix = "tt", rename = "DataFrom")]
    pub data_from: String,

    // The most recent point in time where there is recorded data on the device.
    #[yaserde(prefix = "tt", rename = "DataUntil")]
    pub data_until: String,

    // The device contains this many recordings.
    #[yaserde(prefix = "tt", rename = "NumberRecordings")]
    pub number_recordings: i32,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

// A structure for defining a limited scope when searching in recorded data.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SearchScope {
    // A list of sources that are included in the scope. If this list is included,
    // only data from one of these sources shall be searched.
    #[yaserde(prefix = "tt", rename = "IncludedSources")]
    pub included_sources: Vec<SourceReference>,

    // A list of recordings that are included in the scope. If this list is
    // included, only data from one of these recordings shall be searched.
    #[yaserde(prefix = "tt", rename = "IncludedRecordings")]
    pub included_recordings: Vec<RecordingReference>,

    // An xpath expression used to specify what recordings to search. Only those
    // recordings with an RecordingInformation structure that matches the filter
    // shall be searched.
    #[yaserde(prefix = "tt", rename = "RecordingInformationFilter")]
    pub recording_information_filter: Option<XpathExpression>,

    // Extension point
    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<SearchScopeExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SearchScopeExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct EventFilter {
    // //
//TODO: any_attribute macros
//  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzpositionFilter {
    // The lower boundary of the PTZ volume to look for.
    #[yaserde(prefix = "tt", rename = "MinPosition")]
    pub min_position: Ptzvector,

    // The upper boundary of the PTZ volume to look for.
    #[yaserde(prefix = "tt", rename = "MaxPosition")]
    pub max_position: Ptzvector,

    // If true, search for when entering the specified PTZ volume.
    #[yaserde(prefix = "tt", rename = "EnterOrExit")]
    pub enter_or_exit: bool,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MetadataFilter {
    #[yaserde(prefix = "tt", rename = "MetadataStreamFilter")]
    pub metadata_stream_filter: XpathExpression,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct XpathExpression(pub String);

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct FindRecordingResultList {
    // The state of the search when the result is returned. Indicates if there can
    // be more results, or if the search is completed.
    #[yaserde(prefix = "tt", rename = "SearchState")]
    pub search_state: SearchState,

    // A RecordingInformation structure for each found recording matching the
    // search.
    #[yaserde(prefix = "tt", rename = "RecordingInformation")]
    pub recording_information: Vec<RecordingInformation>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct FindEventResultList {
    // The state of the search when the result is returned. Indicates if there can
    // be more results, or if the search is completed.
    #[yaserde(prefix = "tt", rename = "SearchState")]
    pub search_state: SearchState,

    // A FindEventResult structure for each found event matching the search.
    #[yaserde(prefix = "tt", rename = "Result")]
    pub result: Vec<FindEventResult>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct FindEventResult {
    // The recording where this event was found. Empty string if no recording is
    // associated with this event.
    #[yaserde(prefix = "tt", rename = "RecordingToken")]
    pub recording_token: RecordingReference,

    // A reference to the track where this event was found. Empty string if no track
    // is associated with this event.
    #[yaserde(prefix = "tt", rename = "TrackToken")]
    pub track_token: TrackReference,

    // The time when the event occured.
    #[yaserde(prefix = "tt", rename = "Time")]
    pub time: String,

    // The description of the event.
    #[yaserde(prefix = "tt", rename = "Event")]
    pub event: wsnt::NotificationMessageHolderType,

    // If true, indicates that the event is a virtual event generated for this
    // particular search session to give the state of a property at the start time
    // of the search.
    #[yaserde(prefix = "tt", rename = "StartStateEvent")]
    pub start_state_event: bool,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct FindPTZPositionResultList {
    // The state of the search when the result is returned. Indicates if there can
    // be more results, or if the search is completed.
    #[yaserde(prefix = "tt", rename = "SearchState")]
    pub search_state: SearchState,

    // A FindPTZPositionResult structure for each found PTZ position matching the
    // search.
    #[yaserde(prefix = "tt", rename = "Result")]
    pub result: Vec<FindPTZPositionResult>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct FindPTZPositionResult {
    // A reference to the recording containing the PTZ position.
    #[yaserde(prefix = "tt", rename = "RecordingToken")]
    pub recording_token: RecordingReference,

    // A reference to the metadata track containing the PTZ position.
    #[yaserde(prefix = "tt", rename = "TrackToken")]
    pub track_token: TrackReference,

    // The time when the PTZ position was valid.
    #[yaserde(prefix = "tt", rename = "Time")]
    pub time: String,

    // The PTZ position.
    #[yaserde(prefix = "tt", rename = "Position")]
    pub position: Ptzvector,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct FindMetadataResultList {
    // The state of the search when the result is returned. Indicates if there can
    // be more results, or if the search is completed.
    #[yaserde(prefix = "tt", rename = "SearchState")]
    pub search_state: SearchState,

    // A FindMetadataResult structure for each found set of Metadata matching the
    // search.
    #[yaserde(prefix = "tt", rename = "Result")]
    pub result: Vec<FindMetadataResult>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct FindMetadataResult {
    // A reference to the recording containing the metadata.
    #[yaserde(prefix = "tt", rename = "RecordingToken")]
    pub recording_token: RecordingReference,

    // A reference to the metadata track containing the matching metadata.
    #[yaserde(prefix = "tt", rename = "TrackToken")]
    pub track_token: TrackReference,

    // The point in time when the matching metadata occurs in the metadata track.
    #[yaserde(prefix = "tt", rename = "Time")]
    pub time: String,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum SearchState {
    // The search is queued and not yet started.
    Queued,
    // The search is underway and not yet completed.
    Searching,
    // The search has been completed and no new results will be found.
    Completed,
    // The state of the search is unknown. (This is not a valid response from
    // GetSearchState.)
    Unknown,

    __Unknown__(String),
}

impl Default for SearchState {
    fn default() -> SearchState {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct JobToken(pub ReferenceToken);

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RecordingInformation {
    #[yaserde(prefix = "tt", rename = "RecordingToken")]
    pub recording_token: RecordingReference,

    // Information about the source of the recording. This gives a description of
    // where the data in the recording comes from. Since a single
    // recording is intended to record related material, there is just one source.
    // It is indicates the physical location or the
    // major data source for the recording. Currently the recordingconfiguration
    // cannot describe each individual data source.
    #[yaserde(prefix = "tt", rename = "Source")]
    pub source: RecordingSourceInformation,

    #[yaserde(prefix = "tt", rename = "EarliestRecording")]
    pub earliest_recording: Option<String>,

    #[yaserde(prefix = "tt", rename = "LatestRecording")]
    pub latest_recording: Option<String>,

    #[yaserde(prefix = "tt", rename = "Content")]
    pub content: Description,

    // Basic information about the track. Note that a track may represent a single
    // contiguous time span or consist of multiple slices.
    #[yaserde(prefix = "tt", rename = "Track")]
    pub track: Vec<TrackInformation>,

    #[yaserde(prefix = "tt", rename = "RecordingStatus")]
    pub recording_status: RecordingStatus,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

// A set of informative desciptions of a data source. The Search searvice allows
// a client to filter on recordings based on information in this structure.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RecordingSourceInformation {
    // Identifier for the source chosen by the client that creates the structure.
    // This identifier is opaque to the device. Clients may use any type of URI for
    // this field. A device shall support at least 128 characters.
    #[yaserde(prefix = "tt", rename = "SourceId")]
    pub source_id: String,

    // Informative user readable name of the source, e.g. "Camera23". A device shall
    // support at least 20 characters.
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: Name,

    // Informative description of the physical location of the source, e.g. the
    // coordinates on a map.
    #[yaserde(prefix = "tt", rename = "Location")]
    pub location: Description,

    // Informative description of the source.
    #[yaserde(prefix = "tt", rename = "Description")]
    pub description: Description,

    // URI provided by the service supplying data to be recorded. A device shall
    // support at least 128 characters.
    #[yaserde(prefix = "tt", rename = "Address")]
    pub address: String,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum RecordingStatus {
    Initiated,
    Recording,
    Stopped,
    Removing,
    Removed,
    // This case should never happen.
    Unknown,

    __Unknown__(String),
}

impl Default for RecordingStatus {
    fn default() -> RecordingStatus {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct TrackInformation {
    #[yaserde(prefix = "tt", rename = "TrackToken")]
    pub track_token: TrackReference,

    // Type of the track: "Video", "Audio" or "Metadata".
    // The track shall only be able to hold data of that type.
    #[yaserde(prefix = "tt", rename = "TrackType")]
    pub track_type: TrackType,

    // Informative description of the contents of the track.
    #[yaserde(prefix = "tt", rename = "Description")]
    pub description: Description,

    // The start date and time of the oldest recorded data in the track.
    #[yaserde(prefix = "tt", rename = "DataFrom")]
    pub data_from: String,

    // The stop date and time of the newest recorded data in the track.
    #[yaserde(prefix = "tt", rename = "DataTo")]
    pub data_to: String,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum TrackType {
    Video,
    Audio,
    Metadata,
    // Placeholder for future extension.
    Extended,

    __Unknown__(String),
}

impl Default for TrackType {
    fn default() -> TrackType {
        Self::__Unknown__("No valid variants".into())
    }
}

// A set of media attributes valid for a recording at a point in time or for a
// time interval.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MediaAttributes {
    // A reference to the recording that has these attributes.
    #[yaserde(prefix = "tt", rename = "RecordingToken")]
    pub recording_token: RecordingReference,

    // A set of attributes for each track.
    #[yaserde(prefix = "tt", rename = "TrackAttributes")]
    pub track_attributes: Vec<TrackAttributes>,

    // The attributes are valid from this point in time in the recording.
    #[yaserde(prefix = "tt", rename = "From")]
    pub from: String,

    // The attributes are valid until this point in time in the recording. Can be
    // equal to 'From' to indicate that the attributes are only known to be valid
    // for this particular point in time.
    #[yaserde(prefix = "tt", rename = "Until")]
    pub until: String,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct TrackAttributes {
    // The basic information about the track. Note that a track may represent a
    // single contiguous time span or consist of multiple slices.
    #[yaserde(prefix = "tt", rename = "TrackInformation")]
    pub track_information: TrackInformation,

    // If the track is a video track, exactly one of this structure shall be present
    // and contain the video attributes.
    #[yaserde(prefix = "tt", rename = "VideoAttributes")]
    pub video_attributes: Option<VideoAttributes>,

    // If the track is an audio track, exactly one of this structure shall be
    // present and contain the audio attributes.
    #[yaserde(prefix = "tt", rename = "AudioAttributes")]
    pub audio_attributes: Option<AudioAttributes>,

    // If the track is an metadata track, exactly one of this structure shall be
    // present and contain the metadata attributes.
    #[yaserde(prefix = "tt", rename = "MetadataAttributes")]
    pub metadata_attributes: Option<MetadataAttributes>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<TrackAttributesExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct TrackAttributesExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoAttributes {
    // Average bitrate in kbps.
    #[yaserde(prefix = "tt", rename = "Bitrate")]
    pub bitrate: Option<i32>,

    // The width of the video in pixels.
    #[yaserde(prefix = "tt", rename = "Width")]
    pub width: i32,

    // The height of the video in pixels.
    #[yaserde(prefix = "tt", rename = "Height")]
    pub height: i32,

    // Video encoding of the track. Use value from tt:VideoEncoding for MPEG4.
    // Otherwise use values from tt:VideoEncodingMimeNames and
    #[yaserde(prefix = "tt", rename = "Encoding")]
    pub encoding: String,

    // Average framerate in frames per second.
    #[yaserde(prefix = "tt", rename = "Framerate")]
    pub framerate: f64,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AudioAttributes {
    // The bitrate in kbps.
    #[yaserde(prefix = "tt", rename = "Bitrate")]
    pub bitrate: Option<i32>,

    // Audio encoding of the track. Use values from tt:AudioEncoding for G711 and
    // AAC. Otherwise use values from tt:AudioEncodingMimeNames and
    #[yaserde(prefix = "tt", rename = "Encoding")]
    pub encoding: String,

    // The sample rate in kHz.
    #[yaserde(prefix = "tt", rename = "Samplerate")]
    pub samplerate: i32,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MetadataAttributes {
    // Indicates that there can be PTZ data in the metadata track in the specified
    // time interval.
    #[yaserde(prefix = "tt", rename = "CanContainPTZ")]
    pub can_contain_ptz: bool,

    // Indicates that there can be analytics data in the metadata track in the
    // specified time interval.
    #[yaserde(prefix = "tt", rename = "CanContainAnalytics")]
    pub can_contain_analytics: bool,

    // Indicates that there can be notifications in the metadata track in the
    // specified time interval.
    #[yaserde(prefix = "tt", rename = "CanContainNotifications")]
    pub can_contain_notifications: bool,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // List of all PTZ spaces active for recording. Note that events are only
    // recorded on position changes and the actual point of recording may not
    // necessarily contain an event of the specified type.
    #[yaserde(attribute, rename = "PtzSpaces")]
    pub ptz_spaces: Option<StringAttrList>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct RecordingJobReference(pub ReferenceToken);

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RecordingConfiguration {
    // Information about the source of the recording.
    #[yaserde(prefix = "tt", rename = "Source")]
    pub source: RecordingSourceInformation,

    // Informative description of the source.
    #[yaserde(prefix = "tt", rename = "Content")]
    pub content: Description,

    // Sspecifies the maximum time that data in any track within the
    // recording shall be stored. The device shall delete any data older than the
    // maximum retention
    // time. Such data shall not be accessible anymore. If the
    // MaximumRetentionPeriod is set to 0,
    // the device shall not limit the retention time of stored data, except by
    // resource constraints.
    // Whatever the value of MaximumRetentionTime, the device may automatically
    // delete
    // recordings to free up storage space for new recordings.
    #[yaserde(prefix = "tt", rename = "MaximumRetentionTime")]
    pub maximum_retention_time: xs::Duration,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct TrackConfiguration {
    // Type of the track. It shall be equal to the strings “Video”,
    // “Audio” or “Metadata”. The track shall only be able to hold data of
    // that type.
    #[yaserde(prefix = "tt", rename = "TrackType")]
    pub track_type: TrackType,

    // Informative description of the track.
    #[yaserde(prefix = "tt", rename = "Description")]
    pub description: Description,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct GetRecordingsResponseItem {
    // Token of the recording.
    #[yaserde(prefix = "tt", rename = "RecordingToken")]
    pub recording_token: RecordingReference,

    // Configuration of the recording.
    #[yaserde(prefix = "tt", rename = "Configuration")]
    pub configuration: RecordingConfiguration,

    // List of tracks.
    #[yaserde(prefix = "tt", rename = "Tracks")]
    pub tracks: GetTracksResponseList,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct GetTracksResponseList {
    // Configuration of a track.
    #[yaserde(prefix = "tt", rename = "Track")]
    pub track: Vec<GetTracksResponseItem>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct GetTracksResponseItem {
    // Token of the track.
    #[yaserde(prefix = "tt", rename = "TrackToken")]
    pub track_token: TrackReference,

    // Configuration of the track.
    #[yaserde(prefix = "tt", rename = "Configuration")]
    pub configuration: TrackConfiguration,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RecordingJobConfiguration {
    // Identifies the recording to which this job shall store the received data.
    #[yaserde(prefix = "tt", rename = "RecordingToken")]
    pub recording_token: RecordingReference,

    // The mode of the job. If it is idle, nothing shall happen. If it is active,
    // the device shall try
    // to obtain data from the receivers. A client shall use GetRecordingJobState to
    // determine if data transfer is really taking place.
    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: RecordingJobMode,

    // This shall be a non-negative number. If there are multiple recording jobs
    // that store data to
    // the same track, the device will only store the data for the recording job
    // with the highest
    // priority. The priority is specified per recording job, but the device shall
    // determine the priority
    // of each track individually. If there are two recording jobs with the same
    // priority, the device
    // shall record the data corresponding to the recording job that was activated
    // the latest.
    #[yaserde(prefix = "tt", rename = "Priority")]
    pub priority: i32,

    // Source of the recording.
    #[yaserde(prefix = "tt", rename = "Source")]
    pub source: Vec<RecordingJobSource>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<RecordingJobConfigurationExtension>,

    // This attribute adds an additional requirement for activating the recording
    // job.
    // If this optional field is provided the job shall only record if the schedule
    // exists and is active.
    #[yaserde(attribute, rename = "ScheduleToken")]
    pub schedule_token: Option<String>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct RecordingJobMode(pub String);

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RecordingJobConfigurationExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RecordingJobSource {
    // This field shall be a reference to the source of the data. The type of the
    // source
    // is determined by the attribute Type in the SourceToken structure. If Type is
    // http://www.onvif.org/ver10/schema/Receiver, the token is a ReceiverReference.
    // In this case
    // the device shall receive the data over the network. If Type is
    // http://www.onvif.org/ver10/schema/Profile, the token identifies a media
    // profile, instructing the
    // device to obtain data from a profile that exists on the local device.
    #[yaserde(prefix = "tt", rename = "SourceToken")]
    pub source_token: Option<SourceReference>,

    // If this field is TRUE, and if the SourceToken is omitted, the device
    // shall create a receiver object (through the receiver service) and assign the
    // ReceiverReference to the SourceToken field. When retrieving the
    // RecordingJobConfiguration
    // from the device, the AutoCreateReceiver field shall never be present.
    #[yaserde(prefix = "tt", rename = "AutoCreateReceiver")]
    pub auto_create_receiver: Option<bool>,

    // List of tracks associated with the recording.
    #[yaserde(prefix = "tt", rename = "Tracks")]
    pub tracks: Vec<RecordingJobTrack>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<RecordingJobSourceExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RecordingJobSourceExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RecordingJobTrack {
    // If the received RTSP stream contains multiple tracks of the same type, the
    // SourceTag differentiates between those Tracks. This field can be ignored in
    // case of recording a local source.
    #[yaserde(prefix = "tt", rename = "SourceTag")]
    pub source_tag: String,

    // The destination is the tracktoken of the track to which the device shall
    // store the
    // received data.
    #[yaserde(prefix = "tt", rename = "Destination")]
    pub destination: TrackReference,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RecordingJobStateInformation {
    // Identification of the recording that the recording job records to.
    #[yaserde(prefix = "tt", rename = "RecordingToken")]
    pub recording_token: RecordingReference,

    // Holds the aggregated state over the whole RecordingJobInformation structure.
    #[yaserde(prefix = "tt", rename = "State")]
    pub state: RecordingJobState,

    // Identifies the data source of the recording job.
    #[yaserde(prefix = "tt", rename = "Sources")]
    pub sources: Vec<RecordingJobStateSource>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<RecordingJobStateInformationExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RecordingJobStateInformationExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct RecordingJobState(pub String);

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RecordingJobStateSource {
    // Identifies the data source of the recording job.
    #[yaserde(prefix = "tt", rename = "SourceToken")]
    pub source_token: SourceReference,

    // Holds the aggregated state over all substructures of RecordingJobStateSource.
    #[yaserde(prefix = "tt", rename = "State")]
    pub state: RecordingJobState,

    // List of track items.
    #[yaserde(prefix = "tt", rename = "Tracks")]
    pub tracks: RecordingJobStateTracks,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RecordingJobStateTracks {
    #[yaserde(prefix = "tt", rename = "Track")]
    pub track: Vec<RecordingJobStateTrack>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct RecordingJobStateTrack {
    // Identifies the track of the data source that provides the data.
    #[yaserde(prefix = "tt", rename = "SourceTag")]
    pub source_tag: String,

    // Indicates the destination track.
    #[yaserde(prefix = "tt", rename = "Destination")]
    pub destination: TrackReference,

    // Optionally holds an implementation defined string value that describes the
    // error.
    // The string should be in the English language.
    #[yaserde(prefix = "tt", rename = "Error")]
    pub error: Option<String>,

    // Provides the job state of the track. The valid
    // values of state shall be “Idle”, “Active” and “Error”. If state
    // equals “Error”, the Error field may be filled in with an implementation
    // defined value.
    #[yaserde(prefix = "tt", rename = "State")]
    pub state: RecordingJobState,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct GetRecordingJobsResponseItem {
    #[yaserde(prefix = "tt", rename = "JobToken")]
    pub job_token: RecordingJobReference,

    #[yaserde(prefix = "tt", rename = "JobConfiguration")]
    pub job_configuration: RecordingJobConfiguration,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

// Configuration parameters for the replay service.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ReplayConfiguration {
    // The RTSP session timeout.
    #[yaserde(prefix = "tt", rename = "SessionTimeout")]
    pub session_timeout: xs::Duration,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AnalyticsEngine {
    #[yaserde(prefix = "tt", rename = "AnalyticsEngineConfiguration")]
    pub analytics_engine_configuration: AnalyticsDeviceEngineConfiguration,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // User readable name. Length up to 64 characters.
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: Name,

    // Number of internal references currently using this configuration.
    #[yaserde(prefix = "tt", rename = "UseCount")]
    pub use_count: i32,

    // Token that uniquely references this configuration. Length up to 64
    // characters.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AnalyticsDeviceEngineConfiguration {
    #[yaserde(prefix = "tt", rename = "EngineConfiguration")]
    pub engine_configuration: Vec<EngineConfiguration>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<AnalyticsDeviceEngineConfigurationExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AnalyticsDeviceEngineConfigurationExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct EngineConfiguration {
    #[yaserde(prefix = "tt", rename = "VideoAnalyticsConfiguration")]
    pub video_analytics_configuration: VideoAnalyticsConfiguration,

    #[yaserde(prefix = "tt", rename = "AnalyticsEngineInputInfo")]
    pub analytics_engine_input_info: AnalyticsEngineInputInfo,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AnalyticsEngineInputInfo {
    #[yaserde(prefix = "tt", rename = "InputInfo")]
    pub input_info: Option<Config>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<AnalyticsEngineInputInfoExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AnalyticsEngineInputInfoExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AnalyticsEngineInput {
    #[yaserde(prefix = "tt", rename = "SourceIdentification")]
    pub source_identification: SourceIdentification,

    #[yaserde(prefix = "tt", rename = "VideoInput")]
    pub video_input: VideoEncoderConfiguration,

    #[yaserde(prefix = "tt", rename = "MetadataInput")]
    pub metadata_input: MetadataInput,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // User readable name. Length up to 64 characters.
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: Name,

    // Number of internal references currently using this configuration.
    #[yaserde(prefix = "tt", rename = "UseCount")]
    pub use_count: i32,

    // Token that uniquely references this configuration. Length up to 64
    // characters.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SourceIdentification {
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: String,

    #[yaserde(prefix = "tt", rename = "Token")]
    pub token: Vec<ReferenceToken>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<SourceIdentificationExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct SourceIdentificationExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MetadataInput {
    #[yaserde(prefix = "tt", rename = "MetadataConfig")]
    pub metadata_config: Vec<Config>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<MetadataInputExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MetadataInputExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AnalyticsEngineControl {
    // Token of the analytics engine (AnalyticsEngine) being controlled.
    #[yaserde(prefix = "tt", rename = "EngineToken")]
    pub engine_token: ReferenceToken,

    // Token of the analytics engine configuration (VideoAnalyticsConfiguration) in
    // effect.
    #[yaserde(prefix = "tt", rename = "EngineConfigToken")]
    pub engine_config_token: ReferenceToken,

    // Tokens of the input (AnalyticsEngineInput) configuration applied.
    #[yaserde(prefix = "tt", rename = "InputToken")]
    pub input_token: Vec<ReferenceToken>,

    // Tokens of the receiver providing media input data. The order of ReceiverToken
    // shall exactly match the order of InputToken.
    #[yaserde(prefix = "tt", rename = "ReceiverToken")]
    pub receiver_token: Vec<ReferenceToken>,

    #[yaserde(prefix = "tt", rename = "Multicast")]
    pub multicast: Option<MulticastConfiguration>,

    #[yaserde(prefix = "tt", rename = "Subscription")]
    pub subscription: Config,

    #[yaserde(prefix = "tt", rename = "Mode")]
    pub mode: ModeOfOperation,

    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // User readable name. Length up to 64 characters.
    #[yaserde(prefix = "tt", rename = "Name")]
    pub name: Name,

    // Number of internal references currently using this configuration.
    #[yaserde(prefix = "tt", rename = "UseCount")]
    pub use_count: i32,

    // Token that uniquely references this configuration. Length up to 64
    // characters.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum ModeOfOperation {
    Idle,
    Active,
    // This case should never happen.
    Unknown,

    __Unknown__(String),
}

impl Default for ModeOfOperation {
    fn default() -> ModeOfOperation {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AnalyticsStateInformation {
    // Token of the control object whose status is requested.
    #[yaserde(prefix = "tt", rename = "AnalyticsEngineControlToken")]
    pub analytics_engine_control_token: ReferenceToken,

    #[yaserde(prefix = "tt", rename = "State")]
    pub state: AnalyticsState,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AnalyticsState {
    #[yaserde(prefix = "tt", rename = "Error")]
    pub error: Option<String>,

    #[yaserde(prefix = "tt", rename = "State")]
    pub state: String,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

// Action Engine Event Payload data structure contains the information about the
// ONVIF command invocations. Since this event could be generated by other or
// proprietary actions, the command invocation specific fields are defined as
// optional and additional extension mechanism is provided for future or
// additional action definitions.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ActionEngineEventPayload {
    // Request Message
    #[yaserde(prefix = "tt", rename = "RequestInfo")]
    pub request_info: soapenv::Envelope,

    // Response Message
    #[yaserde(prefix = "tt", rename = "ResponseInfo")]
    pub response_info: soapenv::Envelope,

    // Fault Message
    #[yaserde(prefix = "tt", rename = "Fault")]
    pub fault: soapenv::Fault,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ActionEngineEventPayloadExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ActionEngineEventPayloadExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,

// //
//TODO: any_attribute macros
//  pub any_attribute: AnyAttribute,
}

// AudioClassType acceptable values are;
// gun_shot, scream, glass_breaking, tire_screech
#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]
pub struct AudioClassType(pub String);

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AudioClassCandidate {
    // Indicates audio class label
    #[yaserde(prefix = "tt", rename = "Type")]
    pub _type: AudioClassType,

    // A likelihood/probability that the corresponding audio event belongs to this
    // class. The sum of the likelihoods shall NOT exceed 1
    #[yaserde(prefix = "tt", rename = "Likelihood")]
    pub likelihood: f64,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AudioClassDescriptor {
    // Array of audio class label and class probability
    #[yaserde(prefix = "tt", rename = "ClassCandidate")]
    pub class_candidate: Vec<AudioClassCandidate>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<AudioClassDescriptorExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AudioClassDescriptorExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,

// //
//TODO: any_attribute macros
//  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ActiveConnection {
    #[yaserde(prefix = "tt", rename = "CurrentBitrate")]
    pub current_bitrate: f64,

    #[yaserde(prefix = "tt", rename = "CurrentFps")]
    pub current_fps: f64,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ProfileStatus {
    #[yaserde(prefix = "tt", rename = "ActiveConnections")]
    pub active_connections: Vec<ActiveConnection>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ProfileStatusExtension>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ProfileStatusExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,

// //
//TODO: any_attribute macros
//  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Osdreference {
    // //
//TODO: any_attribute macros
//  pub any_attribute: AnyAttribute,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum Osdtype {
    Text,
    Image,
    Extended,

    __Unknown__(String),
}

impl Default for Osdtype {
    fn default() -> Osdtype {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct OsdposConfiguration {
    // For OSD position type, following are the pre-defined:
    #[yaserde(prefix = "tt", rename = "Type")]
    pub _type: String,

    #[yaserde(prefix = "tt", rename = "Pos")]
    pub pos: Option<Vector>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<OsdposConfigurationExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct OsdposConfigurationExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,

// //
//TODO: any_attribute macros
//  pub any_attribute: AnyAttribute,
}

// The value range of "Transparent" could be defined by vendors only should
// follow this rule: the minimum value means non-transparent and the maximum
// value maens fully transparent.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Osdcolor {
    #[yaserde(prefix = "tt", rename = "Color")]
    pub color: Color,

    #[yaserde(attribute, rename = "Transparent")]
    pub transparent: Option<i32>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct OsdtextConfiguration {
    // The following OSD Text Type are defined:
    #[yaserde(prefix = "tt", rename = "Type")]
    pub _type: String,

    // List of supported OSD date formats. This element shall be present when the
    // value of Type field has Date or DateAndTime. The following DateFormat are
    // defined:
    #[yaserde(prefix = "tt", rename = "DateFormat")]
    pub date_format: Option<String>,

    // List of supported OSD time formats. This element shall be present when the
    // value of Type field has Time or DateAndTime. The following TimeFormat are
    // defined:
    #[yaserde(prefix = "tt", rename = "TimeFormat")]
    pub time_format: Option<String>,

    // Font size of the text in pt.
    #[yaserde(prefix = "tt", rename = "FontSize")]
    pub font_size: Option<i32>,

    // Font color of the text.
    #[yaserde(prefix = "tt", rename = "FontColor")]
    pub font_color: Option<Osdcolor>,

    // Background color of the text.
    #[yaserde(prefix = "tt", rename = "BackgroundColor")]
    pub background_color: Option<Osdcolor>,

    // The content of text to be displayed.
    #[yaserde(prefix = "tt", rename = "PlainText")]
    pub plain_text: Option<String>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<OsdtextConfigurationExtension>,

    // This flag is applicable for Type Plain and defaults to true. When set to
    // false the PlainText content will not be persistent across device reboots.
    #[yaserde(attribute, rename = "IsPersistentText")]
    pub is_persistent_text: Option<bool>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct OsdtextConfigurationExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,

// //
//TODO: any_attribute macros
//  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct OsdimgConfiguration {
    // The URI of the image which to be displayed.
    #[yaserde(prefix = "tt", rename = "ImgPath")]
    pub img_path: String,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<OsdimgConfigurationExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct OsdimgConfigurationExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,

// //
//TODO: any_attribute macros
//  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ColorspaceRange {
    #[yaserde(prefix = "tt", rename = "X")]
    pub x: FloatRange,

    #[yaserde(prefix = "tt", rename = "Y")]
    pub y: FloatRange,

    #[yaserde(prefix = "tt", rename = "Z")]
    pub z: FloatRange,

    // Acceptable values are the same as in tt:Color.
    #[yaserde(prefix = "tt", rename = "Colorspace")]
    pub colorspace: String,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum ColorOptionsChoice {
    // List the supported color.
    ColorList(Vec<Color>),
    // Define the range of color supported.
    ColorspaceRange(Vec<ColorspaceRange>),

    __Unknown__(String),
}

impl Default for ColorOptionsChoice {
    fn default() -> ColorOptionsChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

// Describe the colors supported. Either list each color or define the range of
// color values.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ColorOptions {
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
    #[yaserde(flatten)]
    pub color_options_choice: ColorOptionsChoice,
}

// Describe the option of the color and its transparency.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct OsdcolorOptions {
    // Optional list of supported colors.
    #[yaserde(prefix = "tt", rename = "Color")]
    pub color: Option<ColorOptions>,

    // Range of the transparent level. Larger means more tranparent.
    #[yaserde(prefix = "tt", rename = "Transparent")]
    pub transparent: Option<IntRange>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<OsdcolorOptionsExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct OsdcolorOptionsExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,

// //
//TODO: any_attribute macros
//  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct OsdtextOptions {
    // List of supported OSD text type. When a device indicates the supported number
    // relating to Text type in MaximumNumberOfOSDs, the type shall be presented.
    #[yaserde(prefix = "tt", rename = "Type")]
    pub _type: Vec<String>,

    // Range of the font size value.
    #[yaserde(prefix = "tt", rename = "FontSizeRange")]
    pub font_size_range: Option<IntRange>,

    // List of supported date format.
    #[yaserde(prefix = "tt", rename = "DateFormat")]
    pub date_format: Vec<String>,

    // List of supported time format.
    #[yaserde(prefix = "tt", rename = "TimeFormat")]
    pub time_format: Vec<String>,

    // List of supported font color.
    #[yaserde(prefix = "tt", rename = "FontColor")]
    pub font_color: Option<OsdcolorOptions>,

    // List of supported background color.
    #[yaserde(prefix = "tt", rename = "BackgroundColor")]
    pub background_color: Option<OsdcolorOptions>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<OsdtextOptionsExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct OsdtextOptionsExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,

// //
//TODO: any_attribute macros
//  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct OsdimgOptions {
    // List of available image URIs.
    #[yaserde(prefix = "tt", rename = "ImagePath")]
    pub image_path: Vec<String>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<OsdimgOptionsExtension>,

    // List of supported image MIME types, such as "image/png".
    #[yaserde(attribute, rename = "FormatsSupported")]
    pub formats_supported: Option<StringAttrList>,

    // The maximum size (in bytes) of the image that can be uploaded.
    #[yaserde(attribute, rename = "MaxSize")]
    pub max_size: Option<i32>,

    // The maximum width (in pixels) of the image that can be uploaded.
    #[yaserde(attribute, rename = "MaxWidth")]
    pub max_width: Option<i32>,

    // The maximum height (in pixels) of the image that can be uploaded.
    #[yaserde(attribute, rename = "MaxHeight")]
    pub max_height: Option<i32>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct OsdimgOptionsExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,

// //
//TODO: any_attribute macros
//  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Osdconfiguration {
    // Reference to the video source configuration.
    #[yaserde(prefix = "tt", rename = "VideoSourceConfigurationToken")]
    pub video_source_configuration_token: Osdreference,

    // Type of OSD.
    #[yaserde(prefix = "tt", rename = "Type")]
    pub _type: Osdtype,

    // Position configuration of OSD.
    #[yaserde(prefix = "tt", rename = "Position")]
    pub position: OsdposConfiguration,

    // Text configuration of OSD. It shall be present when the value of Type field
    // is Text.
    #[yaserde(prefix = "tt", rename = "TextString")]
    pub text_string: Option<OsdtextConfiguration>,

    // Image configuration of OSD. It shall be present when the value of Type field
    // is Image
    #[yaserde(prefix = "tt", rename = "Image")]
    pub image: Option<OsdimgConfiguration>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<OsdconfigurationExtension>,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,

    // Unique identifier referencing the physical entity.
    #[yaserde(attribute, rename = "token")]
    pub token: ReferenceToken,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct OsdconfigurationExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,

// //
//TODO: any_attribute macros
//  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MaximumNumberOfOSDs {
    #[yaserde(attribute, rename = "Total")]
    pub total: i32,

    #[yaserde(attribute, rename = "Image")]
    pub image: Option<i32>,

    #[yaserde(attribute, rename = "PlainText")]
    pub plain_text: Option<i32>,

    #[yaserde(attribute, rename = "Date")]
    pub date: Option<i32>,

    #[yaserde(attribute, rename = "Time")]
    pub time: Option<i32>,

    #[yaserde(attribute, rename = "DateAndTime")]
    pub date_and_time: Option<i32>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct OsdconfigurationOptions {
    // The maximum number of OSD configurations supported for the specified video
    // source configuration. If the configuration does not support OSDs, this value
    // shall be zero and the Type and PositionOption elements are ignored. If a
    // device limits the number of instances by OSDType, it shall indicate the
    // supported number for each type via the related attribute.
    #[yaserde(prefix = "tt", rename = "MaximumNumberOfOSDs")]
    pub maximum_number_of_os_ds: MaximumNumberOfOSDs,

    // List supported type of OSD configuration. When a device indicates the
    // supported number for each types in MaximumNumberOfOSDs, related type shall be
    // presented. A device shall return Option element relating to listed type.
    #[yaserde(prefix = "tt", rename = "Type")]
    pub _type: Vec<Osdtype>,

    // List available OSD position type. Following are the pre-defined:
    #[yaserde(prefix = "tt", rename = "PositionOption")]
    pub position_option: Vec<String>,

    // Option of the OSD text configuration. This element shall be returned if the
    // device is signaling the support for Text.
    #[yaserde(prefix = "tt", rename = "TextOption")]
    pub text_option: Option<OsdtextOptions>,

    // Option of the OSD image configuration. This element shall be returned if the
    // device is signaling the support for Image.
    #[yaserde(prefix = "tt", rename = "ImageOption")]
    pub image_option: Option<OsdimgOptions>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<OsdconfigurationOptionsExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct OsdconfigurationOptionsExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,

// //
//TODO: any_attribute macros
//  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct FileProgress {
    // Exported file name
    #[yaserde(prefix = "tt", rename = "FileName")]
    pub file_name: String,

    // Normalized percentage completion for uploading the exported file
    #[yaserde(prefix = "tt", rename = "Progress")]
    pub progress: f64,
    //TODO: yaserde macro for any element
    //  pub any: AnyElement,

    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ArrayOfFileProgress {
    // Exported file name and export progress information
    #[yaserde(prefix = "tt", rename = "FileProgress")]
    pub file_progress: Vec<FileProgress>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ArrayOfFileProgressExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ArrayOfFileProgressExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,

// //
//TODO: any_attribute macros
//  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct StorageReferencePath {
    // identifier of an existing Storage Configuration.
    #[yaserde(prefix = "tt", rename = "StorageToken")]
    pub storage_token: ReferenceToken,

    // gives the relative directory path on the storage
    #[yaserde(prefix = "tt", rename = "RelativePath")]
    pub relative_path: Option<String>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<StorageReferencePathExtension>,
    // //
    //TODO: any_attribute macros
    //  pub any_attribute: AnyAttribute,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct StorageReferencePathExtension {
    //TODO: yaserde macro for any element
//  pub any: AnyElement,

// //
//TODO: any_attribute macros
//  pub any_attribute: AnyAttribute,
}
