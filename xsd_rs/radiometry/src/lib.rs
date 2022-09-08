#![allow(clippy::derive_partial_eq_without_eq)]

use onvif as tt;
use validate::Validate;
use xsd_types::types as xs;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "ttr",
    namespace = "ttr: http://www.onvif.org/ver20/analytics/radiometry"
)]
pub struct RadiometryModuleConfigOptions {
    // The total number of temperature measurement modules that can be created
    // on the
    // device, screen based or geolocated, of any type (spots or boxes).
    #[yaserde(prefix = "ttr", rename = "MaxMeasurementModules")]
    pub max_measurement_modules: i32,

    // The total number of spot measurement modules that can be loaded
    // simultaneously on the
    // screen by the device. A value of 0 shall be used to indicate no support
    // for Spots.
    #[yaserde(prefix = "ttr", rename = "MaxScreenSpots")]
    pub max_screen_spots: i32,

    // The total number of box measurement modules that can be loaded
    // simultaneously on the
    // screen by the device. A value of 0 shall be used to indicate no support
    // for Boxes.
    #[yaserde(prefix = "ttr", rename = "MaxScreenBoxes")]
    pub max_screen_boxes: i32,

    // Specifies valid ranges for the different radiometry parameters used for
    // temperature calculation.
    #[yaserde(prefix = "ttr", rename = "RadiometryParameterOptions")]
    pub radiometry_parameter_options: Option<RadiometryParameterOptions>,
}

impl Validate for RadiometryModuleConfigOptions {}

// Describes valid ranges for the different radiometry parameters used for
// accurate temperature calculation.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "ttr",
    namespace = "ttr: http://www.onvif.org/ver20/analytics/radiometry"
)]
pub struct RadiometryParameterOptions {
    // Valid range of temperature values, in Kelvin.
    #[yaserde(prefix = "ttr", rename = "ReflectedAmbientTemperature")]
    pub reflected_ambient_temperature: Option<tt::FloatRange>,

    // Valid range of emissivity values for the objects to measure.
    #[yaserde(prefix = "ttr", rename = "Emissivity")]
    pub emissivity: Option<tt::FloatRange>,

    // Valid range of distance between camera and object for a valid temperature
    // reading, in meters.
    #[yaserde(prefix = "ttr", rename = "DistanceToObject")]
    pub distance_to_object: Option<tt::FloatRange>,

    // Valid range of relative humidity values, in percentage.
    #[yaserde(prefix = "ttr", rename = "RelativeHumidity")]
    pub relative_humidity: Option<tt::FloatRange>,

    // Valid range of temperature values, in Kelvin.
    #[yaserde(prefix = "ttr", rename = "AtmosphericTemperature")]
    pub atmospheric_temperature: Option<tt::FloatRange>,

    // Valid range of atmospheric transmittance values.
    #[yaserde(prefix = "ttr", rename = "AtmosphericTransmittance")]
    pub atmospheric_transmittance: Option<tt::FloatRange>,

    // Valid range of temperature values, in Kelvin.
    #[yaserde(prefix = "ttr", rename = "ExtOpticsTemperature")]
    pub ext_optics_temperature: Option<tt::FloatRange>,

    // Valid range of external optics transmittance.
    #[yaserde(prefix = "ttr", rename = "ExtOpticsTransmittance")]
    pub ext_optics_transmittance: Option<tt::FloatRange>,
}

impl Validate for RadiometryParameterOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "ttr",
    namespace = "ttr: http://www.onvif.org/ver20/analytics/radiometry"
)]
pub struct RadiometrySpotModuleConfig {
    // Screen coordinates, if spot is currently on screen. Assumes normalized
    // screen limits (-1.0, 1.0).
    #[yaserde(prefix = "ttr", rename = "ScreenCoords")]
    pub screen_coords: tt::Vector,

    // Absolute orientation of the PTZ Vector with the Spot on screen. If no
    // PTZVector is present
    // the spot shall behave as a screen element, and stay on the same screen
    // coordinates as the PTZ
    // moves (like a head up display mask). If PTZVector is present the Spot
    // element shall appear on
    // display only when contained in the Field of View. In this case
    // SpotScreenCoords shall be
    // reported as relative to PTZVector.
    #[yaserde(prefix = "ttr", rename = "AbsoluteCoords")]
    pub absolute_coords: Option<tt::Ptzvector>,

    // Not present parameter means the Device shall use its value from Global
    // Parameters in Thermal Service.
    #[yaserde(prefix = "ttr", rename = "RadiometryParameters")]
    pub radiometry_parameters: Option<RadiometryParameters>,

