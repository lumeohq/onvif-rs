use crate::onvif as tt;
use crate::transport;
use crate::validate::Validate;
use std::str::FromStr;
use xsd_macro_utils::*;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetServiceCapabilitiesResponse {
    // The capabilities for the media service is returned in the Capabilities
    // element.
    #[yaserde(prefix = "trt", rename = "Capabilities")]
    pub capabilities: Capabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct Capabilities {
    // Media profile capabilities.
    #[yaserde(prefix = "trt", rename = "ProfileCapabilities")]
    pub profile_capabilities: Vec<ProfileCapabilities>,

    // Streaming capabilities.
    #[yaserde(prefix = "trt", rename = "StreamingCapabilities")]
    pub streaming_capabilities: Vec<StreamingCapabilities>,

    // Indicates if GetSnapshotUri is supported.
    #[yaserde(attribute, rename = "SnapshotUri")]
    pub snapshot_uri: Option<bool>,

    // Indicates whether or not Rotation feature is supported.
    #[yaserde(attribute, rename = "Rotation")]
    pub rotation: Option<bool>,

    // Indicates the support for changing video source mode.
    #[yaserde(attribute, rename = "VideoSourceMode")]
    pub video_source_mode: Option<bool>,

    // Indicates if OSD is supported.
    #[yaserde(attribute, rename = "OSD")]
    pub osd: Option<bool>,

    // Indicates the support for temporary osd text configuration.
    #[yaserde(attribute, rename = "TemporaryOSDText")]
    pub temporary_osd_text: Option<bool>,

    // Indicates the support for the Efficient XML Interchange (EXI) binary XML
    // format.
    #[yaserde(attribute, rename = "EXICompression")]
    pub exi_compression: Option<bool>,
}

impl Validate for Capabilities {}

// pub type Capabilities = Capabilities;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct ProfileCapabilities {
    // Maximum number of profiles supported.
    #[yaserde(attribute, rename = "MaximumNumberOfProfiles")]
    pub maximum_number_of_profiles: Option<i32>,
}

impl Validate for ProfileCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct StreamingCapabilities {
    // Indicates support for RTP multicast.
    #[yaserde(attribute, rename = "RTPMulticast")]
    pub rtp_multicast: Option<bool>,

    // Indicates support for RTP over TCP.
    #[yaserde(attribute, rename = "RTP_TCP")]
    pub rtp_tcp: Option<bool>,

    // Indicates support for RTP/RTSP/TCP.
    #[yaserde(attribute, rename = "RTP_RTSP_TCP")]
    pub rtp_rtsp_tcp: Option<bool>,

    // Indicates support for non aggregate RTSP control.
    #[yaserde(attribute, rename = "NonAggregateControl")]
    pub non_aggregate_control: Option<bool>,

    // Indicates the device does not support live media streaming via RTSP.
    #[yaserde(attribute, rename = "NoRTSPStreaming")]
    pub no_rtsp_streaming: Option<bool>,
}

impl Validate for StreamingCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetVideoSources {}

impl Validate for GetVideoSources {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetVideoSourcesResponse {
    // List of existing Video Sources
    #[yaserde(prefix = "trt", rename = "VideoSources")]
    pub video_sources: Vec<tt::VideoSource>,
}

impl Validate for GetVideoSourcesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioSources {}

impl Validate for GetAudioSources {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioSourcesResponse {
    // List of existing Audio Sources
    #[yaserde(prefix = "trt", rename = "AudioSources")]
    pub audio_sources: Vec<tt::AudioSource>,
}

impl Validate for GetAudioSourcesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioOutputs {}

impl Validate for GetAudioOutputs {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioOutputsResponse {
    // List of existing Audio Outputs
    #[yaserde(prefix = "trt", rename = "AudioOutputs")]
    pub audio_outputs: Vec<tt::AudioOutput>,
}

impl Validate for GetAudioOutputsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct CreateProfile {
    // friendly name of the profile to be created
    #[yaserde(prefix = "trt", rename = "Name")]
    pub name: tt::Name,

