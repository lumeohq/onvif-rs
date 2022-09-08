#![allow(clippy::derive_partial_eq_without_eq)]

use onvif as tt;
use validate::Validate;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum Polarity {
    WhiteHot,
    BlackHot,
    __Unknown__(String),
}

impl Default for Polarity {
    fn default() -> Polarity {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for Polarity {}

// Describes standard Color Palette types, used to facilitate Multi-language
// support and client display.
// "Custom" Type shall be used when Color Palette Name does not match any of the
// types included in the standard classification.
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum ColorPaletteType {
    Custom,
    Grayscale,
    BlackHot,
    WhiteHot,
    Sepia,
    Red,
    Iron,
    Rain,
    Rainbow,
    Isotherm,
    __Unknown__(String),
}

impl Default for ColorPaletteType {
    fn default() -> ColorPaletteType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for ColorPaletteType {}

// Describes a Color Palette element.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct ColorPalette {
    // User readable Color Palette name.
    #[yaserde(prefix = "tth", rename = "Name")]
    pub name: tt::Name,

    // Unique identifier of this Color Palette.
    #[yaserde(attribute, rename = "token")]
    pub token: tt::ReferenceToken,

    // Indicates Color Palette Type. Use tth:ColorPaletteType.
    // Used for multi-language support and display.
    #[yaserde(attribute, rename = "Type")]
    pub _type: String,
}

impl Validate for ColorPalette {}

// Type describing a NUC Table element.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct Nuctable {
    // User reabable name for the Non-Uniformity Correction (NUC) Table.
    #[yaserde(prefix = "tth", rename = "Name")]
    pub name: tt::Name,

    // Unique identifier of this NUC Table.
    #[yaserde(attribute, rename = "token")]
    pub token: tt::ReferenceToken,

    // Low Temperature limit for application of NUC Table, in Kelvin.
    #[yaserde(attribute, rename = "LowTemperature")]
    pub low_temperature: Option<f64>,

    // High Temperature limit for application of NUC Table, in Kelvin.
    #[yaserde(attribute, rename = "HighTemperature")]
    pub high_temperature: Option<f64>,
}

impl Validate for Nuctable {}

// Type describing the Cooler settings.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct Cooler {
    // Indicates whether the Cooler is enabled (running) or not.
    #[yaserde(prefix = "tth", rename = "Enabled")]
    pub enabled: bool,

    // Number of hours the Cooler has been running (unit: hours). Read-only.
    #[yaserde(prefix = "tth", rename = "RunTime")]
    pub run_time: Option<f64>,
}

impl Validate for Cooler {}

// Describes valid ranges for the thermal device cooler settings.
// Only applicable to cooled thermal devices.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct CoolerOptions {
    // Indicates the Device allows cooler status to be changed from running
    // (Enabled) to stopped (Disabled), and viceversa.
    #[yaserde(prefix = "tth", rename = "Enabled")]
    pub enabled: Option<bool>,
}

impl Validate for CoolerOptions {}

// Holds default values that will be used in measurement modules when local
// parameters are not specified for the module (these are still required for
// valid temperature calculations).
// Having ReflectedAmbientTemperature, Emissivity and DistanceToObject as
// mandatory ensures minimum parameters are available to obtain valid
// temperature values.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct RadiometryGlobalParameters {
    // Reflected Ambient Temperature for the environment in which the thermal
    // device and the object being measured is located.
    #[yaserde(prefix = "tth", rename = "ReflectedAmbientTemperature")]
    pub reflected_ambient_temperature: f64,

    // Emissivity of the surface of the object on which temperature is being
    // measured.
    #[yaserde(prefix = "tth", rename = "Emissivity")]
    pub emissivity: f64,

    // Distance from the thermal device to the measured object.
    #[yaserde(prefix = "tth", rename = "DistanceToObject")]
    pub distance_to_object: f64,

    // Relative Humidity in the environment in which the measurement is located.
    #[yaserde(prefix = "tth", rename = "RelativeHumidity")]
    pub relative_humidity: Option<f64>,

    // Temperature of the atmosphere between the thermal device and the object
    // being measured.
    #[yaserde(prefix = "tth", rename = "AtmosphericTemperature")]
    pub atmospheric_temperature: Option<f64>,

    // Transmittance value for the atmosphere between the thermal device and the
    // object being measured.
    #[yaserde(prefix = "tth", rename = "AtmosphericTransmittance")]
    pub atmospheric_transmittance: Option<f64>,

    // Temperature of the optics elements between the thermal device and the
    // object being measured.
    #[yaserde(prefix = "tth", rename = "ExtOpticsTemperature")]
    pub ext_optics_temperature: Option<f64>,

    // Transmittance value for the optics elements between the thermal device
    // and the object being measured.
    #[yaserde(prefix = "tth", rename = "ExtOpticsTransmittance")]
    pub ext_optics_transmittance: Option<f64>,
}

impl Validate for RadiometryGlobalParameters {}

