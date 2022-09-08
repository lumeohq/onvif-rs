#![allow(clippy::derive_partial_eq_without_eq)]

use onvif as tt;
use types as pt;
use validate::Validate;
use yaserde_derive::{YaDeserialize, YaSerialize};

// The service capabilities reflect optional functionality of a service. The
// information is static
// and does not change during device operation. The following capabilities are
// available:
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct ServiceCapabilities {
    // The maximum number of entries returned by a single Get<Entity>List or
    // Get<Entity>
    // request.
    // The device shall never return more than this number of entities in a
    // single response.
    #[yaserde(attribute, rename = "MaxLimit")]
    pub max_limit: pt::PositiveInteger,

    // Indicates the maximum number of authentication profiles the device
    // supports. The device
    // shall
    // support at least one authentication profile.
    #[yaserde(attribute, rename = "MaxAuthenticationProfiles")]
    pub max_authentication_profiles: pt::PositiveInteger,

    // Indicates the maximum number of authentication policies per
    // authentication profile supported
    // by the device.
    #[yaserde(attribute, rename = "MaxPoliciesPerAuthenticationProfile")]
    pub max_policies_per_authentication_profile: pt::PositiveInteger,

    // Indicates the maximum number of security levels the device supports. The
    // device shall
    // support at least one
    // security level.
    #[yaserde(attribute, rename = "MaxSecurityLevels")]
    pub max_security_levels: pt::PositiveInteger,

    // Indicates the maximum number of recognition groups per security level
    // supported by the
    // device.
    #[yaserde(attribute, rename = "MaxRecognitionGroupsPerSecurityLevel")]
    pub max_recognition_groups_per_security_level: pt::PositiveInteger,

    // Indicates the maximum number of recognition methods per recognition group
    // supported by the
    // device.
    #[yaserde(attribute, rename = "MaxRecognitionMethodsPerRecognitionGroup")]
    pub max_recognition_methods_per_recognition_group: pt::PositiveInteger,

    // Indicates that the client is allowed to supply the token when creating
    // authentication
    // profiles and
    // security levels. To enable the use of the commands
    // SetAuthenticationProfile and
    // SetSecurityLevel, the
    // value must be set to true.
    #[yaserde(attribute, rename = "ClientSuppliedTokenSupported")]
    pub client_supplied_token_supported: Option<bool>,

    // A list of supported authentication modes (including custom modes).
    // This field is optional, and when omitted, the client shall assume that
    // the
    // device supports "pt:SingleCredential" only.
    #[yaserde(attribute, rename = "SupportedAuthenticationModes")]
    pub supported_authentication_modes: Option<tt::StringAttrList>,
}

impl Validate for ServiceCapabilities {}

// pub type Capabilities = ServiceCapabilities;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct AuthenticationProfileInfo {
    // A user readable name. It shall be up to 64 characters.
    #[yaserde(prefix = "tab", rename = "Name")]
    pub name: pt::Name,

    // User readable description for the access profile. It shall be up
    // to 1024 characters.
    #[yaserde(prefix = "tab", rename = "Description")]
    pub description: Option<pt::Description>,

    pub base: pt::DataEntity,
}

impl Validate for AuthenticationProfileInfo {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct AuthenticationProfile {
    // The default security level is used if none of the authentication policies
    // has a schedule covering the time of access (or if no authentication
    // policies
    // are defined).
    #[yaserde(prefix = "tab", rename = "DefaultSecurityLevelToken")]
    pub default_security_level_token: pt::ReferenceToken,

    // Each authentication policy associates a security level with a schedule
    // (during
    // which the specified security level will be required at the access point).
    #[yaserde(prefix = "tab", rename = "AuthenticationPolicy")]
    pub authentication_policy: Vec<AuthenticationPolicy>,

    #[yaserde(prefix = "tab", rename = "Extension")]
    pub extension: Option<AuthenticationProfileExtension>,

    pub base: AuthenticationProfileInfo,
}

impl Validate for AuthenticationProfile {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct AuthenticationProfileExtension {}

impl Validate for AuthenticationProfileExtension {}

// The authentication policy is an association of a security level and a
// schedule. It defines when
// a certain security level is required to grant access to a credential holder.
// Each security
// level is given a unique priority. If authentication policies have overlapping
// schedules,
// the security level with the highest priority is used.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct AuthenticationPolicy {
    // Reference to the schedule used by the authentication policy.
    #[yaserde(prefix = "tab", rename = "ScheduleToken")]
    pub schedule_token: pt::ReferenceToken,

