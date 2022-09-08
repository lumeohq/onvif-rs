#![allow(clippy::derive_partial_eq_without_eq)]

use types as pt;
use validate::Validate;
use yaserde_derive::{YaDeserialize, YaSerialize};

// The service capabilities reflect optional functionality of a service. The
// information is static
// and does not change during device operation. The following capabilities are
// available:
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tar",
    namespace = "tar: http://www.onvif.org/ver10/accessrules/wsdl"
)]
pub struct ServiceCapabilities {
    // The maximum number of entries returned by a single Get<Entity>List or
    // Get<Entity>
    // request. The device shall never return more than this number of entities
    // in a single
    // response.
    #[yaserde(attribute, rename = "MaxLimit")]
    pub max_limit: String,

    // Indicates the maximum number of access profiles supported by the device.
    #[yaserde(attribute, rename = "MaxAccessProfiles")]
    pub max_access_profiles: String,

    // Indicates the maximum number of access policies per access profile
    // supported by the device.
    #[yaserde(attribute, rename = "MaxAccessPoliciesPerAccessProfile")]
    pub max_access_policies_per_access_profile: String,

    // Indicates whether or not several access policies can refer to the same
    // access point in an
    // access profile.
    #[yaserde(attribute, rename = "MultipleSchedulesPerAccessPointSupported")]
    pub multiple_schedules_per_access_point_supported: bool,

    // Indicates that the client is allowed to supply the token when creating
    // access profiles. To
    // enable the use of the command SetAccessProfile, the value must be set to
    // true.
    #[yaserde(attribute, rename = "ClientSuppliedTokenSupported")]
    pub client_supplied_token_supported: Option<bool>,
}

impl Validate for ServiceCapabilities {}

// pub type Capabilities = ServiceCapabilities;
// The access policy is an association of an access point and a schedule. It
// defines when an access
// point can be accessed using an access profile which contains this access
// policy. If an access
// profile contains several access policies specifying different schedules for
// the same access
// point will result in a union of the schedules.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tar",
    namespace = "tar: http://www.onvif.org/ver10/accessrules/wsdl"
)]
pub struct AccessPolicy {
    // Reference to the schedule used by the access policy.
    #[yaserde(prefix = "tar", rename = "ScheduleToken")]
    pub schedule_token: pt::ReferenceToken,

    // Reference to the entity used by the rule engine, the entity type may be
    // specified by the
    // optional EntityType field explained below but is typically an access
    // point.
    #[yaserde(prefix = "tar", rename = "Entity")]
    pub entity: pt::ReferenceToken,

    // Optional entity type; if missing, an access point type as defined by the
    // ONVIF Access
    // Control Service Specification should be assumed. This can also be
    // represented by the
    // QName value “tac:AccessPoint” where tac is the namespace of ONVIF
    // Access Control
    // Service Specification. This field is provided for future extensions; it
    // will allow an
    // access policy being extended to cover entity types other than access
    // points as well.
    #[yaserde(prefix = "tar", rename = "EntityType")]
    pub entity_type: Option<String>,

    #[yaserde(prefix = "tar", rename = "Extension")]
    pub extension: Option<AccessPolicyExtension>,
}

impl Validate for AccessPolicy {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tar",
    namespace = "tar: http://www.onvif.org/ver10/accessrules/wsdl"
)]
pub struct AccessPolicyExtension {}

impl Validate for AccessPolicyExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tar",
    namespace = "tar: http://www.onvif.org/ver10/accessrules/wsdl"
)]
pub struct AccessProfileInfo {
    // A user readable name. It shall be up to 64 characters.
    #[yaserde(prefix = "tar", rename = "Name")]
    pub name: pt::Name,

    // User readable description for the access profile. It shall be up
    // to 1024 characters.
    #[yaserde(prefix = "tar", rename = "Description")]
    pub description: Option<pt::Description>,

    pub base: pt::DataEntity,
}

impl Validate for AccessProfileInfo {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tar",
    namespace = "tar: http://www.onvif.org/ver10/accessrules/wsdl"
)]
pub struct AccessProfile {
    // A list of access policy structures, where each access policy
    // defines during which schedule an access point can be accessed.
    #[yaserde(prefix = "tar", rename = "AccessPolicy")]
    pub access_policy: Vec<AccessPolicy>,

    #[yaserde(prefix = "tar", rename = "Extension")]
    pub extension: Option<AccessProfileExtension>,

    pub base: AccessProfileInfo,
}

impl Validate for AccessProfile {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tar",
    namespace = "tar: http://www.onvif.org/ver10/accessrules/wsdl"
)]
pub struct AccessProfileExtension {}

