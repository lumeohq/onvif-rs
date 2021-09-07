use crate::transport;
use crate::types as pt;
use crate::validate::Validate;
use std::str::FromStr;
use xsd_macro_utils::*;
use xsd_types::types as xs;

// The service capabilities reflect optional functionality of a service. The
// information is static
// and does not change during device operation. The following capabilities are
// available:
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct ServiceCapabilities {
    // A list of identifier types that the device supports. Identifiers types
    // starting with
    // the prefix pt: are reserved to define ONVIF specific types. For custom
    // defined identifier types
    // shall all share the "pt:<Name>" syntax.
    #[yaserde(prefix = "tcr", rename = "SupportedIdentifierType")]
    pub supported_identifier_type: Vec<pt::Name>,

    #[yaserde(prefix = "tcr", rename = "Extension")]
    pub extension: Option<ServiceCapabilitiesExtension>,

    // The maximum number of entries returned by a single Get<Entity>List or
    // Get<Entity>
    // request. The device shall never return more than this number of entities
    // in a single response.
    #[yaserde(attribute, rename = "MaxLimit")]
    pub max_limit: pt::PositiveInteger,

    // Indicates that the device supports credential validity.
    #[yaserde(attribute, rename = "CredentialValiditySupported")]
    pub credential_validity_supported: bool,

    // Indicates that the device supports validity on the association between a
    // credential and an
    // access profile.
    #[yaserde(attribute, rename = "CredentialAccessProfileValiditySupported")]
    pub credential_access_profile_validity_supported: bool,

    // Indicates that the device supports both date and time value for validity.
    // If set to false,
    // then the time value is ignored.
    #[yaserde(attribute, rename = "ValiditySupportsTimeValue")]
    pub validity_supports_time_value: bool,

    // The maximum number of credential supported by the device.
    #[yaserde(attribute, rename = "MaxCredentials")]
    pub max_credentials: pt::PositiveInteger,

    // The maximum number of access profiles for a credential.
    #[yaserde(attribute, rename = "MaxAccessProfilesPerCredential")]
    pub max_access_profiles_per_credential: pt::PositiveInteger,

    // Indicates the device supports resetting of anti-passback violations and
    // notifying on
    // anti-passback violations.
    #[yaserde(attribute, rename = "ResetAntipassbackSupported")]
    pub reset_antipassback_supported: bool,

    // Indicates that the client is allowed to supply the token when creating
    // credentials.
    // To enable the use of the command SetCredential, the value must be set to
    // true.
    #[yaserde(attribute, rename = "ClientSuppliedTokenSupported")]
    pub client_supplied_token_supported: Option<bool>,

    // The default time period that the credential will temporary be suspended
    // (e.g. by using
    // the wrong PIN a predetermined number of times).
    // The time period is defined as an [ISO 8601] duration string (e.g.
    // “PT5M”).
    #[yaserde(attribute, rename = "DefaultCredentialSuspensionDuration")]
    pub default_credential_suspension_duration: Option<xs::Duration>,
}

impl Validate for ServiceCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct ServiceCapabilitiesExtension {
    // A list of exemptions that the device supports. Supported exemptions
    // starting with the
    // prefix pt: are reserved to define ONVIF specific exemption types and
    // these reserved
    // exemption types shall all share "pt:<Name>" syntax.
    #[yaserde(prefix = "tcr", rename = "SupportedExemptionType")]
    pub supported_exemption_type: Vec<pt::Name>,
}

impl Validate for ServiceCapabilitiesExtension {}

// pub type Capabilities = ServiceCapabilities;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct CredentialInfo {
    // User readable description for the credential. It shall be up to 1024
    // characters.
    #[yaserde(prefix = "tcr", rename = "Description")]
    pub description: Option<pt::Description>,

    // An external reference to a person holding this credential. The
    // reference is a username or used ID in an external system, such as a
    // directory
    // service.
    #[yaserde(prefix = "tcr", rename = "CredentialHolderReference")]
    pub credential_holder_reference: credential_info::CredentialHolderReferenceType,

    // The start date/time validity of the credential. If the
    // ValiditySupportsTimeValue capability is set to false, then only date is
    // supported (time is ignored).
    #[yaserde(prefix = "tcr", rename = "ValidFrom")]
    pub valid_from: Option<xs::DateTime>,

    // The expiration date/time validity of the credential. If the
    // ValiditySupportsTimeValue capability is set to false, then only date is
    // supported (time is ignored).
    #[yaserde(prefix = "tcr", rename = "ValidTo")]
    pub valid_to: Option<xs::DateTime>,

    pub base: pt::DataEntity,
}

