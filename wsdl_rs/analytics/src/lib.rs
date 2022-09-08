#![allow(clippy::derive_partial_eq_without_eq)]

use onvif as tt;
use validate::Validate;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct GetServiceCapabilitiesResponse {
    // The capabilities for the analytics service is returned in the
    // Capabilities element.
    #[yaserde(prefix = "tan", rename = "Capabilities")]
    pub capabilities: Capabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct Capabilities {
    // Indication that the device supports the rules interface and the rules
    // syntax.
    #[yaserde(attribute, rename = "RuleSupport")]
    pub rule_support: Option<bool>,

    // Indication that the device supports the scene analytics module interface.
    #[yaserde(attribute, rename = "AnalyticsModuleSupport")]
    pub analytics_module_support: Option<bool>,

    // Indication that the device produces the cell based scene description
    #[yaserde(attribute, rename = "CellBasedSceneDescriptionSupported")]
    pub cell_based_scene_description_supported: Option<bool>,

    // Indication that the device supports the GetRuleOptions operation on the
    // rules interface
    #[yaserde(attribute, rename = "RuleOptionsSupported")]
    pub rule_options_supported: Option<bool>,

    // Indication that the device supports the GetAnalyticsModuleOptions
    // operation on the analytics interface
    #[yaserde(attribute, rename = "AnalyticsModuleOptionsSupported")]
    pub analytics_module_options_supported: Option<bool>,
}

impl Validate for Capabilities {}

// pub type Capabilities = Capabilities;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct GetSupportedRules {
    // References an existing Video Analytics configuration. The list of
    // available tokens can be obtained
    // via the Media service GetVideoAnalyticsConfigurations method.
    #[yaserde(prefix = "tan", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetSupportedRules {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct GetSupportedRulesResponse {
    #[yaserde(prefix = "tan", rename = "SupportedRules")]
    pub supported_rules: tt::SupportedRules,
}

impl Validate for GetSupportedRulesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct CreateRules {
    // Reference to an existing VideoAnalyticsConfiguration.
    #[yaserde(prefix = "tan", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,

    #[yaserde(prefix = "tan", rename = "Rule")]
    pub rule: Vec<tt::Config>,
}

impl Validate for CreateRules {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct CreateRulesResponse {}

impl Validate for CreateRulesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct DeleteRules {
    // Reference to an existing VideoAnalyticsConfiguration.
    #[yaserde(prefix = "tan", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,

    // References the specific rule to be deleted (e.g. "MyLineDetector").
    #[yaserde(prefix = "tan", rename = "RuleName")]
    pub rule_name: Vec<String>,
}

impl Validate for DeleteRules {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct DeleteRulesResponse {}

impl Validate for DeleteRulesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct ModifyRules {
    // Reference to an existing VideoAnalyticsConfiguration.
    #[yaserde(prefix = "tan", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,

    #[yaserde(prefix = "tan", rename = "Rule")]
    pub rule: Vec<tt::Config>,
}

impl Validate for ModifyRules {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct ModifyRulesResponse {}

impl Validate for ModifyRulesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct GetRules {
    // Reference to an existing VideoAnalyticsConfiguration.
    #[yaserde(prefix = "tan", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetRules {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct GetRulesResponse {
    #[yaserde(prefix = "tan", rename = "Rule")]
    pub rule: Vec<tt::Config>,
}

impl Validate for GetRulesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct GetRuleOptions {
    // Reference to an SupportedRule Type returned from GetSupportedRules.
    #[yaserde(prefix = "tan", rename = "RuleType")]
    pub rule_type: Option<String>,

    // Reference to an existing analytics configuration.
    #[yaserde(prefix = "tan", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetRuleOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct GetRuleOptionsResponse {
    // A device shall provide respective ConfigOptions.RuleType for each
    // RuleOption if the request does not specify RuleType. The response Options
    // shall not contain any AnalyticsModule attribute.
    #[yaserde(prefix = "tan", rename = "RuleOptions")]
    pub rule_options: Vec<ConfigOptions>,
}

impl Validate for GetRuleOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct ConfigOptions {
    // The RuleType the ConfigOptions applies to if the Name attribute is
    // ambiguous.
    #[yaserde(attribute, rename = "RuleType")]
    pub rule_type: Option<String>,