impl Validate for AccessProfileExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tar",
    namespace = "tar: http://www.onvif.org/ver10/accessrules/wsdl"
)]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tar",
    namespace = "tar: http://www.onvif.org/ver10/accessrules/wsdl"
)]
pub struct GetServiceCapabilitiesResponse {
    // The capability response message contains the requested access rules
    // service capabilities using a hierarchical XML capability structure.
    #[yaserde(prefix = "tar", rename = "Capabilities")]
    pub capabilities: ServiceCapabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tar",
    namespace = "tar: http://www.onvif.org/ver10/accessrules/wsdl"
)]
pub struct GetAccessProfileInfo {
    // Tokens of AccessProfileInfo items to get.
    #[yaserde(prefix = "tar", rename = "Token")]
    pub token: Vec<pt::ReferenceToken>,
}

impl Validate for GetAccessProfileInfo {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tar",
    namespace = "tar: http://www.onvif.org/ver10/accessrules/wsdl"
)]
pub struct GetAccessProfileInfoResponse {
    // List of AccessProfileInfo items.
    #[yaserde(prefix = "tar", rename = "AccessProfileInfo")]
    pub access_profile_info: Vec<AccessProfileInfo>,
}

impl Validate for GetAccessProfileInfoResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tar",
    namespace = "tar: http://www.onvif.org/ver10/accessrules/wsdl"
)]
pub struct GetAccessProfileInfoList {
    // Maximum number of entries to return. If not specified, less than one
    // or higher than what the device supports, the number of items is
    // determined by the
    // device.
    #[yaserde(prefix = "tar", rename = "Limit")]
    pub limit: Option<i32>,

    // Start returning entries from this start reference. If not specified,
    // entries shall start from the beginning of the dataset.
    #[yaserde(prefix = "tar", rename = "StartReference")]
    pub start_reference: Option<String>,
}

impl Validate for GetAccessProfileInfoList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tar",
    namespace = "tar: http://www.onvif.org/ver10/accessrules/wsdl"
)]
pub struct GetAccessProfileInfoListResponse {
    // StartReference to use in next call to get the following items. If
    // absent, no more items to get.
    #[yaserde(prefix = "tar", rename = "NextStartReference")]
    pub next_start_reference: Option<String>,

    // List of AccessProfileInfo items.
    #[yaserde(prefix = "tar", rename = "AccessProfileInfo")]
    pub access_profile_info: Vec<AccessProfileInfo>,
}

impl Validate for GetAccessProfileInfoListResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tar",
    namespace = "tar: http://www.onvif.org/ver10/accessrules/wsdl"
)]
pub struct GetAccessProfiles {
    // Tokens of AccessProfile items to get.
    #[yaserde(prefix = "tar", rename = "Token")]
    pub token: Vec<pt::ReferenceToken>,
}

impl Validate for GetAccessProfiles {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tar",
    namespace = "tar: http://www.onvif.org/ver10/accessrules/wsdl"
)]
pub struct GetAccessProfilesResponse {
    // List of Access Profile items.
    #[yaserde(prefix = "tar", rename = "AccessProfile")]
    pub access_profile: Vec<AccessProfile>,
}

impl Validate for GetAccessProfilesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tar",
    namespace = "tar: http://www.onvif.org/ver10/accessrules/wsdl"
)]
pub struct GetAccessProfileList {
    // Maximum number of entries to return. If not specified, less than one
    // or higher than what the device supports, the number of items is
    // determined by the
    // device.
    #[yaserde(prefix = "tar", rename = "Limit")]
    pub limit: Option<i32>,

    // Start returning entries from this start reference. If not specified,
    // entries shall start from the beginning of the dataset.
    #[yaserde(prefix = "tar", rename = "StartReference")]
    pub start_reference: Option<String>,
}

impl Validate for GetAccessProfileList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tar",
    namespace = "tar: http://www.onvif.org/ver10/accessrules/wsdl"
)]
pub struct GetAccessProfileListResponse {
    // StartReference to use in next call to get the following items. If
    // absent, no more items to get.
    #[yaserde(prefix = "tar", rename = "NextStartReference")]
    pub next_start_reference: Option<String>,

    // List of Access Profile items.
    #[yaserde(prefix = "tar", rename = "AccessProfile")]
    pub access_profile: Vec<AccessProfile>,
}

impl Validate for GetAccessProfileListResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tar",
    namespace = "tar: http://www.onvif.org/ver10/accessrules/wsdl"
)]
pub struct CreateAccessProfile {
    // The AccessProfile to create.
    #[yaserde(prefix = "tar", rename = "AccessProfile")]
    pub access_profile: AccessProfile,
}

impl Validate for CreateAccessProfile {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tar",
    namespace = "tar: http://www.onvif.org/ver10/accessrules/wsdl"
)]
pub struct CreateAccessProfileResponse {
    // The Token of created AccessProfile.
    #[yaserde(prefix = "tar", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for CreateAccessProfileResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tar",
    namespace = "tar: http://www.onvif.org/ver10/accessrules/wsdl"
)]
pub struct ModifyAccessProfile {
    // The details of Access Profile
    #[yaserde(prefix = "tar", rename = "AccessProfile")]
    pub access_profile: AccessProfile,
}

impl Validate for ModifyAccessProfile {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tar",
    namespace = "tar: http://www.onvif.org/ver10/accessrules/wsdl"
)]
pub struct ModifyAccessProfileResponse {}