impl Validate for CredentialInfo {}

pub mod credential_info {
    use super::*;

    #[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
    pub struct CredentialHolderReferenceType(pub String);

    impl Validate for CredentialHolderReferenceType {
        fn validate(&self) -> Result<(), String> {
            if self.0.len() > "64".parse().unwrap() {
                return Err(format!("MaxLength validation error. \nExpected: 0 length <= 64 \nActual: 0 length == {}", self.0.len()));
            }
            if self.0.len() < "0".parse().unwrap() {
                return Err(format!("MinLength validation error. \nExpected: 0 length >= 0 \nActual: 0 length == {}", self.0.len()));
            }
            Ok(())
        }
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct Credential {
    // A list of credential identifier structures. At least one
    // credential identifier is required. Maximum one credential identifier
    // structure
    // per type is allowed.
    #[yaserde(prefix = "tcr", rename = "CredentialIdentifier")]
    pub credential_identifier: Vec<CredentialIdentifier>,

    // A list of credential access profile structures.
    #[yaserde(prefix = "tcr", rename = "CredentialAccessProfile")]
    pub credential_access_profile: Vec<CredentialAccessProfile>,

    // A boolean indicating that the credential holder needs extra time to get
    // through the door.
    // ExtendedReleaseTime will be added to ReleaseTime, and ExtendedOpenTime
    // will be added to OpenTime
    #[yaserde(prefix = "tcr", rename = "ExtendedGrantTime")]
    pub extended_grant_time: Option<bool>,

    // A list of credential attributes as name value pairs. Key names
    // starting with the prefix pt: are reserved to define PACS specific
    // attributes
    // following the "pt:<Name>" syntax.
    #[yaserde(prefix = "tcr", rename = "Attribute")]
    pub attribute: Vec<pt::Attribute>,

    #[yaserde(prefix = "tcr", rename = "Extension")]
    pub extension: Option<CredentialExtension>,

    pub base: CredentialInfo,
}

impl Validate for Credential {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct CredentialExtension {}

impl Validate for CredentialExtension {}

// A credential identifier is a card number, unique card information, PIN or
// biometric information such as fingerprint, iris, vein, face recognition, that
// can be validated
// in an access point.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct CredentialIdentifier {
    // Contains the details of the credential identifier type. Is of type
    // CredentialIdentifierType.
    #[yaserde(prefix = "tcr", rename = "Type")]
    pub _type: CredentialIdentifierType,

    // If set to true, this credential identifier is not considered for
    // authentication. For example if the access point requests Card plus PIN,
    // and the credential
    // identifier of type PIN is exempted from authentication, then the access
    // point will not prompt
    // for the PIN.
    #[yaserde(prefix = "tcr", rename = "ExemptedFromAuthentication")]
    pub exempted_from_authentication: bool,

    // The value of the identifier in hexadecimal representation.
    #[yaserde(prefix = "tcr", rename = "Value")]
    pub value: String,
}

impl Validate for CredentialIdentifier {}

// Specifies the name of credential identifier type and its format for the
// credential
// value.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct CredentialIdentifierType {
    // The name of the credential identifier type, such as pt:Card, pt:PIN,
    // etc.
    #[yaserde(prefix = "tcr", rename = "Name")]
    pub name: pt::Name,

    // Specifies the format of the credential value for the specified identifier
    // type name.
    #[yaserde(prefix = "tcr", rename = "FormatType")]
    pub format_type: String,
}

impl Validate for CredentialIdentifierType {}

// The association between a credential and an access profile.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct CredentialAccessProfile {
    // The reference token of the associated access profile.
    #[yaserde(prefix = "tcr", rename = "AccessProfileToken")]
    pub access_profile_token: pt::ReferenceToken,

    // The start date/time of the validity for the association between the
    // credential and the access profile. If the ValiditySupportsTimeValue
    // capability is set to
    // false, then only date is supported (time is ignored).
    #[yaserde(prefix = "tcr", rename = "ValidFrom")]
    pub valid_from: Option<xs::DateTime>,

