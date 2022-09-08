#![allow(clippy::derive_partial_eq_without_eq)]

use b_2 as wsnt;
use onvif as tt;
use validate::Validate;
use xsd_types::types as xs;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct GetSupportedActions {}

impl Validate for GetSupportedActions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct GetSupportedActionsResponse {
    // Array of supported Action types
    #[yaserde(prefix = "tae", rename = "SupportedActions")]
    pub supported_actions: SupportedActions,
}

impl Validate for GetSupportedActionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct GetActions {}

impl Validate for GetActions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct GetActionsResponse {
    // Array of current Action configurations
    #[yaserde(prefix = "tae", rename = "Action")]
    pub action: Vec<Action>,
}

impl Validate for GetActionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct CreateActions {
    // Array of Actions to be configured on service provider
    #[yaserde(prefix = "tae", rename = "Action")]
    pub action: Vec<ActionConfiguration>,
}

impl Validate for CreateActions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct CreateActionsResponse {
    // Array of configured Actions with service provider assigned unique
    // identifiers
    #[yaserde(prefix = "tae", rename = "Action")]
    pub action: Vec<Action>,
}

impl Validate for CreateActionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct DeleteActions {
    // Array of tokens referencing existing Action configurations to be removed
    #[yaserde(prefix = "tae", rename = "Token")]
    pub token: Vec<tt::ReferenceToken>,
}

impl Validate for DeleteActions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct DeleteActionsResponse {}

impl Validate for DeleteActionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct ModifyActions {
    // Array of Action configurations to update the existing action
    // configurations
    #[yaserde(prefix = "tae", rename = "Action")]
    pub action: Vec<Action>,
}

impl Validate for ModifyActions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct ModifyActionsResponse {}

impl Validate for ModifyActionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct GetServiceCapabilitiesResponse {
    #[yaserde(prefix = "tae", rename = "Capabilities")]
    pub capabilities: ActionEngineCapabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}

// pub type Capabilities = ActionEngineCapabilities;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct GetActionTriggers {}

impl Validate for GetActionTriggers {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct GetActionTriggersResponse {
    // Array of current Action Trigger configurations
    #[yaserde(prefix = "tae", rename = "ActionTrigger")]
    pub action_trigger: Vec<ActionTrigger>,
}

impl Validate for GetActionTriggersResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct CreateActionTriggers {
    // Action Triggers to be configured
    #[yaserde(prefix = "tae", rename = "ActionTrigger")]
    pub action_trigger: Vec<ActionTriggerConfiguration>,
}

impl Validate for CreateActionTriggers {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct CreateActionTriggersResponse {
    // Returns configured Action Triggers with service provider assigned unique
    // identifers
    #[yaserde(prefix = "tae", rename = "ActionTrigger")]
    pub action_trigger: Vec<ActionTrigger>,
}

impl Validate for CreateActionTriggersResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct ModifyActionTriggers {
    // Array of Action Trigger configurations to be updated.
    #[yaserde(prefix = "tae", rename = "ActionTrigger")]
    pub action_trigger: Vec<ActionTrigger>,
}

impl Validate for ModifyActionTriggers {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct ModifyActionTriggersResponse {}

impl Validate for ModifyActionTriggersResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct DeleteActionTriggers {
    // Array of tokens referencing existing Action Trigger configurations to be
    // removed
    #[yaserde(prefix = "tae", rename = "Token")]
    pub token: Vec<tt::ReferenceToken>,
}

impl Validate for DeleteActionTriggers {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct DeleteActionTriggersResponse {}

impl Validate for DeleteActionTriggersResponse {}

// Describes the configuration parameters of an action.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct ActionConfigDescription {
    // Action configuration parameter descriptions
    #[yaserde(prefix = "tae", rename = "ParameterDescription")]
    pub parameter_description: tt::ItemListDescription,

