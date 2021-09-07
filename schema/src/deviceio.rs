use crate::transport;
use crate::validate::Validate;
use crate::{devicemgmt as tds, onvif as tt};
use std::str::FromStr;
use xsd_macro_utils::*;
use xsd_types::types as xs;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetServiceCapabilitiesResponse {
    // The capabilities for the device IO service is returned in the
    // Capabilities element.
    #[yaserde(prefix = "tmd", rename = "Capabilities")]
    pub capabilities: Capabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct Capabilities {
    // Number of video sources (defaults to none).
    #[yaserde(attribute, rename = "VideoSources")]
    pub video_sources: Option<i32>,

    // Number of video outputs (defaults to none).
    #[yaserde(attribute, rename = "VideoOutputs")]
    pub video_outputs: Option<i32>,

    // Number of audio sources (defaults to none).
    #[yaserde(attribute, rename = "AudioSources")]
    pub audio_sources: Option<i32>,

    // Number of audio outputs (defaults to none).
    #[yaserde(attribute, rename = "AudioOutputs")]
    pub audio_outputs: Option<i32>,

    // Number of relay outputs (defaults to none).
    #[yaserde(attribute, rename = "RelayOutputs")]
    pub relay_outputs: Option<i32>,

    // Number of serial ports (defaults to none).
    #[yaserde(attribute, rename = "SerialPorts")]
    pub serial_ports: Option<i32>,

    // Number of digital inputs (defaults to none).
    #[yaserde(attribute, rename = "DigitalInputs")]
    pub digital_inputs: Option<i32>,

    // Indicates support for DigitalInput configuration of the idle state
    // (defaults to false).
    #[yaserde(attribute, rename = "DigitalInputOptions")]
    pub digital_input_options: Option<bool>,
}

impl Validate for Capabilities {}

// pub type Capabilities = Capabilities;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetRelayOutputOptions {
    // Optional reference token to the relay for which the options are
    // requested.
    #[yaserde(prefix = "tmd", rename = "RelayOutputToken")]
    pub relay_output_token: Option<tt::ReferenceToken>,
}

impl Validate for GetRelayOutputOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetRelayOutputOptionsResponse {
    // Valid values and ranges for the configuration of a relay output.
    #[yaserde(prefix = "tmd", rename = "RelayOutputOptions")]
    pub relay_output_options: Vec<RelayOutputOptions>,
}

impl Validate for GetRelayOutputOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct RelayOutputOptions {
    // Supported Modes.
    #[yaserde(prefix = "tmd", rename = "Mode")]
    pub mode: Vec<tt::RelayMode>,

    // Supported delay time range or discrete values in seconds. This element
    // must be present if MonoStable mode is supported.
    #[yaserde(prefix = "tmd", rename = "DelayTimes")]
    pub delay_times: Option<DelayTimes>,

    // True if the relay only supports the exact values for the DelayTimes
    // listed. Default is false.
    #[yaserde(prefix = "tmd", rename = "Discrete")]
    pub discrete: Option<bool>,

    #[yaserde(prefix = "tmd", rename = "Extension")]
    pub extension: Option<RelayOutputOptionsExtension>,

    // Token of the relay output.
    #[yaserde(attribute, rename = "token")]
    pub token: tt::ReferenceToken,
}

impl Validate for RelayOutputOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct RelayOutputOptionsExtension {}

impl Validate for RelayOutputOptionsExtension {}

#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct DelayTimes(pub Vec<f64>);

impl Validate for DelayTimes {}
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct Get {}