    // The end date/time of the validity for the association between the
    // credential and the access profile. If the ValiditySupportsTimeValue
    // capability is set to
    // false, then only date is supported (time is ignored).
    #[yaserde(prefix = "tcr", rename = "ValidTo")]
    pub valid_to: Option<xs::DateTime>,
}

impl Validate for CredentialAccessProfile {}

// The CredentialState structure contains information about the state of the
// credential and
// optionally the reason of why the credential was disabled.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct CredentialState {
    // True if the credential is enabled or false if the credential is
    // disabled.
    #[yaserde(prefix = "tcr", rename = "Enabled")]
    pub enabled: bool,

    // Predefined ONVIF reasons as mentioned in the section 5.4.2.7
    // of credential service specification document. For any other reason, free
    // text can be used.
    #[yaserde(prefix = "tcr", rename = "Reason")]
    pub reason: Option<pt::Name>,

    // A structure indicating the anti-passback state. This field shall be
    // supported if the ResetAntipassbackSupported capability is set to true.
    #[yaserde(prefix = "tcr", rename = "AntipassbackState")]
    pub antipassback_state: Option<AntipassbackState>,

    #[yaserde(prefix = "tcr", rename = "Extension")]
    pub extension: Option<CredentialStateExtension>,
}

impl Validate for CredentialState {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct CredentialStateExtension {}

impl Validate for CredentialStateExtension {}

// A structure containing anti-passback related state information.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct AntipassbackState {
    // Indicates if anti-passback is violated for the credential.
    #[yaserde(prefix = "tcr", rename = "AntipassbackViolated")]
    pub antipassback_violated: bool,
}

impl Validate for AntipassbackState {}

// Contains information about a format type.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct CredentialIdentifierFormatTypeInfo {
    // A format type supported by the device. A list of supported format types
    // is
    // provided in [ISO 16484-5:2014-09 Annex P]. The BACnet type "CUSTOM" is
    // not used in this
    // specification. Instead device manufacturers can define their own format
    // types.
    #[yaserde(prefix = "tcr", rename = "FormatType")]
    pub format_type: String,

    // User readable description of the credential identifier format type. It
    // shall be up to 1024 characters. For custom types, it is recommended to
    // describe how the
    // octet string is encoded (following the structure in column Authentication
    // Factor Value
    // Encoding of [ISO 16484-5:2014-09 Annex P]).
    #[yaserde(prefix = "tcr", rename = "Description")]
    pub description: pt::Description,

    #[yaserde(prefix = "tcr", rename = "Extension")]
    pub extension: Option<CredentialIdentifierFormatTypeInfoExtension>,
}

impl Validate for CredentialIdentifierFormatTypeInfo {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct CredentialIdentifierFormatTypeInfoExtension {}

impl Validate for CredentialIdentifierFormatTypeInfoExtension {}

// Contains information about a format type.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct CredentialData {
    // A format type supported by the device. A list of supported format types
    // is
    // provided in [ISO 16484-5:2014-09 Annex P]. The BACnet type "CUSTOM" is
    // not used in this
    // specification. Instead device manufacturers can define their own format
    // types.
    #[yaserde(prefix = "tcr", rename = "Credential")]
    pub credential: Credential,

    // User readable description of the credential identifier format type. It
    // shall be up to 1024 characters. For custom types, it is recommended to
    // describe how the
    // octet string is encoded (following the structure in column Authentication
    // Factor Value
    // Encoding of [ISO 16484-5:2014-09 Annex P]).
    #[yaserde(prefix = "tcr", rename = "CredentialState")]
    pub credential_state: CredentialState,

    #[yaserde(prefix = "tcr", rename = "Extension")]
    pub extension: Option<CredentialDataExtension>,
}

impl Validate for CredentialData {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct CredentialDataExtension {}

impl Validate for CredentialDataExtension {}

// Contains information about a format type.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct FaultResponse {
    // A format type supported by the device. A list of supported format types
    // is
    // provided in [ISO 16484-5:2014-09 Annex P]. The BACnet type "CUSTOM" is
    // not used in this
    // specification. Instead device manufacturers can define their own format
    // types.
    #[yaserde(prefix = "tcr", rename = "Token")]
    pub token: pt::ReferenceToken,