impl Validate for ModifyAccessProfileResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tar",
    namespace = "tar: http://www.onvif.org/ver10/accessrules/wsdl"
)]
pub struct SetAccessProfile {
    // The AccessProfile item to create or modify
    #[yaserde(prefix = "tar", rename = "AccessProfile")]
    pub access_profile: AccessProfile,
}

impl Validate for SetAccessProfile {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tar",
    namespace = "tar: http://www.onvif.org/ver10/accessrules/wsdl"
)]
pub struct SetAccessProfileResponse {}

impl Validate for SetAccessProfileResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tar",
    namespace = "tar: http://www.onvif.org/ver10/accessrules/wsdl"
)]
pub struct DeleteAccessProfile {
    // The token of the access profile to delete.
    #[yaserde(prefix = "tar", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for DeleteAccessProfile {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tar",
    namespace = "tar: http://www.onvif.org/ver10/accessrules/wsdl"
)]
pub struct DeleteAccessProfileResponse {}

impl Validate for DeleteAccessProfileResponse {}

// This operation returns the capabilities of the access rules service.
pub async fn get_service_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetServiceCapabilities,
) -> Result<GetServiceCapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of AccessProfileInfo items matching the given
// tokens. The device shall
// ignore tokens it cannot resolve and shall return an empty list if there are
// no items matching the
// specified tokens. The device shall not return a fault in this case.
// If the number of requested items is greater than MaxLimit, a TooManyItems
// fault shall be returned.
pub async fn get_access_profile_info<T: transport::Transport>(
    transport: &T,
    request: &GetAccessProfileInfo,
) -> Result<GetAccessProfileInfoResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of all of AccessProfileInfo items provided by
// the device.
// A call to this method shall return a StartReference when not all data is
// returned and more data is
// available. The reference shall be valid for retrieving the next set of data.
// The number of items returned shall not be greater than the Limit parameter.
pub async fn get_access_profile_info_list<T: transport::Transport>(
    transport: &T,
    request: &GetAccessProfileInfoList,
) -> Result<GetAccessProfileInfoListResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns the specified access profile item matching the given
// tokens.
// The device shall ignore tokens it cannot resolve and shall return an empty
// list if there are no items
// matching specified tokens. The device shall not return a fault in this case.
// If the number of requested items is greater than MaxLimit, a TooManyItems
// fault shall be returned.
pub async fn get_access_profiles<T: transport::Transport>(
    transport: &T,
    request: &GetAccessProfiles,
) -> Result<GetAccessProfilesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of all of access profile items provided by the
// device.
// A call to this method shall return a StartReference when not all data is
// returned and more data is
// available. The reference shall be valid for retrieving the next set of data.
// The number of items returned shall not be greater than the Limit parameter.
pub async fn get_access_profile_list<T: transport::Transport>(
    transport: &T,
    request: &GetAccessProfileList,
) -> Result<GetAccessProfileListResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation creates the specified access profile in the device. The token
// field of the access profile shall be
// empty, the service shall allocate a token for the access profile. The
// allocated token shall be returned
// in the response. If the client sends any value in the token field, the device
// shall return InvalidArgVal
// as generic fault code.
// In an access profile, if several access policies specifying different
// schedules for the same access
// point will result in a union of the schedules.
pub async fn create_access_profile<T: transport::Transport>(
    transport: &T,
    request: &CreateAccessProfile,
) -> Result<CreateAccessProfileResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation will modify the access profile for the specified access
// profile token. The token of the
// access profile to modify is specified in the token field of the AccessProile
// structure and shall not
// be empty. All other fields in the structure shall overwrite the fields in the
// specified access profile.
// If several access policies specifying different schedules for the same access
// point will result in a
// union of the schedules.
// If the device could not store the access profile information then a fault
// will be generated.
pub async fn modify_access_profile<T: transport::Transport>(
    transport: &T,
    request: &ModifyAccessProfile,
) -> Result<ModifyAccessProfileResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation will synchronize an access profile in a client with the
// device.
// If an access profile with the specified token does not exist in the device,
// the access profile is
// created. If an access profile with the specified token exists, then the
// access profile is modified.
// A call to this method takes an access profile structure as input parameter.
// The token field of the
// access profile must not be empty.
// A device that signals support for the ClientSuppliedTokenSupported capability
// shall implement this command.
pub async fn set_access_profile<T: transport::Transport>(
    transport: &T,
    request: &SetAccessProfile,
) -> Result<SetAccessProfileResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation will delete the specified access profile.
// If the access profile is deleted, all access policies associated to the
// access profile will also be
// deleted.
// If it is associated with one or more entities some devices may not be able to
// delete the access profile,
// and consequently a ReferenceInUse fault shall be generated.
pub async fn delete_access_profile<T: transport::Transport>(
    transport: &T,
    request: &DeleteAccessProfile,
) -> Result<DeleteAccessProfileResponse, transport::Error> {
    transport::request(transport, request).await
}