    // Action type name
    #[yaserde(attribute, rename = "Name")]
    pub name: String,
}

impl Validate for ActionConfigDescription {}

// SupportedActions data structure lists the available action types that service
// provider supports. For each action type, data structure contains the action
// configuration parameters.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct SupportedActions {
    // Lists the location of all schemas that are referenced in the supported
    // actions. If the action descriptions reference data types in the ONVIF
    // schema file,the ONVIF schema file MUST be explicitly listed.
    #[yaserde(prefix = "tae", rename = "ActionContentSchemaLocation")]
    pub action_content_schema_location: Vec<String>,

    // List of actions supported by Action Engine Service provider.
    #[yaserde(prefix = "tae", rename = "ActionDescription")]
    pub action_description: Vec<ActionConfigDescription>,

    #[yaserde(prefix = "tae", rename = "Extension")]
    pub extension: Option<SupportedActionsExtension>,
}

impl Validate for SupportedActions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct SupportedActionsExtension {}

impl Validate for SupportedActionsExtension {}

// Action Engine Capabilities data structure contains the maximum number of
// supported actions and number of actions in use for generic as well as
// specific action types
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct ActionEngineCapabilities {
    // Limits for each action type
    #[yaserde(prefix = "tae", rename = "ActionCapabilities")]
    pub action_capabilities: Vec<ActionTypeLimits>,

    #[yaserde(prefix = "tae", rename = "Extension")]
    pub extension: Option<ActionEngineCapabilitiesExtension>,

    // The maximum number of trigger configurations that the service provider
    // can concurrently support
    #[yaserde(attribute, rename = "MaximumTriggers")]
    pub maximum_triggers: Option<xs::Integer>,

    // The maximum number of actions that the service provider can concurrently
    // support
    #[yaserde(attribute, rename = "MaximumActions")]
    pub maximum_actions: Option<xs::Integer>,
}

impl Validate for ActionEngineCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct ActionEngineCapabilitiesExtension {}

impl Validate for ActionEngineCapabilitiesExtension {}

// ActionTypeLimits data structure contains maximum and current usage
// information for a specific action type in the service provider
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct ActionTypeLimits {
    // Action Type
    #[yaserde(attribute, rename = "Type")]
    pub _type: String,

    // For the specific action type, the maximum number of actions that could be
    // concurrently supported by the service provider
    #[yaserde(attribute, rename = "Maximum")]
    pub maximum: xs::Integer,

    // For the specific action type, the number of actions in use by the service
    // provider
    #[yaserde(attribute, rename = "InUse")]
    pub in_use: Option<xs::Integer>,
}

impl Validate for ActionTypeLimits {}

// Action Configuration data type contains the configuration settings of action
// configuration parameters, service requester given action Name, and service
// provider supported action type value
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct ActionConfiguration {
    // Action configuration parameter settings.
    #[yaserde(prefix = "tae", rename = "Parameters")]
    pub parameters: tt::ItemList,

    // User given name.
    #[yaserde(attribute, rename = "Name")]
    pub name: String,

    // Denotes the action type.
    #[yaserde(attribute, rename = "Type")]
    pub _type: String,
}

impl Validate for ActionConfiguration {}

// Action data type contains the configuration settings of one action instance
// and service provider assigned unique identifier for this action
// configuration.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct Action {
    // Action configuration contains action type, user given action name, and
    // configuratin parameter settings.
    #[yaserde(prefix = "tae", rename = "Configuration")]
    pub configuration: ActionConfiguration,

    // Unique Action identifier that service provider assigned to the action
    // configuration.
    #[yaserde(attribute, rename = "Token")]
    pub token: tt::ReferenceToken,
}

impl Validate for Action {}

// Action Trigger configuration data type contains mandatory Topic Expression
// (Section Topic Filter in [Core Specification]), optional Message content
// expression (Section Message Content Filter in [Core Specification]), and set
// of actions to be triggered.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct ActionTriggerConfiguration {
    // Topic expression, for example, to trigger only for relays. Trigger based
    // on event topic.
    #[yaserde(prefix = "tae", rename = "TopicExpression")]
    pub topic_expression: wsnt::TopicExpressionType,