    // A list of security level constraint structures defining the conditions
    // for what security level to use.
    // Minimum one security level constraint must be specified.
    #[yaserde(prefix = "tab", rename = "SecurityLevelConstraint")]
    pub security_level_constraint: Vec<SecurityLevelConstraint>,

    #[yaserde(prefix = "tab", rename = "Extension")]
    pub extension: Option<AuthenticationPolicyExtension>,
}

impl Validate for AuthenticationPolicy {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct AuthenticationPolicyExtension {}

impl Validate for AuthenticationPolicyExtension {}

// This structure defines what security level should be active depending on the
// state of the
// schedule.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct SecurityLevelConstraint {
    // Corresponds to the Active field in the ScheduleState structure in
    // [ONVIF Schedule Service Specification].
    #[yaserde(prefix = "tab", rename = "ActiveRegularSchedule")]
    pub active_regular_schedule: bool,

    // Corresponds to the SpecialDay field in the ScheduleState structure in
    // [ONVIF Schedule Service Specification].
    // This field will be ignored if the device do not support special days.
    #[yaserde(prefix = "tab", rename = "ActiveSpecialDaySchedule")]
    pub active_special_day_schedule: bool,

    // Defines the mode of authentication. Authentication modes starting with
    // the prefix
    // pt: are reserved to define ONVIF-specific authentication modes. For
    // custom defined
    // authentication modes, free text can be used.
    // The following authentication modes are defined by ONVIF:
    // pt:SingleCredential - Normal mode where only one credential holder is
    // required to be granted access.
    // pt:DualCredential - Two credential holders are required to be granted
    // access
    #[yaserde(prefix = "tab", rename = "AuthenticationMode")]
    pub authentication_mode: Option<pt::Name>,

    // Reference to the security level used by the authentication policy.
    #[yaserde(prefix = "tab", rename = "SecurityLevelToken")]
    pub security_level_token: pt::ReferenceToken,

    #[yaserde(prefix = "tab", rename = "Extension")]
    pub extension: Option<SecurityLevelConstraintExtension>,
}

impl Validate for SecurityLevelConstraint {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct SecurityLevelConstraintExtension {}

impl Validate for SecurityLevelConstraintExtension {}

// Recognition is the action of identifying authorized users requesting access
// by the comparison of
// presented
// credential data with recorded credential data. A recognition method is either
// memorized,
// biometric or held
// within a physical credential. A recognition type is either a recognition
// method or a physical
// input such as
// a request-to-exit button.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct RecognitionMethod {
    // The requested type of recognition.
    #[yaserde(prefix = "tab", rename = "RecognitionType")]
    pub recognition_type: String,

    // The order value defines when this recognition method will be requested in
    // relation
    // to the other recognition methods in the same security level. A lower
    // number indicates
    // that the recognition method will be requested before recognition methods
    // with a higher number.
    #[yaserde(prefix = "tab", rename = "Order")]
    pub order: i32,

    #[yaserde(prefix = "tab", rename = "Extension")]
    pub extension: Option<RecognitionMethodExtension>,
}

impl Validate for RecognitionMethod {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct RecognitionMethodExtension {}

impl Validate for RecognitionMethodExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct RecognitionGroup {
    // A list of recognition methods to request for at the access point.
    #[yaserde(prefix = "tab", rename = "RecognitionMethod")]
    pub recognition_method: Vec<RecognitionMethod>,

    #[yaserde(prefix = "tab", rename = "Extension")]
    pub extension: Option<RecognitionGroupExtension>,
}

impl Validate for RecognitionGroup {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct RecognitionGroupExtension {}

impl Validate for RecognitionGroupExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct SecurityLevelInfo {
    // A user readable name. It shall be up to 64 characters.
    #[yaserde(prefix = "tab", rename = "Name")]
    pub name: pt::Name,

    // A higher number indicates that the security level is considered more
    // secure
    // than security levels with lower priorities. The priority is used when an
    // authentication profile have overlapping schedules with different security
    // levels. When an access point is accessed, the authentication policies are
    // walked through in priority order (highest priority first). When a
    // schedule is
    // found covering the time of access, the associated security level is used
    // and
    // processing stops. Two security levels cannot have the same priority.
    #[yaserde(prefix = "tab", rename = "Priority")]
    pub priority: i32,