// Describes valid ranges for the different radiometry parameters required for
// accurate temperature calculation.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct RadiometryGlobalParameterOptions {
    // Valid range of temperature values, in Kelvin.
    #[yaserde(prefix = "tth", rename = "ReflectedAmbientTemperature")]
    pub reflected_ambient_temperature: tt::FloatRange,

    // Valid range of emissivity values for the objects to measure.
    #[yaserde(prefix = "tth", rename = "Emissivity")]
    pub emissivity: tt::FloatRange,

    // Valid range of distance between camera and object for a valid temperature
    // reading, in meters.
    #[yaserde(prefix = "tth", rename = "DistanceToObject")]
    pub distance_to_object: tt::FloatRange,

    // Valid range of relative humidity values, in percentage.
    #[yaserde(prefix = "tth", rename = "RelativeHumidity")]
    pub relative_humidity: Option<tt::FloatRange>,

    // Valid range of temperature values, in Kelvin.
    #[yaserde(prefix = "tth", rename = "AtmosphericTemperature")]
    pub atmospheric_temperature: Option<tt::FloatRange>,

    // Valid range of atmospheric transmittance values.
    #[yaserde(prefix = "tth", rename = "AtmosphericTransmittance")]
    pub atmospheric_transmittance: Option<tt::FloatRange>,

    // Valid range of temperature values, in Kelvin.
    #[yaserde(prefix = "tth", rename = "ExtOpticsTemperature")]
    pub ext_optics_temperature: Option<tt::FloatRange>,

    // Valid range of external optics transmittance.
    #[yaserde(prefix = "tth", rename = "ExtOpticsTransmittance")]
    pub ext_optics_transmittance: Option<tt::FloatRange>,
}

impl Validate for RadiometryGlobalParameterOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct Configuration {
    // Current Color Palette in use by the Thermal Device.
    #[yaserde(prefix = "tth", rename = "ColorPalette")]
    pub color_palette: ColorPalette,

    // Polarity configuration of the Thermal Device.
    #[yaserde(prefix = "tth", rename = "Polarity")]
    pub polarity: Polarity,

    // Current Non-Uniformity Correction (NUC) Table in use by the Thermal
    // Device.
    #[yaserde(prefix = "tth", rename = "NUCTable")]
    pub nuc_table: Option<Nuctable>,

    // Cooler settings of the Thermal Device.
    #[yaserde(prefix = "tth", rename = "Cooler")]
    pub cooler: Option<Cooler>,
}

impl Validate for Configuration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct Configurations {
    // Current Thermal Settings for the VideoSource.
    #[yaserde(prefix = "tth", rename = "Configuration")]
    pub configuration: Configuration,

    // Reference token to the thermal VideoSource.
    #[yaserde(attribute, rename = "token")]
    pub token: tt::ReferenceToken,
}

impl Validate for Configurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct RadiometryConfiguration {
    // Global Parameters for Radiometry Measurements. Shall exist if Radiometry
    // Capability is reported,
    // and Global Parameters are supported by the device.
    #[yaserde(prefix = "tth", rename = "RadiometryGlobalParameters")]
    pub radiometry_global_parameters: Option<RadiometryGlobalParameters>,
}

impl Validate for RadiometryConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct ConfigurationOptions {
    // List of Color Palettes available for the requested Thermal VideoSource.
    #[yaserde(prefix = "tth", rename = "ColorPalette")]
    pub color_palette: Vec<ColorPalette>,

    // List of Non-Uniformity Correction (NUC) Tables available for the
    // requested Thermal VideoSource.
    #[yaserde(prefix = "tth", rename = "NUCTable")]
    pub nuc_table: Vec<Nuctable>,

    // Specifies Cooler Options for cooled thermal devices.
    #[yaserde(prefix = "tth", rename = "CoolerOptions")]
    pub cooler_options: Option<CoolerOptions>,
}

impl Validate for ConfigurationOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct RadiometryConfigurationOptions {
    // Specifies valid ranges and options for the global radiometry parameters
    // used as default parameter values
    // for temperature measurement modules (spots and boxes).
    #[yaserde(prefix = "tth", rename = "RadiometryGlobalParameterOptions")]
    pub radiometry_global_parameter_options: Option<RadiometryGlobalParameterOptions>,
}

impl Validate for RadiometryConfigurationOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct GetServiceCapabilitiesResponse {
    // The capabilities of the thermal service are returned in the Capabilities
    // element.
    #[yaserde(prefix = "tth", rename = "Capabilities")]
    pub capabilities: Capabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct Capabilities {
    // Indicates whether or not radiometric thermal measurements are supported
    // by the thermal device.
    #[yaserde(attribute, rename = "Radiometry")]
    pub radiometry: Option<bool>,
}

impl Validate for Capabilities {}

// pub type Capabilities = Capabilities;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct GetConfigurationOptions {
    // Reference token to the VideoSource for which the Thermal Configuration
    // Options are requested.
    #[yaserde(prefix = "tth", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,
}

