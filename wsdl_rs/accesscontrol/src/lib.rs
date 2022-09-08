#![allow(clippy::derive_partial_eq_without_eq)]

use types as pt;
use validate::Validate;
use yaserde_derive::{YaDeserialize, YaSerialize};

// The service capabilities reflect optional functionality of a service.
// The information is static and does not change during device operation.
// The following capabilities are available:
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct ServiceCapabilities {
    // The maximum number of entries returned by a single Get<Entity>List or
    // Get<Entity> request.
    // The device shall never return more than this number of entities in a
    // single response.
    #[yaserde(attribute, rename = "MaxLimit")]
    pub max_limit: u32,

    // Indicates the maximum number of access points supported by the device.
    #[yaserde(attribute, rename = "MaxAccessPoints")]
    pub max_access_points: Option<u32>,

    // Indicates the maximum number of areas supported by the device.
    #[yaserde(attribute, rename = "MaxAreas")]
    pub max_areas: Option<u32>,

    // Indicates that the client is allowed to supply the token when creating
    // access
    // points and areas.
    // To enable the use of the commands SetAccessPoint and SetArea, the value
    // must be set to true.
    #[yaserde(attribute, rename = "ClientSuppliedTokenSupported")]
    pub client_supplied_token_supported: Option<bool>,
}

impl Validate for ServiceCapabilities {}

// pub type Capabilities = ServiceCapabilities;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct AccessPointInfoBase {
    // A user readable name. It shall be up to 64 characters.
    #[yaserde(prefix = "tac", rename = "Name")]
    pub name: pt::Name,

    // Optional user readable description for the AccessPoint. It shall
    // be up to 1024 characters.
    #[yaserde(prefix = "tac", rename = "Description")]
    pub description: Option<pt::Description>,

    // Optional reference to the Area from which access is requested.
    #[yaserde(prefix = "tac", rename = "AreaFrom")]
    pub area_from: Option<pt::ReferenceToken>,

    // Optional reference to the Area to which access is requested.
    #[yaserde(prefix = "tac", rename = "AreaTo")]
    pub area_to: Option<pt::ReferenceToken>,

    // Optional entity type; if missing, a Door type as defined by [ONVIF Door
    // Control
    // Service Specification] should be assumed. This can also be represented by
    // the
    // QName value "tdc:Door" – where tdc is the namespace of the door control
    // service:
    // "http://www.onvif.org/ver10/doorcontrol/wsdl". This field is provided for
    // future
    // extensions; it will allow an access point being extended to cover entity
    // types
    // other than doors as well.
    #[yaserde(prefix = "tac", rename = "EntityType")]
    pub entity_type: Option<String>,

    // Reference to the entity used to control access; the entity type
    // may be specified by the optional EntityType field explained below but is
    // typically a Door.
    #[yaserde(prefix = "tac", rename = "Entity")]
    pub entity: pt::ReferenceToken,

    pub base: pt::DataEntity,
}

impl Validate for AccessPointInfoBase {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct AccessPointInfo {
    // The capabilities for the AccessPoint.
    #[yaserde(prefix = "tac", rename = "Capabilities")]
    pub capabilities: AccessPointCapabilities,

    pub base: AccessPointInfoBase,
}

impl Validate for AccessPointInfo {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct AccessPoint {
    // A reference to an authentication profile which defines the authentication
    // behavior of the access point.
    #[yaserde(prefix = "tac", rename = "AuthenticationProfileToken")]
    pub authentication_profile_token: Option<pt::ReferenceToken>,

    #[yaserde(prefix = "tac", rename = "Extension")]
    pub extension: Option<AccessPointExtension>,