    // User readable description of the credential identifier format type. It
    // shall be up to 1024 characters. For custom types, it is recommended to
    // describe how the
    // octet string is encoded (following the structure in column Authentication
    // Factor Value
    // Encoding of [ISO 16484-5:2014-09 Annex P]).
    #[yaserde(prefix = "tcr", rename = "Fault")]
    pub fault: String,

    #[yaserde(prefix = "tcr", rename = "Extension")]
    pub extension: Option<FaultResponseExtension>,
}

impl Validate for FaultResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct FaultResponseExtension {}

impl Validate for FaultResponseExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct GetServiceCapabilitiesResponse {
    // The capability response message contains the requested credential
    // service capabilities using a hierarchical XML capability structure.
    #[yaserde(prefix = "tcr", rename = "Capabilities")]
    pub capabilities: ServiceCapabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct GetSupportedFormatTypes {
    // Name of the credential identifier type
    #[yaserde(prefix = "tcr", rename = "CredentialIdentifierTypeName")]
    pub credential_identifier_type_name: String,
}

impl Validate for GetSupportedFormatTypes {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct GetSupportedFormatTypesResponse {
    // Identifier format type
    #[yaserde(prefix = "tcr", rename = "FormatTypeInfo")]
    pub format_type_info: Vec<CredentialIdentifierFormatTypeInfo>,
}

impl Validate for GetSupportedFormatTypesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct GetCredentialInfo {
    // Tokens of CredentialInfo items to get.
    #[yaserde(prefix = "tcr", rename = "Token")]
    pub token: Vec<pt::ReferenceToken>,
}

impl Validate for GetCredentialInfo {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct GetCredentialInfoResponse {
    // List of CredentialInfo items.
    #[yaserde(prefix = "tcr", rename = "CredentialInfo")]
    pub credential_info: Vec<CredentialInfo>,
}

impl Validate for GetCredentialInfoResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct GetCredentialInfoList {
    // Maximum number of entries to return. If not specified, less than one
    // or higher than what the device supports, the number of items is
    // determined by the
    // device.
    #[yaserde(prefix = "tcr", rename = "Limit")]
    pub limit: Option<i32>,

    // Start returning entries from this start reference. If not specified,
    // entries shall start from the beginning of the dataset.
    #[yaserde(prefix = "tcr", rename = "StartReference")]
    pub start_reference: Option<String>,
}

impl Validate for GetCredentialInfoList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct GetCredentialInfoListResponse {
    // StartReference to use in next call to get the following items. If
    // absent, no more items to get.
    #[yaserde(prefix = "tcr", rename = "NextStartReference")]
    pub next_start_reference: Option<String>,

    // List of CredentialInfo items.
    #[yaserde(prefix = "tcr", rename = "CredentialInfo")]
    pub credential_info: Vec<CredentialInfo>,
}

impl Validate for GetCredentialInfoListResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct GetCredentials {
    // Token of Credentials to get
    #[yaserde(prefix = "tcr", rename = "Token")]
    pub token: Vec<pt::ReferenceToken>,
}

impl Validate for GetCredentials {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct GetCredentialsResponse {
    // List of Credential items.
    #[yaserde(prefix = "tcr", rename = "Credential")]
    pub credential: Vec<Credential>,
}

impl Validate for GetCredentialsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct GetCredentialList {
    // Maximum number of entries to return. If not specified, less than one
    // or higher than what the device supports, the number of items is
    // determined by the
    // device.
    #[yaserde(prefix = "tcr", rename = "Limit")]
    pub limit: Option<i32>,

    // Start returning entries from this start reference. If not specified,
    // entries shall start from the beginning of the dataset.
    #[yaserde(prefix = "tcr", rename = "StartReference")]
    pub start_reference: Option<String>,
}

impl Validate for GetCredentialList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct GetCredentialListResponse {
    // StartReference to use in next call to get the following items. If
    // absent, no more items to get.
    #[yaserde(prefix = "tcr", rename = "NextStartReference")]
    pub next_start_reference: Option<String>,

    // List of Credential items.
    #[yaserde(prefix = "tcr", rename = "Credential")]
    pub credential: Vec<Credential>,
}

impl Validate for GetCredentialListResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct CreateCredential {
    // The credential to create.
    #[yaserde(prefix = "tcr", rename = "Credential")]
    pub credential: Credential,