impl Validate for GetConfigurationOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct GetConfigurationOptionsResponse {
    // Valid ranges for the Thermal configuration parameters that are
    // categorized as device specific.
    #[yaserde(prefix = "tth", rename = "ConfigurationOptions")]
    pub configuration_options: ConfigurationOptions,
}

impl Validate for GetConfigurationOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct GetConfiguration {
    // Reference token to the VideoSource for which the Thermal Settings are
    // requested.
    #[yaserde(prefix = "tth", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,
}

impl Validate for GetConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct GetConfigurationResponse {
    // Thermal Settings for the VideoSource that was requested.
    #[yaserde(prefix = "tth", rename = "Configuration")]
    pub configuration: Configuration,
}

impl Validate for GetConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct GetConfigurations {}

impl Validate for GetConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct GetConfigurationsResponse {
    // This element contains a list of thermal VideoSource configurations.
    #[yaserde(prefix = "tth", rename = "Configurations")]
    pub configurations: Vec<Configurations>,
}

impl Validate for GetConfigurationsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct SetConfiguration {
    // Reference token to the VideoSource for which the Thermal Settings are
    // configured.
    #[yaserde(prefix = "tth", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,

    // Thermal Settings to be configured.
    #[yaserde(prefix = "tth", rename = "Configuration")]
    pub configuration: Configuration,
}

impl Validate for SetConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct SetConfigurationResponse {}

impl Validate for SetConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct GetRadiometryConfigurationOptions {
    // Reference token to the VideoSource for which the Thermal Radiometry
    // Options are requested.
    #[yaserde(prefix = "tth", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,
}

impl Validate for GetRadiometryConfigurationOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct GetRadiometryConfigurationOptionsResponse {
    // Valid ranges for the Thermal Radiometry parameters that are categorized
    // as device specific.
    #[yaserde(prefix = "tth", rename = "ConfigurationOptions")]
    pub configuration_options: RadiometryConfigurationOptions,
}

impl Validate for GetRadiometryConfigurationOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct GetRadiometryConfiguration {
    // Reference token to the VideoSource for which the Radiometry Configuration
    // is requested.
    #[yaserde(prefix = "tth", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,
}

impl Validate for GetRadiometryConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct GetRadiometryConfigurationResponse {
    // Radiometry Configuration for the VideoSource that was requested.
    #[yaserde(prefix = "tth", rename = "Configuration")]
    pub configuration: RadiometryConfiguration,
}

impl Validate for GetRadiometryConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct SetRadiometryConfiguration {
    // Reference token to the VideoSource for which the Radiometry settings are
    // configured.
    #[yaserde(prefix = "tth", rename = "VideoSourceToken")]
    pub video_source_token: tt::ReferenceToken,

    // Radiometry settings to be configured.
    #[yaserde(prefix = "tth", rename = "Configuration")]
    pub configuration: RadiometryConfiguration,
}

impl Validate for SetRadiometryConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tth",
    namespace = "tth: http://www.onvif.org/ver10/thermal/wsdl"
)]
pub struct SetRadiometryConfigurationResponse {}

impl Validate for SetRadiometryConfigurationResponse {}

// Returns the capabilities of the thermal service. The result is returned in a
// typed answer.
pub async fn get_service_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetServiceCapabilities,
) -> Result<GetServiceCapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// Gets the valid ranges for the Thermal parameters that have device specific
// ranges.
// This command is mandatory for all devices implementing the Thermal service.
// The command shall return all supported parameters
// and their ranges, such that these can be applied to the SetConfiguration
// command.
pub async fn get_configuration_options<T: transport::Transport>(
    transport: &T,
    request: &GetConfigurationOptions,
) -> Result<GetConfigurationOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// Gets the Thermal Configuration for the requested VideoSource.
pub async fn get_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetConfiguration,
) -> Result<GetConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// Gets the Thermal Configuration for all thermal VideoSources of the Device.
pub async fn get_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetConfigurations,
) -> Result<GetConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// Sets the Thermal Configuration for the requested VideoSource.
pub async fn set_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetConfiguration,
) -> Result<SetConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// Gets the valid ranges for the Radiometry parameters that have device specific
// ranges.
// The command shall return all supported parameters and their ranges, such that
// these can be applied
// to the SetRadiometryConfiguration command.
pub async fn get_radiometry_configuration_options<T: transport::Transport>(
    transport: &T,
    request: &GetRadiometryConfigurationOptions,
) -> Result<GetRadiometryConfigurationOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// Gets the Radiometry Configuration for the requested VideoSource.
pub async fn get_radiometry_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetRadiometryConfiguration,
) -> Result<GetRadiometryConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// Sets the Radiometry Configuration for the requested VideoSource.
pub async fn set_radiometry_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetRadiometryConfiguration,
) -> Result<SetRadiometryConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}