    // Unique identifier for this Spot Temperature Measurement Analytics Module.
    #[yaserde(attribute, rename = "ItemID")]
    pub item_id: Option<tt::ReferenceToken>,

    // Indicates if the Temperature Measurement Item is enabled to provide
    // temperature readings.
    #[yaserde(attribute, rename = "Active")]
    pub active: Option<bool>,
}

impl Validate for RadiometrySpotModuleConfig {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "ttr",
    namespace = "ttr: http://www.onvif.org/ver20/analytics/radiometry"
)]
pub struct RadiometryBoxModuleConfig {
    // Screen coordinates, if box is currently on screen. Assumes normalized
    // screen limits (-1.0, 1.0).
    #[yaserde(prefix = "ttr", rename = "ScreenCoords")]
    pub screen_coords: tt::Rectangle,

    // Absolute orientation of the PTZ Vector with the Box on screen. If no
    // PTZVector is present
    // the box shall behave as a screen element, and stay on the same screen
    // coordinates as the PTZ
    // moves (like a head up display mask). If PTZVector is present the Box
    // element shall appear on
    // display only when contained in the Field of View. In this case
    // BoxScreenCoords shall be
    // reported as relative to PTZVector.
    #[yaserde(prefix = "ttr", rename = "AbsoluteCoords")]
    pub absolute_coords: Option<tt::Ptzvector>,

    // Not present parameter means the Device shall use its value from Global
    // Parameters in Thermal Service.
    #[yaserde(prefix = "ttr", rename = "RadiometryParameters")]
    pub radiometry_parameters: Option<RadiometryParameters>,

    // Unique identifier for this Box Temperature Measurement Analytics Module.
    #[yaserde(attribute, rename = "ItemID")]
    pub item_id: Option<tt::ReferenceToken>,

    // Indicates if the Temperature Measurement Item is enabled to provide
    // temperature readings.
    #[yaserde(attribute, rename = "Active")]
    pub active: Option<bool>,
}

impl Validate for RadiometryBoxModuleConfig {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "ttr",
    namespace = "ttr: http://www.onvif.org/ver20/analytics/radiometry"
)]
pub struct SpotTemperatureReading {
    // Not present means Global Parameters from Thermal Service are being used.
    #[yaserde(prefix = "ttr", rename = "RadiometryParameters")]
    pub radiometry_parameters: Option<RadiometryParameters>,

    #[yaserde(attribute, rename = "ItemID")]
    pub item_id: Option<tt::ReferenceToken>,

    #[yaserde(attribute, rename = "SpotTemperature")]
    pub spot_temperature: f64,
}

impl Validate for SpotTemperatureReading {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "ttr",
    namespace = "ttr: http://www.onvif.org/ver20/analytics/radiometry"
)]
pub struct BoxTemperatureReading {
    // Not present means Global Parameters from Thermal Service are being used.
    #[yaserde(prefix = "ttr", rename = "RadiometryParameters")]
    pub radiometry_parameters: Option<RadiometryParameters>,

    #[yaserde(attribute, rename = "ItemID")]
    pub item_id: tt::ReferenceToken,

    #[yaserde(attribute, rename = "MaxTemperature")]
    pub max_temperature: f64,

    #[yaserde(attribute, rename = "MinTemperature")]
    pub min_temperature: f64,

    #[yaserde(attribute, rename = "AverageTemperature")]
    pub average_temperature: Option<f64>,

    #[yaserde(attribute, rename = "MedianTemperature")]
    pub median_temperature: Option<f64>,
}

impl Validate for BoxTemperatureReading {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "ttr",
    namespace = "ttr: http://www.onvif.org/ver20/analytics/radiometry"
)]
pub struct RadiometryParameters {
    #[yaserde(prefix = "ttr", rename = "ReflectedAmbientTemperature")]
    pub reflected_ambient_temperature: Option<f64>,

    #[yaserde(prefix = "ttr", rename = "Emissivity")]
    pub emissivity: Option<f64>,

    #[yaserde(prefix = "ttr", rename = "DistanceToObject")]
    pub distance_to_object: Option<f64>,

    #[yaserde(prefix = "ttr", rename = "RelativeHumidity")]
    pub relative_humidity: Option<f64>,

    #[yaserde(prefix = "ttr", rename = "AtmosphericTemperature")]
    pub atmospheric_temperature: Option<f64>,

    #[yaserde(prefix = "ttr", rename = "AtmosphericTransmittance")]
    pub atmospheric_transmittance: Option<f64>,