    // User readable description for the access profile. It shall be up
    // to 1024 characters.
    #[yaserde(prefix = "tab", rename = "Description")]
    pub description: Option<pt::Description>,

    pub base: pt::DataEntity,
}

impl Validate for SecurityLevelInfo {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct SecurityLevel {
    // The recognition groups are used to define a logical OR between the
    // groups. Each
    // recognition group consists of one or more recognition methods.
    #[yaserde(prefix = "tab", rename = "RecognitionGroup")]
    pub recognition_group: Vec<RecognitionGroup>,

    #[yaserde(prefix = "tab", rename = "Extension")]
    pub extension: Option<SecurityLevelExtension>,

    pub base: SecurityLevelInfo,
}

impl Validate for SecurityLevel {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct SecurityLevelExtension {}

impl Validate for SecurityLevelExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct GetServiceCapabilitiesResponse {
    // The capability response message contains the requested access rules
    // service capabilities using a hierarchical XML capability structure.
    #[yaserde(prefix = "tab", rename = "Capabilities")]
    pub capabilities: ServiceCapabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct GetAuthenticationProfileInfo {
    // Tokens of AuthenticationProfileInfo items to get.
    #[yaserde(prefix = "tab", rename = "Token")]
    pub token: Vec<pt::ReferenceToken>,
}

impl Validate for GetAuthenticationProfileInfo {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct GetAuthenticationProfileInfoResponse {
    // List of AuthenticationProfileInfo items.
    #[yaserde(prefix = "tab", rename = "AuthenticationProfileInfo")]
    pub authentication_profile_info: Vec<AuthenticationProfileInfo>,
}

impl Validate for GetAuthenticationProfileInfoResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct GetAuthenticationProfileInfoList {
    // Maximum number of entries to return. If not specified, less than one
    // or higher than what the device supports, the number of items is
    // determined by the
    // device.
    #[yaserde(prefix = "tab", rename = "Limit")]
    pub limit: Option<i32>,

    // Start returning entries from this start reference. If not specified,
    // entries shall start from the beginning of the dataset.
    #[yaserde(prefix = "tab", rename = "StartReference")]
    pub start_reference: Option<String>,
}

impl Validate for GetAuthenticationProfileInfoList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct GetAuthenticationProfileInfoListResponse {
    // StartReference to use in next call to get the following items. If
    // absent, no more items to get.
    #[yaserde(prefix = "tab", rename = "NextStartReference")]
    pub next_start_reference: Option<String>,

    // List of AuthenticationProfileInfo items.
    #[yaserde(prefix = "tab", rename = "AuthenticationProfileInfo")]
    pub authentication_profile_info: Vec<AuthenticationProfileInfo>,
}

impl Validate for GetAuthenticationProfileInfoListResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct GetAuthenticationProfiles {
    // Tokens of AuthenticationProfile items to get.
    #[yaserde(prefix = "tab", rename = "Token")]
    pub token: Vec<pt::ReferenceToken>,
}

impl Validate for GetAuthenticationProfiles {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct GetAuthenticationProfilesResponse {
    // List of AuthenticationProfile items.
    #[yaserde(prefix = "tab", rename = "AuthenticationProfile")]
    pub authentication_profile: Vec<AuthenticationProfile>,
}

impl Validate for GetAuthenticationProfilesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct GetAuthenticationProfileList {
    // Maximum number of entries to return. If not specified, less than one
    // or higher than what the device supports, the number of items is
    // determined by the
    // device.
    #[yaserde(prefix = "tab", rename = "Limit")]
    pub limit: Option<i32>,

    // Start returning entries from this start reference. If not specified,
    // entries shall start from the beginning of the dataset.
    #[yaserde(prefix = "tab", rename = "StartReference")]
    pub start_reference: Option<String>,
}

impl Validate for GetAuthenticationProfileList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct GetAuthenticationProfileListResponse {
    // StartReference to use in next call to get the following items. If
    // absent, no more items to get.
    #[yaserde(prefix = "tab", rename = "NextStartReference")]
    pub next_start_reference: Option<String>,