impl Validate for Get {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetResponse {
    // List tokens of a physical IO of a device.
    #[yaserde(prefix = "tmd", rename = "Token")]
    pub token: Vec<tt::ReferenceToken>,
}

impl Validate for GetResponse {}

pub type GetVideoSources = Get;
pub type GetVideoSourcesResponse = GetResponse;
pub type GetAudioSources = Get;
pub type GetAudioSourcesResponse = GetResponse;
pub type GetAudioOutputs = Get;
pub type GetAudioOutputsResponse = GetResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetVideoOutputs {}

impl Validate for GetVideoOutputs {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetVideoOutputsResponse {
    // List containing all physical Video output connections of a device.
    #[yaserde(prefix = "tmd", rename = "VideoOutputs")]
    pub video_outputs: Vec<tt::VideoOutput>,
}

impl Validate for GetVideoOutputsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetAudioSourceConfiguration {
    // Token of the requested AudioSource.
    #[yaserde(prefix = "tmd", rename = "AudioSourceToken")]
    pub audio_source_token: tt::ReferenceToken,
}

impl Validate for GetAudioSourceConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetAudioSourceConfigurationResponse {
    // Current configuration of the Audio input.
    #[yaserde(prefix = "tmd", rename = "AudioSourceConfiguration")]
    pub audio_source_configuration: tt::AudioSourceConfiguration,
}

impl Validate for GetAudioSourceConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetAudioOutputConfiguration {
    // Token of the physical Audio output.
    #[yaserde(prefix = "tmd", rename = "AudioOutputToken")]
    pub audio_output_token: tt::ReferenceToken,
}

impl Validate for GetAudioOutputConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetAudioOutputConfigurationResponse {
    // Current configuration of the Audio output.
    #[yaserde(prefix = "tmd", rename = "AudioOutputConfiguration")]
    pub audio_output_configuration: tt::AudioOutputConfiguration,
}

impl Validate for GetAudioOutputConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetVideoSourceConfiguration {
    // Token of the requested VideoSource.
    #[yaserde(prefix = "tmd", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,
}

impl Validate for GetVideoSourceConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetVideoSourceConfigurationResponse {
    // Current configuration of the Video input.
    #[yaserde(prefix = "tmd", rename = "VideoSourceConfiguration")]
    pub video_source_configuration: tt::VideoSourceConfiguration,
}

impl Validate for GetVideoSourceConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetVideoOutputConfiguration {
    // Token of the requested VideoOutput.
    #[yaserde(prefix = "tmd", rename = "VideoOutputToken")]
    pub video_output_token: tt::ReferenceToken,
}

impl Validate for GetVideoOutputConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetVideoOutputConfigurationResponse {
    // Current configuration of the Video output.
    #[yaserde(prefix = "tmd", rename = "VideoOutputConfiguration")]
    pub video_output_configuration: tt::VideoOutputConfiguration,
}

impl Validate for GetVideoOutputConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct SetAudioSourceConfiguration {
    #[yaserde(prefix = "tmd", rename = "Configuration")]
    pub configuration: tt::AudioSourceConfiguration,

    // The ForcePersistence element determines how configuration
    // changes shall be stored. If true, changes shall be persistent. If false,
    // changes MAY revert to previous values
    // after reboot.
    #[yaserde(prefix = "tmd", rename = "ForcePersistence")]
    pub force_persistence: bool,
}

impl Validate for SetAudioSourceConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct SetAudioSourceConfigurationResponse {}

impl Validate for SetAudioSourceConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct SetAudioOutputConfiguration {
    #[yaserde(prefix = "tmd", rename = "Configuration")]
    pub configuration: tt::AudioOutputConfiguration,

    // The ForcePersistence element determines how configuration
    // changes shall be stored. If true, changes shall be persistent. If false,
    // changes MAY revert to previous values
    // after reboot.
    #[yaserde(prefix = "tmd", rename = "ForcePersistence")]
    pub force_persistence: bool,
}

impl Validate for SetAudioOutputConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct SetAudioOutputConfigurationResponse {}

impl Validate for SetAudioOutputConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct SetVideoSourceConfiguration {
    #[yaserde(prefix = "tmd", rename = "Configuration")]
    pub configuration: tt::VideoSourceConfiguration,

    // The ForcePersistence element determines how configuration
    // changes shall be stored. If true, changes shall be persistent. If false,
    // changes MAY revert to previous values
    // after reboot.
    #[yaserde(prefix = "tmd", rename = "ForcePersistence")]
    pub force_persistence: bool,
}

impl Validate for SetVideoSourceConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct SetVideoSourceConfigurationResponse {}

impl Validate for SetVideoSourceConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct SetVideoOutputConfiguration {
    #[yaserde(prefix = "tmd", rename = "Configuration")]
    pub configuration: tt::VideoOutputConfiguration,