    // Content expression, for example, to trigger only when the relay value is
    // on. Trigger based on content data in event.
    #[yaserde(prefix = "tae", rename = "ContentExpression")]
    pub content_expression: Option<wsnt::QueryExpressionType>,

    // Reference to actions to be triggered when the conditions are satisfied.
    #[yaserde(prefix = "tae", rename = "ActionToken")]
    pub action_token: Vec<tt::ReferenceToken>,

    #[yaserde(prefix = "tae", rename = "Extension")]
    pub extension: Option<ActionTriggerConfigurationExtension>,
}

impl Validate for ActionTriggerConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct ActionTriggerConfigurationExtension {}

impl Validate for ActionTriggerConfigurationExtension {}

// Action Trigger data type contains the service provider assigned unique
// identifier for the configuration and action trigger configuration data.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct ActionTrigger {
    // Action Trigger Configuration
    #[yaserde(prefix = "tae", rename = "Configuration")]
    pub configuration: ActionTriggerConfiguration,

    // Unique Action Trigger identifier that service provider assigned to the
    // action trigger configuration.
    #[yaserde(attribute, rename = "Token")]
    pub token: tt::ReferenceToken,
}

impl Validate for ActionTrigger {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct OnvifAction {
    #[yaserde(prefix = "tae", rename = "ActionDescription")]
    pub action_description: Vec<ActionConfigDescription>,
}

impl Validate for OnvifAction {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct EmailServerConfiguration {
    // SMTP EMail Server configuration
    #[yaserde(prefix = "tae", rename = "SMTPConfig")]
    pub smtp_config: Smtpconfig,

    // POP EMail Server configuration
    #[yaserde(prefix = "tae", rename = "POPConfig")]
    pub pop_config: Popconfig,

    // Credentials configuration
    #[yaserde(prefix = "tae", rename = "AuthenticationConfig")]
    pub authentication_config: AuthenticationConfig,
}

impl Validate for EmailServerConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct Smtpconfig {
    // Destination SMTP Address configuration
    #[yaserde(prefix = "tae", rename = "HostAddress")]
    pub host_address: HostAddress,

    #[yaserde(attribute, rename = "portNo")]
    pub port_no: Option<xs::Integer>,
}

impl Validate for Smtpconfig {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct Popconfig {
    // Destination POP Server Address configuration
    #[yaserde(prefix = "tae", rename = "HostAddress")]
    pub host_address: HostAddress,
}

impl Validate for Popconfig {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct HostAddress {
    // IP Address
    #[yaserde(prefix = "tae", rename = "Value")]
    pub value: String,

    // IP Address format type such as IPv4 or IPv6
    #[yaserde(attribute, rename = "formatType")]
    pub format_type: AddressFormatType,
}

impl Validate for HostAddress {}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum AddressFormatType {
    #[yaserde(rename = "hostname")]
    Hostname,
    #[yaserde(rename = "ipv4")]
    Ipv4,
    #[yaserde(rename = "ipv6")]
    Ipv6,
    Extended,
    __Unknown__(String),
}

impl Default for AddressFormatType {
    fn default() -> AddressFormatType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for AddressFormatType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct UserCredentials {
    // Username
    #[yaserde(prefix = "tae", rename = "username")]
    pub username: String,

    // Password
    #[yaserde(prefix = "tae", rename = "password")]
    pub password: Option<String>,

    #[yaserde(prefix = "tae", rename = "Extension")]
    pub extension: Option<UserCredentialsExtension>,
}

impl Validate for UserCredentials {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct UserCredentialsExtension {}

impl Validate for UserCredentialsExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct AuthenticationConfig {
    // Username-password
    #[yaserde(prefix = "tae", rename = "User")]
    pub user: UserCredentials,

