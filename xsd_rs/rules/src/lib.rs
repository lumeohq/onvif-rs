use onvif as tt;
use validate::Validate;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "axt",
    namespace = "axt: http://www.onvif.org/ver20/analytics"
)]
pub struct MotionRegionConfigOptions {
    // The total number of Motion Region Detector rules that can be created on
    // the device.
    // This element is deprecated. maxInstances in the GetSupportedRules shall
    // be used instead.
    #[yaserde(prefix = "axt", rename = "MaxRegions")]
    pub max_regions: Option<i32>,

    // True if the device supports disarming a Motion Region Detector rule.
    #[yaserde(prefix = "axt", rename = "DisarmSupport")]
    pub disarm_support: Option<bool>,

    // True if the device supports defining a region using a Polygon instead of
    // a rectangle.
    // The rectangle points are still passed using a Polygon element if the
    // device does not support polygon regions.
    // In this case, the points provided in the Polygon element shall represent
    // a rectangle.
    #[yaserde(prefix = "axt", rename = "PolygonSupport")]
    pub polygon_support: Option<bool>,

    // For devices that support Polygons with limitations on the number of
    // sides,
    // provides the minimum and maximum number of sides that can be defined in
    // the
    // Polygon.
    #[yaserde(prefix = "axt", rename = "PolygonLimits")]
    pub polygon_limits: Option<tt::IntRange>,

    // Indicates the device can only support one sensitivity level for all
    // defines
    // motion detection regions. Changing the sensitivity for one region would
    // be
    // applied to all regions.
    #[yaserde(prefix = "axt", rename = "SingleSensitivitySupport")]
    pub single_sensitivity_support: Option<bool>,

    // True if the device will include the Name of the Rule to indicate the
    // region
    // that motion was detected in.
    #[yaserde(prefix = "axt", rename = "RuleNotification")]
    pub rule_notification: Option<bool>,

    // Indicates the support for PTZ preset based motion detection, if supported
    // Preset token can be associated with a motion region.
    #[yaserde(attribute, rename = "PTZPresetMotionSupport")]
    pub ptz_preset_motion_support: Option<bool>,
}

impl Validate for MotionRegionConfigOptions {}

// pub type MotionRegionConfigOptions = MotionRegionConfigOptions;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "axt",
    namespace = "axt: http://www.onvif.org/ver20/analytics"
)]
pub struct MotionRegionConfig {
    // Provides the points of a Polygon in the VideoSourceConfiguration's Bounds
    // element. If the device does not support Polygons, this structure must
    // contain
    // four points that represent a Rectangle.
    #[yaserde(prefix = "tt", rename = "Polygon")]
    pub polygon: Option<tt::Polygon>,

    // Preset position associated with the motion region defined by Polygon.
    #[yaserde(prefix = "axt", rename = "PresetToken")]
    pub preset_token: Option<tt::ReferenceToken>,

    // Indicates if the Motion Region is Armed (detecting motion) or Disarmed
    // (motion is
    // not being detected).
    #[yaserde(attribute, rename = "Armed")]
    pub armed: Option<bool>,

    // Indicates the sensitivity level of the motion detector for this region.
    // The
    // sensitivity value is normalized where 0 represents the lower sensitivity
    // where
    // significant motion is required to trigger an alarm and 1 represents the
    // higher
    // sensitivity where very little motion is required to trigger an alarm.
    #[yaserde(attribute, rename = "Sensitivity")]
    pub sensitivity: Option<f64>,
}

impl Validate for MotionRegionConfig {}