    // The ForcePersistence element determines how configuration
    // changes shall be stored. If true, changes shall be persistent. If false,
    // changes MAY revert to previous values
    // after reboot.
    #[yaserde(prefix = "tmd", rename = "ForcePersistence")]
    pub force_persistence: bool,
}

impl Validate for SetVideoOutputConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct SetVideoOutputConfigurationResponse {}

impl Validate for SetVideoOutputConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetVideoSourceConfigurationOptions {
    // Token of the Video input whose options are requested..
    #[yaserde(prefix = "tmd", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,
}

impl Validate for GetVideoSourceConfigurationOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetVideoSourceConfigurationOptionsResponse {
    #[yaserde(prefix = "tmd", rename = "VideoSourceConfigurationOptions")]
    pub video_source_configuration_options: tt::VideoSourceConfigurationOptions,
}

impl Validate for GetVideoSourceConfigurationOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetVideoOutputConfigurationOptions {
    // Token of the Video Output whose options are requested..
    #[yaserde(prefix = "tmd", rename = "VideoOutputToken")]
    pub video_output_token: tt::ReferenceToken,
}

impl Validate for GetVideoOutputConfigurationOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetVideoOutputConfigurationOptionsResponse {
    #[yaserde(prefix = "tmd", rename = "VideoOutputConfigurationOptions")]
    pub video_output_configuration_options: tt::VideoOutputConfigurationOptions,
}

impl Validate for GetVideoOutputConfigurationOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetAudioSourceConfigurationOptions {
    // Token of the physical Audio input whose options are requested..
    #[yaserde(prefix = "tmd", rename = "AudioSourceToken")]
    pub audio_source_token: tt::ReferenceToken,
}

impl Validate for GetAudioSourceConfigurationOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetAudioSourceConfigurationOptionsResponse {
    // Returns the AudioSourceToken available.
    #[yaserde(prefix = "tmd", rename = "AudioSourceOptions")]
    pub audio_source_options: tt::AudioSourceConfigurationOptions,
}

impl Validate for GetAudioSourceConfigurationOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetAudioOutputConfigurationOptions {
    // Token of the physical Audio Output whose options are requested..
    #[yaserde(prefix = "tmd", rename = "AudioOutputToken")]
    pub audio_output_token: tt::ReferenceToken,
}

impl Validate for GetAudioOutputConfigurationOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetAudioOutputConfigurationOptionsResponse {
    // Available settings and ranges for the requested Audio output.
    #[yaserde(prefix = "tmd", rename = "AudioOutputOptions")]
    pub audio_output_options: tt::AudioOutputConfigurationOptions,
}

impl Validate for GetAudioOutputConfigurationOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct SetRelayOutputSettings {
    #[yaserde(prefix = "tmd", rename = "RelayOutput")]
    pub relay_output: tt::RelayOutput,
}

impl Validate for SetRelayOutputSettings {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct SetRelayOutputSettingsResponse {}

impl Validate for SetRelayOutputSettingsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetDigitalInputs {}

impl Validate for GetDigitalInputs {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetDigitalInputsResponse {
    #[yaserde(prefix = "tmd", rename = "DigitalInputs")]
    pub digital_inputs: Vec<tt::DigitalInput>,
}

impl Validate for GetDigitalInputsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct DigitalInputConfigurationOptions {
    // Configuration Options for a digital input.
    #[yaserde(prefix = "tmd", rename = "IdleState")]
    pub idle_state: Vec<tt::DigitalIdleState>,
}

impl Validate for DigitalInputConfigurationOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetDigitalInputConfigurationOptions {
    #[yaserde(prefix = "tmd", rename = "Token")]
    pub token: Option<tt::ReferenceToken>,
}

impl Validate for GetDigitalInputConfigurationOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetDigitalInputConfigurationOptionsResponse {
    #[yaserde(prefix = "tmd", rename = "DigitalInputOptions")]
    pub digital_input_options: DigitalInputConfigurationOptions,
}

impl Validate for GetDigitalInputConfigurationOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct SetDigitalInputConfigurations {
    #[yaserde(prefix = "tmd", rename = "DigitalInputs")]
    pub digital_inputs: Vec<tt::DigitalInput>,
}

impl Validate for SetDigitalInputConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct SetDigitalInputConfigurationsResponse {}

impl Validate for SetDigitalInputConfigurationsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetSerialPorts {}

impl Validate for GetSerialPorts {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetSerialPortsResponse {
    #[yaserde(prefix = "tmd", rename = "SerialPort")]
    pub serial_port: Vec<SerialPort>,
}

impl Validate for GetSerialPortsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetSerialPortConfiguration {
    #[yaserde(prefix = "tmd", rename = "SerialPortToken")]
    pub serial_port_token: tt::ReferenceToken,
}

impl Validate for GetSerialPortConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetSerialPortConfigurationResponse {
    #[yaserde(prefix = "tmd", rename = "SerialPortConfiguration")]
    pub serial_port_configuration: SerialPortConfiguration,
}

impl Validate for GetSerialPortConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct SetSerialPortConfiguration {
    #[yaserde(prefix = "tmd", rename = "SerialPortConfiguration")]
    pub serial_port_configuration: SerialPortConfiguration,

