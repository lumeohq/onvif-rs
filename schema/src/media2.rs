use crate::onvif as tt;
use crate::transport;
use crate::validate::Validate;
use std::str::FromStr;
use xsd_macro_utils::*;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetServiceCapabilitiesResponse {
    // The capabilities for the media service is returned in the Capabilities
    // element.
    #[yaserde(prefix = "tr2", rename = "Capabilities")]
    pub capabilities: Capabilities2,
}

impl Validate for GetServiceCapabilitiesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct Capabilities2 {
    // Media profile capabilities.
    #[yaserde(prefix = "tr2", rename = "ProfileCapabilities")]
    pub profile_capabilities: ProfileCapabilities,

    // Streaming capabilities.
    #[yaserde(prefix = "tr2", rename = "StreamingCapabilities")]
    pub streaming_capabilities: StreamingCapabilities,

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

    // Indicates if Masking is supported.
    #[yaserde(attribute, rename = "Mask")]
    pub mask: Option<bool>,

    // Indicates that privacy masks are only supported at the video source level
    // and not the video source configuration level.
    // If this is true any addition, deletion or change of a privacy mask done
    // for one video source configuration will automatically be
    // applied by the device to a corresponding privacy mask for all other video
    // source configuration associated with the same video source.
    #[yaserde(attribute, rename = "SourceMask")]
    pub source_mask: Option<bool>,
}

impl Validate for Capabilities2 {}

pub type Capabilities = Capabilities2;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct ProfileCapabilities {
    // Maximum number of profiles supported.
    #[yaserde(attribute, rename = "MaximumNumberOfProfiles")]
    pub maximum_number_of_profiles: Option<i32>,

    // The configurations supported by the device as defined by
    // tr2:ConfigurationEnumeration. The enumeration value "All" shall not be
    // included in this list.
    #[yaserde(attribute, rename = "ConfigurationsSupported")]
    pub configurations_supported: Option<tt::StringAttrList>,
}

impl Validate for ProfileCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct StreamingCapabilities {
    // Indicates support for live media streaming via RTSP.
    #[yaserde(attribute, rename = "RTSPStreaming")]
    pub rtsp_streaming: Option<bool>,

    // Indicates support for RTP multicast.
    #[yaserde(attribute, rename = "RTPMulticast")]
    pub rtp_multicast: Option<bool>,

    // Indicates support for RTP/RTSP/TCP.
    #[yaserde(attribute, rename = "RTP_RTSP_TCP")]
    pub rtp_rtsp_tcp: Option<bool>,

    // Indicates support for non aggregate RTSP control.
    #[yaserde(attribute, rename = "NonAggregateControl")]
    pub non_aggregate_control: Option<bool>,

    // If streaming over WebSocket is supported, this shall return the RTSP
    // WebSocket URI as described in Streaming Specification Section 5.1.1.5.
    #[yaserde(attribute, rename = "RTSPWebSocketUri")]
    pub rtsp_web_socket_uri: Option<String>,

    // Indicates support for non-RTSP controlled multicast streaming.
    #[yaserde(attribute, rename = "AutoStartMulticast")]
    pub auto_start_multicast: Option<bool>,
}

impl Validate for StreamingCapabilities {}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum ConfigurationEnumeration {
    All,
    VideoSource,
    VideoEncoder,
    AudioSource,
    AudioEncoder,
    AudioOutput,
    AudioDecoder,
    Metadata,
    Analytics,
    #[yaserde(rename = "PTZ")]
    Ptz,
    __Unknown__(String),
}