    // Email server authentication mode
    #[yaserde(attribute, rename = "mode")]
    pub mode: EmailAuthenticationMode,
}

impl Validate for AuthenticationConfig {}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum EmailAuthenticationMode {
    #[yaserde(rename = "none")]
    None,
    #[yaserde(rename = "SMTP")]
    Smtp,
    #[yaserde(rename = "POPSMTP")]
    Popsmtp,
    Extended,
    __Unknown__(String),
}

impl Default for EmailAuthenticationMode {
    fn default() -> EmailAuthenticationMode {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for EmailAuthenticationMode {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct EmailReceiverConfiguration {
    // Configuration for E-mail TO
    #[yaserde(prefix = "tae", rename = "TO")]
    pub to: Vec<String>,

    // Configuration for E-mail CC
    #[yaserde(prefix = "tae", rename = "CC")]
    pub cc: Vec<String>,

    #[yaserde(prefix = "tae", rename = "Extension")]
    pub extension: Option<EmailReceiverConfigurationExtension>,
}

impl Validate for EmailReceiverConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct EmailReceiverConfigurationExtension {}

impl Validate for EmailReceiverConfigurationExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct EmailAttachmentConfiguration {
    #[yaserde(prefix = "tae", rename = "FileName")]
    pub file_name: Option<String>,

    #[yaserde(prefix = "tae", rename = "doSuffix")]
    pub do_suffix: Option<FileSuffixType>,

    #[yaserde(prefix = "tae", rename = "Extension")]
    pub extension: Option<EmailAttachmentConfigurationExtension>,
}

impl Validate for EmailAttachmentConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct EmailAttachmentConfigurationExtension {}

impl Validate for EmailAttachmentConfigurationExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct EmailBodyTextConfiguration {
    // Whether content of E-mail message contains event data
    #[yaserde(attribute, rename = "includeEvent")]
    pub include_event: Option<bool>,

    #[yaserde(attribute, rename = "type")]
    pub _type: Option<String>,
}

impl Validate for EmailBodyTextConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct MediaSource {
    // MediaSource profile reference token
    #[yaserde(prefix = "tae", rename = "ProfileToken")]
    pub profile_token: tt::ReferenceToken,
}

impl Validate for MediaSource {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct HttpHostConfigurations {
    // Destination HTTP Server configuration
    #[yaserde(prefix = "tae", rename = "HttpDestination")]
    pub http_destination: Vec<HttpDestinationConfiguration>,

    #[yaserde(prefix = "tae", rename = "Extension")]
    pub extension: Option<HttpHostConfigurationsExtension>,
}

impl Validate for HttpHostConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct HttpHostConfigurationsExtension {}

impl Validate for HttpHostConfigurationsExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct HttpDestinationConfiguration {
    // Destination HTTP Server address configuration
    #[yaserde(prefix = "tae", rename = "HostAddress")]
    pub host_address: HttpHostAddress,

    // User Credentials configuration for destination HTTP Server
    #[yaserde(prefix = "tae", rename = "HttpAuthentication")]
    pub http_authentication: Option<HttpAuthenticationConfiguration>,

    #[yaserde(prefix = "tae", rename = "Extension")]
    pub extension: Option<HttpDestinationConfigurationExtension>,

    // URI for POST Message destination
    #[yaserde(attribute, rename = "uri")]
    pub uri: Option<String>,

    // HTTP/HTTPS protocol selection (default is http)
    #[yaserde(attribute, rename = "protocol")]
    pub protocol: Option<HttpProtocolType>,
}

impl Validate for HttpDestinationConfiguration {}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum HttpProtocolType {
    #[yaserde(rename = "http")]
    Http,
    #[yaserde(rename = "https")]
    Https,
    Extended,
    __Unknown__(String),
}

impl Default for HttpProtocolType {
    fn default() -> HttpProtocolType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for HttpProtocolType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct HttpDestinationConfigurationExtension {}

impl Validate for HttpDestinationConfigurationExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct HttpAuthenticationConfiguration {
    // User credentials
    #[yaserde(prefix = "tae", rename = "User")]
    pub user: Option<UserCredentials>,

    #[yaserde(prefix = "tae", rename = "Extension")]
    pub extension: Option<HttpAuthenticationConfigurationExtension>,

    // HTTP Authentication Method
    #[yaserde(attribute, rename = "method")]
    pub method: Option<HttpAuthenticationMethodType>,
}

impl Validate for HttpAuthenticationConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct HttpAuthenticationConfigurationExtension {}

impl Validate for HttpAuthenticationConfigurationExtension {}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum HttpAuthenticationMethodType {
    #[yaserde(rename = "none")]
    None,
    #[yaserde(rename = "MD5Digest")]
    Md5Digest,
    Extended,
    __Unknown__(String),
}

impl Default for HttpAuthenticationMethodType {
    fn default() -> HttpAuthenticationMethodType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for HttpAuthenticationMethodType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct HttpHostAddress {
    // Destination HTTP Server IP Address
    #[yaserde(prefix = "tae", rename = "Value")]
    pub value: String,

    // IPv4 or IPv6
    #[yaserde(attribute, rename = "formatType")]
    pub format_type: AddressFormatType,

    // Port Number if different from 80
    #[yaserde(attribute, rename = "portNo")]
    pub port_no: Option<xs::Integer>,
}

impl Validate for HttpHostAddress {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct PostContentConfiguration {
    // MediaSource reference when the media is attached to POST message
    #[yaserde(prefix = "tae", rename = "MediaReference")]
    pub media_reference: Option<MediaSource>,

    // Configuration for POST Message content
    #[yaserde(prefix = "tae", rename = "PostBody")]
    pub post_body: PostBodyConfiguration,
}

impl Validate for PostContentConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct PostBodyConfiguration {
    #[yaserde(attribute, rename = "formData")]
    pub form_data: Option<String>,

    // Whether include event into POST message
    #[yaserde(attribute, rename = "includeEvent")]
    pub include_event: Option<bool>,

    // Whether attach media into POST message
    #[yaserde(attribute, rename = "includeMedia")]
    pub include_media: Option<bool>,
}

impl Validate for PostBodyConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct FtpHostConfigurations {
    // FTP Action destination configuration
    #[yaserde(prefix = "tae", rename = "FtpDestination")]
    pub ftp_destination: Vec<FtpDestinationConfiguration>,

    #[yaserde(prefix = "tae", rename = "Extension")]
    pub extension: Option<FtpHostConfigurationsExtension>,
}

impl Validate for FtpHostConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct FtpHostConfigurationsExtension {}

impl Validate for FtpHostConfigurationsExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct FtpDestinationConfiguration {
    // FTP Server IP Address
    #[yaserde(prefix = "tae", rename = "HostAddress")]
    pub host_address: FtpHostAddress,

    // Upload Directory Path
    #[yaserde(prefix = "tae", rename = "UploadPath")]
    pub upload_path: String,

    // User credentials confguration for target FTP Server
    #[yaserde(prefix = "tae", rename = "FtpAuthentication")]
    pub ftp_authentication: FtpAuthenticationConfiguration,

    #[yaserde(prefix = "tae", rename = "Extension")]
    pub extension: Option<FtpDestinationConfigurationExtension>,
}

impl Validate for FtpDestinationConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct FtpDestinationConfigurationExtension {}

impl Validate for FtpDestinationConfigurationExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct FtpAuthenticationConfiguration {
    // User Credentials
    #[yaserde(prefix = "tae", rename = "User")]
    pub user: Option<UserCredentials>,

    #[yaserde(prefix = "tae", rename = "Extension")]
    pub extension: Option<FtpAuthenticationConfigurationExtension>,
}

impl Validate for FtpAuthenticationConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct FtpAuthenticationConfigurationExtension {}

impl Validate for FtpAuthenticationConfigurationExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct FtpHostAddress {
    // FTP Server IP Address
    #[yaserde(prefix = "tae", rename = "Value")]
    pub value: String,

    // IPv4 or IPv6
    #[yaserde(attribute, rename = "formatType")]
    pub format_type: AddressFormatType,

    // Port Number
    #[yaserde(attribute, rename = "portNo")]
    pub port_no: Option<xs::Integer>,
}

impl Validate for FtpHostAddress {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct FtpContent {
    #[yaserde(prefix = "tae", rename = "FtpContentConfig")]
    pub ftp_content_config: FtpContentConfiguration,
}

impl Validate for FtpContent {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct FtpFileNameConfigurations {
    // Name of file
    #[yaserde(attribute, rename = "file_name")]
    pub file_name: Option<String>,

    // Suffix of file
    #[yaserde(attribute, rename = "suffix")]
    pub suffix: Option<FileSuffixType>,
}

impl Validate for FtpFileNameConfigurations {}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum FileSuffixType {
    #[yaserde(rename = "none")]
    None,
    #[yaserde(rename = "sequence")]
    Sequence,
    #[yaserde(rename = "dateTime")]
    DateTime,
    Extended,
    __Unknown__(String),
}

impl Default for FileSuffixType {
    fn default() -> FileSuffixType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for FileSuffixType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct FtpContentConfiguration {
    #[yaserde(prefix = "tae", rename = "FtpContentConfigurationChoice")]
    pub ftp_content_configuration_choice: ftp_content_configuration::FtpContentConfigurationChoice,

    // Type of FTP Upload action
    #[yaserde(attribute, rename = "Type")]
    pub _type: String,
}

impl Validate for FtpContentConfiguration {}

pub mod ftp_content_configuration {
    use super::*;

    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    pub enum FtpContentConfigurationChoice {
        // Upload Images action configuration
        UploadImages(FtpContentConfigurationUploadImages),
        // Upload files action configuration
        UploadFile(FtpContentConfigurationUploadFile),
        __Unknown__(String),
    }

    impl Default for FtpContentConfigurationChoice {
        fn default() -> FtpContentConfigurationChoice {
            Self::__Unknown__("No valid variants".into())
        }
    }

    impl Validate for FtpContentConfigurationChoice {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct FtpContentConfigurationUploadImages {
    // Upload Image action; how long?
    #[yaserde(prefix = "tae", rename = "HowLong")]
    pub how_long: xs::Duration,

    // Upload Image action; sample interval?
    #[yaserde(prefix = "tae", rename = "SampleInterval")]
    pub sample_interval: xs::Duration,

    // Upload Image action; name of destination file
    #[yaserde(prefix = "tae", rename = "FileName")]
    pub file_name: FtpFileNameConfigurations,
}

impl Validate for FtpContentConfigurationUploadImages {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct FtpContentConfigurationUploadFile {
    // Name of source file
    #[yaserde(prefix = "tae", rename = "sourceFileName")]
    pub source_file_name: String,

    // Name of destination file
    #[yaserde(prefix = "tae", rename = "destinationFileName")]
    pub destination_file_name: String,
}

impl Validate for FtpContentConfigurationUploadFile {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct SmsproviderConfiguration {
    // SMS Provider's URL
    #[yaserde(prefix = "tae", rename = "ProviderURL")]
    pub provider_url: String,

    // Username and password
    #[yaserde(prefix = "tae", rename = "User")]
    pub user: UserCredentials,
}

impl Validate for SmsproviderConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct SmssenderConfiguration {
    // Sender's e-mail address
    #[yaserde(prefix = "tae", rename = "EMail")]
    pub e_mail: String,
}

impl Validate for SmssenderConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct Smsmessage {
    // Text Message
    #[yaserde(prefix = "tae", rename = "Text")]
    pub text: String,
}

impl Validate for Smsmessage {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct TriggeredRecordingConfiguration {
    // Length of recording time before the triggering event
    #[yaserde(prefix = "tae", rename = "PreRecordDuration")]
    pub pre_record_duration: xs::Duration,

    // Recording after alarm recording duration
    #[yaserde(prefix = "tae", rename = "PostRecordDuration")]
    pub post_record_duration: xs::Duration,

    // Record duration
    #[yaserde(prefix = "tae", rename = "RecordDuration")]
    pub record_duration: xs::Duration,

    // Recording frame rate
    #[yaserde(prefix = "tae", rename = "RecordFrameRate")]
    pub record_frame_rate: Option<xs::Integer>,

    // Whether Audio recording on/off
    #[yaserde(prefix = "tae", rename = "DoRecordAudio")]
    pub do_record_audio: bool,
}

impl Validate for TriggeredRecordingConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tae",
    namespace = "tae: http://www.onvif.org/ver10/actionengine/wsdl"
)]
pub struct RecordingActionConfiguration {
    // Recording configuration
    #[yaserde(prefix = "tae", rename = "RecordConfig")]
    pub record_config: TriggeredRecordingConfiguration,
}

impl Validate for RecordingActionConfiguration {}

// The service provider returns the supported action types.
pub async fn get_supported_actions<T: transport::Transport>(
    transport: &T,
    request: &GetSupportedActions,
) -> Result<GetSupportedActionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// The service provider returns currently installed Actions.
pub async fn get_actions<T: transport::Transport>(
    transport: &T,
    request: &GetActions,
) -> Result<GetActionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// The create action operation adds actions to configuration. The create action
// operation is atomic. If a service provider can not create all of requested
// actions, the service provider responds with a fault message.
pub async fn create_actions<T: transport::Transport>(
    transport: &T,
    request: &CreateActions,
) -> Result<CreateActionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// The delete operation deletes actions. The delete action operation is atomic.
// If a service provider can not delete all of requested actions, the service
// provider responds with a fault message.
pub async fn delete_actions<T: transport::Transport>(
    transport: &T,
    request: &DeleteActions,
) -> Result<DeleteActionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// The modify action operation modifies action configurations.
pub async fn modify_actions<T: transport::Transport>(
    transport: &T,
    request: &ModifyActions,
) -> Result<ModifyActionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// The get capabilities operation returns the Action Engine capabilities
pub async fn get_service_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetServiceCapabilities,
) -> Result<GetServiceCapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// The service provider returns existing action triggers
pub async fn get_action_triggers<T: transport::Transport>(
    transport: &T,
    request: &GetActionTriggers,
) -> Result<GetActionTriggersResponse, transport::Error> {
    transport::request(transport, request).await
}

// Creates action triggers. The create action triggers operation is atomic. If a
// service provider can not create all of requested action triggers, the service
// provider responds with a fault message.
pub async fn create_action_triggers<T: transport::Transport>(
    transport: &T,
    request: &CreateActionTriggers,
) -> Result<CreateActionTriggersResponse, transport::Error> {
    transport::request(transport, request).await
}

// Deletes action triggers. The delete action triggers operation is atomic. If a
// service provider can not delete all of requested action triggers, the service
// provider responds with a fault message.
pub async fn delete_action_triggers<T: transport::Transport>(
    transport: &T,
    request: &DeleteActionTriggers,
) -> Result<DeleteActionTriggersResponse, transport::Error> {
    transport::request(transport, request).await
}

// Modifies existing action triggers. The modify action triggers operation is
// atomic. If a service provider can not modify all of requested action trigger
// configurations, the service provider responds with a fault message.
pub async fn modify_action_triggers<T: transport::Transport>(
    transport: &T,
    request: &ModifyActionTriggers,
) -> Result<ModifyActionTriggersResponse, transport::Error> {
    transport::request(transport, request).await
}