    #[yaserde(prefix = "tmd", rename = "ForcePersistance")]
    pub force_persistance: bool,
}

impl Validate for SetSerialPortConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct SetSerialPortConfigurationResponse {}

impl Validate for SetSerialPortConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetSerialPortConfigurationOptions {
    #[yaserde(prefix = "tmd", rename = "SerialPortToken")]
    pub serial_port_token: tt::ReferenceToken,
}

impl Validate for GetSerialPortConfigurationOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct GetSerialPortConfigurationOptionsResponse {
    #[yaserde(prefix = "tmd", rename = "SerialPortOptions")]
    pub serial_port_options: SerialPortConfigurationOptions,
}

impl Validate for GetSerialPortConfigurationOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct SendReceiveSerialCommand {
    // The physical serial port reference to be used when this request is
    // invoked.
    #[yaserde(prefix = "tmd", rename = "Token")]
    pub token: Option<tt::ReferenceToken>,

    // The serial port data.
    #[yaserde(prefix = "tmd", rename = "SerialData")]
    pub serial_data: Option<SerialData>,

    // Indicates that the command should be responded back within the specified
    // period of time.
    #[yaserde(prefix = "tmd", rename = "TimeOut")]
    pub time_out: Option<xs::Duration>,

    // This element may be put in the case that data length returned from the
    // connected serial device is already determined as some fixed bytes length.
    // It indicates the length of received data which can be regarded as
    // available.
    #[yaserde(prefix = "tmd", rename = "DataLength")]
    pub data_length: Option<xs::Integer>,

    // This element may be put in the case that the delimiter codes returned
    // from the connected serial device is already known. It indicates the
    // termination data sequence of the responded data. In case the string has
    // more than one character a device shall interpret the whole string as a
    // single delimiter. Furthermore a device shall return the delimiter
    // character(s) to the client.
    #[yaserde(prefix = "tmd", rename = "Delimiter")]
    pub delimiter: Option<String>,
}

impl Validate for SendReceiveSerialCommand {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct SendReceiveSerialCommandResponse {
    #[yaserde(prefix = "tmd", rename = "SerialData")]
    pub serial_data: Option<SerialData>,
}

impl Validate for SendReceiveSerialCommandResponse {}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum SerialDataChoice {
    Binary(String),
    String(String),
    __Unknown__(String),
}

impl Default for SerialDataChoice {
    fn default() -> SerialDataChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for SerialDataChoice {}

// The serial port data.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct SerialData {
    #[yaserde(flatten)]
    pub serial_data_choice: SerialDataChoice,
}

impl Validate for SerialData {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct SerialPort {
    pub base: tt::DeviceEntity,
}

impl Validate for SerialPort {}

// The type of serial port.Generic can be signaled as a vendor specific serial
// port type.
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum SerialPortType {
    #[yaserde(rename = "RS232")]
    Rs232,
    #[yaserde(rename = "RS422HalfDuplex")]
    Rs422HalfDuplex,
    #[yaserde(rename = "RS422FullDuplex")]
    Rs422FullDuplex,
    #[yaserde(rename = "RS485HalfDuplex")]
    Rs485HalfDuplex,
    #[yaserde(rename = "RS485FullDuplex")]
    Rs485FullDuplex,
    Generic,
    __Unknown__(String),
}

impl Default for SerialPortType {
    fn default() -> SerialPortType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for SerialPortType {}

// The parameters for configuring the serial port.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct SerialPortConfiguration {
    // The transfer bitrate.
    #[yaserde(prefix = "tmd", rename = "BaudRate")]
    pub baud_rate: i32,

    // The parity for the data error detection.
    #[yaserde(prefix = "tmd", rename = "ParityBit")]
    pub parity_bit: ParityBit,

    // The bit length for each character.
    #[yaserde(prefix = "tmd", rename = "CharacterLength")]
    pub character_length: i32,

    // The number of stop bits used to terminate each character.
    #[yaserde(prefix = "tmd", rename = "StopBit")]
    pub stop_bit: f64,

    #[yaserde(attribute, rename = "token")]
    pub token: tt::ReferenceToken,