    // The Name of the SimpleItemDescription/ElementItemDescription
    // the ConfigOptions applies to.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,

    // Type of the Rule Options represented by a unique QName.
    // The Type defines the element contained in this structure.
    // This attribute is deprecated since its value must be identical to the
    // embedded element.
    #[yaserde(attribute, rename = "Type")]
    pub _type: Option<String>,

    // Optional name of the analytics module this constraint applies to. This
    // option is only necessary in cases where different constraints for
    // elements with the same Name exist.
    #[yaserde(attribute, rename = "AnalyticsModule")]
    pub analytics_module: Option<String>,
}

impl Validate for ConfigOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct GetSupportedAnalyticsModules {
    // Reference to an existing VideoAnalyticsConfiguration.
    #[yaserde(prefix = "tan", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetSupportedAnalyticsModules {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct GetSupportedAnalyticsModulesResponse {
    #[yaserde(prefix = "tan", rename = "SupportedAnalyticsModules")]
    pub supported_analytics_modules: tt::SupportedAnalyticsModules,
}

impl Validate for GetSupportedAnalyticsModulesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct CreateAnalyticsModules {
    // Reference to an existing VideoAnalyticsConfiguration.
    #[yaserde(prefix = "tan", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,

    #[yaserde(prefix = "tan", rename = "AnalyticsModule")]
    pub analytics_module: Vec<tt::Config>,
}

impl Validate for CreateAnalyticsModules {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct CreateAnalyticsModulesResponse {}

impl Validate for CreateAnalyticsModulesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct DeleteAnalyticsModules {
    // Reference to an existing Video Analytics configuration.
    #[yaserde(prefix = "tan", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,

    // Name of the AnalyticsModule to be deleted.
    #[yaserde(prefix = "tan", rename = "AnalyticsModuleName")]
    pub analytics_module_name: Vec<String>,
}

impl Validate for DeleteAnalyticsModules {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct DeleteAnalyticsModulesResponse {}

impl Validate for DeleteAnalyticsModulesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct ModifyAnalyticsModules {
    // Reference to an existing VideoAnalyticsConfiguration.
    #[yaserde(prefix = "tan", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,

    #[yaserde(prefix = "tan", rename = "AnalyticsModule")]
    pub analytics_module: Vec<tt::Config>,
}

impl Validate for ModifyAnalyticsModules {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct ModifyAnalyticsModulesResponse {}

impl Validate for ModifyAnalyticsModulesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct GetAnalyticsModules {
    // Reference to an existing VideoAnalyticsConfiguration.
    #[yaserde(prefix = "tan", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetAnalyticsModules {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct GetAnalyticsModulesResponse {
    #[yaserde(prefix = "tan", rename = "AnalyticsModule")]
    pub analytics_module: Vec<tt::Config>,
}

impl Validate for GetAnalyticsModulesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct GetAnalyticsModuleOptions {
    // Reference to an SupportedAnalyticsModule Type returned from
    // GetSupportedAnalyticsModules.
    #[yaserde(prefix = "tan", rename = "Type")]
    pub _type: Option<String>,

