use crate::onvif as tt;
use crate::transport;
use crate::validate::Validate;
use std::str::FromStr;
use xsd_macro_utils::*;
use xsd_types::types as xs;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetServices {
    // Indicates if the service capabilities (untyped) should be included in the
    // response.
    #[yaserde(prefix = "tds", rename = "IncludeCapability")]
    pub include_capability: bool,
}

impl Validate for GetServices {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetServicesResponse {
    // Each Service element contains information about one service.
    #[yaserde(prefix = "tds", rename = "Service")]
    pub service: Vec<Service>,
}

impl Validate for GetServicesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct Service {
    // Namespace of the service being described. This parameter allows to match
    // the service capabilities to the service. Note that only one set of
    // capabilities is supported per namespace.
    #[yaserde(prefix = "tds", rename = "Namespace")]
    pub namespace: String,

    // The transport addresses where the service can be reached. The scheme and
    // IP part shall match the one used in the request (i.e. the GetServices
    // request).
    #[yaserde(prefix = "tds", rename = "XAddr")]
    pub x_addr: String,

    #[yaserde(prefix = "tds", rename = "Capabilities")]
    pub capabilities: Option<service::CapabilitiesType>,

    // The version of the service (not the ONVIF core spec version).
    #[yaserde(prefix = "tds", rename = "Version")]
    pub version: tt::OnvifVersion,
}

impl Validate for Service {}

pub mod service {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tds",
        namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
    )]
    pub struct CapabilitiesType {}

    impl Validate for CapabilitiesType {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetServiceCapabilitiesResponse {
    // The capabilities for the device service is returned in the Capabilities
    // element.
    #[yaserde(prefix = "tds", rename = "Capabilities")]
    pub capabilities: DeviceServiceCapabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct DeviceServiceCapabilities {
    // Network capabilities.
    #[yaserde(prefix = "tds", rename = "Network")]
    pub network: NetworkCapabilities,

    // Security capabilities.
    #[yaserde(prefix = "tds", rename = "Security")]
    pub security: SecurityCapabilities,

    // System capabilities.
    #[yaserde(prefix = "tds", rename = "System")]
    pub system: SystemCapabilities,

    // Capabilities that do not fit in any of the other categories.
    #[yaserde(prefix = "tds", rename = "Misc")]
    pub misc: Option<MiscCapabilities>,
}

impl Validate for DeviceServiceCapabilities {}

// pub type Capabilities = DeviceServiceCapabilities;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct NetworkCapabilities {
    // Indicates support for IP filtering.
    #[yaserde(attribute, rename = "IPFilter")]
    pub ip_filter: Option<bool>,

    // Indicates support for zeroconf.
    #[yaserde(attribute, rename = "ZeroConfiguration")]
    pub zero_configuration: Option<bool>,

    // Indicates support for IPv6.
    #[yaserde(attribute, rename = "IPVersion6")]
    pub ip_version_6: Option<bool>,

    // Indicates support for dynamic DNS configuration.
    #[yaserde(attribute, rename = "DynDNS")]
    pub dyn_dns: Option<bool>,

    // Indicates support for IEEE 802.11 configuration.
    #[yaserde(attribute, rename = "Dot11Configuration")]
    pub dot_11_configuration: Option<bool>,

    // Indicates the maximum number of Dot1X configurations supported by the
    // device
    #[yaserde(attribute, rename = "Dot1XConfigurations")]
    pub dot_1x_configurations: Option<i32>,

    // Indicates support for retrieval of hostname from DHCP.
    #[yaserde(attribute, rename = "HostnameFromDHCP")]
    pub hostname_from_dhcp: Option<bool>,

    // Maximum number of NTP servers supported by the devices SetNTP command.
    #[yaserde(attribute, rename = "NTP")]
    pub ntp: Option<i32>,

    // Indicates support for Stateful IPv6 DHCP.
    #[yaserde(attribute, rename = "DHCPv6")]
    pub dhc_pv_6: Option<bool>,
}

impl Validate for NetworkCapabilities {}

#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct EapmethodTypes(pub Vec<i32>);

impl Validate for EapmethodTypes {}
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SecurityCapabilities {
    // Indicates support for TLS 1.0.
    #[yaserde(attribute, rename = "TLS1.0")]
    pub tls1_0: Option<bool>,

    // Indicates support for TLS 1.1.
    #[yaserde(attribute, rename = "TLS1.1")]
    pub tls1_1: Option<bool>,

    // Indicates support for TLS 1.2.
    #[yaserde(attribute, rename = "TLS1.2")]
    pub tls1_2: Option<bool>,

    // Indicates support for onboard key generation.
    #[yaserde(attribute, rename = "OnboardKeyGeneration")]
    pub onboard_key_generation: Option<bool>,

    // Indicates support for access policy configuration.
    #[yaserde(attribute, rename = "AccessPolicyConfig")]
    pub access_policy_config: Option<bool>,

    // Indicates support for the ONVIF default access policy.
    #[yaserde(attribute, rename = "DefaultAccessPolicy")]
    pub default_access_policy: Option<bool>,

    // Indicates support for IEEE 802.1X configuration.
    #[yaserde(attribute, rename = "Dot1X")]
    pub dot_1x: Option<bool>,

    // Indicates support for remote user configuration. Used when accessing
    // another device.
    #[yaserde(attribute, rename = "RemoteUserHandling")]
    pub remote_user_handling: Option<bool>,

    // Indicates support for WS-Security X.509 token.
    #[yaserde(attribute, rename = "X.509Token")]
    pub x_509_token: Option<bool>,

    // Indicates support for WS-Security SAML token.
    #[yaserde(attribute, rename = "SAMLToken")]
    pub saml_token: Option<bool>,

    // Indicates support for WS-Security Kerberos token.
    #[yaserde(attribute, rename = "KerberosToken")]
    pub kerberos_token: Option<bool>,

    // Indicates support for WS-Security Username token.
    #[yaserde(attribute, rename = "UsernameToken")]
    pub username_token: Option<bool>,

    // Indicates support for WS over HTTP digest authenticated communication
    // layer.
    #[yaserde(attribute, rename = "HttpDigest")]
    pub http_digest: Option<bool>,

    // Indicates support for WS-Security REL token.
    #[yaserde(attribute, rename = "RELToken")]
    pub rel_token: Option<bool>,

    // EAP Methods supported by the device. The int values refer to the
    #[yaserde(attribute, rename = "SupportedEAPMethods")]
    pub supported_eap_methods: Option<EapmethodTypes>,

    // The maximum number of users that the device supports.
    #[yaserde(attribute, rename = "MaxUsers")]
    pub max_users: Option<i32>,

    // Maximum number of characters supported for the username by CreateUsers.
    #[yaserde(attribute, rename = "MaxUserNameLength")]
    pub max_user_name_length: Option<i32>,

    // Maximum number of characters supported for the password by CreateUsers
    // and SetUser.
    #[yaserde(attribute, rename = "MaxPasswordLength")]
    pub max_password_length: Option<i32>,
}

impl Validate for SecurityCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SystemCapabilities {
    // Indicates support for WS Discovery resolve requests.
    #[yaserde(attribute, rename = "DiscoveryResolve")]
    pub discovery_resolve: Option<bool>,

    // Indicates support for WS-Discovery Bye.
    #[yaserde(attribute, rename = "DiscoveryBye")]
    pub discovery_bye: Option<bool>,

    // Indicates support for remote discovery.
    #[yaserde(attribute, rename = "RemoteDiscovery")]
    pub remote_discovery: Option<bool>,

    // Indicates support for system backup through MTOM.
    #[yaserde(attribute, rename = "SystemBackup")]
    pub system_backup: Option<bool>,

    // Indicates support for retrieval of system logging through MTOM.
    #[yaserde(attribute, rename = "SystemLogging")]
    pub system_logging: Option<bool>,

    // Indicates support for firmware upgrade through MTOM.
    #[yaserde(attribute, rename = "FirmwareUpgrade")]
    pub firmware_upgrade: Option<bool>,

    // Indicates support for firmware upgrade through HTTP.
    #[yaserde(attribute, rename = "HttpFirmwareUpgrade")]
    pub http_firmware_upgrade: Option<bool>,

    // Indicates support for system backup through HTTP.
    #[yaserde(attribute, rename = "HttpSystemBackup")]
    pub http_system_backup: Option<bool>,

    // Indicates support for retrieval of system logging through HTTP.
    #[yaserde(attribute, rename = "HttpSystemLogging")]
    pub http_system_logging: Option<bool>,

    // Indicates support for retrieving support information through HTTP.
    #[yaserde(attribute, rename = "HttpSupportInformation")]
    pub http_support_information: Option<bool>,

    // Indicates support for storage configuration interfaces.
    #[yaserde(attribute, rename = "StorageConfiguration")]
    pub storage_configuration: Option<bool>,

    // Indicates maximum number of storage configurations supported.
    #[yaserde(attribute, rename = "MaxStorageConfigurations")]
    pub max_storage_configurations: Option<i32>,

    // If present signals support for geo location. The value signals the
    // supported number of entries.
    #[yaserde(attribute, rename = "GeoLocationEntries")]
    pub geo_location_entries: Option<i32>,

    // List of supported automatic GeoLocation adjustment supported by the
    // device. Valid items are defined by tds:AutoGeoMode.
    #[yaserde(attribute, rename = "AutoGeo")]
    pub auto_geo: Option<tt::StringAttrList>,

    // Enumerates the supported StorageTypes, see tds:StorageType.
    #[yaserde(attribute, rename = "StorageTypesSupported")]
    pub storage_types_supported: Option<tt::StringAttrList>,
}

impl Validate for SystemCapabilities {}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum AutoGeoModes {
    // Automatic adjustment of the device location.
    Location,
    // Automatic adjustment of the device orientation relative to the compass
    // also called yaw.
    Heading,
    // Automatic adjustment of the deviation from the horizon also called pitch
    // and roll.
    Leveling,
    __Unknown__(String),
}

impl Default for AutoGeoModes {
    fn default() -> AutoGeoModes {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for AutoGeoModes {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct MiscCapabilities {
    // Lists of commands supported by SendAuxiliaryCommand.
    #[yaserde(attribute, rename = "AuxiliaryCommands")]
    pub auxiliary_commands: Option<tt::StringAttrList>,
}

impl Validate for MiscCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetDeviceInformation {}

impl Validate for GetDeviceInformation {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetDeviceInformationResponse {
    // The manufactor of the device.
    #[yaserde(prefix = "tds", rename = "Manufacturer")]
    pub manufacturer: String,

    // The device model.
    #[yaserde(prefix = "tds", rename = "Model")]
    pub model: String,

    // The firmware version in the device.
    #[yaserde(prefix = "tds", rename = "FirmwareVersion")]
    pub firmware_version: String,

    // The serial number of the device.
    #[yaserde(prefix = "tds", rename = "SerialNumber")]
    pub serial_number: String,

    // The hardware ID of the device.
    #[yaserde(prefix = "tds", rename = "HardwareId")]
    pub hardware_id: String,
}

impl Validate for GetDeviceInformationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetSystemDateAndTime {
    // Defines if the date and time is set via NTP or manually.
    #[yaserde(prefix = "tds", rename = "DateTimeType")]
    pub date_time_type: tt::SetDateTimeType,

    // Automatically adjust Daylight savings if defined in TimeZone.
    #[yaserde(prefix = "tds", rename = "DaylightSavings")]
    pub daylight_savings: bool,

    // The time zone in POSIX 1003.1 format
    #[yaserde(prefix = "tds", rename = "TimeZone")]
    pub time_zone: Option<tt::TimeZone>,

    // Date and time in UTC. If time is obtained via NTP, UTCDateTime has no
    // meaning
    #[yaserde(prefix = "tds", rename = "UTCDateTime")]
    pub utc_date_time: Option<tt::DateTime>,
}

impl Validate for SetSystemDateAndTime {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetSystemDateAndTimeResponse {}

impl Validate for SetSystemDateAndTimeResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetSystemDateAndTime {}

impl Validate for GetSystemDateAndTime {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetSystemDateAndTimeResponse {
    // Contains information whether system date and time are set manually or by
    // NTP, daylight savings is on or off, time zone in POSIX 1003.1 format and
    // system date and time in UTC and also local system date and time.
    #[yaserde(prefix = "tds", rename = "SystemDateAndTime")]
    pub system_date_and_time: tt::SystemDateTime,
}

impl Validate for GetSystemDateAndTimeResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetSystemFactoryDefault {
    // Specifies the factory default action type.
    #[yaserde(prefix = "tds", rename = "FactoryDefault")]
    pub factory_default: tt::FactoryDefaultType,
}

impl Validate for SetSystemFactoryDefault {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetSystemFactoryDefaultResponse {}

impl Validate for SetSystemFactoryDefaultResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct UpgradeSystemFirmware {
    #[yaserde(prefix = "tds", rename = "Firmware")]
    pub firmware: tt::AttachmentData,
}

impl Validate for UpgradeSystemFirmware {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct UpgradeSystemFirmwareResponse {
    #[yaserde(prefix = "tds", rename = "Message")]
    pub message: String,
}

impl Validate for UpgradeSystemFirmwareResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SystemReboot {}

impl Validate for SystemReboot {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SystemRebootResponse {
    // Contains the reboot message sent by the device.
    #[yaserde(prefix = "tds", rename = "Message")]
    pub message: String,
}

impl Validate for SystemRebootResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct RestoreSystem {
    #[yaserde(prefix = "tds", rename = "BackupFiles")]
    pub backup_files: Vec<tt::BackupFile>,
}

impl Validate for RestoreSystem {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct RestoreSystemResponse {}

impl Validate for RestoreSystemResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetSystemBackup {}

impl Validate for GetSystemBackup {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetSystemBackupResponse {
    #[yaserde(prefix = "tds", rename = "BackupFiles")]
    pub backup_files: Vec<tt::BackupFile>,
}

impl Validate for GetSystemBackupResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetSystemSupportInformation {}

impl Validate for GetSystemSupportInformation {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetSystemSupportInformationResponse {
    // Contains the arbitary device diagnostics information.
    #[yaserde(prefix = "tds", rename = "SupportInformation")]
    pub support_information: tt::SupportInformation,
}

impl Validate for GetSystemSupportInformationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetSystemLog {
    // Specifies the type of system log to get.
    #[yaserde(prefix = "tds", rename = "LogType")]
    pub log_type: tt::SystemLogType,
}

impl Validate for GetSystemLog {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetSystemLogResponse {
    // Contains the system log information.
    #[yaserde(prefix = "tds", rename = "SystemLog")]
    pub system_log: tt::SystemLog,
}

impl Validate for GetSystemLogResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetScopes {}

impl Validate for GetScopes {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetScopesResponse {
    // Contains a list of URI definining the device scopes. Scope parameters can
    // be of two types: fixed and configurable. Fixed parameters can not be
    // altered.
    #[yaserde(prefix = "tds", rename = "Scopes")]
    pub scopes: Vec<tt::Scope>,
}

impl Validate for GetScopesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetScopes {
    // Contains a list of scope parameters that will replace all existing
    // configurable scope parameters.
    #[yaserde(prefix = "tds", rename = "Scopes")]
    pub scopes: Vec<String>,
}

impl Validate for SetScopes {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetScopesResponse {}

impl Validate for SetScopesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct AddScopes {
    // Contains a list of new configurable scope parameters that will be added
    // to the existing configurable scope.
    #[yaserde(prefix = "tds", rename = "ScopeItem")]
    pub scope_item: Vec<String>,
}

impl Validate for AddScopes {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct AddScopesResponse {}

impl Validate for AddScopesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct RemoveScopes {
    // Contains a list of URIs that should be removed from the device scope.
    #[yaserde(prefix = "tds", rename = "ScopeItem")]
    pub scope_item: Vec<String>,
}

impl Validate for RemoveScopes {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct RemoveScopesResponse {
    // Contains a list of URIs that has been removed from the device scope
    #[yaserde(prefix = "tds", rename = "ScopeItem")]
    pub scope_item: Vec<String>,
}

impl Validate for RemoveScopesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetDiscoveryMode {}

impl Validate for GetDiscoveryMode {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetDiscoveryModeResponse {
    // Indicator of discovery mode: Discoverable, NonDiscoverable.
    #[yaserde(prefix = "tds", rename = "DiscoveryMode")]
    pub discovery_mode: tt::DiscoveryMode,
}

impl Validate for GetDiscoveryModeResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetDiscoveryMode {
    // Indicator of discovery mode: Discoverable, NonDiscoverable.
    #[yaserde(prefix = "tds", rename = "DiscoveryMode")]
    pub discovery_mode: tt::DiscoveryMode,
}

impl Validate for SetDiscoveryMode {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetDiscoveryModeResponse {}

impl Validate for SetDiscoveryModeResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetRemoteDiscoveryMode {}

impl Validate for GetRemoteDiscoveryMode {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetRemoteDiscoveryModeResponse {
    // Indicator of discovery mode: Discoverable, NonDiscoverable.
    #[yaserde(prefix = "tds", rename = "RemoteDiscoveryMode")]
    pub remote_discovery_mode: tt::DiscoveryMode,
}

impl Validate for GetRemoteDiscoveryModeResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetRemoteDiscoveryMode {
    // Indicator of discovery mode: Discoverable, NonDiscoverable.
    #[yaserde(prefix = "tds", rename = "RemoteDiscoveryMode")]
    pub remote_discovery_mode: tt::DiscoveryMode,
}

impl Validate for SetRemoteDiscoveryMode {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetRemoteDiscoveryModeResponse {}

impl Validate for SetRemoteDiscoveryModeResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetDPAddresses {}

impl Validate for GetDPAddresses {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetDPAddressesResponse {
    #[yaserde(prefix = "tds", rename = "DPAddress")]
    pub dp_address: Vec<tt::NetworkHost>,
}

impl Validate for GetDPAddressesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetDPAddresses {
    #[yaserde(prefix = "tds", rename = "DPAddress")]
    pub dp_address: Vec<tt::NetworkHost>,
}

impl Validate for SetDPAddresses {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetDPAddressesResponse {}

impl Validate for SetDPAddressesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetEndpointReference {}

impl Validate for GetEndpointReference {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetEndpointReferenceResponse {
    #[yaserde(prefix = "tds", rename = "GUID")]
    pub guid: String,
}

impl Validate for GetEndpointReferenceResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetRemoteUser {}

impl Validate for GetRemoteUser {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetRemoteUserResponse {
    #[yaserde(prefix = "tds", rename = "RemoteUser")]
    pub remote_user: Option<tt::RemoteUser>,
}

impl Validate for GetRemoteUserResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetRemoteUser {
    #[yaserde(prefix = "tds", rename = "RemoteUser")]
    pub remote_user: Option<tt::RemoteUser>,
}

impl Validate for SetRemoteUser {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetRemoteUserResponse {}

impl Validate for SetRemoteUserResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetUsers {}

impl Validate for GetUsers {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetUsersResponse {
    // Contains a list of the onvif users and following information is included
    // in each entry: username and user level.
    #[yaserde(prefix = "tds", rename = "User")]
    pub user: Vec<tt::User>,
}

impl Validate for GetUsersResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct CreateUsers {
    // Creates new device users and corresponding credentials. Each user entry
    // includes: username, password and user level. Either all users are created
    // successfully or a fault message MUST be returned without creating any
    // user. If trying to create several users with exactly the same username
    // the request is rejected and no users are created. If password is missing,
    // then fault message Too weak password is returned.
    #[yaserde(prefix = "tds", rename = "User")]
    pub user: Vec<tt::User>,
}

impl Validate for CreateUsers {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct CreateUsersResponse {}

impl Validate for CreateUsersResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct DeleteUsers {
    // Deletes users on an device and there may exist users that cannot be
    // deleted to ensure access to the unit. Either all users are deleted
    // successfully or a fault message MUST be returned and no users be deleted.
    // If a username exists multiple times in the request, then a fault message
    // is returned.
    #[yaserde(prefix = "tds", rename = "Username")]
    pub username: Vec<String>,
}

impl Validate for DeleteUsers {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct DeleteUsersResponse {}

impl Validate for DeleteUsersResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetUser {
    // Updates the credentials for one or several users on an device. Either all
    // change requests are processed successfully or a fault message MUST be
    // returned. If the request contains the same username multiple times, a
    // fault message is returned.
    #[yaserde(prefix = "tds", rename = "User")]
    pub user: Vec<tt::User>,
}

impl Validate for SetUser {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetUserResponse {}

impl Validate for SetUserResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetWsdlUrl {}

impl Validate for GetWsdlUrl {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetWsdlUrlResponse {
    #[yaserde(prefix = "tds", rename = "WsdlUrl")]
    pub wsdl_url: String,
}

impl Validate for GetWsdlUrlResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetCapabilities {
    // List of categories to retrieve capability information on.
    #[yaserde(prefix = "tds", rename = "Category")]
    pub category: Vec<tt::CapabilityCategory>,
}

impl Validate for GetCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetCapabilitiesResponse {
    // Capability information.
    #[yaserde(prefix = "tds", rename = "Capabilities")]
    pub capabilities: tt::Capabilities,
}

impl Validate for GetCapabilitiesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetHostname {}

impl Validate for GetHostname {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetHostnameResponse {
    // Contains the hostname information.
    #[yaserde(prefix = "tds", rename = "HostnameInformation")]
    pub hostname_information: tt::HostnameInformation,
}

impl Validate for GetHostnameResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetHostname {
    // The hostname to set.
    #[yaserde(prefix = "tds", rename = "Name")]
    pub name: String,
}

impl Validate for SetHostname {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetHostnameResponse {}

impl Validate for SetHostnameResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetHostnameFromDHCP {
    // True if the hostname shall be obtained via DHCP.
    #[yaserde(prefix = "tds", rename = "FromDHCP")]
    pub from_dhcp: bool,
}

impl Validate for SetHostnameFromDHCP {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetHostnameFromDHCPResponse {
    // Indicates whether or not a reboot is required after configuration
    // updates.
    #[yaserde(prefix = "tds", rename = "RebootNeeded")]
    pub reboot_needed: bool,
}

impl Validate for SetHostnameFromDHCPResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetDNS {}

impl Validate for GetDNS {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetDNSResponse {
    // DNS information.
    #[yaserde(prefix = "tds", rename = "DNSInformation")]
    pub dns_information: tt::Dnsinformation,
}

impl Validate for GetDNSResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetDNS {
    // Indicate if the DNS address is to be retrieved using DHCP.
    #[yaserde(prefix = "tds", rename = "FromDHCP")]
    pub from_dhcp: bool,

    // DNS search domain.
    #[yaserde(prefix = "tds", rename = "SearchDomain")]
    pub search_domain: Vec<String>,

    // DNS address(es) set manually.
    #[yaserde(prefix = "tds", rename = "DNSManual")]
    pub dns_manual: Vec<tt::Ipaddress>,
}

impl Validate for SetDNS {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetDNSResponse {}

impl Validate for SetDNSResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetNTP {}

impl Validate for GetNTP {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetNTPResponse {
    // NTP information.
    #[yaserde(prefix = "tds", rename = "NTPInformation")]
    pub ntp_information: tt::Ntpinformation,
}

impl Validate for GetNTPResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetNTP {
    // Indicate if NTP address information is to be retrieved using DHCP.
    #[yaserde(prefix = "tds", rename = "FromDHCP")]
    pub from_dhcp: bool,

    // Manual NTP settings.
    #[yaserde(prefix = "tds", rename = "NTPManual")]
    pub ntp_manual: Vec<tt::NetworkHost>,
}

impl Validate for SetNTP {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetNTPResponse {}

impl Validate for SetNTPResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetDynamicDNS {}

impl Validate for GetDynamicDNS {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetDynamicDNSResponse {
    // Dynamic DNS information.
    #[yaserde(prefix = "tds", rename = "DynamicDNSInformation")]
    pub dynamic_dns_information: tt::DynamicDNSInformation,
}

impl Validate for GetDynamicDNSResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetDynamicDNS {
    // Dynamic DNS type.
    #[yaserde(prefix = "tds", rename = "Type")]
    pub _type: tt::DynamicDNSType,

    // DNS name.
    #[yaserde(prefix = "tds", rename = "Name")]
    pub name: Option<tt::Dnsname>,

    // DNS record time to live.
    #[yaserde(prefix = "tds", rename = "TTL")]
    pub ttl: Option<xs::Duration>,
}

impl Validate for SetDynamicDNS {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetDynamicDNSResponse {}

impl Validate for SetDynamicDNSResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetNetworkInterfaces {}

impl Validate for GetNetworkInterfaces {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetNetworkInterfacesResponse {
    // List of network interfaces.
    #[yaserde(prefix = "tds", rename = "NetworkInterfaces")]
    pub network_interfaces: Vec<tt::NetworkInterface>,
}

impl Validate for GetNetworkInterfacesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetNetworkInterfaces {
    // Symbolic network interface name.
    #[yaserde(prefix = "tds", rename = "InterfaceToken")]
    pub interface_token: tt::ReferenceToken,

    // Network interface name.
    #[yaserde(prefix = "tds", rename = "NetworkInterface")]
    pub network_interface: tt::NetworkInterfaceSetConfiguration,
}

impl Validate for SetNetworkInterfaces {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetNetworkInterfacesResponse {
    // Indicates whether or not a reboot is required after configuration
    // updates.
    // If a device responds with RebootNeeded set to false, the device can be
    // reached
    // via the new IP address without further action. A client should be aware
    // that a device
    // may not be responsive for a short period of time until it signals
    // availability at
    // the new address via the discovery Hello messages.
    // If a device responds with RebootNeeded set to true, it will be further
    // available under
    // its previous IP address. The settings will only be activated when the
    // device is
    // rebooted via the SystemReboot command.
    #[yaserde(prefix = "tds", rename = "RebootNeeded")]
    pub reboot_needed: bool,
}

impl Validate for SetNetworkInterfacesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetNetworkProtocols {}

impl Validate for GetNetworkProtocols {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetNetworkProtocolsResponse {
    // Contains an array of defined protocols supported by the device. There are
    // three protocols defined; HTTP, HTTPS and RTSP. The following parameters
    // can be retrieved for each protocol: port and enable/disable.
    #[yaserde(prefix = "tds", rename = "NetworkProtocols")]
    pub network_protocols: Vec<tt::NetworkProtocol>,
}

impl Validate for GetNetworkProtocolsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetNetworkProtocols {
    // Configures one or more defined network protocols supported by the device.
    // There are currently three protocols defined; HTTP, HTTPS and RTSP. The
    // following parameters can be set for each protocol: port and
    // enable/disable.
    #[yaserde(prefix = "tds", rename = "NetworkProtocols")]
    pub network_protocols: Vec<tt::NetworkProtocol>,
}

impl Validate for SetNetworkProtocols {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetNetworkProtocolsResponse {}

impl Validate for SetNetworkProtocolsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetNetworkDefaultGateway {}

impl Validate for GetNetworkDefaultGateway {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetNetworkDefaultGatewayResponse {
    // Gets the default IPv4 and IPv6 gateway settings from the device.
    #[yaserde(prefix = "tds", rename = "NetworkGateway")]
    pub network_gateway: tt::NetworkGateway,
}

impl Validate for GetNetworkDefaultGatewayResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetNetworkDefaultGateway {
    // Sets IPv4 gateway address used as default setting.
    #[yaserde(prefix = "tds", rename = "IPv4Address")]
    pub i_pv_4_address: Vec<tt::Ipv4Address>,

    // Sets IPv6 gateway address used as default setting.
    #[yaserde(prefix = "tds", rename = "IPv6Address")]
    pub i_pv_6_address: Vec<tt::Ipv6Address>,
}

impl Validate for SetNetworkDefaultGateway {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetNetworkDefaultGatewayResponse {}

impl Validate for SetNetworkDefaultGatewayResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetZeroConfiguration {}

impl Validate for GetZeroConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetZeroConfigurationResponse {
    // Contains the zero-configuration.
    #[yaserde(prefix = "tds", rename = "ZeroConfiguration")]
    pub zero_configuration: tt::NetworkZeroConfiguration,
}

impl Validate for GetZeroConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetZeroConfiguration {
    // Unique identifier referencing the physical interface.
    #[yaserde(prefix = "tds", rename = "InterfaceToken")]
    pub interface_token: tt::ReferenceToken,

    // Specifies if the zero-configuration should be enabled or not.
    #[yaserde(prefix = "tds", rename = "Enabled")]
    pub enabled: bool,
}

impl Validate for SetZeroConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetZeroConfigurationResponse {}

impl Validate for SetZeroConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetIPAddressFilter {}

impl Validate for GetIPAddressFilter {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetIPAddressFilterResponse {
    #[yaserde(prefix = "tds", rename = "IPAddressFilter")]
    pub ip_address_filter: tt::IpaddressFilter,
}

impl Validate for GetIPAddressFilterResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetIPAddressFilter {
    #[yaserde(prefix = "tds", rename = "IPAddressFilter")]
    pub ip_address_filter: tt::IpaddressFilter,
}

impl Validate for SetIPAddressFilter {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetIPAddressFilterResponse {}

impl Validate for SetIPAddressFilterResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct AddIPAddressFilter {
    #[yaserde(prefix = "tds", rename = "IPAddressFilter")]
    pub ip_address_filter: tt::IpaddressFilter,
}

impl Validate for AddIPAddressFilter {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct AddIPAddressFilterResponse {}

impl Validate for AddIPAddressFilterResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct RemoveIPAddressFilter {
    #[yaserde(prefix = "tds", rename = "IPAddressFilter")]
    pub ip_address_filter: tt::IpaddressFilter,
}

impl Validate for RemoveIPAddressFilter {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct RemoveIPAddressFilterResponse {}

impl Validate for RemoveIPAddressFilterResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetAccessPolicy {}

impl Validate for GetAccessPolicy {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetAccessPolicyResponse {
    #[yaserde(prefix = "tds", rename = "PolicyFile")]
    pub policy_file: tt::BinaryData,
}

impl Validate for GetAccessPolicyResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetAccessPolicy {
    #[yaserde(prefix = "tds", rename = "PolicyFile")]
    pub policy_file: tt::BinaryData,
}

impl Validate for SetAccessPolicy {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetAccessPolicyResponse {}

impl Validate for SetAccessPolicyResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct CreateCertificate {
    // Certificate id.
    #[yaserde(prefix = "tds", rename = "CertificateID")]
    pub certificate_id: Option<String>,

    // Identification of the entity associated with the public-key.
    #[yaserde(prefix = "tds", rename = "Subject")]
    pub subject: Option<String>,

    // Certificate validity start date.
    #[yaserde(prefix = "tds", rename = "ValidNotBefore")]
    pub valid_not_before: Option<xs::DateTime>,

    // Certificate expiry start date.
    #[yaserde(prefix = "tds", rename = "ValidNotAfter")]
    pub valid_not_after: Option<xs::DateTime>,
}

impl Validate for CreateCertificate {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct CreateCertificateResponse {
    // base64 encoded DER representation of certificate.
    #[yaserde(prefix = "tds", rename = "NvtCertificate")]
    pub nvt_certificate: tt::Certificate,
}

impl Validate for CreateCertificateResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetCertificates {}

impl Validate for GetCertificates {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetCertificatesResponse {
    // Id and base64 encoded DER representation of all available certificates.
    #[yaserde(prefix = "tds", rename = "NvtCertificate")]
    pub nvt_certificate: Vec<tt::Certificate>,
}

impl Validate for GetCertificatesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetCertificatesStatus {}

impl Validate for GetCertificatesStatus {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetCertificatesStatusResponse {
    // Indicates if a certificate is used in an optional HTTPS configuration of
    // the device.
    #[yaserde(prefix = "tds", rename = "CertificateStatus")]
    pub certificate_status: Vec<tt::CertificateStatus>,
}

impl Validate for GetCertificatesStatusResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetCertificatesStatus {
    // Indicates if a certificate is to be used in an optional HTTPS
    // configuration of the device.
    #[yaserde(prefix = "tds", rename = "CertificateStatus")]
    pub certificate_status: Vec<tt::CertificateStatus>,
}

impl Validate for SetCertificatesStatus {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetCertificatesStatusResponse {}

impl Validate for SetCertificatesStatusResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct DeleteCertificates {
    // List of ids of certificates to delete.
    #[yaserde(prefix = "tds", rename = "CertificateID")]
    pub certificate_id: Vec<String>,
}

impl Validate for DeleteCertificates {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct DeleteCertificatesResponse {}

impl Validate for DeleteCertificatesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetPkcs10Request {
    // List of ids of certificates to delete.
    #[yaserde(prefix = "tds", rename = "CertificateID")]
    pub certificate_id: String,

    // Relative Dinstinguished Name(RDN) CommonName(CN).
    #[yaserde(prefix = "tds", rename = "Subject")]
    pub subject: Option<String>,

    // Optional base64 encoded DER attributes.
    #[yaserde(prefix = "tds", rename = "Attributes")]
    pub attributes: Option<tt::BinaryData>,
}

impl Validate for GetPkcs10Request {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetPkcs10RequestResponse {
    // base64 encoded DER representation of certificate.
    #[yaserde(prefix = "tds", rename = "Pkcs10Request")]
    pub pkcs_10_request: tt::BinaryData,
}

impl Validate for GetPkcs10RequestResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct LoadCertificates {
    // Optional id and base64 encoded DER representation of certificate.
    #[yaserde(prefix = "tds", rename = "NVTCertificate")]
    pub nvt_certificate: Vec<tt::Certificate>,
}

impl Validate for LoadCertificates {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct LoadCertificatesResponse {}

impl Validate for LoadCertificatesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetClientCertificateMode {}

impl Validate for GetClientCertificateMode {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetClientCertificateModeResponse {
    // Indicates whether or not client certificates are required by device.
    #[yaserde(prefix = "tds", rename = "Enabled")]
    pub enabled: bool,
}

impl Validate for GetClientCertificateModeResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetClientCertificateMode {
    // Indicates whether or not client certificates are required by device.
    #[yaserde(prefix = "tds", rename = "Enabled")]
    pub enabled: bool,
}

impl Validate for SetClientCertificateMode {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetClientCertificateModeResponse {}

impl Validate for SetClientCertificateModeResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetCACertificates {}

impl Validate for GetCACertificates {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetCACertificatesResponse {
    #[yaserde(prefix = "tds", rename = "CACertificate")]
    pub ca_certificate: Vec<tt::Certificate>,
}

impl Validate for GetCACertificatesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct LoadCertificateWithPrivateKey {
    #[yaserde(prefix = "tds", rename = "CertificateWithPrivateKey")]
    pub certificate_with_private_key: Vec<tt::CertificateWithPrivateKey>,
}

impl Validate for LoadCertificateWithPrivateKey {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct LoadCertificateWithPrivateKeyResponse {}

impl Validate for LoadCertificateWithPrivateKeyResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetCertificateInformation {
    #[yaserde(prefix = "tds", rename = "CertificateID")]
    pub certificate_id: String,
}

impl Validate for GetCertificateInformation {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetCertificateInformationResponse {
    #[yaserde(prefix = "tds", rename = "CertificateInformation")]
    pub certificate_information: tt::CertificateInformation,
}

impl Validate for GetCertificateInformationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct LoadCACertificates {
    #[yaserde(prefix = "tds", rename = "CACertificate")]
    pub ca_certificate: Vec<tt::Certificate>,
}

impl Validate for LoadCACertificates {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct LoadCACertificatesResponse {}

impl Validate for LoadCACertificatesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct CreateDot1XConfiguration {
    #[yaserde(prefix = "tds", rename = "Dot1XConfiguration")]
    pub dot_1x_configuration: tt::Dot1XConfiguration,
}

impl Validate for CreateDot1XConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct CreateDot1XConfigurationResponse {}

impl Validate for CreateDot1XConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetDot1XConfiguration {
    #[yaserde(prefix = "tds", rename = "Dot1XConfiguration")]
    pub dot_1x_configuration: tt::Dot1XConfiguration,
}

impl Validate for SetDot1XConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetDot1XConfigurationResponse {}

impl Validate for SetDot1XConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetDot1XConfiguration {
    #[yaserde(prefix = "tds", rename = "Dot1XConfigurationToken")]
    pub dot_1x_configuration_token: tt::ReferenceToken,
}

impl Validate for GetDot1XConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetDot1XConfigurationResponse {
    #[yaserde(prefix = "tds", rename = "Dot1XConfiguration")]
    pub dot_1x_configuration: tt::Dot1XConfiguration,
}

impl Validate for GetDot1XConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetDot1XConfigurations {}

impl Validate for GetDot1XConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetDot1XConfigurationsResponse {
    #[yaserde(prefix = "tds", rename = "Dot1XConfiguration")]
    pub dot_1x_configuration: Vec<tt::Dot1XConfiguration>,
}

impl Validate for GetDot1XConfigurationsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct DeleteDot1XConfiguration {
    #[yaserde(prefix = "tds", rename = "Dot1XConfigurationToken")]
    pub dot_1x_configuration_token: Vec<tt::ReferenceToken>,
}

impl Validate for DeleteDot1XConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct DeleteDot1XConfigurationResponse {}

impl Validate for DeleteDot1XConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetRelayOutputs {}

impl Validate for GetRelayOutputs {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetRelayOutputsResponse {
    #[yaserde(prefix = "tds", rename = "RelayOutputs")]
    pub relay_outputs: Vec<tt::RelayOutput>,
}

impl Validate for GetRelayOutputsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetRelayOutputSettings {
    #[yaserde(prefix = "tds", rename = "RelayOutputToken")]
    pub relay_output_token: tt::ReferenceToken,

    #[yaserde(prefix = "tds", rename = "Properties")]
    pub properties: tt::RelayOutputSettings,
}

impl Validate for SetRelayOutputSettings {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetRelayOutputSettingsResponse {}

impl Validate for SetRelayOutputSettingsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetRelayOutputState {
    #[yaserde(prefix = "tds", rename = "RelayOutputToken")]
    pub relay_output_token: tt::ReferenceToken,

    #[yaserde(prefix = "tds", rename = "LogicalState")]
    pub logical_state: tt::RelayLogicalState,
}

impl Validate for SetRelayOutputState {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetRelayOutputStateResponse {}

impl Validate for SetRelayOutputStateResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SendAuxiliaryCommand {
    #[yaserde(prefix = "tds", rename = "AuxiliaryCommand")]
    pub auxiliary_command: tt::AuxiliaryData,
}

impl Validate for SendAuxiliaryCommand {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SendAuxiliaryCommandResponse {
    #[yaserde(prefix = "tds", rename = "AuxiliaryCommandResponse")]
    pub auxiliary_command_response: tt::AuxiliaryData,
}

impl Validate for SendAuxiliaryCommandResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetDot11Capabilities {}

impl Validate for GetDot11Capabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetDot11CapabilitiesResponse {
    #[yaserde(prefix = "tds", rename = "Capabilities")]
    pub capabilities: tt::Dot11Capabilities,
}

impl Validate for GetDot11CapabilitiesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetDot11Status {
    #[yaserde(prefix = "tds", rename = "InterfaceToken")]
    pub interface_token: tt::ReferenceToken,
}

impl Validate for GetDot11Status {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetDot11StatusResponse {
    #[yaserde(prefix = "tds", rename = "Status")]
    pub status: tt::Dot11Status,
}

impl Validate for GetDot11StatusResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct ScanAvailableDot11Networks {
    #[yaserde(prefix = "tds", rename = "InterfaceToken")]
    pub interface_token: tt::ReferenceToken,
}

impl Validate for ScanAvailableDot11Networks {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct ScanAvailableDot11NetworksResponse {
    #[yaserde(prefix = "tds", rename = "Networks")]
    pub networks: Vec<tt::Dot11AvailableNetworks>,
}

impl Validate for ScanAvailableDot11NetworksResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetSystemUris {}

impl Validate for GetSystemUris {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetSystemUrisResponse {
    #[yaserde(prefix = "tds", rename = "SystemLogUris")]
    pub system_log_uris: tt::SystemLogUriList,

    #[yaserde(prefix = "tds", rename = "SupportInfoUri")]
    pub support_info_uri: String,

    #[yaserde(prefix = "tds", rename = "SystemBackupUri")]
    pub system_backup_uri: String,

    #[yaserde(prefix = "tds", rename = "Extension")]
    pub extension: Option<get_system_uris_response::ExtensionType>,
}

impl Validate for GetSystemUrisResponse {}

pub mod get_system_uris_response {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tds",
        namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
    )]
    pub struct ExtensionType {}

    impl Validate for ExtensionType {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct StartFirmwareUpgrade {}

impl Validate for StartFirmwareUpgrade {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct StartFirmwareUpgradeResponse {
    #[yaserde(prefix = "tds", rename = "UploadUri")]
    pub upload_uri: String,

    #[yaserde(prefix = "tds", rename = "UploadDelay")]
    pub upload_delay: xs::Duration,

    #[yaserde(prefix = "tds", rename = "ExpectedDownTime")]
    pub expected_down_time: xs::Duration,
}

impl Validate for StartFirmwareUpgradeResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct StartSystemRestore {}

impl Validate for StartSystemRestore {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct StartSystemRestoreResponse {
    #[yaserde(prefix = "tds", rename = "UploadUri")]
    pub upload_uri: String,

    #[yaserde(prefix = "tds", rename = "ExpectedDownTime")]
    pub expected_down_time: xs::Duration,
}

impl Validate for StartSystemRestoreResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct UserCredential {
    // User name
    #[yaserde(prefix = "tds", rename = "UserName")]
    pub user_name: String,

    // optional password
    #[yaserde(prefix = "tds", rename = "Password")]
    pub password: Option<String>,

    #[yaserde(prefix = "tds", rename = "Extension")]
    pub extension: Option<user_credential::ExtensionType>,
}

impl Validate for UserCredential {}

pub mod user_credential {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tds",
        namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
    )]
    pub struct ExtensionType {}

    impl Validate for ExtensionType {}
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum StorageType {
    // NFS protocol
    #[yaserde(rename = "NFS")]
    Nfs,
    // CIFS protocol
    #[yaserde(rename = "CIFS")]
    Cifs,
    // CDMI protocol
    #[yaserde(rename = "CDMI")]
    Cdmi,
    // FTP protocol
    #[yaserde(rename = "FTP")]
    Ftp,
    __Unknown__(String),
}

impl Default for StorageType {
    fn default() -> StorageType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for StorageType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct StorageConfigurationData {
    // local path
    #[yaserde(prefix = "tds", rename = "LocalPath")]
    pub local_path: Option<String>,

    // Storage server address
    #[yaserde(prefix = "tds", rename = "StorageUri")]
    pub storage_uri: Option<String>,

    // User credential for the storage server
    #[yaserde(prefix = "tds", rename = "User")]
    pub user: Option<UserCredential>,

    #[yaserde(prefix = "tds", rename = "Extension")]
    pub extension: Option<storage_configuration_data::ExtensionType>,

    // StorageType lists the acceptable values for type attribute
    #[yaserde(attribute, rename = "type")]
    pub _type: String,
}

impl Validate for StorageConfigurationData {}

pub mod storage_configuration_data {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tds",
        namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
    )]
    pub struct ExtensionType {}

    impl Validate for ExtensionType {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct StorageConfiguration {
    #[yaserde(prefix = "tds", rename = "Data")]
    pub data: StorageConfigurationData,

    pub base: tt::DeviceEntity,
}

impl Validate for StorageConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetStorageConfigurations {}

impl Validate for GetStorageConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetStorageConfigurationsResponse {
    #[yaserde(prefix = "tds", rename = "StorageConfigurations")]
    pub storage_configurations: Vec<StorageConfiguration>,
}

impl Validate for GetStorageConfigurationsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct CreateStorageConfiguration {
    #[yaserde(prefix = "tds", rename = "StorageConfiguration")]
    pub storage_configuration: StorageConfigurationData,
}

impl Validate for CreateStorageConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct CreateStorageConfigurationResponse {
    #[yaserde(prefix = "tds", rename = "Token")]
    pub token: tt::ReferenceToken,
}

impl Validate for CreateStorageConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetStorageConfiguration {
    #[yaserde(prefix = "tds", rename = "Token")]
    pub token: tt::ReferenceToken,
}

impl Validate for GetStorageConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetStorageConfigurationResponse {
    #[yaserde(prefix = "tds", rename = "StorageConfiguration")]
    pub storage_configuration: StorageConfiguration,
}

impl Validate for GetStorageConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetStorageConfiguration {
    #[yaserde(prefix = "tds", rename = "StorageConfiguration")]
    pub storage_configuration: StorageConfiguration,
}

impl Validate for SetStorageConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetStorageConfigurationResponse {}

impl Validate for SetStorageConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct DeleteStorageConfiguration {
    #[yaserde(prefix = "tds", rename = "Token")]
    pub token: tt::ReferenceToken,
}

impl Validate for DeleteStorageConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct DeleteStorageConfigurationResponse {}

impl Validate for DeleteStorageConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetGeoLocation {}

impl Validate for GetGeoLocation {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetGeoLocationResponse {
    #[yaserde(prefix = "tds", rename = "Location")]
    pub location: Vec<tt::LocationEntity>,
}

impl Validate for GetGeoLocationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetGeoLocation {
    #[yaserde(prefix = "tds", rename = "Location")]
    pub location: Vec<tt::LocationEntity>,
}

impl Validate for SetGeoLocation {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct SetGeoLocationResponse {}

impl Validate for SetGeoLocationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct DeleteGeoLocation {
    #[yaserde(prefix = "tds", rename = "Location")]
    pub location: Vec<tt::LocationEntity>,
}

impl Validate for DeleteGeoLocation {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct DeleteGeoLocationResponse {}

impl Validate for DeleteGeoLocationResponse {}

// Returns information about services on the device.
pub async fn get_services<T: transport::Transport>(
    transport: &T,
    request: &GetServices,
) -> Result<GetServicesResponse, transport::Error> {
    transport::request(transport, request).await
}

// Returns the capabilities of the device service. The result is returned in a
// typed answer.
pub async fn get_service_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetServiceCapabilities,
) -> Result<GetServiceCapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation gets basic device information from the device.
pub async fn get_device_information<T: transport::Transport>(
    transport: &T,
    request: &GetDeviceInformation,
) -> Result<GetDeviceInformationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation sets the device system date and time. The device shall support
// the
// configuration of the daylight saving setting and of the manual system date
// and time (if
// applicable) or indication of NTP time (if applicable) through the
// SetSystemDateAndTime
// command.
pub async fn set_system_date_and_time<T: transport::Transport>(
    transport: &T,
    request: &SetSystemDateAndTime,
) -> Result<SetSystemDateAndTimeResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation gets the device system date and time. The device shall support
// the return of
// the daylight saving setting and of the manual system date and time (if
// applicable) or indication
// of NTP time (if applicable) through the GetSystemDateAndTime command.
pub async fn get_system_date_and_time<T: transport::Transport>(
    transport: &T,
    request: &GetSystemDateAndTime,
) -> Result<GetSystemDateAndTimeResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation reloads the parameters on the device to their factory default
// values.
pub async fn set_system_factory_default<T: transport::Transport>(
    transport: &T,
    request: &SetSystemFactoryDefault,
) -> Result<SetSystemFactoryDefaultResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation upgrades a device firmware version. After a successful upgrade
// the response
// message is sent before the device reboots. The device should support firmware
// upgrade
// through the UpgradeSystemFirmware command. The exact format of the firmware
// data is
// outside the scope of this standard.
pub async fn upgrade_system_firmware<T: transport::Transport>(
    transport: &T,
    request: &UpgradeSystemFirmware,
) -> Result<UpgradeSystemFirmwareResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation reboots the device.
pub async fn system_reboot<T: transport::Transport>(
    transport: &T,
    request: &SystemReboot,
) -> Result<SystemRebootResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation restores the system backup configuration files(s) previously
// retrieved from a
// device. The device should support restore of backup configuration file(s)
// through the
// RestoreSystem command. The exact format of the backup configuration file(s)
// is outside the
// scope of this standard. If the command is supported, it shall accept backup
// files returned by
// the GetSystemBackup command.
pub async fn restore_system<T: transport::Transport>(
    transport: &T,
    request: &RestoreSystem,
) -> Result<RestoreSystemResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation is retrieves system backup configuration file(s) from a
// device. The device
// should support return of back up configuration file(s) through the
// GetSystemBackup command.
// The backup is returned with reference to a name and mime-type together with
// binary data.
// The exact format of the backup configuration files is outside the scope of
// this standard.
pub async fn get_system_backup<T: transport::Transport>(
    transport: &T,
    request: &GetSystemBackup,
) -> Result<GetSystemBackupResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation gets a system log from the device. The exact format of the
// system logs is outside the scope of this standard.
pub async fn get_system_log<T: transport::Transport>(
    transport: &T,
    request: &GetSystemLog,
) -> Result<GetSystemLogResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation gets arbitary device diagnostics information from the device.
pub async fn get_system_support_information<T: transport::Transport>(
    transport: &T,
    request: &GetSystemSupportInformation,
) -> Result<GetSystemSupportInformationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests the scope parameters of a device. The scope
// parameters are used in
// the device discovery to match a probe message, see Section 7. The Scope
// parameters are of
// two different types:
pub async fn get_scopes<T: transport::Transport>(
    transport: &T,
    request: &GetScopes,
) -> Result<GetScopesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation sets the scope parameters of a device. The scope parameters
// are used in the
// device discovery to match a probe message.
// This operation replaces all existing configurable scope parameters (not fixed
// parameters). If
// this shall be avoided, one should use the scope add command instead. The
// device shall
// support configuration of discovery scope parameters through the SetScopes
// command.
pub async fn set_scopes<T: transport::Transport>(
    transport: &T,
    request: &SetScopes,
) -> Result<SetScopesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation adds new configurable scope parameters to a device. The scope
// parameters
// are used in the device discovery to match a probe message. The device shall
// support addition of discovery scope parameters through the AddScopes command.
pub async fn add_scopes<T: transport::Transport>(
    transport: &T,
    request: &AddScopes,
) -> Result<AddScopesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation deletes scope-configurable scope parameters from a device. The
// scope
// parameters are used in the device discovery to match a probe message, see
// Section 7. The
// device shall support deletion of discovery scope parameters through the
// RemoveScopes
// command.
// Table
pub async fn remove_scopes<T: transport::Transport>(
    transport: &T,
    request: &RemoveScopes,
) -> Result<RemoveScopesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation gets the discovery mode of a device. See Section 7.2 for the
// definition of the
// different device discovery modes. The device shall support retrieval of the
// discovery mode
// setting through the GetDiscoveryMode command.
pub async fn get_discovery_mode<T: transport::Transport>(
    transport: &T,
    request: &GetDiscoveryMode,
) -> Result<GetDiscoveryModeResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation sets the discovery mode operation of a device. See Section 7.2
// for the
// definition of the different device discovery modes. The device shall support
// configuration of
// the discovery mode setting through the SetDiscoveryMode command.
pub async fn set_discovery_mode<T: transport::Transport>(
    transport: &T,
    request: &SetDiscoveryMode,
) -> Result<SetDiscoveryModeResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation gets the remote discovery mode of a device. See Section 7.4
// for the definition
// of remote discovery extensions. A device that supports remote discovery shall
// support
// retrieval of the remote discovery mode setting through the
// GetRemoteDiscoveryMode
// command.
pub async fn get_remote_discovery_mode<T: transport::Transport>(
    transport: &T,
    request: &GetRemoteDiscoveryMode,
) -> Result<GetRemoteDiscoveryModeResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation sets the remote discovery mode of operation of a device. See
// Section 7.4 for
// the definition of remote discovery remote extensions. A device that supports
// remote discovery
// shall support configuration of the discovery mode setting through the
// SetRemoteDiscoveryMode command.
pub async fn set_remote_discovery_mode<T: transport::Transport>(
    transport: &T,
    request: &SetRemoteDiscoveryMode,
) -> Result<SetRemoteDiscoveryModeResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation gets the remote DP address or addresses from a device. If the
// device supports
// remote discovery, as specified in Section 7.4, the device shall support
// retrieval of the remote
// DP address(es) through the GetDPAddresses command.
pub async fn get_dp_addresses<T: transport::Transport>(
    transport: &T,
    request: &GetDPAddresses,
) -> Result<GetDPAddressesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation sets the remote DP address or addresses on a device. If the
// device supports
// remote discovery, as specified in Section 7.4, the device shall support
// configuration of the
// remote DP address(es) through the SetDPAddresses command.
pub async fn set_dp_addresses<T: transport::Transport>(
    transport: &T,
    request: &SetDPAddresses,
) -> Result<SetDPAddressesResponse, transport::Error> {
    transport::request(transport, request).await
}

// A client can ask for the device service endpoint reference address property
// that can be used
// to derive the password equivalent for remote user operation. The device shall
// support the
// GetEndpointReference command returning the address property of the device
// service
// endpoint reference.
pub async fn get_endpoint_reference<T: transport::Transport>(
    transport: &T,
    request: &GetEndpointReference,
) -> Result<GetEndpointReferenceResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns the configured remote user (if any). A device
// supporting remote user
// handling shall support this operation. The user is only valid for the
// WS-UserToken profile or
// as a HTTP / RTSP user.
pub async fn get_remote_user<T: transport::Transport>(
    transport: &T,
    request: &GetRemoteUser,
) -> Result<GetRemoteUserResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation sets the remote user. A device supporting remote user handling
// shall support this
// operation. The user is only valid for the WS-UserToken profile or as a HTTP /
// RTSP user.
pub async fn set_remote_user<T: transport::Transport>(
    transport: &T,
    request: &SetRemoteUser,
) -> Result<SetRemoteUserResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation lists the registered users and corresponding credentials on a
// device. The
// device shall support retrieval of registered device users and their
// credentials for the user
// token through the GetUsers command.
pub async fn get_users<T: transport::Transport>(
    transport: &T,
    request: &GetUsers,
) -> Result<GetUsersResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation creates new device users and corresponding credentials on a
// device for authentication purposes.
// The device shall support creation of device users and their credentials
// through the CreateUsers
// command. Either all users are created successfully or a fault message shall
// be returned
// without creating any user.
pub async fn create_users<T: transport::Transport>(
    transport: &T,
    request: &CreateUsers,
) -> Result<CreateUsersResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation deletes users on a device. The device shall support deletion
// of device users and their credentials
// through the DeleteUsers command. A device may have one or more fixed users
// that cannot be deleted to ensure access to the unit. Either all users are
// deleted successfully or a
// fault message shall be returned and no users be deleted.
pub async fn delete_users<T: transport::Transport>(
    transport: &T,
    request: &DeleteUsers,
) -> Result<DeleteUsersResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation updates the settings for one or several users on a device for
// authentication purposes.
// The device shall support update of device users and their credentials through
// the SetUser command.
// Either all change requests are processed successfully or a fault message
// shall be returned and no change requests be processed.
pub async fn set_user<T: transport::Transport>(
    transport: &T,
    request: &SetUser,
) -> Result<SetUserResponse, transport::Error> {
    transport::request(transport, request).await
}

// It is possible for an endpoint to request a URL that can be used to retrieve
// the complete
// schema and WSDL definitions of a device. The command gives in return a URL
// entry point
// where all the necessary product specific WSDL and schema definitions can be
// retrieved. The
// device shall provide a URL for WSDL and schema download through the
// GetWsdlUrl command.
pub async fn get_wsdl_url<T: transport::Transport>(
    transport: &T,
    request: &GetWsdlUrl,
) -> Result<GetWsdlUrlResponse, transport::Error> {
    transport::request(transport, request).await
}

// This method has been replaced by the more generic GetServices method.
// For capabilities of individual services refer to the GetServiceCapabilities
// methods.
pub async fn get_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetCapabilities,
) -> Result<GetCapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation is used by an endpoint to get the hostname from a device. The
// device shall
// return its hostname configurations through the GetHostname command.
pub async fn get_hostname<T: transport::Transport>(
    transport: &T,
    request: &GetHostname,
) -> Result<GetHostnameResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation sets the hostname on a device. It shall be possible to set the
// device hostname
// configurations through the SetHostname command.
pub async fn set_hostname<T: transport::Transport>(
    transport: &T,
    request: &SetHostname,
) -> Result<SetHostnameResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation controls whether the hostname is set manually or retrieved via
// DHCP.
pub async fn set_hostname_from_dhcp<T: transport::Transport>(
    transport: &T,
    request: &SetHostnameFromDHCP,
) -> Result<SetHostnameFromDHCPResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation gets the DNS settings from a device. The device shall return
// its DNS
// configurations through the GetDNS command.
pub async fn get_dns<T: transport::Transport>(
    transport: &T,
    request: &GetDNS,
) -> Result<GetDNSResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation sets the DNS settings on a device. It shall be possible to set
// the device DNS
// configurations through the SetDNS command.
pub async fn set_dns<T: transport::Transport>(
    transport: &T,
    request: &SetDNS,
) -> Result<SetDNSResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation gets the NTP settings from a device. If the device supports
// NTP, it shall be
// possible to get the NTP server settings through the GetNTP command.
pub async fn get_ntp<T: transport::Transport>(
    transport: &T,
    request: &GetNTP,
) -> Result<GetNTPResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation sets the NTP settings on a device. If the device supports NTP,
// it shall be
// possible to set the NTP server settings through the SetNTP command.
pub async fn set_ntp<T: transport::Transport>(
    transport: &T,
    request: &SetNTP,
) -> Result<SetNTPResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation gets the dynamic DNS settings from a device. If the device
// supports dynamic
// DNS as specified in [RFC 2136] and [RFC 4702], it shall be possible to get
// the type, name
// and TTL through the GetDynamicDNS command.
pub async fn get_dynamic_dns<T: transport::Transport>(
    transport: &T,
    request: &GetDynamicDNS,
) -> Result<GetDynamicDNSResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation sets the dynamic DNS settings on a device. If the device
// supports dynamic
// DNS as specified in [RFC 2136] and [RFC 4702], it shall be possible to set
// the type, name
// and TTL through the SetDynamicDNS command.
pub async fn set_dynamic_dns<T: transport::Transport>(
    transport: &T,
    request: &SetDynamicDNS,
) -> Result<SetDynamicDNSResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation gets the network interface configuration from a device. The
// device shall
// support return of network interface configuration settings as defined by the
// NetworkInterface
// type through the GetNetworkInterfaces command.
pub async fn get_network_interfaces<T: transport::Transport>(
    transport: &T,
    request: &GetNetworkInterfaces,
) -> Result<GetNetworkInterfacesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation sets the network interface configuration on a device. The
// device shall support
// network configuration of supported network interfaces through the
// SetNetworkInterfaces
// command.
pub async fn set_network_interfaces<T: transport::Transport>(
    transport: &T,
    request: &SetNetworkInterfaces,
) -> Result<SetNetworkInterfacesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation gets defined network protocols from a device. The device shall
// support the
// GetNetworkProtocols command returning configured network protocols.
pub async fn get_network_protocols<T: transport::Transport>(
    transport: &T,
    request: &GetNetworkProtocols,
) -> Result<GetNetworkProtocolsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation configures defined network protocols on a device. The device
// shall support
// configuration of defined network protocols through the SetNetworkProtocols
// command.
pub async fn set_network_protocols<T: transport::Transport>(
    transport: &T,
    request: &SetNetworkProtocols,
) -> Result<SetNetworkProtocolsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation gets the default gateway settings from a device. The device
// shall support the
// GetNetworkDefaultGateway command returning configured default gateway
// address(es).
pub async fn get_network_default_gateway<T: transport::Transport>(
    transport: &T,
    request: &GetNetworkDefaultGateway,
) -> Result<GetNetworkDefaultGatewayResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation sets the default gateway settings on a device. The device
// shall support
// configuration of default gateway through the SetNetworkDefaultGateway
// command.
pub async fn set_network_default_gateway<T: transport::Transport>(
    transport: &T,
    request: &SetNetworkDefaultGateway,
) -> Result<SetNetworkDefaultGatewayResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation gets the zero-configuration from a device. If the device
// supports dynamic IP
// configuration according to [RFC3927], it shall support the return of IPv4
// zero configuration
// address and status through the GetZeroConfiguration command.
pub async fn get_zero_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetZeroConfiguration,
) -> Result<GetZeroConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation sets the zero-configuration. Use GetCapalities to get if
// zero-zero-configuration is supported or not.
pub async fn set_zero_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetZeroConfiguration,
) -> Result<SetZeroConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation gets the IP address filter settings from a device. If the
// device supports device
// access control based on IP filtering rules (denied or accepted ranges of IP
// addresses), the
// device shall support the GetIPAddressFilter command.
pub async fn get_ip_address_filter<T: transport::Transport>(
    transport: &T,
    request: &GetIPAddressFilter,
) -> Result<GetIPAddressFilterResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation sets the IP address filter settings on a device. If the device
// supports device
// access control based on IP filtering rules (denied or accepted ranges of IP
// addresses), the
// device shall support configuration of IP filtering rules through the
// SetIPAddressFilter
// command.
pub async fn set_ip_address_filter<T: transport::Transport>(
    transport: &T,
    request: &SetIPAddressFilter,
) -> Result<SetIPAddressFilterResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation adds an IP filter address to a device. If the device supports
// device access
// control based on IP filtering rules (denied or accepted ranges of IP
// addresses), the device
// shall support adding of IP filtering addresses through the AddIPAddressFilter
// command.
pub async fn add_ip_address_filter<T: transport::Transport>(
    transport: &T,
    request: &AddIPAddressFilter,
) -> Result<AddIPAddressFilterResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation deletes an IP filter address from a device. If the device
// supports device access
// control based on IP filtering rules (denied or accepted ranges of IP
// addresses), the device
// shall support deletion of IP filtering addresses through the
// RemoveIPAddressFilter command.
pub async fn remove_ip_address_filter<T: transport::Transport>(
    transport: &T,
    request: &RemoveIPAddressFilter,
) -> Result<RemoveIPAddressFilterResponse, transport::Error> {
    transport::request(transport, request).await
}

// Access to different services and sub-sets of services should be subject to
// access control. The
// WS-Security framework gives the prerequisite for end-point authentication.
// Authorization
// decisions can then be taken using an access security policy. This standard
// does not mandate
// any particular policy description format or security policy but this is up to
// the device
// manufacturer or system provider to choose policy and policy description
// format of choice.
// However, an access policy (in arbitrary format) can be requested using this
// command. If the
// device supports access policy settings based on WS-Security authentication,
// then the device
// shall support this command.
pub async fn get_access_policy<T: transport::Transport>(
    transport: &T,
    request: &GetAccessPolicy,
) -> Result<GetAccessPolicyResponse, transport::Error> {
    transport::request(transport, request).await
}

// This command sets the device access security policy (for more details on the
// access security
// policy see the Get command). If the device supports access policy settings
// based on WS-Security authentication, then the device shall support this
// command.
pub async fn set_access_policy<T: transport::Transport>(
    transport: &T,
    request: &SetAccessPolicy,
) -> Result<SetAccessPolicyResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation generates a private/public key pair and also can create a
// self-signed device
// certificate as a result of key pair generation. The certificate is created
// using a suitable
// onboard key pair generation mechanism.
pub async fn create_certificate<T: transport::Transport>(
    transport: &T,
    request: &CreateCertificate,
) -> Result<CreateCertificateResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation gets all device server certificates (including self-signed)
// for the purpose of TLS
// authentication and all device client certificates for the purpose of IEEE
// 802.1X authentication.
// This command lists only the TLS server certificates and IEEE 802.1X client
// certificates for the
// device (neither trusted CA certificates nor trusted root certificates). The
// certificates are
// returned as binary data. A device that supports TLS shall support this
// command and the
// certificates shall be encoded using ASN.1 [X.681], [X.682], [X.683] DER
// [X.690] encoding
// rules.
pub async fn get_certificates<T: transport::Transport>(
    transport: &T,
    request: &GetCertificates,
) -> Result<GetCertificatesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation is specific to TLS functionality. This operation gets the
// status
// (enabled/disabled) of the device TLS server certificates. A device that
// supports TLS shall
// support this command.
pub async fn get_certificates_status<T: transport::Transport>(
    transport: &T,
    request: &GetCertificatesStatus,
) -> Result<GetCertificatesStatusResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation is specific to TLS functionality. This operation sets the
// status (enable/disable)
// of the device TLS server certificates. A device that supports TLS shall
// support this command.
// Typically only one device server certificate is allowed to be enabled at a
// time.
pub async fn set_certificates_status<T: transport::Transport>(
    transport: &T,
    request: &SetCertificatesStatus,
) -> Result<SetCertificatesStatusResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation deletes a certificate or multiple certificates. The device MAY
// also delete a
// private/public key pair which is coupled with the certificate to be deleted.
// The device that
// support either TLS or IEEE 802.1X shall support the deletion of a certificate
// or multiple
// certificates through this command. Either all certificates are deleted
// successfully or a fault
// message shall be returned without deleting any certificate.
pub async fn delete_certificates<T: transport::Transport>(
    transport: &T,
    request: &DeleteCertificates,
) -> Result<DeleteCertificatesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a PKCS #10 certificate signature request from the
// device. The
// returned information field shall be either formatted exactly as specified in
// [PKCS#10] or PEM
// encoded [PKCS#10] format. In order for this command to work, the device must
// already have
// a private/public key pair. This key pair should be referred by CertificateID
// as specified in the
// input parameter description. This CertificateID refers to the key pair
// generated using
// CreateCertificate command.
pub async fn get_pkcs_10_request<T: transport::Transport>(
    transport: &T,
    request: &GetPkcs10Request,
) -> Result<GetPkcs10RequestResponse, transport::Error> {
    transport::request(transport, request).await
}

// TLS server certificate(s) or IEEE 802.1X client certificate(s) created using
// the PKCS#10
// certificate request command can be loaded into the device using this command
// (see Section
// 8.4.13). The certificate ID in the request shall be present. The device may
// sort the received
// certificate(s) based on the public key and subject information in the
// certificate(s).
// The certificate ID in the request will be the ID value the client wish to
// have. The device is
// supposed to scan the generated key pairs present in the device to identify
// which is the
// correspondent key pair with the loaded certificate and then make the link
// between the
// certificate and the key pair.
pub async fn load_certificates<T: transport::Transport>(
    transport: &T,
    request: &LoadCertificates,
) -> Result<LoadCertificatesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation is specific to TLS functionality. This operation gets the
// status
// (enabled/disabled) of the device TLS client authentication. A device that
// supports TLS shall
// support this command.
pub async fn get_client_certificate_mode<T: transport::Transport>(
    transport: &T,
    request: &GetClientCertificateMode,
) -> Result<GetClientCertificateModeResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation is specific to TLS functionality. This operation sets the
// status
// (enabled/disabled) of the device TLS client authentication. A device that
// supports TLS shall
// support this command.
pub async fn set_client_certificate_mode<T: transport::Transport>(
    transport: &T,
    request: &SetClientCertificateMode,
) -> Result<SetClientCertificateModeResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation gets a list of all available relay outputs and their settings.
pub async fn get_relay_outputs<T: transport::Transport>(
    transport: &T,
    request: &GetRelayOutputs,
) -> Result<GetRelayOutputsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation sets the settings of a relay output.
pub async fn set_relay_output_settings<T: transport::Transport>(
    transport: &T,
    request: &SetRelayOutputSettings,
) -> Result<SetRelayOutputSettingsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation sets the state of a relay output.
pub async fn set_relay_output_state<T: transport::Transport>(
    transport: &T,
    request: &SetRelayOutputState,
) -> Result<SetRelayOutputStateResponse, transport::Error> {
    transport::request(transport, request).await
}

// Manage auxiliary commands supported by a device, such as controlling an
// Infrared (IR) lamp,
// a heater or a wiper or a thermometer that is connected to the device.
pub async fn send_auxiliary_command<T: transport::Transport>(
    transport: &T,
    request: &SendAuxiliaryCommand,
) -> Result<SendAuxiliaryCommandResponse, transport::Error> {
    transport::request(transport, request).await
}

// CA certificates will be loaded into a device and be used for the sake of
// following two cases.
// The one is for the purpose of TLS client authentication in TLS server
// function. The other one
// is for the purpose of Authentication Server authentication in IEEE 802.1X
// function. This
// operation gets all CA certificates loaded into a device. A device that
// supports either TLS client
// authentication or IEEE 802.1X shall support this command and the returned
// certificates shall
// be encoded using ASN.1 [X.681], [X.682], [X.683] DER [X.690] encoding rules.
pub async fn get_ca_certificates<T: transport::Transport>(
    transport: &T,
    request: &GetCACertificates,
) -> Result<GetCACertificatesResponse, transport::Error> {
    transport::request(transport, request).await
}

// There might be some cases that a Certificate Authority or some other
// equivalent creates a
// certificate without having PKCS#10 certificate signing request. In such
// cases, the certificate
// will be bundled in conjunction with its private key. This command will be
// used for such use
// case scenarios. The certificate ID in the request is optionally set to the ID
// value the client
// wish to have. If the certificate ID is not specified in the request, device
// can choose the ID
// accordingly.
pub async fn load_certificate_with_private_key<T: transport::Transport>(
    transport: &T,
    request: &LoadCertificateWithPrivateKey,
) -> Result<LoadCertificateWithPrivateKeyResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests the information of a certificate specified by
// certificate ID. The device
// should respond with its “Issuer DN”, “Subject DN”, “Key usage”,
// "Extended key usage”, “Key
// Length”, “Version”, “Serial Number”, “Signature Algorithm” and
// “Validity” data as the
// information of the certificate, as long as the device can retrieve such
// information from the
// specified certificate.
pub async fn get_certificate_information<T: transport::Transport>(
    transport: &T,
    request: &GetCertificateInformation,
) -> Result<GetCertificateInformationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This command is used when it is necessary to load trusted CA certificates or
// trusted root
// certificates for the purpose of verification for its counterpart i.e. client
// certificate verification in
// TLS function or server certificate verification in IEEE 802.1X function.
pub async fn load_ca_certificates<T: transport::Transport>(
    transport: &T,
    request: &LoadCACertificates,
) -> Result<LoadCACertificatesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation newly creates IEEE 802.1X configuration parameter set of the
// device. The
// device shall support this command if it supports IEEE 802.1X. If the device
// receives this
// request with already existing configuration token (Dot1XConfigurationToken)
// specification, the
// device should respond with 'ter:ReferenceToken ' error to indicate there is
// some configuration
// conflict.
pub async fn create_dot_1x_configuration<T: transport::Transport>(
    transport: &T,
    request: &CreateDot1XConfiguration,
) -> Result<CreateDot1XConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// While the CreateDot1XConfiguration command is trying to create a new
// configuration
// parameter set, this operation modifies existing IEEE 802.1X configuration
// parameter set of
// the device. A device that support IEEE 802.1X shall support this command.
pub async fn set_dot_1x_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetDot1XConfiguration,
) -> Result<SetDot1XConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation gets one IEEE 802.1X configuration parameter set from the
// device by
// specifying the configuration token (Dot1XConfigurationToken).
pub async fn get_dot_1x_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetDot1XConfiguration,
) -> Result<GetDot1XConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation gets all the existing IEEE 802.1X configuration parameter sets
// from the device.
// The device shall respond with all the IEEE 802.1X configurations so that the
// client can get to
// know how many IEEE 802.1X configurations are existing and how they are
// configured.
pub async fn get_dot_1x_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetDot1XConfigurations,
) -> Result<GetDot1XConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation deletes an IEEE 802.1X configuration parameter set from the
// device. Which
// configuration should be deleted is specified by the 'Dot1XConfigurationToken'
// in the request.
// A device that support IEEE 802.1X shall support this command.
pub async fn delete_dot_1x_configuration<T: transport::Transport>(
    transport: &T,
    request: &DeleteDot1XConfiguration,
) -> Result<DeleteDot1XConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns the IEEE802.11 capabilities. The device shall support
// this operation.
pub async fn get_dot_11_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetDot11Capabilities,
) -> Result<GetDot11CapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns the status of a wireless network interface. The device
// shall support this
// command.
pub async fn get_dot_11_status<T: transport::Transport>(
    transport: &T,
    request: &GetDot11Status,
) -> Result<GetDot11StatusResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns a lists of the wireless networks in range of the
// device. A device should
// support this operation.
pub async fn scan_available_dot_11_networks<T: transport::Transport>(
    transport: &T,
    request: &ScanAvailableDot11Networks,
) -> Result<ScanAvailableDot11NetworksResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation is used to retrieve URIs from which system information may be
// downloaded
// using HTTP. URIs may be returned for the following system information:
pub async fn get_system_uris<T: transport::Transport>(
    transport: &T,
    request: &GetSystemUris,
) -> Result<GetSystemUrisResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation initiates a firmware upgrade using the HTTP POST mechanism.
// The response
// to the command includes an HTTP URL to which the upgrade file may be
// uploaded. The
// actual upgrade takes place as soon as the HTTP POST operation has completed.
// The device
// should support firmware upgrade through the StartFirmwareUpgrade command. The
// exact
// format of the firmware data is outside the scope of this specification.
// Firmware upgrade over HTTP may be achieved using the following steps:
pub async fn start_firmware_upgrade<T: transport::Transport>(
    transport: &T,
    request: &StartFirmwareUpgrade,
) -> Result<StartFirmwareUpgradeResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation initiates a system restore from backed up configuration data
// using the HTTP
// POST mechanism. The response to the command includes an HTTP URL to which the
// backup
// file may be uploaded. The actual restore takes place as soon as the HTTP POST
// operation
// has completed. Devices should support system restore through the
// StartSystemRestore
// command. The exact format of the backup configuration data is outside the
// scope of this
// specification.
pub async fn start_system_restore<T: transport::Transport>(
    transport: &T,
    request: &StartSystemRestore,
) -> Result<StartSystemRestoreResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation lists all existing storage configurations for the device.
pub async fn get_storage_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetStorageConfigurations,
) -> Result<GetStorageConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation creates a new storage configuration.
// The configuration data shall be created in the device and shall be persistent
// (remain after reboot).
pub async fn create_storage_configuration<T: transport::Transport>(
    transport: &T,
    request: &CreateStorageConfiguration,
) -> Result<CreateStorageConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation retrieves the Storage configuration associated with the given
// storage configuration token.
pub async fn get_storage_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetStorageConfiguration,
) -> Result<GetStorageConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation modifies an existing Storage configuration.
pub async fn set_storage_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetStorageConfiguration,
) -> Result<SetStorageConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation deletes the given storage configuration and configuration
// change shall always be persistent.
pub async fn delete_storage_configuration<T: transport::Transport>(
    transport: &T,
    request: &DeleteStorageConfiguration,
) -> Result<DeleteStorageConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation lists all existing geo location configurations for the device.
pub async fn get_geo_location<T: transport::Transport>(
    transport: &T,
    request: &GetGeoLocation,
) -> Result<GetGeoLocationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation allows to modify one or more geo configuration entries.
pub async fn set_geo_location<T: transport::Transport>(
    transport: &T,
    request: &SetGeoLocation,
) -> Result<SetGeoLocationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation deletes the given geo location entries.
pub async fn delete_geo_location<T: transport::Transport>(
    transport: &T,
    request: &DeleteGeoLocation,
) -> Result<DeleteGeoLocationResponse, transport::Error> {
    transport::request(transport, request).await
}