impl Default for ConfigurationEnumeration {
    fn default() -> ConfigurationEnumeration {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for ConfigurationEnumeration {}

// pub type ConfigurationEnumeration = ConfigurationEnumeration;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct ConfigurationRef {
    // Type of the configuration as defined by tr2:ConfigurationEnumeration.
    #[yaserde(prefix = "tr2", rename = "Type")]
    pub _type: String,

    // Reference token of an existing configuration.
    // Token shall be included in the AddConfiguration request along with the
    // type.
    // Token shall be included in the CreateProfile request when Configuration
    // elements are included and type is selected.
    // Token is optional for RemoveConfiguration request. If no token is
    // provided in RemoveConfiguration request, device shall
    // remove the configuration of the type included in the profile.
    #[yaserde(prefix = "tr2", rename = "Token")]
    pub token: Option<tt::ReferenceToken>,
}

impl Validate for ConfigurationRef {}

// A set of media configurations.

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct ConfigurationSet {
    // Optional configuration of the Video input.
    #[yaserde(prefix = "tr2", rename = "VideoSource")]
    pub video_source: Option<tt::VideoSourceConfiguration>,

    // Optional configuration of the Audio input.
    #[yaserde(prefix = "tr2", rename = "AudioSource")]
    pub audio_source: Option<tt::AudioSourceConfiguration>,

    // Optional configuration of the Video encoder.
    #[yaserde(prefix = "tr2", rename = "VideoEncoder")]
    pub video_encoder: Option<tt::VideoEncoder2Configuration>,

    // Optional configuration of the Audio encoder.
    #[yaserde(prefix = "tr2", rename = "AudioEncoder")]
    pub audio_encoder: Option<tt::AudioEncoder2Configuration>,

    // Optional configuration of the analytics module and rule engine.
    #[yaserde(prefix = "tr2", rename = "Analytics")]
    pub analytics: Option<tt::VideoAnalyticsConfiguration>,

    // Optional configuration of the pan tilt zoom unit.
    #[yaserde(prefix = "tr2", rename = "PTZ")]
    pub ptz: Option<tt::Ptzconfiguration>,

    // Optional configuration of the metadata stream.
    #[yaserde(prefix = "tr2", rename = "Metadata")]
    pub metadata: Option<tt::MetadataConfiguration>,

    // Optional configuration of the Audio output.
    #[yaserde(prefix = "tr2", rename = "AudioOutput")]
    pub audio_output: Option<tt::AudioOutputConfiguration>,

    // Optional configuration of the Audio decoder.
    #[yaserde(prefix = "tr2", rename = "AudioDecoder")]
    pub audio_decoder: Option<tt::AudioDecoderConfiguration>,
}

impl Validate for ConfigurationSet {}

// A media profile consists of a set of media configurations.

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct MediaProfile {
    // User readable name of the profile.
    #[yaserde(prefix = "tr2", rename = "Name")]
    pub name: tt::Name,

    // The configurations assigned to the profile.
    #[yaserde(prefix = "tr2", rename = "Configurations")]
    pub configurations: Option<ConfigurationSet>,

    // Unique identifier of the profile.
    #[yaserde(attribute, rename = "token")]
    pub token: tt::ReferenceToken,

    // A value of true signals that the profile cannot be deleted. Default is
    // false.
    #[yaserde(attribute, rename = "fixed")]
    pub fixed: Option<bool>,
}

impl Validate for MediaProfile {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct CreateProfile {
    // friendly name of the profile to be created
    #[yaserde(prefix = "tr2", rename = "Name")]
    pub name: tt::Name,

    // Optional set of configurations to be assigned to the profile. List
    // entries with tr2:ConfigurationEnumeration value "All" shall be ignored.
    #[yaserde(prefix = "tr2", rename = "Configuration")]
    pub configuration: Vec<ConfigurationRef>,
}

impl Validate for CreateProfile {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct CreateProfileResponse {
    // Token assigned by the device for the newly created profile.
    #[yaserde(prefix = "tr2", rename = "Token")]
    pub token: tt::ReferenceToken,
}

impl Validate for CreateProfileResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetProfiles {
    // Optional token of the requested profile.
    #[yaserde(prefix = "tr2", rename = "Token")]
    pub token: Option<tt::ReferenceToken>,

    // The types shall be provided as defined by tr2:ConfigurationEnumeration.
    #[yaserde(prefix = "tr2", rename = "Type")]
    pub _type: Vec<String>,
}

impl Validate for GetProfiles {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetProfilesResponse {
    // Lists all profiles that exist in the media service. The response provides
    // the requested types of Configurations as far as available.
    // If a profile doesn't contain a type no error shall be provided.
    #[yaserde(prefix = "tr2", rename = "Profiles")]
    pub profiles: Vec<MediaProfile>,
}

impl Validate for GetProfilesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct AddConfiguration {
    // Reference to the profile where the configuration should be added
    #[yaserde(prefix = "tr2", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // Optional item. If present updates the Name property of the profile.
    #[yaserde(prefix = "tr2", rename = "Name")]
    pub name: Option<tt::Name>,

    // List of configurations to be added. The types shall be provided in the
    // order defined by tr2:ConfigurationEnumeration. List entries with
    // tr2:ConfigurationEnumeration value "All" shall be ignored.
    #[yaserde(prefix = "tr2", rename = "Configuration")]
    pub configuration: Vec<ConfigurationRef>,
}

impl Validate for AddConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct AddConfigurationResponse {}

impl Validate for AddConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct RemoveConfiguration {
    // This element contains a reference to the media profile from which the
    // AudioDecoderConfiguration shall be removed.
    #[yaserde(prefix = "tr2", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,

    // List of configurations to be removed. The types shall be provided in the
    // order defined by tr2:ConfigurationEnumeration. Tokens appearing in the
    // configuration list shall be ignored. Presence of the "All" type shall
    // result in an empty profile.
    #[yaserde(prefix = "tr2", rename = "Configuration")]
    pub configuration: Vec<ConfigurationRef>,
}

impl Validate for RemoveConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct RemoveConfigurationResponse {}

impl Validate for RemoveConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct DeleteProfile {
    // This element contains a reference to the profile that should be deleted.
    #[yaserde(prefix = "tr2", rename = "Token")]
    pub token: tt::ReferenceToken,
}

impl Validate for DeleteProfile {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct DeleteProfileResponse {}

impl Validate for DeleteProfileResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetConfiguration {
    // Token of the requested configuration.
    #[yaserde(prefix = "tr2", rename = "ConfigurationToken")]
    pub configuration_token: Option<tt::ReferenceToken>,