    // Optional token, specifying the unique identifier of the new profile.
    #[yaserde(prefix = "trt", rename = "Token")]
    pub token: Option<tt::ReferenceToken>,
}

impl Validate for CreateProfile {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct CreateProfileResponse {
    // returns the new created profile
    #[yaserde(prefix = "trt", rename = "Profile")]
    pub profile: tt::Profile,
}

impl Validate for CreateProfileResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetProfile {
    // this command requests a specific profile
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetProfile {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetProfileResponse {
    // returns the requested media profile
    #[yaserde(prefix = "trt", rename = "Profile")]
    pub profile: tt::Profile,
}

impl Validate for GetProfileResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetProfiles {}

impl Validate for GetProfiles {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetProfilesResponse {
    // lists all profiles that exist in the media service
    #[yaserde(prefix = "trt", rename = "Profiles")]
    pub profiles: Vec<tt::Profile>,
}

impl Validate for GetProfilesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct AddVideoEncoderConfiguration {
    // Reference to the profile where the configuration should be added
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // Contains a reference to the VideoEncoderConfiguration to add
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for AddVideoEncoderConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct AddVideoEncoderConfigurationResponse {}

impl Validate for AddVideoEncoderConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct RemoveVideoEncoderConfiguration {
    // Contains a reference to the media profile from which the
    // VideoEncoderConfiguration shall be removed.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for RemoveVideoEncoderConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct RemoveVideoEncoderConfigurationResponse {}

impl Validate for RemoveVideoEncoderConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct AddVideoSourceConfiguration {
    // Reference to the profile where the configuration should be added
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // Contains a reference to the VideoSourceConfiguration to add
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for AddVideoSourceConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct AddVideoSourceConfigurationResponse {}

impl Validate for AddVideoSourceConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct RemoveVideoSourceConfiguration {
    // Contains a reference to the media profile from which the
    // VideoSourceConfiguration shall be removed.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for RemoveVideoSourceConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct RemoveVideoSourceConfigurationResponse {}

impl Validate for RemoveVideoSourceConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct AddAudioEncoderConfiguration {
    // Reference to the profile where the configuration should be added
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // Contains a reference to the AudioEncoderConfiguration to add
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for AddAudioEncoderConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct AddAudioEncoderConfigurationResponse {}

impl Validate for AddAudioEncoderConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct RemoveAudioEncoderConfiguration {
    // Contains a reference to the media profile from which the
    // AudioEncoderConfiguration shall be removed.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for RemoveAudioEncoderConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct RemoveAudioEncoderConfigurationResponse {}

impl Validate for RemoveAudioEncoderConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct AddAudioSourceConfiguration {
    // Reference to the profile where the configuration should be added
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // Contains a reference to the AudioSourceConfiguration to add
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for AddAudioSourceConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct AddAudioSourceConfigurationResponse {}

impl Validate for AddAudioSourceConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct RemoveAudioSourceConfiguration {
    // Contains a reference to the media profile from which the
    // AudioSourceConfiguration shall be removed.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for RemoveAudioSourceConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct RemoveAudioSourceConfigurationResponse {}

impl Validate for RemoveAudioSourceConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct AddPTZConfiguration {
    // Reference to the profile where the configuration should be added
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // Contains a reference to the PTZConfiguration to add
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for AddPTZConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct AddPTZConfigurationResponse {}

impl Validate for AddPTZConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct RemovePTZConfiguration {
    // Contains a reference to the media profile from which the
    // PTZConfiguration shall be removed.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for RemovePTZConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct RemovePTZConfigurationResponse {}

impl Validate for RemovePTZConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct AddVideoAnalyticsConfiguration {
    // Reference to the profile where the configuration should be added
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // Contains a reference to the VideoAnalyticsConfiguration to add
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for AddVideoAnalyticsConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct AddVideoAnalyticsConfigurationResponse {}

impl Validate for AddVideoAnalyticsConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct RemoveVideoAnalyticsConfiguration {
    // Contains a reference to the media profile from which the
    // VideoAnalyticsConfiguration shall be removed.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for RemoveVideoAnalyticsConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct RemoveVideoAnalyticsConfigurationResponse {}

impl Validate for RemoveVideoAnalyticsConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct AddMetadataConfiguration {
    // Reference to the profile where the configuration should be added
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // Contains a reference to the MetadataConfiguration to add
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for AddMetadataConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct AddMetadataConfigurationResponse {}

impl Validate for AddMetadataConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct RemoveMetadataConfiguration {
    // Contains a reference to the media profile from which the
    // MetadataConfiguration shall be removed.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for RemoveMetadataConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct RemoveMetadataConfigurationResponse {}

impl Validate for RemoveMetadataConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct AddAudioOutputConfiguration {
    // Reference to the profile where the configuration should be added
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // Contains a reference to the AudioOutputConfiguration to add
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for AddAudioOutputConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct AddAudioOutputConfigurationResponse {}

impl Validate for AddAudioOutputConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct RemoveAudioOutputConfiguration {
    // Contains a reference to the media profile from which the
    // AudioOutputConfiguration shall be removed.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for RemoveAudioOutputConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct RemoveAudioOutputConfigurationResponse {}

impl Validate for RemoveAudioOutputConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct AddAudioDecoderConfiguration {
    // This element contains a reference to the profile where the configuration
    // should be added.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // This element contains a reference to the AudioDecoderConfiguration to
    // add.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for AddAudioDecoderConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct AddAudioDecoderConfigurationResponse {}

impl Validate for AddAudioDecoderConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct RemoveAudioDecoderConfiguration {
    // This element contains a reference to the media profile from which the
    // AudioDecoderConfiguration shall be removed.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for RemoveAudioDecoderConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct RemoveAudioDecoderConfigurationResponse {}

impl Validate for RemoveAudioDecoderConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct DeleteProfile {
    // This element contains a reference to the profile that should be deleted.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for DeleteProfile {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct DeleteProfileResponse {}

impl Validate for DeleteProfileResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetVideoEncoderConfigurations {}

impl Validate for GetVideoEncoderConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetVideoEncoderConfigurationsResponse {
    // This element contains a list of video encoder configurations.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::VideoEncoderConfiguration>,
}

impl Validate for GetVideoEncoderConfigurationsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetVideoSourceConfigurations {}

impl Validate for GetVideoSourceConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetVideoSourceConfigurationsResponse {
    // This element contains a list of video source configurations.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::VideoSourceConfiguration>,
}

impl Validate for GetVideoSourceConfigurationsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioEncoderConfigurations {}

impl Validate for GetAudioEncoderConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioEncoderConfigurationsResponse {
    // This element contains a list of audio encoder configurations.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::AudioEncoderConfiguration>,
}

impl Validate for GetAudioEncoderConfigurationsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioSourceConfigurations {}

impl Validate for GetAudioSourceConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioSourceConfigurationsResponse {
    // This element contains a list of audio source configurations.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::AudioSourceConfiguration>,
}

impl Validate for GetAudioSourceConfigurationsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetVideoAnalyticsConfigurations {}

impl Validate for GetVideoAnalyticsConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetVideoAnalyticsConfigurationsResponse {
    // This element contains a list of VideoAnalytics configurations.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::VideoAnalyticsConfiguration>,
}

impl Validate for GetVideoAnalyticsConfigurationsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetMetadataConfigurations {}

impl Validate for GetMetadataConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetMetadataConfigurationsResponse {
    // This element contains a list of metadata configurations
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::MetadataConfiguration>,
}

impl Validate for GetMetadataConfigurationsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioOutputConfigurations {}

impl Validate for GetAudioOutputConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioOutputConfigurationsResponse {
    // This element contains a list of audio output configurations
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::AudioOutputConfiguration>,
}

impl Validate for GetAudioOutputConfigurationsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioDecoderConfigurations {}

impl Validate for GetAudioDecoderConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioDecoderConfigurationsResponse {
    // This element contains a list of audio decoder configurations
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::AudioDecoderConfiguration>,
}

impl Validate for GetAudioDecoderConfigurationsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetVideoSourceConfiguration {
    // Token of the requested video source configuration.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetVideoSourceConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetVideoSourceConfigurationResponse {
    // The requested video source configuration.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::VideoSourceConfiguration,
}

impl Validate for GetVideoSourceConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetVideoEncoderConfiguration {
    // Token of the requested video encoder configuration.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetVideoEncoderConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetVideoEncoderConfigurationResponse {
    // The requested video encoder configuration.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::VideoEncoderConfiguration,
}

impl Validate for GetVideoEncoderConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioSourceConfiguration {
    // Token of the requested audio source configuration.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetAudioSourceConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioSourceConfigurationResponse {
    // The requested audio source configuration.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::AudioSourceConfiguration,
}

impl Validate for GetAudioSourceConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioEncoderConfiguration {
    // Token of the requested audio encoder configuration.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetAudioEncoderConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioEncoderConfigurationResponse {
    // The requested audio encoder configuration
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::AudioEncoderConfiguration,
}

impl Validate for GetAudioEncoderConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetVideoAnalyticsConfiguration {
    // Token of the requested video analytics configuration.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetVideoAnalyticsConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetVideoAnalyticsConfigurationResponse {
    // The requested video analytics configuration.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::VideoAnalyticsConfiguration,
}

impl Validate for GetVideoAnalyticsConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetMetadataConfiguration {
    // Token of the requested metadata configuration.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetMetadataConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetMetadataConfigurationResponse {
    // The requested metadata configuration.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::MetadataConfiguration,
}

impl Validate for GetMetadataConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioOutputConfiguration {
    // Token of the requested audio output configuration.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetAudioOutputConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioOutputConfigurationResponse {
    // The requested audio output configuration.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::AudioOutputConfiguration,
}

impl Validate for GetAudioOutputConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioDecoderConfiguration {
    // Token of the requested audio decoder configuration.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetAudioDecoderConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioDecoderConfigurationResponse {
    // The requested audio decoder configuration
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::AudioDecoderConfiguration,
}

impl Validate for GetAudioDecoderConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetCompatibleVideoEncoderConfigurations {
    // Contains the token of an existing media profile the configurations shall
    // be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetCompatibleVideoEncoderConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetCompatibleVideoEncoderConfigurationsResponse {
    // Contains a list of video encoder configurations that are compatible with
    // the specified media profile.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::VideoEncoderConfiguration>,
}

impl Validate for GetCompatibleVideoEncoderConfigurationsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetCompatibleVideoSourceConfigurations {
    // Contains the token of an existing media profile the configurations shall
    // be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetCompatibleVideoSourceConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetCompatibleVideoSourceConfigurationsResponse {
    // Contains a list of video source configurations that are compatible with
    // the specified media profile.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::VideoSourceConfiguration>,
}

impl Validate for GetCompatibleVideoSourceConfigurationsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetCompatibleAudioEncoderConfigurations {
    // Contains the token of an existing media profile the configurations shall
    // be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetCompatibleAudioEncoderConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetCompatibleAudioEncoderConfigurationsResponse {
    // Contains a list of audio encoder configurations that are compatible with
    // the specified media profile.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::AudioEncoderConfiguration>,
}

impl Validate for GetCompatibleAudioEncoderConfigurationsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetCompatibleAudioSourceConfigurations {
    // Contains the token of an existing media profile the configurations shall
    // be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetCompatibleAudioSourceConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetCompatibleAudioSourceConfigurationsResponse {
    // Contains a list of audio source configurations that are compatible with
    // the specified media profile.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::AudioSourceConfiguration>,
}

impl Validate for GetCompatibleAudioSourceConfigurationsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetCompatibleVideoAnalyticsConfigurations {
    // Contains the token of an existing media profile the configurations shall
    // be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetCompatibleVideoAnalyticsConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetCompatibleVideoAnalyticsConfigurationsResponse {
    // Contains a list of video analytics configurations that are compatible
    // with the specified media profile.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::VideoAnalyticsConfiguration>,
}

impl Validate for GetCompatibleVideoAnalyticsConfigurationsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetCompatibleMetadataConfigurations {
    // Contains the token of an existing media profile the configurations shall
    // be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetCompatibleMetadataConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetCompatibleMetadataConfigurationsResponse {
    // Contains a list of metadata configurations that are compatible with the
    // specified media profile.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::MetadataConfiguration>,
}

impl Validate for GetCompatibleMetadataConfigurationsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetCompatibleAudioOutputConfigurations {
    // Contains the token of an existing media profile the configurations shall
    // be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetCompatibleAudioOutputConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetCompatibleAudioOutputConfigurationsResponse {
    // Contains a list of audio output configurations that are compatible with
    // the specified media profile.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::AudioOutputConfiguration>,
}

impl Validate for GetCompatibleAudioOutputConfigurationsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetCompatibleAudioDecoderConfigurations {
    // Contains the token of an existing media profile the configurations shall
    // be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetCompatibleAudioDecoderConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetCompatibleAudioDecoderConfigurationsResponse {
    // Contains a list of audio decoder configurations that are compatible with
    // the specified media profile.
    #[yaserde(prefix = "trt", rename = "Configurations")]
    pub configurations: Vec<tt::AudioDecoderConfiguration>,
}

impl Validate for GetCompatibleAudioDecoderConfigurationsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct SetVideoEncoderConfiguration {
    // Contains the modified video encoder configuration. The configuration
    // shall exist in the device.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::VideoEncoderConfiguration,

    // The ForcePersistence element is obsolete and should always be assumed to
    // be true.
    #[yaserde(prefix = "trt", rename = "ForcePersistence")]
    pub force_persistence: bool,
}

impl Validate for SetVideoEncoderConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct SetVideoEncoderConfigurationResponse {}

impl Validate for SetVideoEncoderConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct SetVideoSourceConfiguration {
    // Contains the modified video source configuration. The configuration shall
    // exist in the device.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::VideoSourceConfiguration,

    // The ForcePersistence element is obsolete and should always be assumed to
    // be true.
    #[yaserde(prefix = "trt", rename = "ForcePersistence")]
    pub force_persistence: bool,
}

impl Validate for SetVideoSourceConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct SetVideoSourceConfigurationResponse {}

impl Validate for SetVideoSourceConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct SetAudioEncoderConfiguration {
    // Contains the modified audio encoder configuration. The configuration
    // shall exist in the device.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::AudioEncoderConfiguration,

    // The ForcePersistence element is obsolete and should always be assumed to
    // be true.
    #[yaserde(prefix = "trt", rename = "ForcePersistence")]
    pub force_persistence: bool,
}

impl Validate for SetAudioEncoderConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct SetAudioEncoderConfigurationResponse {}

impl Validate for SetAudioEncoderConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct SetAudioSourceConfiguration {
    // Contains the modified audio source configuration. The configuration shall
    // exist in the device.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::AudioSourceConfiguration,

    // The ForcePersistence element is obsolete and should always be assumed to
    // be true.
    #[yaserde(prefix = "trt", rename = "ForcePersistence")]
    pub force_persistence: bool,
}

impl Validate for SetAudioSourceConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct SetAudioSourceConfigurationResponse {}

impl Validate for SetAudioSourceConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct SetVideoAnalyticsConfiguration {
    // Contains the modified video analytics configuration. The configuration
    // shall exist in the device.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::VideoAnalyticsConfiguration,

    // The ForcePersistence element is obsolete and should always be assumed to
    // be true.
    #[yaserde(prefix = "trt", rename = "ForcePersistence")]
    pub force_persistence: bool,
}

impl Validate for SetVideoAnalyticsConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct SetVideoAnalyticsConfigurationResponse {}

impl Validate for SetVideoAnalyticsConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct SetMetadataConfiguration {
    // Contains the modified metadata configuration. The configuration shall
    // exist in the device.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::MetadataConfiguration,

    // The ForcePersistence element is obsolete and should always be assumed to
    // be true.
    #[yaserde(prefix = "trt", rename = "ForcePersistence")]
    pub force_persistence: bool,
}

impl Validate for SetMetadataConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct SetMetadataConfigurationResponse {}

impl Validate for SetMetadataConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct SetAudioOutputConfiguration {
    // Contains the modified audio output configuration. The configuration shall
    // exist in the device.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::AudioOutputConfiguration,

    // The ForcePersistence element is obsolete and should always be assumed to
    // be true.
    #[yaserde(prefix = "trt", rename = "ForcePersistence")]
    pub force_persistence: bool,
}

impl Validate for SetAudioOutputConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct SetAudioOutputConfigurationResponse {}

impl Validate for SetAudioOutputConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct SetAudioDecoderConfiguration {
    // Contains the modified audio decoder configuration. The configuration
    // shall exist in the device.
    #[yaserde(prefix = "trt", rename = "Configuration")]
    pub configuration: tt::AudioDecoderConfiguration,

    // The ForcePersistence element is obsolete and should always be assumed to
    // be true.
    #[yaserde(prefix = "trt", rename = "ForcePersistence")]
    pub force_persistence: bool,
}

impl Validate for SetAudioDecoderConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct SetAudioDecoderConfigurationResponse {}

impl Validate for SetAudioDecoderConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetVideoSourceConfigurationOptions {
    // Optional video source configurationToken that specifies an existing
    // configuration that the options are intended for.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: Option<tt::ReferenceToken>,

    // Optional ProfileToken that specifies an existing media profile that the
    // options shall be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: Option<tt::ReferenceToken>,
}

impl Validate for GetVideoSourceConfigurationOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetVideoSourceConfigurationOptionsResponse {
    // This message contains the video source configuration options. If a video
    // source configuration is specified, the options shall concern that
    // particular configuration. If a media profile is specified, the options
    // shall be compatible with that media profile. If no tokens are specified,
    // the options shall be considered generic for the device.
    #[yaserde(prefix = "trt", rename = "Options")]
    pub options: tt::VideoSourceConfigurationOptions,
}

impl Validate for GetVideoSourceConfigurationOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetVideoEncoderConfigurationOptions {
    // Optional video encoder configuration token that specifies an existing
    // configuration that the options are intended for.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: Option<tt::ReferenceToken>,

    // Optional ProfileToken that specifies an existing media profile that the
    // options shall be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: Option<tt::ReferenceToken>,
}

impl Validate for GetVideoEncoderConfigurationOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetVideoEncoderConfigurationOptionsResponse {
    #[yaserde(prefix = "trt", rename = "Options")]
    pub options: tt::VideoEncoderConfigurationOptions,
}

impl Validate for GetVideoEncoderConfigurationOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioSourceConfigurationOptions {
    // Optional audio source configuration token that specifies an existing
    // configuration that the options are intended for.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: Option<tt::ReferenceToken>,

    // Optional ProfileToken that specifies an existing media profile that the
    // options shall be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: Option<tt::ReferenceToken>,
}

impl Validate for GetAudioSourceConfigurationOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioSourceConfigurationOptionsResponse {
    // This message contains the audio source configuration options. If a audio
    // source configuration is specified, the options shall concern that
    // particular configuration. If a media profile is specified, the options
    // shall be compatible with that media profile. If no tokens are specified,
    // the options shall be considered generic for the device.
    #[yaserde(prefix = "trt", rename = "Options")]
    pub options: tt::AudioSourceConfigurationOptions,
}

impl Validate for GetAudioSourceConfigurationOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioEncoderConfigurationOptions {
    // Optional audio encoder configuration token that specifies an existing
    // configuration that the options are intended for.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: Option<tt::ReferenceToken>,

    // Optional ProfileToken that specifies an existing media profile that the
    // options shall be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: Option<tt::ReferenceToken>,
}

impl Validate for GetAudioEncoderConfigurationOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioEncoderConfigurationOptionsResponse {
    // This message contains the audio encoder configuration options. If a audio
    // encoder configuration is specified, the options shall concern that
    // particular configuration. If a media profile is specified, the options
    // shall be compatible with that media profile. If no tokens are specified,
    // the options shall be considered generic for the device.
    #[yaserde(prefix = "trt", rename = "Options")]
    pub options: tt::AudioEncoderConfigurationOptions,
}

impl Validate for GetAudioEncoderConfigurationOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetMetadataConfigurationOptions {
    // Optional metadata configuration token that specifies an existing
    // configuration that the options are intended for.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: Option<tt::ReferenceToken>,

    // Optional ProfileToken that specifies an existing media profile that the
    // options shall be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: Option<tt::ReferenceToken>,
}

impl Validate for GetMetadataConfigurationOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetMetadataConfigurationOptionsResponse {
    // This message contains the metadata configuration options. If a metadata
    // configuration is specified, the options shall concern that particular
    // configuration. If a media profile is specified, the options shall be
    // compatible with that media profile. If no tokens are specified, the
    // options shall be considered generic for the device.
    #[yaserde(prefix = "trt", rename = "Options")]
    pub options: tt::MetadataConfigurationOptions,
}

impl Validate for GetMetadataConfigurationOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioOutputConfigurationOptions {
    // Optional audio output configuration token that specifies an existing
    // configuration that the options are intended for.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: Option<tt::ReferenceToken>,

    // Optional ProfileToken that specifies an existing media profile that the
    // options shall be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: Option<tt::ReferenceToken>,
}

impl Validate for GetAudioOutputConfigurationOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioOutputConfigurationOptionsResponse {
    // This message contains the audio output configuration options. If a audio
    // output configuration is specified, the options shall concern that
    // particular configuration. If a media profile is specified, the options
    // shall be compatible with that media profile. If no tokens are specified,
    // the options shall be considered generic for the device.
    #[yaserde(prefix = "trt", rename = "Options")]
    pub options: tt::AudioOutputConfigurationOptions,
}

impl Validate for GetAudioOutputConfigurationOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioDecoderConfigurationOptions {
    // Optional audio decoder configuration token that specifies an existing
    // configuration that the options are intended for.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: Option<tt::ReferenceToken>,

    // Optional ProfileToken that specifies an existing media profile that the
    // options shall be compatible with.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: Option<tt::ReferenceToken>,
}

impl Validate for GetAudioDecoderConfigurationOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetAudioDecoderConfigurationOptionsResponse {
    // This message contains the audio decoder configuration options. If a audio
    // decoder configuration is specified, the options shall concern that
    // particular configuration. If a media profile is specified, the options
    // shall be compatible with that media profile. If no tokens are specified,
    // the options shall be considered generic for the device.
    #[yaserde(prefix = "trt", rename = "Options")]
    pub options: tt::AudioDecoderConfigurationOptions,
}

impl Validate for GetAudioDecoderConfigurationOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetGuaranteedNumberOfVideoEncoderInstances {
    // Token of the video source configuration
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetGuaranteedNumberOfVideoEncoderInstances {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetGuaranteedNumberOfVideoEncoderInstancesResponse {
    // The minimum guaranteed total number of encoder instances (applications)
    // per VideoSourceConfiguration. The device is able to deliver the
    // TotalNumber of streams
    #[yaserde(prefix = "trt", rename = "TotalNumber")]
    pub total_number: i32,

    // If a device limits the number of instances for respective Video Codecs
    // the response contains the information how many Jpeg streams can be set up
    // at the same time per VideoSource.
    #[yaserde(prefix = "trt", rename = "JPEG")]
    pub jpeg: Option<i32>,

    // If a device limits the number of instances for respective Video Codecs
    // the response contains the information how many H264 streams can be set up
    // at the same time per VideoSource.
    #[yaserde(prefix = "trt", rename = "H264")]
    pub h264: Option<i32>,

    // If a device limits the number of instances for respective Video Codecs
    // the response contains the information how many Mpeg4 streams can be set
    // up at the same time per VideoSource.
    #[yaserde(prefix = "trt", rename = "MPEG4")]
    pub mpeg4: Option<i32>,
}

impl Validate for GetGuaranteedNumberOfVideoEncoderInstancesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetStreamUri {
    // Stream Setup that should be used with the uri
    #[yaserde(prefix = "trt", rename = "StreamSetup")]
    pub stream_setup: tt::StreamSetup,

    // The ProfileToken element indicates the media profile to use and will
    // define the configuration of the content of the stream.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetStreamUri {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetStreamUriResponse {
    #[yaserde(prefix = "trt", rename = "MediaUri")]
    pub media_uri: tt::MediaUri,
}

impl Validate for GetStreamUriResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct StartMulticastStreaming {
    // Contains the token of the Profile that is used to define the multicast
    // stream.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for StartMulticastStreaming {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct StartMulticastStreamingResponse {}

impl Validate for StartMulticastStreamingResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct StopMulticastStreaming {
    // Contains the token of the Profile that is used to define the multicast
    // stream.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for StopMulticastStreaming {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct StopMulticastStreamingResponse {}

impl Validate for StopMulticastStreamingResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct SetSynchronizationPoint {
    // Contains a Profile reference for which a Synchronization Point is
    // requested.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for SetSynchronizationPoint {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct SetSynchronizationPointResponse {}

impl Validate for SetSynchronizationPointResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetSnapshotUri {
    // The ProfileToken element indicates the media profile to use and will
    // define the source and dimensions of the snapshot.
    #[yaserde(prefix = "trt", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetSnapshotUri {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetSnapshotUriResponse {
    #[yaserde(prefix = "trt", rename = "MediaUri")]
    pub media_uri: tt::MediaUri,
}

impl Validate for GetSnapshotUriResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetVideoSourceModes {
    // Contains a video source reference for which a video source mode is
    // requested.
    #[yaserde(prefix = "trt", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,
}

impl Validate for GetVideoSourceModes {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetVideoSourceModesResponse {
    // Return the information for specified video source mode.
    #[yaserde(prefix = "trt", rename = "VideoSourceModes")]
    pub video_source_modes: Vec<VideoSourceMode>,
}

impl Validate for GetVideoSourceModesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct SetVideoSourceMode {
    // Contains a video source reference for which a video source mode is
    // requested.
    #[yaserde(prefix = "trt", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,

    // Indicate video source mode.
    #[yaserde(prefix = "trt", rename = "VideoSourceModeToken")]
    pub video_source_mode_token: tt::ReferenceToken,
}

impl Validate for SetVideoSourceMode {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct SetVideoSourceModeResponse {
    // The response contains information about rebooting after returning
    // response. When Reboot is set true, a device will reboot automatically
    // after setting mode.
    #[yaserde(prefix = "trt", rename = "Reboot")]
    pub reboot: bool,
}

impl Validate for SetVideoSourceModeResponse {}

// Indication which encodings are supported for this video source. The list may
// contain one or more enumeration values of tt:VideoEncoding.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct EncodingTypes(pub Vec<String>);

impl Validate for EncodingTypes {}
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct VideoSourceMode {
    // Max frame rate in frames per second for this video source mode.
    #[yaserde(prefix = "trt", rename = "MaxFramerate")]
    pub max_framerate: f64,

    // Max horizontal and vertical resolution for this video source mode.
    #[yaserde(prefix = "trt", rename = "MaxResolution")]
    pub max_resolution: tt::VideoResolution,

    // Indication which encodings are supported for this video source. The list
    // may contain one or more enumeration values of tt:VideoEncoding.
    #[yaserde(prefix = "trt", rename = "Encodings")]
    pub encodings: EncodingTypes,

    // After setting the mode if a device starts to reboot this value is true.
    // If a device change the mode without rebooting this value is false. If
    // true, configured parameters may not be guaranteed by the device after
    // rebooting.
    #[yaserde(prefix = "trt", rename = "Reboot")]
    pub reboot: bool,

    // Informative description of this video source mode. This field should be
    // described in English.
    #[yaserde(prefix = "trt", rename = "Description")]
    pub description: Option<tt::Description>,

    #[yaserde(prefix = "trt", rename = "Extension")]
    pub extension: Option<VideoSourceModeExtension>,

    // Indicate token for video source mode.
    #[yaserde(attribute, rename = "token")]
    pub token: tt::ReferenceToken,

    // Indication of whether this mode is active. If active this value is true.
    // In case of non-indication, it means as false. The value of true shall be
    // had by only one video source mode.
    #[yaserde(attribute, rename = "Enabled")]
    pub enabled: Option<bool>,
}

impl Validate for VideoSourceMode {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct VideoSourceModeExtension {}

impl Validate for VideoSourceModeExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetOSDs {
    // Token of the Video Source Configuration, which has OSDs associated with
    // are requested. If token not exist, request all available OSDs.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetOSDs {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetOSDsResponse {
    // This element contains a list of requested OSDs.
    #[yaserde(prefix = "trt", rename = "OSDs")]
    pub os_ds: Vec<tt::Osdconfiguration>,
}

impl Validate for GetOSDsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetOSD {
    // The GetOSD command fetches the OSD configuration if the OSD token is
    // known.
    #[yaserde(prefix = "trt", rename = "OSDToken")]
    pub osd_token: tt::ReferenceToken,
}

impl Validate for GetOSD {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetOSDResponse {
    // The requested OSD configuration.
    #[yaserde(prefix = "trt", rename = "OSD")]
    pub osd: tt::Osdconfiguration,
}

impl Validate for GetOSDResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct SetOSD {
    // Contains the modified OSD configuration.
    #[yaserde(prefix = "trt", rename = "OSD")]
    pub osd: tt::Osdconfiguration,
}

impl Validate for SetOSD {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct SetOSDResponse {}

impl Validate for SetOSDResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetOSDOptions {
    // Video Source Configuration Token that specifies an existing video source
    // configuration that the options shall be compatible with.
    #[yaserde(prefix = "trt", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetOSDOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct GetOSDOptionsResponse {
    #[yaserde(prefix = "trt", rename = "OSDOptions")]
    pub osd_options: tt::OsdconfigurationOptions,
}

impl Validate for GetOSDOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct CreateOSD {
    // Contain the initial OSD configuration for create.
    #[yaserde(prefix = "trt", rename = "OSD")]
    pub osd: tt::Osdconfiguration,
}

impl Validate for CreateOSD {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct CreateOSDResponse {
    // Returns Token of the newly created OSD
    #[yaserde(prefix = "trt", rename = "OSDToken")]
    pub osd_token: tt::ReferenceToken,
}

impl Validate for CreateOSDResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct DeleteOSD {
    // This element contains a reference to the OSD configuration that should be
    // deleted.
    #[yaserde(prefix = "trt", rename = "OSDToken")]
    pub osd_token: tt::ReferenceToken,
}

impl Validate for DeleteOSD {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trt",
    namespace = "trt: http://www.onvif.org/ver10/media/wsdl"
)]
pub struct DeleteOSDResponse {}

impl Validate for DeleteOSDResponse {}

// Returns the capabilities of the media service. The result is returned in a
// typed answer.
pub async fn get_service_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetServiceCapabilities,
) -> Result<GetServiceCapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This command lists all available physical video inputs of the device.
pub async fn get_video_sources<T: transport::Transport>(
    transport: &T,
    request: &GetVideoSources,
) -> Result<GetVideoSourcesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This command lists all available physical audio inputs of the device.
pub async fn get_audio_sources<T: transport::Transport>(
    transport: &T,
    request: &GetAudioSources,
) -> Result<GetAudioSourcesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This command lists all available physical audio outputs of the device.
pub async fn get_audio_outputs<T: transport::Transport>(
    transport: &T,
    request: &GetAudioOutputs,
) -> Result<GetAudioOutputsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation creates a new empty media profile. The media profile shall be
// created in the
// device and shall be persistent (remain after reboot). A created profile shall
// be deletable and a device shall set the “fixed” attribute to false in the
// returned Profile.
pub async fn create_profile<T: transport::Transport>(
    transport: &T,
    request: &CreateProfile,
) -> Result<CreateProfileResponse, transport::Error> {
    transport::request(transport, request).await
}

// If the profile token is already known, a profile can be fetched through the
// GetProfile command.
pub async fn get_profile<T: transport::Transport>(
    transport: &T,
    request: &GetProfile,
) -> Result<GetProfileResponse, transport::Error> {
    transport::request(transport, request).await
}

// Any endpoint can ask for the existing media profiles of a device using the
// GetProfiles
// command. Pre-configured or dynamically configured profiles can be retrieved
// using this
// command. This command lists all configured profiles in a device. The client
// does not need to
// know the media profile in order to use the command.
pub async fn get_profiles<T: transport::Transport>(
    transport: &T,
    request: &GetProfiles,
) -> Result<GetProfilesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation adds a VideoEncoderConfiguration to an existing media profile.
// If a
// configuration exists in the media profile, it will be replaced. The change
// shall be persistent. A device shall
// support adding a compatible VideoEncoderConfiguration to a Profile containing
// a VideoSourceConfiguration and shall
// support streaming video data of such a profile.
pub async fn add_video_encoder_configuration<T: transport::Transport>(
    transport: &T,
    request: &AddVideoEncoderConfiguration,
) -> Result<AddVideoEncoderConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation removes a VideoEncoderConfiguration from an existing media
// profile. If the
// media profile does not contain a VideoEncoderConfiguration, the operation has
// no effect. The removal shall be persistent.
pub async fn remove_video_encoder_configuration<T: transport::Transport>(
    transport: &T,
    request: &RemoveVideoEncoderConfiguration,
) -> Result<RemoveVideoEncoderConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation adds a VideoSourceConfiguration to an existing media profile.
// If such a
// configuration exists in the media profile, it will be replaced. The change
// shall be persistent.
pub async fn add_video_source_configuration<T: transport::Transport>(
    transport: &T,
    request: &AddVideoSourceConfiguration,
) -> Result<AddVideoSourceConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation removes a VideoSourceConfiguration from an existing media
// profile. If the
// media profile does not contain a VideoSourceConfiguration, the operation has
// no effect. The removal shall be persistent. Video source configurations
// should only be removed after removing a
// VideoEncoderConfiguration from the media profile.
pub async fn remove_video_source_configuration<T: transport::Transport>(
    transport: &T,
    request: &RemoveVideoSourceConfiguration,
) -> Result<RemoveVideoSourceConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation adds an AudioEncoderConfiguration to an existing media
// profile. If a
// configuration exists in the media profile, it will be replaced. The change
// shall be persistent. A device shall
// support adding a compatible AudioEncoderConfiguration to a profile containing
// an AudioSourceConfiguration and shall
// support streaming audio data of such a profile.
pub async fn add_audio_encoder_configuration<T: transport::Transport>(
    transport: &T,
    request: &AddAudioEncoderConfiguration,
) -> Result<AddAudioEncoderConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation removes an AudioEncoderConfiguration from an existing media
// profile. If the
// media profile does not contain an AudioEncoderConfiguration, the operation
// has no effect.
// The removal shall be persistent.
pub async fn remove_audio_encoder_configuration<T: transport::Transport>(
    transport: &T,
    request: &RemoveAudioEncoderConfiguration,
) -> Result<RemoveAudioEncoderConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation adds an AudioSourceConfiguration to an existing media profile.
// If a
// configuration exists in the media profile, it will be replaced. The change
// shall be persistent.
pub async fn add_audio_source_configuration<T: transport::Transport>(
    transport: &T,
    request: &AddAudioSourceConfiguration,
) -> Result<AddAudioSourceConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation removes an AudioSourceConfiguration from an existing media
// profile. If the
// media profile does not contain an AudioSourceConfiguration, the operation has
// no effect. The
// removal shall be persistent. Audio source configurations should only be
// removed after removing an
// AudioEncoderConfiguration from the media profile.
pub async fn remove_audio_source_configuration<T: transport::Transport>(
    transport: &T,
    request: &RemoveAudioSourceConfiguration,
) -> Result<RemoveAudioSourceConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation adds a PTZConfiguration to an existing media profile. If a
// configuration exists
// in the media profile, it will be replaced. The change shall be persistent.
// Adding a PTZConfiguration to a media profile means that streams using that
// media profile can
// contain PTZ status (in the metadata), and that the media profile can be used
// for controlling
// PTZ movement.
pub async fn add_ptz_configuration<T: transport::Transport>(
    transport: &T,
    request: &AddPTZConfiguration,
) -> Result<AddPTZConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation removes a PTZConfiguration from an existing media profile. If
// the media profile
// does not contain a PTZConfiguration, the operation has no effect. The removal
// shall be persistent.
pub async fn remove_ptz_configuration<T: transport::Transport>(
    transport: &T,
    request: &RemovePTZConfiguration,
) -> Result<RemovePTZConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation adds a VideoAnalytics configuration to an existing media
// profile. If a
// configuration exists in the media profile, it will be replaced. The change
// shall be persistent. Adding a VideoAnalyticsConfiguration to a media profile
// means that streams using that media
// profile can contain video analytics data (in the metadata) as defined by the
// submitted configuration reference. A profile containing only a video
// analytics configuration but no video source configuration is incomplete.
// Therefore, a client should first add a video source configuration to a
// profile before adding a video analytics configuration. The device can deny
// adding of a video analytics
// configuration before a video source configuration.
pub async fn add_video_analytics_configuration<T: transport::Transport>(
    transport: &T,
    request: &AddVideoAnalyticsConfiguration,
) -> Result<AddVideoAnalyticsConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation removes a VideoAnalyticsConfiguration from an existing media
// profile. If the media profile does not contain a VideoAnalyticsConfiguration,
// the operation has no effect.
// The removal shall be persistent.
pub async fn remove_video_analytics_configuration<T: transport::Transport>(
    transport: &T,
    request: &RemoveVideoAnalyticsConfiguration,
) -> Result<RemoveVideoAnalyticsConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation adds a Metadata configuration to an existing media profile. If
// a configuration exists in the media profile, it will be replaced. The change
// shall be persistent. Adding a MetadataConfiguration to a Profile means that
// streams using that profile contain metadata. Metadata can consist of events,
// PTZ status, and/or video analytics data.
pub async fn add_metadata_configuration<T: transport::Transport>(
    transport: &T,
    request: &AddMetadataConfiguration,
) -> Result<AddMetadataConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation removes a MetadataConfiguration from an existing media
// profile. If the media profile does not contain a MetadataConfiguration, the
// operation has no effect. The removal shall be persistent.
pub async fn remove_metadata_configuration<T: transport::Transport>(
    transport: &T,
    request: &RemoveMetadataConfiguration,
) -> Result<RemoveMetadataConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation adds an AudioOutputConfiguration to an existing media profile.
// If a configuration exists in the media profile, it will be replaced. The
// change shall be persistent.
pub async fn add_audio_output_configuration<T: transport::Transport>(
    transport: &T,
    request: &AddAudioOutputConfiguration,
) -> Result<AddAudioOutputConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation removes an AudioOutputConfiguration from an existing media
// profile. If the media profile does not contain an AudioOutputConfiguration,
// the operation has no effect. The removal shall be persistent.
pub async fn remove_audio_output_configuration<T: transport::Transport>(
    transport: &T,
    request: &RemoveAudioOutputConfiguration,
) -> Result<RemoveAudioOutputConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation adds an AudioDecoderConfiguration to an existing media
// profile. If a configuration exists in the media profile, it shall be
// replaced. The change shall be persistent.
pub async fn add_audio_decoder_configuration<T: transport::Transport>(
    transport: &T,
    request: &AddAudioDecoderConfiguration,
) -> Result<AddAudioDecoderConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation removes an AudioDecoderConfiguration from an existing media
// profile. If the media profile does not contain an AudioDecoderConfiguration,
// the operation has no effect. The removal shall be persistent.
pub async fn remove_audio_decoder_configuration<T: transport::Transport>(
    transport: &T,
    request: &RemoveAudioDecoderConfiguration,
) -> Result<RemoveAudioDecoderConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation deletes a profile. This change shall always be persistent.
// Deletion of a profile is only possible for non-fixed profiles
pub async fn delete_profile<T: transport::Transport>(
    transport: &T,
    request: &DeleteProfile,
) -> Result<DeleteProfileResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation lists all existing video source configurations for a device.
// The client need not know anything about the video source configurations in
// order to use the command.
pub async fn get_video_source_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetVideoSourceConfigurations,
) -> Result<GetVideoSourceConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation lists all existing video encoder configurations of a device.
// This command lists all configured video encoder configurations in a device.
// The client need not know anything apriori about the video encoder
// configurations in order to use the command.
pub async fn get_video_encoder_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetVideoEncoderConfigurations,
) -> Result<GetVideoEncoderConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation lists all existing audio source configurations of a device.
// This command lists all audio source configurations in a device. The client
// need not know anything apriori about the audio source configurations in order
// to use the command.
pub async fn get_audio_source_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetAudioSourceConfigurations,
) -> Result<GetAudioSourceConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation lists all existing device audio encoder configurations. The
// client need not know anything apriori about the audio encoder configurations
// in order to use the command.
pub async fn get_audio_encoder_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetAudioEncoderConfigurations,
) -> Result<GetAudioEncoderConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation lists all video analytics configurations of a device. This
// command lists all configured video analytics in a device. The client need not
// know anything apriori about the video analytics in order to use the command.
pub async fn get_video_analytics_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetVideoAnalyticsConfigurations,
) -> Result<GetVideoAnalyticsConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation lists all existing metadata configurations. The client need
// not know anything apriori about the metadata in order to use the command.
pub async fn get_metadata_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetMetadataConfigurations,
) -> Result<GetMetadataConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This command lists all existing AudioOutputConfigurations of a device. The
// NVC need not know anything apriori about the audio configurations to use this
// command.
pub async fn get_audio_output_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetAudioOutputConfigurations,
) -> Result<GetAudioOutputConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This command lists all existing AudioDecoderConfigurations of a device. The
// NVC need not know anything apriori about the audio decoder configurations in
// order to
// use this command.
pub async fn get_audio_decoder_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetAudioDecoderConfigurations,
) -> Result<GetAudioDecoderConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// If the video source configuration token is already known, the video source
// configuration can be fetched through the GetVideoSourceConfiguration command.
pub async fn get_video_source_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetVideoSourceConfiguration,
) -> Result<GetVideoSourceConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// If the video encoder configuration token is already known, the encoder
// configuration can be fetched through the GetVideoEncoderConfiguration
// command.
pub async fn get_video_encoder_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetVideoEncoderConfiguration,
) -> Result<GetVideoEncoderConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// The GetAudioSourceConfiguration command fetches the audio source
// configurations if the audio source configuration token is already known. An
pub async fn get_audio_source_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetAudioSourceConfiguration,
) -> Result<GetAudioSourceConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// The GetAudioEncoderConfiguration command fetches the encoder configuration if
// the audio encoder configuration token is known.
pub async fn get_audio_encoder_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetAudioEncoderConfiguration,
) -> Result<GetAudioEncoderConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// The GetVideoAnalyticsConfiguration command fetches the video analytics
// configuration if the video analytics token is known.
pub async fn get_video_analytics_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetVideoAnalyticsConfiguration,
) -> Result<GetVideoAnalyticsConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// The GetMetadataConfiguration command fetches the metadata configuration if
// the metadata token is known.
pub async fn get_metadata_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetMetadataConfiguration,
) -> Result<GetMetadataConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// If the audio output configuration token is already known, the output
// configuration can be fetched through the GetAudioOutputConfiguration command.
pub async fn get_audio_output_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetAudioOutputConfiguration,
) -> Result<GetAudioOutputConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// If the audio decoder configuration token is already known, the decoder
// configuration can be fetched through the GetAudioDecoderConfiguration
// command.
pub async fn get_audio_decoder_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetAudioDecoderConfiguration,
) -> Result<GetAudioDecoderConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation lists all the video encoder configurations of the device that
// are compatible with a certain media profile. Each of the returned
// configurations shall be a valid input parameter for the
// AddVideoEncoderConfiguration command on the media profile. The result will
// vary depending on the capabilities, configurations and settings in the
// device.
pub async fn get_compatible_video_encoder_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetCompatibleVideoEncoderConfigurations,
) -> Result<GetCompatibleVideoEncoderConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests all the video source configurations of the device
// that are compatible
// with a certain media profile. Each of the returned configurations shall be a
// valid input
// parameter for the AddVideoSourceConfiguration command on the media profile.
// The result
// will vary depending on the capabilities, configurations and settings in the
// device.
pub async fn get_compatible_video_source_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetCompatibleVideoSourceConfigurations,
) -> Result<GetCompatibleVideoSourceConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests all audio encoder configurations of a device that are
// compatible with a certain media profile. Each of the returned configurations
// shall be a valid input parameter for the AddAudioSourceConfiguration command
// on the media profile. The result varies depending on the capabilities,
// configurations and settings in the device.
pub async fn get_compatible_audio_encoder_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetCompatibleAudioEncoderConfigurations,
) -> Result<GetCompatibleAudioEncoderConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests all audio source configurations of the device that
// are compatible with a certain media profile. Each of the returned
// configurations shall be a valid input parameter for the
// AddAudioEncoderConfiguration command on the media profile. The result varies
// depending on the capabilities, configurations and settings in the device.
pub async fn get_compatible_audio_source_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetCompatibleAudioSourceConfigurations,
) -> Result<GetCompatibleAudioSourceConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests all video analytic configurations of the device that
// are compatible with a certain media profile. Each of the returned
// configurations shall be a valid input parameter for the
// AddVideoAnalyticsConfiguration command on the media profile. The result
// varies depending on the capabilities, configurations and settings in the
// device.
pub async fn get_compatible_video_analytics_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetCompatibleVideoAnalyticsConfigurations,
) -> Result<GetCompatibleVideoAnalyticsConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests all the metadata configurations of the device that
// are compatible with a certain media profile. Each of the returned
// configurations shall be a valid input parameter for the
// AddMetadataConfiguration command on the media profile. The result varies
// depending on the capabilities, configurations and settings in the device.
pub async fn get_compatible_metadata_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetCompatibleMetadataConfigurations,
) -> Result<GetCompatibleMetadataConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This command lists all audio output configurations of a device that are
// compatible with a certain media profile. Each returned configuration shall be
// a valid input for the
// AddAudioOutputConfiguration command.
pub async fn get_compatible_audio_output_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetCompatibleAudioOutputConfigurations,
) -> Result<GetCompatibleAudioOutputConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation lists all the audio decoder configurations of the device that
// are compatible with a certain media profile. Each of the returned
// configurations shall be a valid input parameter for the
// AddAudioDecoderConfiguration command on the media profile.
pub async fn get_compatible_audio_decoder_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetCompatibleAudioDecoderConfigurations,
) -> Result<GetCompatibleAudioDecoderConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation modifies a video source configuration. The ForcePersistence
// flag indicates if the changes shall remain after reboot of the device.
// Running streams using this configuration may be immediately updated according
// to the new settings. The changes are not guaranteed to take effect unless the
// client requests a new stream URI and restarts any affected stream. NVC
// methods for changing a running stream are out of scope for this
// specification.
pub async fn set_video_source_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetVideoSourceConfiguration,
) -> Result<SetVideoSourceConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation modifies a video encoder configuration. The ForcePersistence
// flag indicates if the changes shall remain after reboot of the device.
// Changes in the Multicast settings shall always be persistent. Running streams
// using this configuration may be immediately updated according to the new
// settings. The changes are not guaranteed to take effect unless the client
// requests a new stream URI and restarts any affected stream. NVC methods for
// changing a running stream are out of scope for this specification.
pub async fn set_video_encoder_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetVideoEncoderConfiguration,
) -> Result<SetVideoEncoderConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation modifies an audio source configuration. The ForcePersistence
// flag indicates if
// the changes shall remain after reboot of the device. Running streams using
// this configuration
// may be immediately updated according to the new settings. The changes are not
// guaranteed
// to take effect unless the client requests a new stream URI and restarts any
// affected stream
// NVC methods for changing a running stream are out of scope for this
// specification.
pub async fn set_audio_source_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetAudioSourceConfiguration,
) -> Result<SetAudioSourceConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation modifies an audio encoder configuration. The ForcePersistence
// flag indicates if
// the changes shall remain after reboot of the device. Running streams using
// this configuration may be immediately updated
// according to the new settings. The changes are not guaranteed to take effect
// unless the client
// requests a new stream URI and restarts any affected streams. NVC methods for
// changing a
// running stream are out of scope for this specification.
pub async fn set_audio_encoder_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetAudioEncoderConfiguration,
) -> Result<SetAudioEncoderConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// A video analytics configuration is modified using this command. The
// ForcePersistence flag
// indicates if the changes shall remain after reboot of the device or not.
// Running streams using
// this configuration shall be immediately updated according to the new
// settings. Otherwise
// inconsistencies can occur between the scene description processed by the rule
// engine and
// the notifications produced by analytics engine and rule engine which
// reference the very same
// video analytics configuration token.
pub async fn set_video_analytics_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetVideoAnalyticsConfiguration,
) -> Result<SetVideoAnalyticsConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation modifies a metadata configuration. The ForcePersistence flag
// indicates if the
// changes shall remain after reboot of the device. Changes in the Multicast
// settings shall
// always be persistent. Running streams using this configuration may be updated
// immediately
// according to the new settings. The changes are not guaranteed to take effect
// unless the client
// requests a new stream URI and restarts any affected streams. NVC methods for
// changing a
// running stream are out of scope for this specification.
pub async fn set_metadata_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetMetadataConfiguration,
) -> Result<SetMetadataConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation modifies an audio output configuration. The ForcePersistence
// flag indicates if
// the changes shall remain after reboot of the device.
pub async fn set_audio_output_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetAudioOutputConfiguration,
) -> Result<SetAudioOutputConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation modifies an audio decoder configuration. The ForcePersistence
// flag indicates if
// the changes shall remain after reboot of the device.
pub async fn set_audio_decoder_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetAudioDecoderConfiguration,
) -> Result<SetAudioDecoderConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns the available options (supported values and ranges for
// video source configuration parameters) when the video source parameters are
// reconfigured If a video source configuration is specified, the options shall
// concern that
// particular configuration. If a media profile is specified, the options shall
// be compatible with
// that media profile.
pub async fn get_video_source_configuration_options<T: transport::Transport>(
    transport: &T,
    request: &GetVideoSourceConfigurationOptions,
) -> Result<GetVideoSourceConfigurationOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns the available options (supported values and ranges for
// video encoder
// configuration parameters) when the video encoder parameters are reconfigured.
pub async fn get_video_encoder_configuration_options<T: transport::Transport>(
    transport: &T,
    request: &GetVideoEncoderConfigurationOptions,
) -> Result<GetVideoEncoderConfigurationOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns the available options (supported values and ranges for
// audio source configuration parameters) when the audio source parameters are
// reconfigured. If an audio source configuration is specified, the options
// shall concern that
// particular configuration. If a media profile is specified, the options shall
// be compatible with
// that media profile.
pub async fn get_audio_source_configuration_options<T: transport::Transport>(
    transport: &T,
    request: &GetAudioSourceConfigurationOptions,
) -> Result<GetAudioSourceConfigurationOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns the available options (supported values and ranges for
// audio encoder configuration parameters) when the audio encoder parameters are
// reconfigured.
pub async fn get_audio_encoder_configuration_options<T: transport::Transport>(
    transport: &T,
    request: &GetAudioEncoderConfigurationOptions,
) -> Result<GetAudioEncoderConfigurationOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns the available options (supported values and ranges for
// metadata configuration parameters) for changing the metadata configuration.
pub async fn get_metadata_configuration_options<T: transport::Transport>(
    transport: &T,
    request: &GetMetadataConfigurationOptions,
) -> Result<GetMetadataConfigurationOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns the available options (supported values and ranges for
// audio output configuration parameters) for configuring an audio output.
pub async fn get_audio_output_configuration_options<T: transport::Transport>(
    transport: &T,
    request: &GetAudioOutputConfigurationOptions,
) -> Result<GetAudioOutputConfigurationOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This command list the audio decoding capabilities for a given profile and
// configuration of a
// device.
pub async fn get_audio_decoder_configuration_options<T: transport::Transport>(
    transport: &T,
    request: &GetAudioDecoderConfigurationOptions,
) -> Result<GetAudioDecoderConfigurationOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// The GetGuaranteedNumberOfVideoEncoderInstances command can be used to request
// the
// minimum number of guaranteed video encoder instances (applications) per Video
// Source
// Configuration.
pub async fn get_guaranteed_number_of_video_encoder_instances<T: transport::Transport>(
    transport: &T,
    request: &GetGuaranteedNumberOfVideoEncoderInstances,
) -> Result<GetGuaranteedNumberOfVideoEncoderInstancesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a URI that can be used to initiate a live media
// stream using RTSP as
// the control protocol. The returned URI shall remain valid indefinitely even
// if the profile is
// changed. The ValidUntilConnect, ValidUntilReboot and Timeout Parameter shall
// be set
// accordingly (ValidUntilConnect=false, ValidUntilReboot=false, timeout=PT0S).
pub async fn get_stream_uri<T: transport::Transport>(
    transport: &T,
    request: &GetStreamUri,
) -> Result<GetStreamUriResponse, transport::Error> {
    transport::request(transport, request).await
}

// This command starts multicast streaming using a specified media profile of a
// device.
// Streaming continues until StopMulticastStreaming is called for the same
// Profile. The
// streaming shall continue after a reboot of the device until a
// StopMulticastStreaming request is
// received. The multicast address, port and TTL are configured in the
// VideoEncoderConfiguration, AudioEncoderConfiguration and
// MetadataConfiguration
// respectively.
pub async fn start_multicast_streaming<T: transport::Transport>(
    transport: &T,
    request: &StartMulticastStreaming,
) -> Result<StartMulticastStreamingResponse, transport::Error> {
    transport::request(transport, request).await
}

// This command stop multicast streaming using a specified media profile of a
// device
pub async fn stop_multicast_streaming<T: transport::Transport>(
    transport: &T,
    request: &StopMulticastStreaming,
) -> Result<StopMulticastStreamingResponse, transport::Error> {
    transport::request(transport, request).await
}

// Synchronization points allow clients to decode and correctly use all data
// after the
// synchronization point.
// For example, if a video stream is configured with a large I-frame distance
// and a client loses a
// single packet, the client does not display video until the next I-frame is
// transmitted. In such
// cases, the client can request a Synchronization Point which enforces the
// device to add an I-Frame as soon as possible. Clients can request
// Synchronization Points for profiles. The device
// shall add synchronization points for all streams associated with this
// profile.
// Similarly, a synchronization point is used to get an update on full PTZ or
// event status through
// the metadata stream.
// If a video stream is associated with the profile, an I-frame shall be added
// to this video stream.
// If a PTZ metadata stream is associated to the profile,
// the PTZ position shall be repeated within the metadata stream.
pub async fn set_synchronization_point<T: transport::Transport>(
    transport: &T,
    request: &SetSynchronizationPoint,
) -> Result<SetSynchronizationPointResponse, transport::Error> {
    transport::request(transport, request).await
}

// A client uses the GetSnapshotUri command to obtain a JPEG snapshot from the
// device. The returned URI shall remain valid indefinitely even if the profile
// is changed. The
// ValidUntilConnect, ValidUntilReboot and Timeout Parameter shall be set
// accordingly
// (ValidUntilConnect=false, ValidUntilReboot=false, timeout=PT0S). The URI can
// be used for
// acquiring a JPEG image through a HTTP GET operation. The image encoding will
// always be
// JPEG regardless of the encoding setting in the media profile. The Jpeg
// settings
// (like resolution or quality) may be taken from the profile if suitable. The
// provided
// image will be updated automatically and independent from calls to
// GetSnapshotUri.
pub async fn get_snapshot_uri<T: transport::Transport>(
    transport: &T,
    request: &GetSnapshotUri,
) -> Result<GetSnapshotUriResponse, transport::Error> {
    transport::request(transport, request).await
}

// A device returns the information for current video source mode and settable
// video source modes of specified video source. A device that indicates a
// capability of VideoSourceModes shall support this command.
pub async fn get_video_source_modes<T: transport::Transport>(
    transport: &T,
    request: &GetVideoSourceModes,
) -> Result<GetVideoSourceModesResponse, transport::Error> {
    transport::request(transport, request).await
}

// SetVideoSourceMode changes the media profile structure relating to video
// source for the specified video source mode. A device that indicates a
// capability of VideoSourceModes shall support this command. The behavior after
// changing the mode is not defined in this specification.
pub async fn set_video_source_mode<T: transport::Transport>(
    transport: &T,
    request: &SetVideoSourceMode,
) -> Result<SetVideoSourceModeResponse, transport::Error> {
    transport::request(transport, request).await
}

// Get the OSDs.
pub async fn get_os_ds<T: transport::Transport>(
    transport: &T,
    request: &GetOSDs,
) -> Result<GetOSDsResponse, transport::Error> {
    transport::request(transport, request).await
}

// Get the OSD.
pub async fn get_osd<T: transport::Transport>(
    transport: &T,
    request: &GetOSD,
) -> Result<GetOSDResponse, transport::Error> {
    transport::request(transport, request).await
}

// Get the OSD Options.
pub async fn get_osd_options<T: transport::Transport>(
    transport: &T,
    request: &GetOSDOptions,
) -> Result<GetOSDOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// Set the OSD
pub async fn set_osd<T: transport::Transport>(
    transport: &T,
    request: &SetOSD,
) -> Result<SetOSDResponse, transport::Error> {
    transport::request(transport, request).await
}

// Create the OSD.
pub async fn create_osd<T: transport::Transport>(
    transport: &T,
    request: &CreateOSD,
) -> Result<CreateOSDResponse, transport::Error> {
    transport::request(transport, request).await
}

// Delete the OSD.
pub async fn delete_osd<T: transport::Transport>(
    transport: &T,
    request: &DeleteOSD,
) -> Result<DeleteOSDResponse, transport::Error> {
    transport::request(transport, request).await
}