    #[yaserde(attribute, rename = "type")]
    pub _type: SerialPortType,
}

impl Validate for SerialPortConfiguration {}

// The parity for the data error detection.
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum ParityBit {
    None,
    Even,
    Odd,
    Mark,
    Space,
    Extended,
    __Unknown__(String),
}

impl Default for ParityBit {
    fn default() -> ParityBit {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for ParityBit {}

// The configuration options that relates to serial port.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct SerialPortConfigurationOptions {
    // The list of configurable transfer bitrate.
    #[yaserde(prefix = "tmd", rename = "BaudRateList")]
    pub baud_rate_list: tt::IntList,

    // The list of configurable parity for the data error detection.
    #[yaserde(prefix = "tmd", rename = "ParityBitList")]
    pub parity_bit_list: ParityBitList,

    // The list of configurable bit length for each character.
    #[yaserde(prefix = "tmd", rename = "CharacterLengthList")]
    pub character_length_list: tt::IntList,

    // The list of configurable number of stop bits used to terminate each
    // character.
    #[yaserde(prefix = "tmd", rename = "StopBitList")]
    pub stop_bit_list: tt::FloatList,

    #[yaserde(attribute, rename = "token")]
    pub token: tt::ReferenceToken,
}

impl Validate for SerialPortConfigurationOptions {}

// The list of configurable parity for the data error detection.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tmd",
    namespace = "tmd: http://www.onvif.org/ver10/deviceIO/wsdl"
)]
pub struct ParityBitList {
    #[yaserde(prefix = "tmd", rename = "Items")]
    pub items: Vec<ParityBit>,
}

impl Validate for ParityBitList {}

// Returns the capabilities of the device IO service. The result is returned in
// a typed answer.
pub async fn get_service_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetServiceCapabilities,
) -> Result<GetServiceCapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// Request the available settings and ranges for one or all relay outputs. A
// device that has one or more RelayOutputs should support this command.
pub async fn get_relay_output_options<T: transport::Transport>(
    transport: &T,
    request: &GetRelayOutputOptions,
) -> Result<GetRelayOutputOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// List all available audio sources for the device. The device that has one or
// more audio sources shall support the listing of available audio inputs
// through the GetAudioSources command.
pub async fn get_audio_sources<T: transport::Transport>(
    transport: &T,
    request: &GetAudioSources,
) -> Result<GetAudioSourcesResponse, transport::Error> {
    transport::request(transport, request).await
}

// List all available audio outputs of a device. A device that has one ore more
// physical audio outputs shall support listing of available audio outputs
// through the GetAudioOutputs command.
pub async fn get_audio_outputs<T: transport::Transport>(
    transport: &T,
    request: &GetAudioOutputs,
) -> Result<GetAudioOutputsResponse, transport::Error> {
    transport::request(transport, request).await
}

// List all available video sources for the device. The device that has one or
// more video inputs shall support the listing of available video sources
// through the GetVideoSources command.
pub async fn get_video_sources<T: transport::Transport>(
    transport: &T,
    request: &GetVideoSources,
) -> Result<GetVideoSourcesResponse, transport::Error> {
    transport::request(transport, request).await
}

// List all available video outputs of a device. A device that has one or more
// physical video outputs shall support listing of available video outputs
// through the GetVideoOutputs command.
pub async fn get_video_outputs<T: transport::Transport>(
    transport: &T,
    request: &GetVideoOutputs,
) -> Result<GetVideoOutputsResponse, transport::Error> {
    transport::request(transport, request).await
}

// Get the video source configurations of a VideoSource. A device with one or
// more video sources shall support the GetVideoSourceConfigurations command.
pub async fn get_video_source_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetVideoSourceConfiguration,
) -> Result<GetVideoSourceConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// Get the configuration of a Video Output. A device that has one or more Video
// Outputs shall support the retrieval of the VideoOutputConfiguration through
// this command.
pub async fn get_video_output_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetVideoOutputConfiguration,
) -> Result<GetVideoOutputConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// List the configuration of an Audio Input. A device with one or more audio
// inputs shall support the GetAudioSourceConfiguration command.
pub async fn get_audio_source_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetAudioSourceConfiguration,
) -> Result<GetAudioSourceConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// Request the current configuration of a physical Audio output. A device that
// has one or more AudioOutputs shall support the retrieval of the
// AudioOutputConfiguration through this command.
pub async fn get_audio_output_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetAudioOutputConfiguration,
) -> Result<GetAudioOutputConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// Modify a video input configuration. A device that has one or more video
// sources shall support the setting of the VideoSourceConfiguration through
// this command.
pub async fn set_video_source_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetVideoSourceConfiguration,
) -> Result<SetVideoSourceConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// Modify a video output configuration. A device that has one or more video
// outputs shall support the setting of its video output configuration through
// this command.
pub async fn set_video_output_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetVideoOutputConfiguration,
) -> Result<SetVideoOutputConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// Modify an audio source configuration. A device that has a one or more audio
// sources shall support the setting of the AudioSourceConfiguration through
// this command.
pub async fn set_audio_source_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetAudioSourceConfiguration,
) -> Result<SetAudioSourceConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// Modify an audio output configuration. A device that has one ore more audio
// outputs shall support the setting of the AudioOutputConfiguration through
// this command.
pub async fn set_audio_output_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetAudioOutputConfiguration,
) -> Result<SetAudioOutputConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// Request the VideoSourceConfigurationOptions of a VideoSource. A device with
// one or more video sources shall support this command.
pub async fn get_video_source_configuration_options<T: transport::Transport>(
    transport: &T,
    request: &GetVideoSourceConfigurationOptions,
) -> Result<GetVideoSourceConfigurationOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// Request the VideoOutputConfigurationOptions of a VideoOutput. A device that
// has one or more video outputs shall support the retrieval of
// VideoOutputConfigurationOptions through this command.
pub async fn get_video_output_configuration_options<T: transport::Transport>(
    transport: &T,
    request: &GetVideoOutputConfigurationOptions,
) -> Result<GetVideoOutputConfigurationOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// Request the AudioSourceConfigurationOptions of an AudioSource. A device with
// one ore more AudioSources shall support this command.
pub async fn get_audio_source_configuration_options<T: transport::Transport>(
    transport: &T,
    request: &GetAudioSourceConfigurationOptions,
) -> Result<GetAudioSourceConfigurationOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// Request the available settings and ranges for a physical Audio output. A
// device that has one or more AudioOutputs shall support this command.
pub async fn get_audio_output_configuration_options<T: transport::Transport>(
    transport: &T,
    request: &GetAudioOutputConfigurationOptions,
) -> Result<GetAudioOutputConfigurationOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation gets a list of all available relay outputs and their settings.
pub async fn get_relay_outputs<T: transport::Transport>(
    transport: &T,
    request: &tds::GetRelayOutputs,
) -> Result<tds::GetRelayOutputsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation sets the settings of a relay output.
// The relay can work in two relay modes:
pub async fn set_relay_output_settings<T: transport::Transport>(
    transport: &T,
    request: &SetRelayOutputSettings,
) -> Result<SetRelayOutputSettingsResponse, transport::Error> {
    transport::request(transport, request).await
}