    pub base: AccessPointInfo,
}

impl Validate for AccessPoint {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct AccessPointExtension {}

impl Validate for AccessPointExtension {}

// The AccessPoint capabilities reflect optional functionality of a particular
// physical entity.
// Different AccessPoint instances may have different set of capabilities. This
// information may
// change during device operation, e.g. if hardware settings are changed.
// The following capabilities are available:
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct AccessPointCapabilities {
    // A list of security level tokens that this access point supports.
    // See [Authentication Behavior Service Specification].
    #[yaserde(prefix = "tac", rename = "SupportedSecurityLevels")]
    pub supported_security_levels: Vec<pt::ReferenceToken>,

    #[yaserde(prefix = "tac", rename = "Extension")]
    pub extension: Option<SupportedSecurityLevelsExtension>,

    // Indicates whether or not this AccessPoint instance supports
    // EnableAccessPoint
    // and DisableAccessPoint commands.
    #[yaserde(attribute, rename = "DisableAccessPoint")]
    pub disable_access_point: bool,

    // Indicates whether or not this AccessPoint instance supports generation of
    // duress events.
    #[yaserde(attribute, rename = "Duress")]
    pub duress: Option<bool>,

    // Indicates whether or not this AccessPoint has a REX switch or other input
    // that
    // allows anonymous access.
    #[yaserde(attribute, rename = "AnonymousAccess")]
    pub anonymous_access: Option<bool>,

    // Indicates whether or not this AccessPoint instance supports generation of
    // AccessTaken and AccessNotTaken events. If AnonymousAccess and AccessTaken
    // are both true, it
    // indicates that the Anonymous versions of AccessTaken and AccessNotTaken
    // are supported.
    #[yaserde(attribute, rename = "AccessTaken")]
    pub access_taken: Option<bool>,

    // Indicates whether or not this AccessPoint instance supports the
    // ExternalAuthorization operation and the generation of Request events. If
    // AnonymousAccess and
    // ExternalAuthorization are both true, it indicates that the Anonymous
    // version is supported as
    // well.
    #[yaserde(attribute, rename = "ExternalAuthorization")]
    pub external_authorization: Option<bool>,
}

impl Validate for AccessPointCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct SupportedSecurityLevelsExtension {}

impl Validate for SupportedSecurityLevelsExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct AreaInfoBase {
    // User readable name. It shall be up to 64 characters.
    #[yaserde(prefix = "tac", rename = "Name")]
    pub name: pt::Name,

    // User readable description for the Area. It shall be up to 1024
    // characters.
    #[yaserde(prefix = "tac", rename = "Description")]
    pub description: Option<pt::Description>,

    pub base: pt::DataEntity,
}

impl Validate for AreaInfoBase {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct AreaInfo {
    pub base: AreaInfoBase,
}

impl Validate for AreaInfo {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct Area {
    #[yaserde(prefix = "tac", rename = "Extension")]
    pub extension: Option<AreaExtension>,