    // List of AuthenticationProfile items.
    #[yaserde(prefix = "tab", rename = "AuthenticationProfile")]
    pub authentication_profile: Vec<AuthenticationProfile>,
}

impl Validate for GetAuthenticationProfileListResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct CreateAuthenticationProfile {
    // The AuthenticationProfile to create.
    #[yaserde(prefix = "tab", rename = "AuthenticationProfile")]
    pub authentication_profile: AuthenticationProfile,
}

impl Validate for CreateAuthenticationProfile {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct CreateAuthenticationProfileResponse {
    // The Token of created AuthenticationProfile.
    #[yaserde(prefix = "tab", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for CreateAuthenticationProfileResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct SetAuthenticationProfile {
    // The AuthenticationProfile to create or modify.
    #[yaserde(prefix = "tab", rename = "AuthenticationProfile")]
    pub authentication_profile: AuthenticationProfile,
}

impl Validate for SetAuthenticationProfile {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct SetAuthenticationProfileResponse {}

impl Validate for SetAuthenticationProfileResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct ModifyAuthenticationProfile {
    // The AuthenticationProfile to modify.
    #[yaserde(prefix = "tab", rename = "AuthenticationProfile")]
    pub authentication_profile: AuthenticationProfile,
}

impl Validate for ModifyAuthenticationProfile {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct ModifyAuthenticationProfileResponse {}

impl Validate for ModifyAuthenticationProfileResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct DeleteAuthenticationProfile {
    // The token of the AuthenticationProfile to delete.
    #[yaserde(prefix = "tab", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for DeleteAuthenticationProfile {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct DeleteAuthenticationProfileResponse {}

impl Validate for DeleteAuthenticationProfileResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct GetSecurityLevelInfo {
    // Tokens of SecurityLevelInfo items to get.
    #[yaserde(prefix = "tab", rename = "Token")]
    pub token: Vec<pt::ReferenceToken>,
}

impl Validate for GetSecurityLevelInfo {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct GetSecurityLevelInfoResponse {
    // List of SecurityLevelInfo items.
    #[yaserde(prefix = "tab", rename = "SecurityLevelInfo")]
    pub security_level_info: Vec<SecurityLevelInfo>,
}

impl Validate for GetSecurityLevelInfoResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct GetSecurityLevelInfoList {
    // Maximum number of entries to return. If not specified, less than one
    // or higher than what the device supports, the number of items is
    // determined by the
    // device.
    #[yaserde(prefix = "tab", rename = "Limit")]
    pub limit: Option<i32>,

    // Start returning entries from this start reference. If not specified,
    // entries shall start from the beginning of the dataset.
    #[yaserde(prefix = "tab", rename = "StartReference")]
    pub start_reference: Option<String>,
}

impl Validate for GetSecurityLevelInfoList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct GetSecurityLevelInfoListResponse {
    // StartReference to use in next call to get the following items. If
    // absent, no more items to get.
    #[yaserde(prefix = "tab", rename = "NextStartReference")]
    pub next_start_reference: Option<String>,

    // List of SecurityLevelInfo items.
    #[yaserde(prefix = "tab", rename = "SecurityLevelInfo")]
    pub security_level_info: Vec<SecurityLevelInfo>,
}

impl Validate for GetSecurityLevelInfoListResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct GetSecurityLevels {
    // Tokens of SecurityLevel items to get.
    #[yaserde(prefix = "tab", rename = "Token")]
    pub token: Vec<pt::ReferenceToken>,
}

impl Validate for GetSecurityLevels {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct GetSecurityLevelsResponse {
    // List of SecurityLevel items.
    #[yaserde(prefix = "tab", rename = "SecurityLevel")]
    pub security_level: Vec<SecurityLevel>,
}

impl Validate for GetSecurityLevelsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct GetSecurityLevelList {
    // Maximum number of entries to return. If not specified, less than one
    // or higher than what the device supports, the number of items is
    // determined by the
    // device.
    #[yaserde(prefix = "tab", rename = "Limit")]
    pub limit: Option<i32>,

    // Start returning entries from this start reference. If not specified,
    // entries shall start from the beginning of the dataset.
    #[yaserde(prefix = "tab", rename = "StartReference")]
    pub start_reference: Option<String>,
}

impl Validate for GetSecurityLevelList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct GetSecurityLevelListResponse {
    // StartReference to use in next call to get the following items. If
    // absent, no more items to get.
    #[yaserde(prefix = "tab", rename = "NextStartReference")]
    pub next_start_reference: Option<String>,