// Modify the relay state.
pub async fn set_relay_output_state<T: transport::Transport>(
    transport: &T,
    request: &tds::SetRelayOutputState,
) -> Result<tds::SetRelayOutputStateResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation gets a list of all available digital inputs.
pub async fn get_digital_inputs<T: transport::Transport>(
    transport: &T,
    request: &GetDigitalInputs,
) -> Result<GetDigitalInputsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation lists what configuration is available for digital inputs.
pub async fn get_digital_input_configuration_options<T: transport::Transport>(
    transport: &T,
    request: &GetDigitalInputConfigurationOptions,
) -> Result<GetDigitalInputConfigurationOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// Modify a digital input configuration.
pub async fn set_digital_input_configurations<T: transport::Transport>(
    transport: &T,
    request: &SetDigitalInputConfigurations,
) -> Result<SetDigitalInputConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_serial_ports<T: transport::Transport>(
    transport: &T,
    request: &GetSerialPorts,
) -> Result<GetSerialPortsResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_serial_port_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetSerialPortConfiguration,
) -> Result<GetSerialPortConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn set_serial_port_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetSerialPortConfiguration,
) -> Result<SetSerialPortConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_serial_port_configuration_options<T: transport::Transport>(
    transport: &T,
    request: &GetSerialPortConfigurationOptions,
) -> Result<GetSerialPortConfigurationOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn send_receive_serial_command<T: transport::Transport>(
    transport: &T,
    request: &SendReceiveSerialCommand,
) -> Result<SendReceiveSerialCommandResponse, transport::Error> {
    transport::request(transport, request).await
}