    // The state of the credential.
    #[yaserde(prefix = "tcr", rename = "State")]
    pub state: CredentialState,
}

impl Validate for CreateCredential {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct CreateCredentialResponse {
    // The token of the created credential
    #[yaserde(prefix = "tcr", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for CreateCredentialResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct ModifyCredential {
    // Details of the credential.
    #[yaserde(prefix = "tcr", rename = "Credential")]
    pub credential: Credential,
}

impl Validate for ModifyCredential {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct ModifyCredentialResponse {}

impl Validate for ModifyCredentialResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct SetCredential {
    // Details of the credential.
    #[yaserde(prefix = "tcr", rename = "CredentialData")]
    pub credential_data: CredentialData,
}

impl Validate for SetCredential {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct SetCredentialResponse {}

impl Validate for SetCredentialResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct DeleteCredential {
    // The token of the credential to delete.
    #[yaserde(prefix = "tcr", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for DeleteCredential {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct DeleteCredentialResponse {}

impl Validate for DeleteCredentialResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct GetCredentialState {
    // Token of Credential
    #[yaserde(prefix = "tcr", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for GetCredentialState {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct GetCredentialStateResponse {
    // State of the credential.
    #[yaserde(prefix = "tcr", rename = "State")]
    pub state: CredentialState,
}

impl Validate for GetCredentialStateResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct EnableCredential {
    // The token of the credential
    #[yaserde(prefix = "tcr", rename = "Token")]
    pub token: pt::ReferenceToken,

    // Reason for enabling the credential.
    #[yaserde(prefix = "tcr", rename = "Reason")]
    pub reason: Option<pt::Name>,
}

impl Validate for EnableCredential {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct EnableCredentialResponse {}

impl Validate for EnableCredentialResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct DisableCredential {
    // Token of the Credential
    #[yaserde(prefix = "tcr", rename = "Token")]
    pub token: pt::ReferenceToken,

    // Reason for disabling the credential
    #[yaserde(prefix = "tcr", rename = "Reason")]
    pub reason: Option<pt::Name>,
}

impl Validate for DisableCredential {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct DisableCredentialResponse {}

impl Validate for DisableCredentialResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct ResetAntipassbackViolation {
    // Token of the Credential
    #[yaserde(prefix = "tcr", rename = "CredentialToken")]
    pub credential_token: pt::ReferenceToken,
}

impl Validate for ResetAntipassbackViolation {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct ResetAntipassbackViolationResponse {}

impl Validate for ResetAntipassbackViolationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct GetCredentialIdentifiers {
    // Token of the Credential
    #[yaserde(prefix = "tcr", rename = "CredentialToken")]
    pub credential_token: pt::ReferenceToken,
}

impl Validate for GetCredentialIdentifiers {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct GetCredentialIdentifiersResponse {
    // Identifier of the credential
    #[yaserde(prefix = "tcr", rename = "CredentialIdentifier")]
    pub credential_identifier: Vec<CredentialIdentifier>,
}

impl Validate for GetCredentialIdentifiersResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct SetCredentialIdentifier {
    // Token of the Credential
    #[yaserde(prefix = "tcr", rename = "CredentialToken")]
    pub credential_token: pt::ReferenceToken,

    // Identifier of the credential
    #[yaserde(prefix = "tcr", rename = "CredentialIdentifier")]
    pub credential_identifier: CredentialIdentifier,
}

impl Validate for SetCredentialIdentifier {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct SetCredentialIdentifierResponse {}

impl Validate for SetCredentialIdentifierResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct DeleteCredentialIdentifier {
    // Token of the Credential
    #[yaserde(prefix = "tcr", rename = "CredentialToken")]
    pub credential_token: pt::ReferenceToken,

    // Identifier type name of a credential
    #[yaserde(prefix = "tcr", rename = "CredentialIdentifierTypeName")]
    pub credential_identifier_type_name: pt::Name,
}

impl Validate for DeleteCredentialIdentifier {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct DeleteCredentialIdentifierResponse {}

impl Validate for DeleteCredentialIdentifierResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct GetCredentialAccessProfiles {
    // Token of the Credential
    #[yaserde(prefix = "tcr", rename = "CredentialToken")]
    pub credential_token: pt::ReferenceToken,
}

impl Validate for GetCredentialAccessProfiles {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct GetCredentialAccessProfilesResponse {
    // Access Profiles of the credential
    #[yaserde(prefix = "tcr", rename = "CredentialAccessProfile")]
    pub credential_access_profile: Vec<CredentialAccessProfile>,
}

impl Validate for GetCredentialAccessProfilesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct SetCredentialAccessProfiles {
    // Token of the Credential
    #[yaserde(prefix = "tcr", rename = "CredentialToken")]
    pub credential_token: pt::ReferenceToken,