    // Contains the token of an existing media profile the configurations shall
    // be compatible with.
    #[yaserde(prefix = "tr2", rename = "ProfileToken")]
    pub profile_token: Option<tt::ReferenceToken>,
}

impl Validate for GetConfiguration {}

pub type GetVideoEncoderConfigurations = GetConfiguration;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetVideoEncoderConfigurationsResponse {
    // This element contains a list of video encoder configurations.
    #[yaserde(prefix = "tr2", rename = "Configurations")]
    pub configurations: Vec<tt::VideoEncoder2Configuration>,
}

impl Validate for GetVideoEncoderConfigurationsResponse {}

pub type GetVideoSourceConfigurations = GetConfiguration;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetVideoSourceConfigurationsResponse {
    // This element contains a list of video source configurations.
    #[yaserde(prefix = "tr2", rename = "Configurations")]
    pub configurations: Vec<tt::VideoSourceConfiguration>,
}

impl Validate for GetVideoSourceConfigurationsResponse {}

pub type GetAudioEncoderConfigurations = GetConfiguration;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetAudioEncoderConfigurationsResponse {
    // This element contains a list of audio encoder configurations.
    #[yaserde(prefix = "tr2", rename = "Configurations")]
    pub configurations: Vec<tt::AudioEncoder2Configuration>,
}

impl Validate for GetAudioEncoderConfigurationsResponse {}

pub type GetAudioSourceConfigurations = GetConfiguration;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetAudioSourceConfigurationsResponse {
    // This element contains a list of audio source configurations.
    #[yaserde(prefix = "tr2", rename = "Configurations")]
    pub configurations: Vec<tt::AudioSourceConfiguration>,
}

impl Validate for GetAudioSourceConfigurationsResponse {}

pub type GetAnalyticsConfigurations = GetConfiguration;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetAnalyticsConfigurationsResponse {
    // This element contains a list of Analytics configurations.
    #[yaserde(prefix = "tr2", rename = "Configurations")]
    pub configurations: Vec<tt::VideoAnalyticsConfiguration>,
}

impl Validate for GetAnalyticsConfigurationsResponse {}

pub type GetMetadataConfigurations = GetConfiguration;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetMetadataConfigurationsResponse {
    // This element contains a list of metadata configurations
    #[yaserde(prefix = "tr2", rename = "Configurations")]
    pub configurations: Vec<tt::MetadataConfiguration>,
}

impl Validate for GetMetadataConfigurationsResponse {}

pub type GetAudioOutputConfigurations = GetConfiguration;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetAudioOutputConfigurationsResponse {
    // This element contains a list of audio output configurations
    #[yaserde(prefix = "tr2", rename = "Configurations")]
    pub configurations: Vec<tt::AudioOutputConfiguration>,
}

impl Validate for GetAudioOutputConfigurationsResponse {}

pub type GetAudioDecoderConfigurations = GetConfiguration;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetAudioDecoderConfigurationsResponse {
    // This element contains a list of audio decoder configurations
    #[yaserde(prefix = "tr2", rename = "Configurations")]
    pub configurations: Vec<tt::AudioDecoderConfiguration>,
}

impl Validate for GetAudioDecoderConfigurationsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct SetVideoEncoderConfiguration {
    // Contains the modified video encoder configuration. The configuration
    // shall exist in the device.
    #[yaserde(prefix = "tr2", rename = "Configuration")]
    pub configuration: tt::VideoEncoder2Configuration,
}

impl Validate for SetVideoEncoderConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct SetConfigurationResponse {}

impl Validate for SetConfigurationResponse {}

pub type SetVideoEncoderConfigurationResponse = SetConfigurationResponse;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct SetVideoSourceConfiguration {
    // Contains the modified video source configuration. The configuration shall
    // exist in the device.
    #[yaserde(prefix = "tr2", rename = "Configuration")]
    pub configuration: tt::VideoSourceConfiguration,
}

impl Validate for SetVideoSourceConfiguration {}

pub type SetVideoSourceConfigurationResponse = SetConfigurationResponse;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct SetAudioEncoderConfiguration {
    // Contains the modified audio encoder configuration. The configuration
    // shall exist in the device.
    #[yaserde(prefix = "tr2", rename = "Configuration")]
    pub configuration: tt::AudioEncoder2Configuration,
}

impl Validate for SetAudioEncoderConfiguration {}

pub type SetAudioEncoderConfigurationResponse = SetConfigurationResponse;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct SetAudioSourceConfiguration {
    // Contains the modified audio source configuration. The configuration shall
    // exist in the device.
    #[yaserde(prefix = "tr2", rename = "Configuration")]
    pub configuration: tt::AudioSourceConfiguration,
}

impl Validate for SetAudioSourceConfiguration {}

pub type SetAudioSourceConfigurationResponse = SetConfigurationResponse;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct SetMetadataConfiguration {
    // Contains the modified metadata configuration. The configuration shall
    // exist in the device.
    #[yaserde(prefix = "tr2", rename = "Configuration")]
    pub configuration: tt::MetadataConfiguration,
}

impl Validate for SetMetadataConfiguration {}

pub type SetMetadataConfigurationResponse = SetConfigurationResponse;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct SetAudioOutputConfiguration {
    // Contains the modified audio output configuration. The configuration shall
    // exist in the device.
    #[yaserde(prefix = "tr2", rename = "Configuration")]
    pub configuration: tt::AudioOutputConfiguration,
}

impl Validate for SetAudioOutputConfiguration {}

pub type SetAudioOutputConfigurationResponse = SetConfigurationResponse;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct SetAudioDecoderConfiguration {
    // Contains the modified audio decoder configuration. The configuration
    // shall exist in the device.
    #[yaserde(prefix = "tr2", rename = "Configuration")]
    pub configuration: tt::AudioDecoderConfiguration,
}

impl Validate for SetAudioDecoderConfiguration {}

pub type SetAudioDecoderConfigurationResponse = SetConfigurationResponse;
pub type GetVideoSourceConfigurationOptions = GetConfiguration;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetVideoSourceConfigurationOptionsResponse {
    // This message contains the video source configuration options. If a video
    // source configuration is specified, the options shall concern that
    // particular configuration. If a media profile is specified, the options
    // shall be compatible with that media profile. If no tokens are specified,
    // the options shall be considered generic for the device.
    #[yaserde(prefix = "tr2", rename = "Options")]
    pub options: tt::VideoSourceConfigurationOptions,
}

impl Validate for GetVideoSourceConfigurationOptionsResponse {}

pub type GetVideoEncoderConfigurationOptions = GetConfiguration;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetVideoEncoderConfigurationOptionsResponse {
    #[yaserde(prefix = "tr2", rename = "Options")]
    pub options: Vec<tt::VideoEncoder2ConfigurationOptions>,
}

impl Validate for GetVideoEncoderConfigurationOptionsResponse {}

pub type GetAudioSourceConfigurationOptions = GetConfiguration;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetAudioSourceConfigurationOptionsResponse {
    // This message contains the audio source configuration options. If a audio
    // source configuration is specified, the options shall concern that
    // particular configuration. If a media profile is specified, the options
    // shall be compatible with that media profile. If no tokens are specified,
    // the options shall be considered generic for the device.
    #[yaserde(prefix = "tr2", rename = "Options")]
    pub options: tt::AudioSourceConfigurationOptions,
}

impl Validate for GetAudioSourceConfigurationOptionsResponse {}

pub type GetAudioEncoderConfigurationOptions = GetConfiguration;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetAudioEncoderConfigurationOptionsResponse {
    // This message contains the audio encoder configuration options. If a audio
    // encoder configuration is specified, the options shall concern that
    // particular configuration. If a media profile is specified, the options
    // shall be compatible with that media profile. If no tokens are specified,
    // the options shall be considered generic for the device.
    #[yaserde(prefix = "tr2", rename = "Options")]
    pub options: Vec<tt::AudioEncoder2ConfigurationOptions>,
}

impl Validate for GetAudioEncoderConfigurationOptionsResponse {}

pub type GetMetadataConfigurationOptions = GetConfiguration;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetMetadataConfigurationOptionsResponse {
    // This message contains the metadata configuration options. If a metadata
    // configuration is specified, the options shall concern that particular
    // configuration. If a media profile is specified, the options shall be
    // compatible with that media profile. If no tokens are specified, the
    // options shall be considered generic for the device.
    #[yaserde(prefix = "tr2", rename = "Options")]
    pub options: tt::MetadataConfigurationOptions,
}

impl Validate for GetMetadataConfigurationOptionsResponse {}

pub type GetAudioOutputConfigurationOptions = GetConfiguration;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetAudioOutputConfigurationOptionsResponse {
    // This message contains the audio output configuration options. If a audio
    // output configuration is specified, the options shall concern that
    // particular configuration. If a media profile is specified, the options
    // shall be compatible with that media profile. If no tokens are specified,
    // the options shall be considered generic for the device.
    #[yaserde(prefix = "tr2", rename = "Options")]
    pub options: tt::AudioOutputConfigurationOptions,
}

impl Validate for GetAudioOutputConfigurationOptionsResponse {}

pub type GetAudioDecoderConfigurationOptions = GetConfiguration;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetAudioDecoderConfigurationOptionsResponse {
    // This message contains the audio decoder configuration options. If a audio
    // decoder configuration is specified, the options shall concern that
    // particular configuration. If a media profile is specified, the options
    // shall be compatible with that media profile. If no tokens are specified,
    // the options shall be considered generic for the device.
    #[yaserde(prefix = "tr2", rename = "Options")]
    pub options: Vec<tt::AudioEncoder2ConfigurationOptions>,
}

impl Validate for GetAudioDecoderConfigurationOptionsResponse {}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum TransportProtocol {
    RtspUnicast,
    RtspMulticast,
    #[yaserde(rename = "RTSP")]
    Rtsp,
    RtspOverHttp,
    __Unknown__(String),
}

impl Default for TransportProtocol {
    fn default() -> TransportProtocol {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for TransportProtocol {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetVideoEncoderInstances {
    // Token of the video source configuration
    #[yaserde(prefix = "tr2", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetVideoEncoderInstances {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct EncoderInstance {
    // Video Media Subtype for the video format. For definitions see
    // tt:VideoEncodingMimeNames and
    #[yaserde(prefix = "tr2", rename = "Encoding")]
    pub encoding: String,

    // The minimum guaranteed number of encoder instances (applications) for the
    // VideoSourceConfiguration.
    #[yaserde(prefix = "tr2", rename = "Number")]
    pub number: i32,
}

impl Validate for EncoderInstance {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct EncoderInstanceInfo {
    // If a device limits the number of instances for respective Video Codecs
    // the response contains the information how many streams can be set up at
    // the same time per VideoSource.
    #[yaserde(prefix = "tr2", rename = "Codec")]
    pub codec: Vec<EncoderInstance>,

    // The minimum guaranteed total number of encoder instances (applications)
    // per VideoSourceConfiguration. The device is able to deliver the Total
    // number of streams
    #[yaserde(prefix = "tr2", rename = "Total")]
    pub total: i32,
}

impl Validate for EncoderInstanceInfo {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetVideoEncoderInstancesResponse {
    // The minimum guaranteed total number of encoder instances (applications)
    // per VideoSourceConfiguration.
    #[yaserde(prefix = "tr2", rename = "Info")]
    pub info: EncoderInstanceInfo,
}

impl Validate for GetVideoEncoderInstancesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetStreamUri {
    // Defines the network protocol for streaming as defined by
    // tr2:TransportProtocol
    #[yaserde(prefix = "tr2", rename = "Protocol")]
    pub protocol: String,

    // The ProfileToken element indicates the media profile to use and will
    // define the configuration of the content of the stream.
    #[yaserde(prefix = "tr2", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetStreamUri {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetStreamUriResponse {
    // Stable Uri to be used for requesting the media stream
    #[yaserde(prefix = "tr2", rename = "Uri")]
    pub uri: String,
}

impl Validate for GetStreamUriResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct SetSynchronizationPoint {
    // Contains a Profile reference for which a Synchronization Point is
    // requested.
    #[yaserde(prefix = "tr2", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for SetSynchronizationPoint {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct SetSynchronizationPointResponse {}

impl Validate for SetSynchronizationPointResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetSnapshotUri {
    // The ProfileToken element indicates the media profile to use and will
    // define the source and dimensions of the snapshot.
    #[yaserde(prefix = "tr2", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for GetSnapshotUri {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetSnapshotUriResponse {
    // Stable Uri to be used for requesting snapshot images.
    #[yaserde(prefix = "tr2", rename = "Uri")]
    pub uri: String,
}

impl Validate for GetSnapshotUriResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct StartStopMulticastStreaming {
    // Contains the token of the Profile that is used to define the multicast
    // stream.
    #[yaserde(prefix = "tr2", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for StartStopMulticastStreaming {}

pub type StartMulticastStreaming = StartStopMulticastStreaming;
pub type StartMulticastStreamingResponse = SetConfigurationResponse;
pub type StopMulticastStreaming = StartStopMulticastStreaming;
pub type StopMulticastStreamingResponse = SetConfigurationResponse;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetVideoSourceModes {
    // Contains a video source reference for which a video source mode is
    // requested.
    #[yaserde(prefix = "tr2", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,
}

impl Validate for GetVideoSourceModes {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetVideoSourceModesResponse {
    // Return the information for specified video source mode.
    #[yaserde(prefix = "tr2", rename = "VideoSourceModes")]
    pub video_source_modes: Vec<VideoSourceMode>,
}

impl Validate for GetVideoSourceModesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct SetVideoSourceMode {
    // Contains a video source reference for which a video source mode is
    // requested.
    #[yaserde(prefix = "tr2", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,

    // Indicate video source mode.
    #[yaserde(prefix = "tr2", rename = "VideoSourceModeToken")]
    pub video_source_mode_token: tt::ReferenceToken,
}

impl Validate for SetVideoSourceMode {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct SetVideoSourceModeResponse {
    // The response contains information about rebooting after returning
    // response. When Reboot is set true, a device will reboot automatically
    // after setting mode.
    #[yaserde(prefix = "tr2", rename = "Reboot")]
    pub reboot: bool,
}

impl Validate for SetVideoSourceModeResponse {}

// List of one or more encodings supported for this video source. For name
// definitions see tt:VideoEncodingMimeNames, and see

#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct EncodingTypes(pub Vec<String>);

impl Validate for EncodingTypes {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct VideoSourceMode {
    // Max frame rate in frames per second for this video source mode.
    #[yaserde(prefix = "tr2", rename = "MaxFramerate")]
    pub max_framerate: f64,

    // Max horizontal and vertical resolution for this video source mode.
    #[yaserde(prefix = "tr2", rename = "MaxResolution")]
    pub max_resolution: tt::VideoResolution,

    // List of one or more encodings supported for this video source. For name
    // definitions see tt:VideoEncodingMimeNames, and see
    #[yaserde(prefix = "tr2", rename = "Encodings")]
    pub encodings: EncodingTypes,

    // After setting the mode if a device starts to reboot this value is true.
    // If a device change the mode without rebooting this value is false. If
    // true, configured parameters may not be guaranteed by the device after
    // rebooting.
    #[yaserde(prefix = "tr2", rename = "Reboot")]
    pub reboot: bool,

    // Informative description of this video source mode. This field should be
    // described in English.
    #[yaserde(prefix = "tr2", rename = "Description")]
    pub description: Option<tt::Description>,

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
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetOSDs {
    // The GetOSDs command fetches the OSD configuration if the OSD token is
    // known.
    #[yaserde(prefix = "tr2", rename = "OSDToken")]
    pub osd_token: Option<tt::ReferenceToken>,

    // Token of the Video Source Configuration, which has OSDs associated with
    // are requested. If token not exist, request all available OSDs.
    #[yaserde(prefix = "tr2", rename = "ConfigurationToken")]
    pub configuration_token: Option<tt::ReferenceToken>,
}

impl Validate for GetOSDs {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetOSDsResponse {
    // This element contains a list of requested OSDs.
    #[yaserde(prefix = "tr2", rename = "OSDs")]
    pub os_ds: Vec<tt::Osdconfiguration>,
}

impl Validate for GetOSDsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct SetOSD {
    // Contains the modified OSD configuration.
    #[yaserde(prefix = "tr2", rename = "OSD")]
    pub osd: tt::Osdconfiguration,
}

impl Validate for SetOSD {}

pub type SetOSDResponse = SetConfigurationResponse;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetOSDOptions {
    // Video Source Configuration Token that specifies an existing video source
    // configuration that the options shall be compatible with.
    #[yaserde(prefix = "tr2", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetOSDOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetOSDOptionsResponse {
    #[yaserde(prefix = "tr2", rename = "OSDOptions")]
    pub osd_options: tt::OsdconfigurationOptions,
}

impl Validate for GetOSDOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct CreateOSD {
    // Contain the initial OSD configuration for create.
    #[yaserde(prefix = "tr2", rename = "OSD")]
    pub osd: tt::Osdconfiguration,
}

impl Validate for CreateOSD {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct CreateOSDResponse {
    // Returns Token of the newly created OSD
    #[yaserde(prefix = "tr2", rename = "OSDToken")]
    pub osd_token: tt::ReferenceToken,
}

impl Validate for CreateOSDResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct DeleteOSD {
    // This element contains a reference to the OSD configuration that should be
    // deleted.
    #[yaserde(prefix = "tr2", rename = "OSDToken")]
    pub osd_token: tt::ReferenceToken,
}

impl Validate for DeleteOSD {}

pub type DeleteOSDResponse = SetConfigurationResponse;

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum MaskType {
    Color,
    Pixelated,
    Blurred,
    __Unknown__(String),
}

impl Default for MaskType {
    fn default() -> MaskType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for MaskType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct Mask {
    // Token of the VideoSourceConfiguration the Mask is associated with.
    #[yaserde(prefix = "tr2", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,

    // Geometric representation of the mask area.
    #[yaserde(prefix = "tr2", rename = "Polygon")]
    pub polygon: tt::Polygon,

    // Type of masking as defined by tr2:MaskType:
    #[yaserde(prefix = "tr2", rename = "Type")]
    pub _type: String,

    // Color of the masked area.
    #[yaserde(prefix = "tr2", rename = "Color")]
    pub color: Option<tt::Color>,

    // If set the mask will cover the image, otherwise it will be fully
    // transparent.
    #[yaserde(prefix = "tr2", rename = "Enabled")]
    pub enabled: bool,

    // Token of the mask.
    #[yaserde(attribute, rename = "token")]
    pub token: Option<tt::ReferenceToken>,
}

impl Validate for Mask {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetMasks {
    // Optional mask token of an existing mask.
    #[yaserde(prefix = "tr2", rename = "Token")]
    pub token: Option<tt::ReferenceToken>,

    // Optional token of a Video Source Configuration.
    #[yaserde(prefix = "tr2", rename = "ConfigurationToken")]
    pub configuration_token: Option<tt::ReferenceToken>,
}

impl Validate for GetMasks {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetMasksResponse {
    // List of Mask configurations.
    #[yaserde(prefix = "tr2", rename = "Masks")]
    pub masks: Vec<Mask>,
}

impl Validate for GetMasksResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct SetMask {
    // Mask to be updated.
    #[yaserde(prefix = "tr2", rename = "Mask")]
    pub mask: Mask,
}

impl Validate for SetMask {}

pub type SetMaskResponse = SetConfigurationResponse;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetMaskOptions {
    // Video Source Configuration Token that specifies an existing video source
    // configuration that the options shall be compatible with.
    #[yaserde(prefix = "tr2", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetMaskOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct MaskOptions {
    // Maximum supported number of masks per VideoSourceConfiguration.
    #[yaserde(prefix = "tr2", rename = "MaxMasks")]
    pub max_masks: i32,

    // Maximum supported number of points per mask.
    #[yaserde(prefix = "tr2", rename = "MaxPoints")]
    pub max_points: i32,

    // Information which types of tr2:MaskType are supported. Valid values are
    // 'Color', 'Pixelated' and 'Blurred'.
    #[yaserde(prefix = "tr2", rename = "Types")]
    pub types: Vec<String>,

    // Colors supported.
    #[yaserde(prefix = "tr2", rename = "Color")]
    pub color: tt::ColorOptions,

    // Information whether the polygon must have four points and a rectangular
    // shape.
    #[yaserde(attribute, rename = "RectangleOnly")]
    pub rectangle_only: Option<bool>,

    // Indicates the device capability of change in color of privacy mask for
    // one video source configuration will automatically be applied to all the
    // privacy masks associated with the same video source configuration.
    #[yaserde(attribute, rename = "SingleColorOnly")]
    pub single_color_only: Option<bool>,
}

impl Validate for MaskOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct GetMaskOptionsResponse {
    #[yaserde(prefix = "tr2", rename = "Options")]
    pub options: MaskOptions,
}

impl Validate for GetMaskOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct CreateMask {
    // Contain the initial mask configuration for create.
    #[yaserde(prefix = "tr2", rename = "Mask")]
    pub mask: Mask,
}

impl Validate for CreateMask {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct CreateMaskResponse {
    // Returns Token of the newly created Mask
    #[yaserde(prefix = "tr2", rename = "Token")]
    pub token: tt::ReferenceToken,
}

impl Validate for CreateMaskResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tr2",
    namespace = "tr2: http://www.onvif.org/ver20/media/wsdl"
)]
pub struct DeleteMask {
    // This element contains a reference to the Mask configuration that should
    // be deleted.
    #[yaserde(prefix = "tr2", rename = "Token")]
    pub token: tt::ReferenceToken,
}

impl Validate for DeleteMask {}

pub type DeleteMaskResponse = SetConfigurationResponse;

// Returns the capabilities of the media service. The result is returned in a
// typed answer.
pub async fn get_service_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetServiceCapabilities,
) -> Result<GetServiceCapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation creates a new media profile.
// A created profile created via this method may be deleted via the
// DeleteProfile method.
// Optionally Configurations can be assinged to the profile on creation. For
// details regarding profile assignement
// check also the method AddConfiguration.
pub async fn create_profile<T: transport::Transport>(
    transport: &T,
    request: &CreateProfile,
) -> Result<CreateProfileResponse, transport::Error> {
    transport::request(transport, request).await
}

// Retrieve the profile with the specified token or all defined media profiles.
pub async fn get_profiles<T: transport::Transport>(
    transport: &T,
    request: &GetProfiles,
) -> Result<GetProfilesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation adds one or more Configurations to an existing media profile.
// If a
// configuration exists in the media profile, it will be replaced. A device
// shall
// support adding a compatible Configuration to a Profile containing a
// VideoSourceConfiguration and shall
// support streaming video data of such a profile.
pub async fn add_configuration<T: transport::Transport>(
    transport: &T,
    request: &AddConfiguration,
) -> Result<AddConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation removes the listed configurations from an existing media
// profile. If the
// media profile does not contain one of the listed configurations that item
// shall be ignored.
pub async fn remove_configuration<T: transport::Transport>(
    transport: &T,
    request: &RemoveConfiguration,
) -> Result<RemoveConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation deletes a profile. Deletion of a profile is only possible for
// non-fixed profiles
pub async fn delete_profile<T: transport::Transport>(
    transport: &T,
    request: &DeleteProfile,
) -> Result<DeleteProfileResponse, transport::Error> {
    transport::request(transport, request).await
}

// By default this operation lists all existing video source configurations for
// a device. Provide a profile token to list only configurations that are
// compatible with the profile. If a configuration token is provided only a
// single configuration will be returned.
pub async fn get_video_source_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetVideoSourceConfigurations,
) -> Result<GetVideoSourceConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// By default this operation lists all existing video encoder configurations for
// a device. Provide a profile token to list only configurations that are
// compatible with the profile. If a configuration token is provided only a
// single configuration will be returned.
pub async fn get_video_encoder_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetVideoEncoderConfigurations,
) -> Result<GetVideoEncoderConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// By default this operation lists all existing audio source configurations for
// a device. Provide a profile token to list only configurations that are
// compatible with the profile. If a configuration token is provided only a
// single configuration will be returned.
pub async fn get_audio_source_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetAudioSourceConfigurations,
) -> Result<GetAudioSourceConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// By default this operation lists all existing audio encoder configurations for
// a device. Provide a profile token to list only configurations that are
// compatible with the profile. If a configuration token is provided only a
// single configuration will be returned.
pub async fn get_audio_encoder_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetAudioEncoderConfigurations,
) -> Result<GetAudioEncoderConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// By default this operation lists all existing video analytics configurations
// for a device. Provide a profile token to list only configurations that are
// compatible with the profile. If a configuration token is provided only a
// single configuration will be returned.
pub async fn get_analytics_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetAnalyticsConfigurations,
) -> Result<GetAnalyticsConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// By default this operation lists all existing metadata configurations for a
// device. Provide a profile token to list only configurations that are
// compatible with the profile. If a configuration token is provided only a
// single configuration will be returned.
pub async fn get_metadata_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetMetadataConfigurations,
) -> Result<GetMetadataConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// By default this operation lists all existing audio output configurations for
// a device. Provide a profile token to list only configurations that are
// compatible with the profile. If a configuration token is provided only a
// single configuration will be returned.
pub async fn get_audio_output_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetAudioOutputConfigurations,
) -> Result<GetAudioOutputConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// By default this operation lists all existing audio decoder configurations for
// a device. Provide a profile token to list only configurations that are
// compatible with the profile. If a configuration token is provided only a
// single configuration will be returned.
pub async fn get_audio_decoder_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetAudioDecoderConfigurations,
) -> Result<GetAudioDecoderConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation modifies a video source configuration. Running streams using
// this configuration may be immediately updated according to the new settings.
// The changes are not guaranteed to take effect unless the client requests a
// new stream URI and restarts any affected stream. NVC methods for changing a
// running stream are out of scope for this specification.
pub async fn set_video_source_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetVideoSourceConfiguration,
) -> Result<SetVideoSourceConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation modifies a video encoder configuration. Running streams using
// this configuration may be immediately updated according to the new settings.
// The changes are not guaranteed to take effect unless the client requests a
// new stream URI and restarts any affected stream. NVC methods for changing a
// running stream are out of scope for this specification.
pub async fn set_video_encoder_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetVideoEncoderConfiguration,
) -> Result<SetVideoEncoderConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation modifies an audio source configuration. Running streams using
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

// This operation modifies an audio encoder configuration. Running streams using
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

// This operation modifies a metadata configuration. Running streams using this
// configuration may be updated immediately
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

// This operation modifies an audio output configuration.
pub async fn set_audio_output_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetAudioOutputConfiguration,
) -> Result<SetAudioOutputConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation modifies an audio decoder configuration.
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

// The GetVideoEncoderInstances command can be used to request the
// minimum number of guaranteed video encoder instances (applications) per Video
// Source
// Configuration.
pub async fn get_video_encoder_instances<T: transport::Transport>(
    transport: &T,
    request: &GetVideoEncoderInstances,
) -> Result<GetVideoEncoderInstancesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a URI that can be used to initiate a live media
// stream using RTSP as
// the control protocol. The returned URI shall remain valid indefinitely even
// if the profile is changed.
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

// This command stops multicast streaming using a specified media profile of a
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

// This operation lists existing OSD configurations for the device.
pub async fn get_os_ds<T: transport::Transport>(
    transport: &T,
    request: &GetOSDs,
) -> Result<GetOSDsResponse, transport::Error> {
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

// This operation lists existing Mask configurations for the device.
pub async fn get_masks<T: transport::Transport>(
    transport: &T,
    request: &GetMasks,
) -> Result<GetMasksResponse, transport::Error> {
    transport::request(transport, request).await
}

// Get the Mask Options.
pub async fn get_mask_options<T: transport::Transport>(
    transport: &T,
    request: &GetMaskOptions,
) -> Result<GetMaskOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// Set the Mask
pub async fn set_mask<T: transport::Transport>(
    transport: &T,
    request: &SetMask,
) -> Result<SetMaskResponse, transport::Error> {
    transport::request(transport, request).await
}

// Create the Mask.
pub async fn create_mask<T: transport::Transport>(
    transport: &T,
    request: &CreateMask,
) -> Result<CreateMaskResponse, transport::Error> {
    transport::request(transport, request).await
}

// Delete the Mask.
pub async fn delete_mask<T: transport::Transport>(
    transport: &T,
    request: &DeleteMask,
) -> Result<DeleteMaskResponse, transport::Error> {
    transport::request(transport, request).await
}