    // List of SecurityLevel items.
    #[yaserde(prefix = "tab", rename = "SecurityLevel")]
    pub security_level: Vec<SecurityLevel>,
}

impl Validate for GetSecurityLevelListResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct CreateSecurityLevel {
    // The SecurityLevel to create.
    #[yaserde(prefix = "tab", rename = "SecurityLevel")]
    pub security_level: SecurityLevel,
}

impl Validate for CreateSecurityLevel {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct CreateSecurityLevelResponse {
    // The Token of created SecurityLevel.
    #[yaserde(prefix = "tab", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for CreateSecurityLevelResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct SetSecurityLevel {
    // The SecurityLevel to create or modify.
    #[yaserde(prefix = "tab", rename = "SecurityLevel")]
    pub security_level: SecurityLevel,
}

impl Validate for SetSecurityLevel {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct SetSecurityLevelResponse {}

impl Validate for SetSecurityLevelResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct ModifySecurityLevel {
    // The SecurityLevel to modify.
    #[yaserde(prefix = "tab", rename = "SecurityLevel")]
    pub security_level: SecurityLevel,
}

impl Validate for ModifySecurityLevel {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct ModifySecurityLevelResponse {}

impl Validate for ModifySecurityLevelResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct DeleteSecurityLevel {
    // The token of the SecurityLevel to delete.
    #[yaserde(prefix = "tab", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for DeleteSecurityLevel {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tab",
    namespace = "tab: http://www.onvif.org/ver10/authenticationbehavior/wsdl"
)]
pub struct DeleteSecurityLevelResponse {}

impl Validate for DeleteSecurityLevelResponse {}

// This operation returns the capabilities of the authentication behavior
// service.
pub async fn get_service_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetServiceCapabilities,
) -> Result<GetServiceCapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of AuthenticationProfileInfo items matching
// the given tokens.
// The device shall ignore tokens it cannot resolve and shall return an empty
// list if there are no items
// matching the specified tokens. The device shall not return a fault in this
// case.
pub async fn get_authentication_profile_info<T: transport::Transport>(
    transport: &T,
    request: &GetAuthenticationProfileInfo,
) -> Result<GetAuthenticationProfileInfoResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of all of AuthenticationProfileInfo items
// provided by the device.
// A call to this method shall return a StartReference when not all data is
// returned and more data is
// available. The reference shall be valid for retrieving the next set of data.
// Please refer Access Control
// Service Specification for more details.
// The number of items returned shall not be greater than Limit parameter.
pub async fn get_authentication_profile_info_list<T: transport::Transport>(
    transport: &T,
    request: &GetAuthenticationProfileInfoList,
) -> Result<GetAuthenticationProfileInfoListResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns the specified AuthenticationProfile item matching the
// given tokens.
// The device shall ignore tokens it cannot resolve and shall return an empty
// list if there are no items
// matching specified tokens. The device shall not return a fault in this case.
pub async fn get_authentication_profiles<T: transport::Transport>(
    transport: &T,
    request: &GetAuthenticationProfiles,
) -> Result<GetAuthenticationProfilesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of all of AuthenticationProfile items provided
// by the device.
// A call to this method shall return a StartReference when not all data is
// returned and more data is
// available. The reference shall be valid for retrieving the next set of data.
// Please refer Access Control
// Service Specification for more details.
// The number of items returned shall not be greater the Limit parameter.
pub async fn get_authentication_profile_list<T: transport::Transport>(
    transport: &T,
    request: &GetAuthenticationProfileList,
) -> Result<GetAuthenticationProfileListResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation creates the specified authentication profile in the device.
// The token field of the AuthenticationProfile structure shall be empty and the
// device shall allocate a
// token for the authentication profile. The allocated token shall be returned
// in the response.
// If the client sends any value in the token field, the device shall return
// InvalidArgVal as a generic
// fault code.
pub async fn create_authentication_profile<T: transport::Transport>(
    transport: &T,
    request: &CreateAuthenticationProfile,
) -> Result<CreateAuthenticationProfileResponse, transport::Error> {
    transport::request(transport, request).await
}

// This method is used to synchronize an authentication profile in a client with
// the device.
// If an authentication profile with the specified token does not exist in the
// device, the authentication
// profile is
// created. If an authentication profile with the specified token exists, then
// the authentication profile
// is modified.
// A call to this method takes an AuthenticationProfile structure as input
// parameter. The token field of
// the
// AuthenticationProfile shall not be empty.
// A device that signals support for the ClientSuppliedTokenSupported capability
// shall implement this
// command.
pub async fn set_authentication_profile<T: transport::Transport>(
    transport: &T,
    request: &SetAuthenticationProfile,
) -> Result<SetAuthenticationProfileResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation modifies the specified authentication profile.
// The token of the authentication profile to modify is specified in the token
// field of the
// AuthenticationProfile
// structure and shall not be empty. All other fields in the structure shall
// overwrite the fields in the
// specified authentication profile.
pub async fn modify_authentication_profile<T: transport::Transport>(
    transport: &T,
    request: &ModifyAuthenticationProfile,
) -> Result<ModifyAuthenticationProfileResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation deletes the specified authentication profile.
// If the authentication profile is deleted, all authentication policies
// associated with the authentication
// profile
// will also be deleted.
// If it is associated with one or more entities some devices may not be able to
// delete the authentication
// profile,
// and consequently a ReferenceInUse fault shall be generated.
pub async fn delete_authentication_profile<T: transport::Transport>(
    transport: &T,
    request: &DeleteAuthenticationProfile,
) -> Result<DeleteAuthenticationProfileResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of SecurityLevelInfo items matching the given
// tokens.
// The device shall ignore tokens it cannot resolve and shall return an empty
// list if there are no items
// matching the specified tokens. The device shall not return a fault in this
// case.
pub async fn get_security_level_info<T: transport::Transport>(
    transport: &T,
    request: &GetSecurityLevelInfo,
) -> Result<GetSecurityLevelInfoResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of all of SecurityLevelInfo items provided by
// the device.
// A call to this method shall return a StartReference when not all data is
// returned and more data is
// available. The reference shall be valid for retrieving the next set of data.
// Please refer Access Control
// Service Specification for more details.
// The number of items returned shall not be greater than Limit parameter.
pub async fn get_security_level_info_list<T: transport::Transport>(
    transport: &T,
    request: &GetSecurityLevelInfoList,
) -> Result<GetSecurityLevelInfoListResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns the specified SecurityLevel item matching the given
// tokens.
// The device shall ignore tokens it cannot resolve and shall return an empty
// list if there are no items
// matching specified tokens. The device shall not return a fault in this case.
pub async fn get_security_levels<T: transport::Transport>(
    transport: &T,
    request: &GetSecurityLevels,
) -> Result<GetSecurityLevelsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of all of SecurityLevel items provided by the
// device.
// A call to this method shall return a StartReference when not all data is
// returned and more data is
// available. The reference shall be valid for retrieving the next set of data.
// Please refer Access Control
// Service Specification for more details.
// The number of items returned shall not be greater the Limit parameter.
pub async fn get_security_level_list<T: transport::Transport>(
    transport: &T,
    request: &GetSecurityLevelList,
) -> Result<GetSecurityLevelListResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation creates the specified security level in the device.
// The token field of the SecurityLevel structure shall be empty and the device
// shall allocate a
// token for the security level. The allocated token shall be returned in the
// response.
// If the client sends any value in the token field, the device shall return
// InvalidArgVal as a generic
// fault code.
pub async fn create_security_level<T: transport::Transport>(
    transport: &T,
    request: &CreateSecurityLevel,
) -> Result<CreateSecurityLevelResponse, transport::Error> {
    transport::request(transport, request).await
}

// This method is used to synchronize an security level in a client with the
// device.
// If an security level with the specified token does not exist in the device,
// the security level is
// created. If an security level with the specified token exists, then the
// security level is modified.
// A call to this method takes an SecurityLevel structure as input parameter.
// The token field of the
// SecurityLevel shall not be empty.
// A device that signals support for the ClientSuppliedTokenSupported capability
// shall implement this
// command.
pub async fn set_security_level<T: transport::Transport>(
    transport: &T,
    request: &SetSecurityLevel,
) -> Result<SetSecurityLevelResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation modifies the specified security level.
// The token of the security level to modify is specified in the token field of
// the SecurityLevel
// structure and shall not be empty. All other fields in the structure shall
// overwrite the fields in the
// specified security level.
pub async fn modify_security_level<T: transport::Transport>(
    transport: &T,
    request: &ModifySecurityLevel,
) -> Result<ModifySecurityLevelResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation deletes the specified security level.
// If the security level is deleted, all authentication policies associated with
// the security level
// will also be deleted.
// If it is associated with one or more entities some devices may not be able to
// delete the security level,
// and consequently a ReferenceInUse fault shall be generated.
pub async fn delete_security_level<T: transport::Transport>(
    transport: &T,
    request: &DeleteSecurityLevel,
) -> Result<DeleteSecurityLevelResponse, transport::Error> {
    transport::request(transport, request).await
}