    pub base: AreaInfo,
}

impl Validate for Area {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct AreaExtension {}

impl Validate for AreaExtension {}

// The AccessPointState contains state information for an AccessPoint.
// An ONVIF compliant device shall provide the following fields for each
// AccessPoint instance:
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct AccessPointState {
    // Indicates that the AccessPoint is enabled. By default this field value
    // shall be True, if the DisableAccessPoint capabilities is not supported.
    #[yaserde(prefix = "tac", rename = "Enabled")]
    pub enabled: bool,
}

impl Validate for AccessPointState {}

// The Decision enumeration represents a choice of two available options for an
// access request:
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum Decision {
    // The decision is to grant access.
    Granted,
    // The decision is to deny access.
    Denied,
    __Unknown__(String),
}

impl Default for Decision {
    fn default() -> Decision {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for Decision {}

// Non-normative enum that describes the various reasons for denying access.
// The following strings shall be used for the reason field:
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum DenyReason {
    // The device shall provide the following event, whenever a valid credential
    // is not enabled or has been disabled (e.g., due to credential being lost
    // etc.) to prevent
    // unauthorized entry.
    CredentialNotEnabled,
    // The device shall provide the following event, whenever a valid credential
    // is presented though it is not active yet;: e.g, the credential was
    // presented before the
    // start date.
    CredentialNotActive,
    // The device shall provide the following event, whenever a valid credential
    // was presented after its expiry date.
    CredentialExpired,
    // The device shall provide the following event, whenever an entered PIN
    // code
    // does not match the credential.
    InvalidPIN,
    // The device shall provide the following event, whenever a valid credential
    // is denied access to the requested AccessPoint because the credential is
    // not permitted at
    // the moment.
    NotPermittedAtThisTime,
    // The device shall provide the following event, whenever the presented
    // credential is not authorized.
    Unauthorized,
    // The device shall provide the following event, whenever the request is
    // denied and no other specific event matches it or is supported by the
    // service.
    Other,
    __Unknown__(String),
}

impl Default for DenyReason {
    fn default() -> DenyReason {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for DenyReason {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct GetServiceCapabilitiesResponse {
    // The capability response message contains the requested Access Control
    // service capabilities using a hierarchical XML capability structure.
    #[yaserde(prefix = "tac", rename = "Capabilities")]
    pub capabilities: ServiceCapabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct GetAccessPointInfoList {
    // Maximum number of entries to return. If not specified, less than one
    // or higher than what the device supports, the number of items is
    // determined by the
    // device.
    #[yaserde(prefix = "tac", rename = "Limit")]
    pub limit: Option<i32>,

    // Start returning entries from this start reference. If not specified,
    // entries shall start from the beginning of the dataset.
    #[yaserde(prefix = "tac", rename = "StartReference")]
    pub start_reference: Option<String>,
}

impl Validate for GetAccessPointInfoList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct GetAccessPointInfoListResponse {
    // StartReference to use in next call to get the following items. If
    // absent, no more items to get.
    #[yaserde(prefix = "tac", rename = "NextStartReference")]
    pub next_start_reference: Option<String>,

    // List of AccessPointInfo items.
    #[yaserde(prefix = "tac", rename = "AccessPointInfo")]
    pub access_point_info: Vec<AccessPointInfo>,
}

impl Validate for GetAccessPointInfoListResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct GetAccessPointInfo {
    // Tokens of AccessPointInfo items to get.
    #[yaserde(prefix = "tac", rename = "Token")]
    pub token: Vec<pt::ReferenceToken>,
}

impl Validate for GetAccessPointInfo {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct GetAccessPointInfoResponse {
    // List of AccessPointInfo items.
    #[yaserde(prefix = "tac", rename = "AccessPointInfo")]
    pub access_point_info: Vec<AccessPointInfo>,
}

impl Validate for GetAccessPointInfoResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct GetAccessPointList {
    // Maximum number of entries to return. If not specified, less than one
    // or higher than what the device supports, the number of items is
    // determined by the
    // device.
    #[yaserde(prefix = "tac", rename = "Limit")]
    pub limit: Option<i32>,

    // Start returning entries from this start reference. If not specified,
    // entries shall start from the beginning of the dataset.
    #[yaserde(prefix = "tac", rename = "StartReference")]
    pub start_reference: Option<String>,
}

impl Validate for GetAccessPointList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct GetAccessPointListResponse {
    // StartReference to use in next call to get the following items. If
    // absent, no more items to get.
    #[yaserde(prefix = "tac", rename = "NextStartReference")]
    pub next_start_reference: Option<String>,

    // List of AccessPoint items.
    #[yaserde(prefix = "tac", rename = "AccessPoint")]
    pub access_point: Vec<AccessPoint>,
}

impl Validate for GetAccessPointListResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct GetAccessPoints {
    // Tokens of AccessPoint items to get.
    #[yaserde(prefix = "tac", rename = "Token")]
    pub token: Vec<pt::ReferenceToken>,
}

impl Validate for GetAccessPoints {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct GetAccessPointsResponse {
    // List of AccessPoint items.
    #[yaserde(prefix = "tac", rename = "AccessPoint")]
    pub access_point: Vec<AccessPoint>,
}

impl Validate for GetAccessPointsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct CreateAccessPoint {
    // AccessPoint item to create
    #[yaserde(prefix = "tac", rename = "AccessPoint")]
    pub access_point: AccessPoint,
}

impl Validate for CreateAccessPoint {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct CreateAccessPointResponse {
    // Token of created AccessPoint item
    #[yaserde(prefix = "tac", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for CreateAccessPointResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct SetAccessPoint {
    // AccessPoint item to create or modify
    #[yaserde(prefix = "tac", rename = "AccessPoint")]
    pub access_point: AccessPoint,
}

impl Validate for SetAccessPoint {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct SetAccessPointResponse {}

impl Validate for SetAccessPointResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct ModifyAccessPoint {
    // AccessPoint item to modify
    #[yaserde(prefix = "tac", rename = "AccessPoint")]
    pub access_point: AccessPoint,
}

impl Validate for ModifyAccessPoint {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct ModifyAccessPointResponse {}

impl Validate for ModifyAccessPointResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct DeleteAccessPoint {
    // Token of AccessPoint item to delete.
    #[yaserde(prefix = "tac", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for DeleteAccessPoint {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct DeleteAccessPointResponse {}

impl Validate for DeleteAccessPointResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct SetAccessPointAuthenticationProfile {
    // Token of the AccessPoint.
    #[yaserde(prefix = "tac", rename = "Token")]
    pub token: pt::ReferenceToken,

    // Token of the AuthenticationProfile.
    #[yaserde(prefix = "tac", rename = "AuthenticationProfileToken")]
    pub authentication_profile_token: pt::ReferenceToken,
}

impl Validate for SetAccessPointAuthenticationProfile {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct SetAccessPointAuthenticationProfileResponse {}

impl Validate for SetAccessPointAuthenticationProfileResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct DeleteAccessPointAuthenticationProfile {
    // Token of the AccessPoint.
    #[yaserde(prefix = "tac", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for DeleteAccessPointAuthenticationProfile {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct DeleteAccessPointAuthenticationProfileResponse {}

impl Validate for DeleteAccessPointAuthenticationProfileResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct GetAreaInfoList {
    // Maximum number of entries to return. If not specified, less than one
    // or higher than what the device supports, the number of items is
    // determined by the
    // device.
    #[yaserde(prefix = "tac", rename = "Limit")]
    pub limit: Option<i32>,

    // Start returning entries from this start reference. If not specified,
    // entries shall start from the beginning of the dataset.
    #[yaserde(prefix = "tac", rename = "StartReference")]
    pub start_reference: Option<String>,
}

impl Validate for GetAreaInfoList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct GetAreaInfoListResponse {
    // StartReference to use in next call to get the following items. If
    // absent, no more items to get.
    #[yaserde(prefix = "tac", rename = "NextStartReference")]
    pub next_start_reference: Option<String>,

    // List of AreaInfo items.
    #[yaserde(prefix = "tac", rename = "AreaInfo")]
    pub area_info: Vec<AreaInfo>,
}

impl Validate for GetAreaInfoListResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct GetAreaInfo {
    // Tokens of AreaInfo items to get.
    #[yaserde(prefix = "tac", rename = "Token")]
    pub token: Vec<pt::ReferenceToken>,
}

impl Validate for GetAreaInfo {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct GetAreaInfoResponse {
    // List of AreaInfo items.
    #[yaserde(prefix = "tac", rename = "AreaInfo")]
    pub area_info: Vec<AreaInfo>,
}

impl Validate for GetAreaInfoResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct GetAreaList {
    // Maximum number of entries to return. If not specified, less than one
    // or higher than what the device supports, the number of items is
    // determined by the
    // device.
    #[yaserde(prefix = "tac", rename = "Limit")]
    pub limit: Option<i32>,

    // Start returning entries from this start reference. If not specified,
    // entries shall start from the beginning of the dataset.
    #[yaserde(prefix = "tac", rename = "StartReference")]
    pub start_reference: Option<String>,
}

impl Validate for GetAreaList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct GetAreaListResponse {
    // StartReference to use in next call to get the following items. If
    // absent, no more items to get.
    #[yaserde(prefix = "tac", rename = "NextStartReference")]
    pub next_start_reference: Option<String>,

    // List of Area items.
    #[yaserde(prefix = "tac", rename = "Area")]
    pub area: Vec<Area>,
}

impl Validate for GetAreaListResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct GetAreas {
    // Tokens of Area items to get.
    #[yaserde(prefix = "tac", rename = "Token")]
    pub token: Vec<pt::ReferenceToken>,
}

impl Validate for GetAreas {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct GetAreasResponse {
    // List of Area items.
    #[yaserde(prefix = "tac", rename = "Area")]
    pub area: Vec<Area>,
}

impl Validate for GetAreasResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct CreateArea {
    // Area item to create
    #[yaserde(prefix = "tac", rename = "Area")]
    pub area: Area,
}

impl Validate for CreateArea {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct CreateAreaResponse {
    // Token of created Area item
    #[yaserde(prefix = "tac", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for CreateAreaResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct SetArea {
    // Area item to create or modify
    #[yaserde(prefix = "tac", rename = "Area")]
    pub area: Area,
}

impl Validate for SetArea {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct SetAreaResponse {}

impl Validate for SetAreaResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct ModifyArea {
    // Area item to modify
    #[yaserde(prefix = "tac", rename = "Area")]
    pub area: Area,
}

impl Validate for ModifyArea {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct ModifyAreaResponse {}

impl Validate for ModifyAreaResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct DeleteArea {
    // Token of Area item to delete.
    #[yaserde(prefix = "tac", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for DeleteArea {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct DeleteAreaResponse {}

impl Validate for DeleteAreaResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct GetAccessPointState {
    // Token of AccessPoint instance to get AccessPointState for.
    #[yaserde(prefix = "tac", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for GetAccessPointState {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct GetAccessPointStateResponse {
    // AccessPointState item.
    #[yaserde(prefix = "tac", rename = "AccessPointState")]
    pub access_point_state: AccessPointState,
}

impl Validate for GetAccessPointStateResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct EnableAccessPoint {
    // Token of the AccessPoint instance to enable.
    #[yaserde(prefix = "tac", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for EnableAccessPoint {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct EnableAccessPointResponse {}

impl Validate for EnableAccessPointResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct DisableAccessPoint {
    // Token of the AccessPoint instance to disable.
    #[yaserde(prefix = "tac", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for DisableAccessPoint {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct DisableAccessPointResponse {}

impl Validate for DisableAccessPointResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct ExternalAuthorization {
    // Token of the Access Point instance.
    #[yaserde(prefix = "tac", rename = "AccessPointToken")]
    pub access_point_token: pt::ReferenceToken,

    // Optional token of the Credential involved.
    #[yaserde(prefix = "tac", rename = "CredentialToken")]
    pub credential_token: Option<pt::ReferenceToken>,

    // Optional reason for decision.
    #[yaserde(prefix = "tac", rename = "Reason")]
    pub reason: Option<String>,

    // Decision - Granted or Denied.
    #[yaserde(prefix = "tac", rename = "Decision")]
    pub decision: Decision,
}

impl Validate for ExternalAuthorization {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tac",
    namespace = "tac: http://www.onvif.org/ver10/accesscontrol/wsdl"
)]
pub struct ExternalAuthorizationResponse {}

impl Validate for ExternalAuthorizationResponse {}

// This operation returns the capabilities of the access control service.
// A device which provides the access control service shall implement this
// method.
pub async fn get_service_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetServiceCapabilities,
) -> Result<GetServiceCapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of AccessPointInfo items matching the given
// tokens.
// The device shall ignore tokens it cannot resolve and shall return an empty
// list if
// there are no items matching the specified tokens.
// The device shall not return a fault in this case.
// If the number of requested items is greater than MaxLimit, a TooManyItems
// fault shall be returned.
pub async fn get_access_point_info<T: transport::Transport>(
    transport: &T,
    request: &GetAccessPointInfo,
) -> Result<GetAccessPointInfoResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of all AccessPointInfo items provided by the
// device.
// A call to this method shall return a StartReference when not all data is
// returned and more
// data is available. The reference shall be valid for retrieving the next set
// of data.
// Please refer to section 4.8.3 in [ONVIF PACS Architecture and Design
// Considerations] for more details.
// The number of items returned shall not be greater than the Limit parameter.
pub async fn get_access_point_info_list<T: transport::Transport>(
    transport: &T,
    request: &GetAccessPointInfoList,
) -> Result<GetAccessPointInfoListResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of AccessPoint items matching the given
// tokens.
// The device shall ignore tokens it cannot resolve and shall return an empty
// list if there are
// no items matching the specified tokens. The device shall not return a fault
// in this case.
// If the number of requested items is greater than MaxLimit, a TooManyItems
// fault shall be returned.
pub async fn get_access_points<T: transport::Transport>(
    transport: &T,
    request: &GetAccessPoints,
) -> Result<GetAccessPointsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of all AccessPoint items provided by the
// device. A call to
// this method shall return a StartReference when not all data is returned and
// more data is available.
// The reference shall be valid for retrieving the next set of data.
// The number of items returned shall not be greater than the Limit parameter.
pub async fn get_access_point_list<T: transport::Transport>(
    transport: &T,
    request: &GetAccessPointList,
) -> Result<GetAccessPointListResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation creates the specified access point in the device. The token
// field of the
// AccessPoint structure shall be empty and the device shall allocate a token
// for the access point.
// The allocated token shall be returned in the response. If the client sends
// any value in the
// token field, the device shall return InvalidArgVal as a generic fault code.
pub async fn create_access_point<T: transport::Transport>(
    transport: &T,
    request: &CreateAccessPoint,
) -> Result<CreateAccessPointResponse, transport::Error> {
    transport::request(transport, request).await
}

// This method is used to synchronize an access point in a client with the
// device. If an access
// point with the specified token does not exist in the device, the access point
// is created.
// If an access point with the specified token exists, then the access point is
// modified.
// A call to this method takes an AccessPoint structure as input parameter.
// The token field of the AccessPoint structure shall not be empty.
// A device that signals support for the ClientSuppliedTokenSupported capability
// shall implement this command.
// If no token was specified in the request, the device shall return InvalidArgs
// as a generic fault code.
pub async fn set_access_point<T: transport::Transport>(
    transport: &T,
    request: &SetAccessPoint,
) -> Result<SetAccessPointResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation modifies the specified access point. The token of the access
// point to modify
// is specified in the token field of the AccessPoint structure and shall not be
// empty.
// All other fields in the structure shall overwrite the fields in the specified
// access point.
// If no token was specified in the request, the device shall return InvalidArgs
// as a generic fault code.
pub async fn modify_access_point<T: transport::Transport>(
    transport: &T,
    request: &ModifyAccessPoint,
) -> Result<ModifyAccessPointResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation deletes the specified access point. If it is associated with
// one or more
// entities some devices may not be able to delete the access point, and
// consequently a
// ReferenceInUse fault shall be generated.
// If no token was specified in the request, the device shall return InvalidArgs
// as a generic fault code.
pub async fn delete_access_point<T: transport::Transport>(
    transport: &T,
    request: &DeleteAccessPoint,
) -> Result<DeleteAccessPointResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation defines the authentication behavior for an access point.
pub async fn set_access_point_authentication_profile<T: transport::Transport>(
    transport: &T,
    request: &SetAccessPointAuthenticationProfile,
) -> Result<SetAccessPointAuthenticationProfileResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation reverts the authentication behavior for an access point to its
// default behavior.
pub async fn delete_access_point_authentication_profile<T: transport::Transport>(
    transport: &T,
    request: &DeleteAccessPointAuthenticationProfile,
) -> Result<DeleteAccessPointAuthenticationProfileResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of AreaInfo items matching the given tokens.
// The device shall
// ignore tokens it cannot resolve and shall return an empty list if there are
// no items
// matching the specified tokens. The device shall not return a fault in this
// case.
// If the number of requested items is greater than MaxLimit, a TooManyItems
// fault shall be returned.
pub async fn get_area_info<T: transport::Transport>(
    transport: &T,
    request: &GetAreaInfo,
) -> Result<GetAreaInfoResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of all AreaInfo items provided by the device.
// A call to this
// method shall return a StartReference when not all data is returned and more
// data is available.
// The reference shall be valid for retrieving the next set of data.
// The number of items returned shall not be greater than the Limit parameter.
pub async fn get_area_info_list<T: transport::Transport>(
    transport: &T,
    request: &GetAreaInfoList,
) -> Result<GetAreaInfoListResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of Area items matching the given tokens. The
// device shall
// ignore tokens it cannot resolve and shall return an empty list if there are
// no items
// matching the specified tokens. The device shall not return a fault in this
// case.
// If the number of requested items is greater than MaxLimit, a TooManyItems
// fault shall be returned.
pub async fn get_areas<T: transport::Transport>(
    transport: &T,
    request: &GetAreas,
) -> Result<GetAreasResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of all Area items provided by the device. A
// call to this
// method shall return a StartReference when not all data is returned and more
// data is available.
// The reference shall be valid for retrieving the next set of data.
// The number of items returned shall not be greater than the Limit parameter.
pub async fn get_area_list<T: transport::Transport>(
    transport: &T,
    request: &GetAreaList,
) -> Result<GetAreaListResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation creates the specified area in the device. The token field of
// the Area
// structure shall be empty and the device shall allocate a token for the area.
// The allocated token shall be returned in the response.
// If the client sends any value in the token field, the device shall return
// InvalidArgVal as a generic fault code.
pub async fn create_area<T: transport::Transport>(
    transport: &T,
    request: &CreateArea,
) -> Result<CreateAreaResponse, transport::Error> {
    transport::request(transport, request).await
}

// This method is used to synchronize an area in a client with the device. If an
// area with the
// specified token does not exist in the device, the area is created. If an area
// with the
// specified token exists, then the area is modified. A call to this method
// takes an Area
// structure as input parameter. The token field of the Area structure shall not
// be empty.
// A device that signals support for the ClientSuppliedTokenSupported capability
// shall
// implement this command.
// If no token was specified in the request, the device shall return
// InvalidArgs as a generic fault code.
pub async fn set_area<T: transport::Transport>(
    transport: &T,
    request: &SetArea,
) -> Result<SetAreaResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation modifies the specified area. The token of the area to modify
// is specified in
// the token field of the Area structure and shall not be empty. All other
// fields in the
// structure shall overwrite the fields in the specified area.
// If no token was specified in the request, the device shall return InvalidArgs
// as a generic fault code.
pub async fn modify_area<T: transport::Transport>(
    transport: &T,
    request: &ModifyArea,
) -> Result<ModifyAreaResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation deletes the specified area. If it is associated with one or
// more entities
// some devices may not be able to delete the area, and consequently a
// ReferenceInUse fault shall be generated.
// If no token was specified in the request, the device shall return InvalidArgs
// as a generic fault code.
pub async fn delete_area<T: transport::Transport>(
    transport: &T,
    request: &DeleteArea,
) -> Result<DeleteAreaResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests the AccessPointState for the access point instance
// specified by the token.
pub async fn get_access_point_state<T: transport::Transport>(
    transport: &T,
    request: &GetAccessPointState,
) -> Result<GetAccessPointStateResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation allows enabling an access point. A device that signals support
// for
// DisableAccessPoint capability for a particular access point instance shall
// implement this command.
pub async fn enable_access_point<T: transport::Transport>(
    transport: &T,
    request: &EnableAccessPoint,
) -> Result<EnableAccessPointResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation allows disabling an access point. A device that signals
// support for the
// DisableAccessPoint capability for a particular access point instance shall
// implement this command.
pub async fn disable_access_point<T: transport::Transport>(
    transport: &T,
    request: &DisableAccessPoint,
) -> Result<DisableAccessPointResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation allows to deny or grant decision at an access point instance.
// A device that
// signals support for ExternalAuthorization capability for a particular access
// point instance
// shall implement this method.
pub async fn external_authorization<T: transport::Transport>(
    transport: &T,
    request: &ExternalAuthorization,
) -> Result<ExternalAuthorizationResponse, transport::Error> {
    transport::request(transport, request).await
}