    // Reference to an existing AnalyticsConfiguration.
    #[yaserde(prefix = "tan", rename = "ConfigurationToken")]
    pub configuration_token: tt::ReferenceToken,
}

impl Validate for GetAnalyticsModuleOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tan",
    namespace = "tan: http://www.onvif.org/ver20/analytics/wsdl"
)]
pub struct GetAnalyticsModuleOptionsResponse {
    // List of options for the specified analytics module. The response Options
    // shall not contain any RuleType attribute.
    #[yaserde(prefix = "tan", rename = "Options")]
    pub options: Vec<ConfigOptions>,
}

impl Validate for GetAnalyticsModuleOptionsResponse {}

// Returns the capabilities of the analytics service. The result is returned in
// a typed answer.
pub async fn get_service_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetServiceCapabilities,
) -> Result<GetServiceCapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// List all analytics modules that are supported by the given
// VideoAnalyticsConfiguration.
pub async fn get_supported_analytics_modules<T: transport::Transport>(
    transport: &T,
    request: &GetSupportedAnalyticsModules,
) -> Result<GetSupportedAnalyticsModulesResponse, transport::Error> {
    transport::request(transport, request).await
}

// Return the options for the supported analytics modules that specify an Option
// attribute.
pub async fn get_analytics_module_options<T: transport::Transport>(
    transport: &T,
    request: &GetAnalyticsModuleOptions,
) -> Result<GetAnalyticsModuleOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// Add one or more analytics modules to an existing VideoAnalyticsConfiguration.
// The available supported types can be retrieved via
pub async fn create_analytics_modules<T: transport::Transport>(
    transport: &T,
    request: &CreateAnalyticsModules,
) -> Result<CreateAnalyticsModulesResponse, transport::Error> {
    transport::request(transport, request).await
}

// Remove one or more analytics modules from a VideoAnalyticsConfiguration
// referenced by their names.
pub async fn delete_analytics_modules<T: transport::Transport>(
    transport: &T,
    request: &DeleteAnalyticsModules,
) -> Result<DeleteAnalyticsModulesResponse, transport::Error> {
    transport::request(transport, request).await
}

// List the currently assigned set of analytics modules of a
// VideoAnalyticsConfiguration.
pub async fn get_analytics_modules<T: transport::Transport>(
    transport: &T,
    request: &GetAnalyticsModules,
) -> Result<GetAnalyticsModulesResponse, transport::Error> {
    transport::request(transport, request).await
}

// Modify the settings of one or more analytics modules of a
// VideoAnalyticsConfiguration. The modules are referenced by their names.
// It is allowed to pass only a subset to be modified.
pub async fn modify_analytics_modules<T: transport::Transport>(
    transport: &T,
    request: &ModifyAnalyticsModules,
) -> Result<ModifyAnalyticsModulesResponse, transport::Error> {
    transport::request(transport, request).await
}

// List all rules that are supported by the given VideoAnalyticsConfiguration.
// The result of this method may depend on the overall Video analytics
// configuration of the device,
// which is available via the current set of profiles.
pub async fn get_supported_rules<T: transport::Transport>(
    transport: &T,
    request: &GetSupportedRules,
) -> Result<GetSupportedRulesResponse, transport::Error> {
    transport::request(transport, request).await
}

// Add one or more rules to an existing VideoAnalyticsConfiguration.
// The available supported types can be retrieved via
pub async fn create_rules<T: transport::Transport>(
    transport: &T,
    request: &CreateRules,
) -> Result<CreateRulesResponse, transport::Error> {
    transport::request(transport, request).await
}

// Remove one or more rules from a VideoAnalyticsConfiguration.
pub async fn delete_rules<T: transport::Transport>(
    transport: &T,
    request: &DeleteRules,
) -> Result<DeleteRulesResponse, transport::Error> {
    transport::request(transport, request).await
}

// List the currently assigned set of rules of a VideoAnalyticsConfiguration.
pub async fn get_rules<T: transport::Transport>(
    transport: &T,
    request: &GetRules,
) -> Result<GetRulesResponse, transport::Error> {
    transport::request(transport, request).await
}

// Return the options for the supported rules that specify an Option attribute.
pub async fn get_rule_options<T: transport::Transport>(
    transport: &T,
    request: &GetRuleOptions,
) -> Result<GetRuleOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// Modify one or more rules of a VideoAnalyticsConfiguration. The rules are
// referenced by their names.
pub async fn modify_rules<T: transport::Transport>(
    transport: &T,
    request: &ModifyRules,
) -> Result<ModifyRulesResponse, transport::Error> {
    transport::request(transport, request).await
}