    // Access Profiles of the credential
    #[yaserde(prefix = "tcr", rename = "CredentialAccessProfile")]
    pub credential_access_profile: Vec<CredentialAccessProfile>,
}

impl Validate for SetCredentialAccessProfiles {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct SetCredentialAccessProfilesResponse {}

impl Validate for SetCredentialAccessProfilesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct DeleteCredentialAccessProfiles {
    // Token of the Credential
    #[yaserde(prefix = "tcr", rename = "CredentialToken")]
    pub credential_token: pt::ReferenceToken,

    // Tokens of Access Profiles
    #[yaserde(prefix = "tcr", rename = "AccessProfileToken")]
    pub access_profile_token: Vec<pt::ReferenceToken>,
}

impl Validate for DeleteCredentialAccessProfiles {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tcr",
    namespace = "tcr: http://www.onvif.org/ver10/credential/wsdl"
)]
pub struct DeleteCredentialAccessProfilesResponse {}

impl Validate for DeleteCredentialAccessProfilesResponse {}

// This operation returns the capabilities of the credential service.
pub async fn get_service_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetServiceCapabilities,
) -> Result<GetServiceCapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This method returns all the supported format types of a specified identifier
// type that is supported by
// the device.
pub async fn get_supported_format_types<T: transport::Transport>(
    transport: &T,
    request: &GetSupportedFormatTypes,
) -> Result<GetSupportedFormatTypesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of CredentialInfo items matching the given
// tokens.
// The device shall ignore tokens it cannot resolve and shall return an empty
// list if there are no
// items matching the specified tokens. The device shall not return a fault in
// this case.
// If the number of requested items is greater than MaxLimit, a TooManyItems
// fault shall be returned.
pub async fn get_credential_info<T: transport::Transport>(
    transport: &T,
    request: &GetCredentialInfo,
) -> Result<GetCredentialInfoResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of all CredentialInfo items provided by the
// device.
// A call to this method shall return a StartReference when not all data is
// returned and more data is available.
// The reference shall be valid for retrieving the next set of data. Please
// refer to section 4.8.3 in
// [ONVIF Access Control Service Specification] for more details.
// The number of items returned shall not be greater than the Limit parameter.
pub async fn get_credential_info_list<T: transport::Transport>(
    transport: &T,
    request: &GetCredentialInfoList,
) -> Result<GetCredentialInfoListResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns the specified credential items matching the given
// tokens.
// The device shall ignore tokens it cannot resolve and shall return an empty
// list if there are no items
// matching specified tokens. The device shall not return a fault in this case.
// If the number of requested items is greater than MaxLimit, a TooManyItems
// fault shall be returned.
pub async fn get_credentials<T: transport::Transport>(
    transport: &T,
    request: &GetCredentials,
) -> Result<GetCredentialsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of all credential items provided by the
// device.
// A call to this method shall return a StartReference when not all data is
// returned and more data is
// available. The reference shall be valid for retrieving the next set of data.
// Please refer section 4.8.3
// in [Access Control Service Specification] for more details. The number of
// items returned shall not be
// greater the Limit parameter.
pub async fn get_credential_list<T: transport::Transport>(
    transport: &T,
    request: &GetCredentialList,
) -> Result<GetCredentialListResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation creates a credential. A call to this method takes a credential
// structure and a credential
// state structure as input parameters. The credential state can be created in
// disabled or enabled state.
// The token field of the credential shall be empty, the device shall allocate a
// token for the credential.
// The allocated token shall be returned in the response. If the client sends
// any value in the token field,
// the device shall return InvalidArgVal as generic fault code.
pub async fn create_credential<T: transport::Transport>(
    transport: &T,
    request: &CreateCredential,
) -> Result<CreateCredentialResponse, transport::Error> {
    transport::request(transport, request).await
}

// This method is used to synchronize a credential in a client with the device.
pub async fn set_credential<T: transport::Transport>(
    transport: &T,
    request: &SetCredential,
) -> Result<SetCredentialResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation modifies the specified credential.
// The token of the credential to modify is specified in the token field of the
// Credential structure and
// shall not be empty. All other fields in the structure shall overwrite the
// fields in the specified credential.
// When an existing credential is modified, the state is not modified
// explicitly. The only way for a client to
// change the state of a credential is to explicitly call the EnableCredential,
// DisableCredential or
// ResetAntipassback command.
// All existing credential identifiers and credential access profiles are
// removed and replaced with the
// specified entities.
pub async fn modify_credential<T: transport::Transport>(
    transport: &T,
    request: &ModifyCredential,
) -> Result<ModifyCredentialResponse, transport::Error> {
    transport::request(transport, request).await
}

// This method deletes the specified credential.
// If it is associated with one or more entities some devices may not be able to
// delete the credential,
// and consequently a ReferenceInUse fault shall be generated.
pub async fn delete_credential<T: transport::Transport>(
    transport: &T,
    request: &DeleteCredential,
) -> Result<DeleteCredentialResponse, transport::Error> {
    transport::request(transport, request).await
}

// This method returns the state for the specified credential.
// If the capability ResetAntipassbackSupported is set to true, then the device
// shall supply the
// anti-passback state in the returned credential state structure.
pub async fn get_credential_state<T: transport::Transport>(
    transport: &T,
    request: &GetCredentialState,
) -> Result<GetCredentialStateResponse, transport::Error> {
    transport::request(transport, request).await
}

// This method is used to enable a credential.
pub async fn enable_credential<T: transport::Transport>(
    transport: &T,
    request: &EnableCredential,
) -> Result<EnableCredentialResponse, transport::Error> {
    transport::request(transport, request).await
}

// This method is used to disable a credential.
pub async fn disable_credential<T: transport::Transport>(
    transport: &T,
    request: &DisableCredential,
) -> Result<DisableCredentialResponse, transport::Error> {
    transport::request(transport, request).await
}

// This method is used to reset anti-passback violations for a specified
// credential.
pub async fn reset_antipassback_violation<T: transport::Transport>(
    transport: &T,
    request: &ResetAntipassbackViolation,
) -> Result<ResetAntipassbackViolationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This method returns all the credential identifiers for a credential.
pub async fn get_credential_identifiers<T: transport::Transport>(
    transport: &T,
    request: &GetCredentialIdentifiers,
) -> Result<GetCredentialIdentifiersResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation creates or updates a credential identifier for a credential.
// If the type of specified credential identifier already exists, the current
// credential identifier of that
// type is replaced. Otherwise the credential identifier is added.
pub async fn set_credential_identifier<T: transport::Transport>(
    transport: &T,
    request: &SetCredentialIdentifier,
) -> Result<SetCredentialIdentifierResponse, transport::Error> {
    transport::request(transport, request).await
}

// This method deletes all the identifier values for the specified type.
// However, if the identifier type
// name doesn’t exist in the device, it will be silently ignored without any
// response.
pub async fn delete_credential_identifier<T: transport::Transport>(
    transport: &T,
    request: &DeleteCredentialIdentifier,
) -> Result<DeleteCredentialIdentifierResponse, transport::Error> {
    transport::request(transport, request).await
}

// This method returns all the credential access profiles for a credential.
pub async fn get_credential_access_profiles<T: transport::Transport>(
    transport: &T,
    request: &GetCredentialAccessProfiles,
) -> Result<GetCredentialAccessProfilesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation add or updates the credential access profiles for a
// credential.
// The device shall update the credential access profile if the access profile
// token in the specified
// credential access profile matches. Otherwise the credential access profile is
// added.
pub async fn set_credential_access_profiles<T: transport::Transport>(
    transport: &T,
    request: &SetCredentialAccessProfiles,
) -> Result<SetCredentialAccessProfilesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This method deletes all the credential access profiles for the specified
// tokens.
// However, if no matching credential access profiles are found, the
// corresponding access profile tokens
// are silently ignored without any response.
pub async fn delete_credential_access_profiles<T: transport::Transport>(
    transport: &T,
    request: &DeleteCredentialAccessProfiles,
) -> Result<DeleteCredentialAccessProfilesResponse, transport::Error> {
    transport::request(transport, request).await
}