    #[yaserde(prefix = "ttr", rename = "ExtOpticsTemperature")]
    pub ext_optics_temperature: Option<f64>,

    #[yaserde(prefix = "ttr", rename = "ExtOpticsTransmittance")]
    pub ext_optics_transmittance: Option<f64>,
}

impl Validate for RadiometryParameters {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "ttr",
    namespace = "ttr: http://www.onvif.org/ver20/analytics/radiometry"
)]
pub struct RadiometryRuleConfigOptions {
    // Specifies valid ranges for thresholds and reference parameters used for
    // triggering radiometric rules.
    #[yaserde(prefix = "ttr", rename = "RadiometryRuleOptions")]
    pub radiometry_rule_options: Option<RadiometryRuleOptions>,

    // Specifies valid rule conditions for temperature comparisions in
    // radiometric rules.
    #[yaserde(prefix = "ttr", rename = "TemperatureConditionOptions")]
    pub temperature_condition_options: Vec<TemperatureCondition>,

    // Specifies temperature measurement types provided by radiometry analytics
    // modules in the device.
    #[yaserde(prefix = "ttr", rename = "TemperatureTypeOptions")]
    pub temperature_type_options: Vec<TemperatureType>,
}

impl Validate for RadiometryRuleConfigOptions {}

// Describes valid ranges for radiometric rule condition thresholds and
// reference parameters.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "ttr",
    namespace = "ttr: http://www.onvif.org/ver20/analytics/radiometry"
)]
pub struct RadiometryRuleOptions {
    // Valid range of temperature values, in Kelvin.
    #[yaserde(prefix = "ttr", rename = "ThresholdTemperature")]
    pub threshold_temperature: tt::FloatRange,

    // Valid range of hysteresis time interval for temperature conditions, in
    // seconds.
    #[yaserde(prefix = "ttr", rename = "ThresholdTime")]
    pub threshold_time: Option<tt::FloatRange>,

    // Valid range of temperature hysteresis values, in Kelvin.
    #[yaserde(prefix = "ttr", rename = "HysteresisTemperature")]
    pub hysteresis_temperature: Option<tt::FloatRange>,
}

impl Validate for RadiometryRuleOptions {}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum TemperatureCondition {
    LessThan,
    MoreThan,
    EqualTo,
    Change,
    __Unknown__(String),
}

impl Default for TemperatureCondition {
    fn default() -> TemperatureCondition {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for TemperatureCondition {}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum TemperatureType {
    MaxTemp,
    MinTemp,
    AverageTemp,
    StdDeviation,
    MedianTemp,
    #[yaserde(rename = "ISOCoverage")]
    Isocoverage,
    __Unknown__(String),
}

impl Default for TemperatureType {
    fn default() -> TemperatureType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for TemperatureType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "ttr",
    namespace = "ttr: http://www.onvif.org/ver20/analytics/radiometry"
)]
pub struct RadiometryTemperatureRuleConfig {
    // Indicates which of the temperature values provided by the input Analytics
    // Module
    // shall be used by the rule. In the case of Analytics Modules providing a
    // single
    // Temperature Value (e.g. Spot) this parameter is ignored, and is therefore
    // optional.
    #[yaserde(prefix = "ttr", rename = "TemperatureType")]
    pub temperature_type: Option<TemperatureType>,

    // Indicates the type of temperature condition to check.
    #[yaserde(prefix = "ttr", rename = "RuleCondition")]
    pub rule_condition: TemperatureCondition,

    // Indicates the temperature reference value the rule shall be checked
    // against.
    #[yaserde(prefix = "ttr", rename = "ThresholdTemperature")]
    pub threshold_temperature: f64,

    // Indicates the time interval during which the rule condition shall be met
    // to trigger an event.
    #[yaserde(prefix = "ttr", rename = "ThresholdTime")]
    pub threshold_time: xs::Duration,

    // Indicates the width in Kelvin of the temerature hysteresis band to be
    // considered by the rule.
    #[yaserde(prefix = "ttr", rename = "HysteresisTemperature")]
    pub hysteresis_temperature: f64,

    // Reference Token to the Temperature Measurement Analytics Module providing
    // the Temperature on which rule is defined.
    #[yaserde(attribute, rename = "RadiometryModuleID")]
    pub radiometry_module_id: Option<tt::ReferenceToken>,

    // Indicates if the Temperature Rule is enabled to provide temperature alarm
    // events.
    #[yaserde(attribute, rename = "Enabled")]
    pub enabled: Option<bool>,
}

impl Validate for RadiometryTemperatureRuleConfig {}
