#![allow(clippy::derive_partial_eq_without_eq)]

use types as pt;
use validate::Validate;
use xsd_types::types as xs;
use yaserde_derive::{YaDeserialize, YaSerialize};

// ServiceCapabilities structure reflects optional functionality of a service.
// The information is static and does not change during device operation.
// The following capabilities are available:
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct ServiceCapabilities {
    // The maximum number of entries returned by a single Get<Entity>List or
    // Get<Entity> request. The device shall never return more than this number
    // of entities
    // in a single response.
    #[yaserde(attribute, rename = "MaxLimit")]
    pub max_limit: u32,

    // Indicates the maximum number of doors supported by the device.
    #[yaserde(attribute, rename = "MaxDoors")]
    pub max_doors: Option<u32>,

    // Indicates that the client is allowed to supply the token when creating
    // doors.
    // To enable the use of the command SetDoor, the value must be set to true.
    #[yaserde(attribute, rename = "ClientSuppliedTokenSupported")]
    pub client_supplied_token_supported: Option<bool>,
}

impl Validate for ServiceCapabilities {}

// pub type Capabilities = ServiceCapabilities;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct DoorInfoBase {
    // A user readable name. It shall be up to 64 characters.
    #[yaserde(prefix = "tdc", rename = "Name")]
    pub name: pt::Name,

    // A user readable description. It shall be up to 1024 characters.
    #[yaserde(prefix = "tdc", rename = "Description")]
    pub description: Option<pt::Description>,

    pub base: pt::DataEntity,
}

impl Validate for DoorInfoBase {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct DoorInfo {
    // The capabilities of the Door.
    #[yaserde(prefix = "tdc", rename = "Capabilities")]
    pub capabilities: DoorCapabilities,

    pub base: DoorInfoBase,
}

impl Validate for DoorInfo {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct Door {
    // The type of door. Is of type text. Can be either one of the following
    // reserved
    // ONVIF types: "pt:Door", "pt:ManTrap", "pt:Turnstile", "pt:RevolvingDoor",
    // "pt:Barrier", or a custom defined type.
    #[yaserde(prefix = "tdc", rename = "DoorType")]
    pub door_type: pt::Name,

    // A structure defining times such as how long the door is unlocked when
    // accessed, extended grant time, etc.
    #[yaserde(prefix = "tdc", rename = "Timings")]
    pub timings: Timings,

    #[yaserde(prefix = "tdc", rename = "Extension")]
    pub extension: Option<DoorExtension>,

    pub base: DoorInfo,
}

impl Validate for Door {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct DoorExtension {}

impl Validate for DoorExtension {}

// A structure defining times such as how long the door is unlocked when
// accessed,
// extended grant time, etc.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct Timings {
    // When access is granted (door mode becomes Accessed), the latch is
    // unlocked.
    // ReleaseTime is the time from when the latch is unlocked until it is
    // relocked again (unless the door is physically opened).
    #[yaserde(prefix = "tdc", rename = "ReleaseTime")]
    pub release_time: xs::Duration,

    // The time from when the door is physically opened until the door is set in
    // the
    // DoorOpenTooLong alarm state.
    #[yaserde(prefix = "tdc", rename = "OpenTime")]
    pub open_time: xs::Duration,

    // Some individuals need extra time to open the door before the latch
    // relocks.
    // If supported, ExtendedReleaseTime shall be added to ReleaseTime if
    // UseExtendedTime
    // is set to true in the AccessDoor command.
    #[yaserde(prefix = "tdc", rename = "ExtendedReleaseTime")]
    pub extended_release_time: Option<xs::Duration>,

    // If the door is physically opened after access is granted,
    // then DelayTimeBeforeRelock is the time from when the door is physically
    // opened until the latch goes back to locked state.
    #[yaserde(prefix = "tdc", rename = "DelayTimeBeforeRelock")]
    pub delay_time_before_relock: Option<xs::Duration>,

    // Some individuals need extra time to pass through the door. If supported,
    // ExtendedOpenTime shall be added to OpenTime if UseExtendedTime is set to
    // true
    // in the AccessDoor command.
    #[yaserde(prefix = "tdc", rename = "ExtendedOpenTime")]
    pub extended_open_time: Option<xs::Duration>,

    // Before a DoorOpenTooLong alarm state is generated, a signal will sound to
    // indicate
    // that the door must be closed. PreAlarmTime defines how long before
    // DoorOpenTooLong
    // the warning signal shall sound.
    #[yaserde(prefix = "tdc", rename = "PreAlarmTime")]
    pub pre_alarm_time: Option<xs::Duration>,

    #[yaserde(prefix = "tdc", rename = "Extension")]
    pub extension: Option<TimingsExtension>,
}

impl Validate for Timings {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct TimingsExtension {}

impl Validate for TimingsExtension {}

// DoorCapabilities reflect optional functionality of a particular physical
// entity.
// Different door instances may have different set of capabilities.
// This information may change during device operation, e.g. if hardware
// settings are changed.
// The following capabilities are available:
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct DoorCapabilities {
    // Indicates whether or not this Door instance supports AccessDoor command
    // to
    // perform momentary access.
    #[yaserde(attribute, rename = "Access")]
    pub access: Option<bool>,

    // Indicates that this Door instance supports overriding configured timing
    // in the
    // AccessDoor command.
    #[yaserde(attribute, rename = "AccessTimingOverride")]
    pub access_timing_override: Option<bool>,

    // Indicates that this Door instance supports LockDoor command to lock the
    // door.
    #[yaserde(attribute, rename = "Lock")]
    pub lock: Option<bool>,

    // Indicates that this Door instance supports UnlockDoor command to unlock
    // the
    // door.
    #[yaserde(attribute, rename = "Unlock")]
    pub unlock: Option<bool>,

    // Indicates that this Door instance supports BlockDoor command to block the
    // door.
    #[yaserde(attribute, rename = "Block")]
    pub block: Option<bool>,

    // Indicates that this Door instance supports DoubleLockDoor command to lock
    // multiple locks on the door.
    #[yaserde(attribute, rename = "DoubleLock")]
    pub double_lock: Option<bool>,

    // Indicates that this Door instance supports LockDown (and LockDownRelease)
    // commands to lock the door and put it in LockedDown mode.
    #[yaserde(attribute, rename = "LockDown")]
    pub lock_down: Option<bool>,

    // Indicates that this Door instance supports LockOpen (and LockOpenRelease)
    // commands to unlock the door and put it in LockedOpen mode.
    #[yaserde(attribute, rename = "LockOpen")]
    pub lock_open: Option<bool>,

    // Indicates that this Door instance has a DoorMonitor and supports the
    // DoorPhysicalState event.
    #[yaserde(attribute, rename = "DoorMonitor")]
    pub door_monitor: Option<bool>,

    // Indicates that this Door instance has a LockMonitor and supports the
    // LockPhysicalState event.
    #[yaserde(attribute, rename = "LockMonitor")]
    pub lock_monitor: Option<bool>,

    // Indicates that this Door instance has a DoubleLockMonitor and supports
    // the
    // DoubleLockPhysicalState event.
    #[yaserde(attribute, rename = "DoubleLockMonitor")]
    pub double_lock_monitor: Option<bool>,

    // Indicates that this Door instance supports door alarm and the DoorAlarm
    // event.
    #[yaserde(attribute, rename = "Alarm")]
    pub alarm: Option<bool>,

    // Indicates that this Door instance has a Tamper detector and supports the
    // DoorTamper event.
    #[yaserde(attribute, rename = "Tamper")]
    pub tamper: Option<bool>,

    // Indicates that this Door instance supports door fault and the DoorFault
    // event.
    #[yaserde(attribute, rename = "Fault")]
    pub fault: Option<bool>,
}

impl Validate for DoorCapabilities {}

// The DoorState structure contains current aggregate runtime status of Door.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct DoorState {
    // Physical state of the Door; it is of type DoorPhysicalState. A device
    // that
    // signals support for DoorMonitor capability for a particular door instance
    // shall provide
    // this field.
    #[yaserde(prefix = "tdc", rename = "DoorPhysicalState")]
    pub door_physical_state: Option<DoorPhysicalState>,

    // Physical state of the Lock; it is of type LockPhysicalState. A device
    // that
    // signals support for LockMonitor capability for a particular door instance
    // shall provide
    // this field.
    #[yaserde(prefix = "tdc", rename = "LockPhysicalState")]
    pub lock_physical_state: Option<LockPhysicalState>,

    // Physical state of the DoubleLock; it is of type LockPhysicalState. A
    // device that signals support for DoubleLockMonitor capability for a
    // particular door
    // instance shall provide this field.
    #[yaserde(prefix = "tdc", rename = "DoubleLockPhysicalState")]
    pub double_lock_physical_state: Option<LockPhysicalState>,

    // Alarm state of the door; it is of type DoorAlarmState. A device that
    // signals support for Alarm capability for a particular door instance shall
    // provide this
    // field.
    #[yaserde(prefix = "tdc", rename = "Alarm")]
    pub alarm: Option<DoorAlarmState>,

    // Tampering state of the door; it is of type DoorTamper. A device that
    // signals support for Tamper capability for a particular door instance
    // shall provide this
    // field.
    #[yaserde(prefix = "tdc", rename = "Tamper")]
    pub tamper: Option<DoorTamper>,

    // Fault information for door; it is of type DoorFault. A device that
    // signals
    // support for Fault capability for a particular door instance shall provide
    // this field.
    #[yaserde(prefix = "tdc", rename = "Fault")]
    pub fault: Option<DoorFault>,

    // The logical operating mode of the door; it is of type DoorMode. An ONVIF
    // compatible device shall report current operating mode in this field.
    #[yaserde(prefix = "tdc", rename = "DoorMode")]
    pub door_mode: DoorMode,
}

impl Validate for DoorState {}

// The physical state of a Door.
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum DoorPhysicalState {
    // Value is currently unknown (possibly due to initialization or monitors
    // not
    // giving a conclusive result).
    Unknown,
    // Door is open.
    Open,
    // Door is closed.
    Closed,
    // Door monitor fault is detected.
    Fault,
    __Unknown__(String),
}

impl Default for DoorPhysicalState {
    fn default() -> DoorPhysicalState {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for DoorPhysicalState {}

// The physical state of a Lock (including Double Lock).
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum LockPhysicalState {
    // Value is currently not known.
    Unknown,
    // Lock is activated.
    Locked,
    // Lock is not activated.
    Unlocked,
    // Lock fault is detected.
    Fault,
    __Unknown__(String),
}

impl Default for LockPhysicalState {
    fn default() -> LockPhysicalState {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for LockPhysicalState {}

// Describes the state of a Door with regard to alarms.
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum DoorAlarmState {
    // No alarm.
    Normal,
    // Door is forced open.
    DoorForcedOpen,
    // Door is held open too long.
    DoorOpenTooLong,
    __Unknown__(String),
}

impl Default for DoorAlarmState {
    fn default() -> DoorAlarmState {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for DoorAlarmState {}

// Tampering information for a Door.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct DoorTamper {
    // Optional field; Details describing tampering state change (e.g., reason,
    // place and time).
    // NOTE: All fields (including this one) which are designed to give
    // end-user prompts can be localized to the customer's native language.
    #[yaserde(prefix = "tdc", rename = "Reason")]
    pub reason: Option<String>,

    // State of the tamper detector; it is of type DoorTamperState.
    #[yaserde(prefix = "tdc", rename = "State")]
    pub state: DoorTamperState,
}

impl Validate for DoorTamper {}

// Describes the state of a Tamper detector.
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum DoorTamperState {
    // Value is currently not known.
    Unknown,
    // No tampering is detected.
    NotInTamper,
    // Tampering is detected.
    TamperDetected,
    __Unknown__(String),
}

impl Default for DoorTamperState {
    fn default() -> DoorTamperState {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for DoorTamperState {}

// Fault information for a Door.
// This can be extended with optional attributes in the future.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct DoorFault {
    // Optional reason for fault.
    #[yaserde(prefix = "tdc", rename = "Reason")]
    pub reason: Option<String>,

    // Overall fault state for the door; it is of type DoorFaultState. If there
    // are any faults, the value shall be: FaultDetected. Details of the
    // detected fault shall
    // be found in the Reason field, and/or the various DoorState fields and/or
    // in extensions
    // to this structure.
    #[yaserde(prefix = "tdc", rename = "State")]
    pub state: DoorFaultState,
}

impl Validate for DoorFault {}

// Describes the state of a Door fault.
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum DoorFaultState {
    // Fault state is unknown.
    Unknown,
    // No fault is detected.
    NotInFault,
    // Fault is detected.
    FaultDetected,
    __Unknown__(String),
}

impl Default for DoorFaultState {
    fn default() -> DoorFaultState {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for DoorFaultState {}

// The DoorMode describe the mode of operation from a logical perspective.
// Setting a door mode reflects the intent to set a door in a physical state.
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum DoorMode {
    // The mode of operation is unknown.
    Unknown,
    // The intention is to set the door to a physical locked state.
    // In this mode the device shall provide momentary access using the
    // AccessDoor
    // method if supported by the door instance.
    Locked,
    // The intention is to set the door to a physical unlocked state.
    // Alarms related to door timing operations such as open too long
    // or forced open are masked in this mode.
    Unlocked,
    // The intention is to momentary set the door to a physical unlocked state.
    // After a predefined time the device shall revert the door to its previous
    // mode.
    // Alarms related to timing operations such as door forced open are masked
    // in this mode.
    Accessed,
    // The intention is to set the door to a physical locked state and the
    // device shall not allow AccessDoor requests, i.e. it is not possible
    // for the door to go to the accessed mode.
    // All other requests to change the door mode are allowed.
    Blocked,
    // The intention is to set the door to a physical locked state and the
    // device
    // shall only allow the LockDownReleaseDoor request.
    // All other requests to change the door mode are not allowed.
    LockedDown,
    // The intention is to set the door to a physical unlocked state and the
    // device shall only allow the LockOpenReleaseDoor request.
    // All other requests to change the door mode are not allowed.
    LockedOpen,
    // The intention is to set the door with multiple locks to a physical double
    // locked state.
    // If the door does not support double locking the devices shall
    // treat this as a normal locked mode.
    // When changing to an unlocked mode from the double locked mode, the
    // physical state
    // of the door may first go to locked state before unlocking.
    DoubleLocked,
    __Unknown__(String),
}

impl Default for DoorMode {
    fn default() -> DoorMode {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for DoorMode {}

// Extension for the AccessDoor command.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct AccessDoorExtension {}

impl Validate for AccessDoorExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct GetServiceCapabilitiesResponse {
    // The capability response message contains the requested DoorControl
    // service capabilities using a hierarchical XML capability structure.
    #[yaserde(prefix = "tdc", rename = "Capabilities")]
    pub capabilities: ServiceCapabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct GetDoorInfoList {
    // Maximum number of entries to return. If Limit is omitted or if the
    // value of Limit is higher than what the device supports, then the device
    // shall
    // return its maximum amount of entries.
    #[yaserde(prefix = "tdc", rename = "Limit")]
    pub limit: Option<i32>,

    // Start returning entries from this start reference. If not specified,
    // entries shall start from the beginning of the dataset.
    #[yaserde(prefix = "tdc", rename = "StartReference")]
    pub start_reference: Option<String>,
}

impl Validate for GetDoorInfoList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct GetDoorInfoListResponse {
    // StartReference to use in next call to get the following items. If
    // absent, no more items to get.
    #[yaserde(prefix = "tdc", rename = "NextStartReference")]
    pub next_start_reference: Option<String>,

    // List of DoorInfo items.
    #[yaserde(prefix = "tdc", rename = "DoorInfo")]
    pub door_info: Vec<DoorInfo>,
}

impl Validate for GetDoorInfoListResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct GetDoorInfo {
    // Tokens of DoorInfo items to get.
    #[yaserde(prefix = "tdc", rename = "Token")]
    pub token: Vec<pt::ReferenceToken>,
}

impl Validate for GetDoorInfo {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct GetDoorInfoResponse {
    // List of DoorInfo items.
    #[yaserde(prefix = "tdc", rename = "DoorInfo")]
    pub door_info: Vec<DoorInfo>,
}

impl Validate for GetDoorInfoResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct GetDoorList {
    // Maximum number of entries to return. If not specified, less than one
    // or higher than what the device supports, the number of items is
    // determined by the
    // device.
    #[yaserde(prefix = "tdc", rename = "Limit")]
    pub limit: Option<i32>,

    // Start returning entries from this start reference. If not specified,
    // entries shall start from the beginning of the dataset.
    #[yaserde(prefix = "tdc", rename = "StartReference")]
    pub start_reference: Option<String>,
}

impl Validate for GetDoorList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct GetDoorListResponse {
    // StartReference to use in next call to get the following items. If
    // absent, no more items to get.
    #[yaserde(prefix = "tdc", rename = "NextStartReference")]
    pub next_start_reference: Option<String>,

    // List of Door items.
    #[yaserde(prefix = "tdc", rename = "Door")]
    pub door: Vec<Door>,
}

impl Validate for GetDoorListResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct GetDoors {
    // Tokens of Door items to get.
    #[yaserde(prefix = "tdc", rename = "Token")]
    pub token: Vec<pt::ReferenceToken>,
}

impl Validate for GetDoors {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct GetDoorsResponse {
    // List of Door items.
    #[yaserde(prefix = "tdc", rename = "Door")]
    pub door: Vec<Door>,
}

impl Validate for GetDoorsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct CreateDoor {
    // Door item to create
    #[yaserde(prefix = "tdc", rename = "Door")]
    pub door: Door,
}

impl Validate for CreateDoor {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct CreateDoorResponse {
    // Token of created Door item
    #[yaserde(prefix = "tdc", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for CreateDoorResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct SetDoor {
    // The Door item to create or modify
    #[yaserde(prefix = "tdc", rename = "Door")]
    pub door: Door,
}

impl Validate for SetDoor {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct SetDoorResponse {}

impl Validate for SetDoorResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct ModifyDoor {
    // The details of the door
    #[yaserde(prefix = "tdc", rename = "Door")]
    pub door: Door,
}

impl Validate for ModifyDoor {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct ModifyDoorResponse {}

impl Validate for ModifyDoorResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct DeleteDoor {
    // The Token of the door to delete.
    #[yaserde(prefix = "tdc", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for DeleteDoor {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct DeleteDoorResponse {}

impl Validate for DeleteDoorResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct GetDoorState {
    // Token of the Door instance to get the state for.
    #[yaserde(prefix = "tdc", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for GetDoorState {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct GetDoorStateResponse {
    // The state of the door.
    #[yaserde(prefix = "tdc", rename = "DoorState")]
    pub door_state: DoorState,
}

impl Validate for GetDoorStateResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct AccessDoor {
    // Token of the Door instance to control.
    #[yaserde(prefix = "tdc", rename = "Token")]
    pub token: pt::ReferenceToken,

    // Optional - Indicates that the configured extended time should be
    // used.
    #[yaserde(prefix = "tdc", rename = "UseExtendedTime")]
    pub use_extended_time: Option<bool>,

    // Optional - overrides ReleaseTime if specified.
    #[yaserde(prefix = "tdc", rename = "AccessTime")]
    pub access_time: Option<xs::Duration>,

    // Optional - overrides OpenTime if specified.
    #[yaserde(prefix = "tdc", rename = "OpenTooLongTime")]
    pub open_too_long_time: Option<xs::Duration>,

    // Optional - overrides PreAlarmTime if specified.
    #[yaserde(prefix = "tdc", rename = "PreAlarmTime")]
    pub pre_alarm_time: Option<xs::Duration>,

    // Future extension.
    #[yaserde(prefix = "tdc", rename = "Extension")]
    pub extension: Option<AccessDoorExtension>,
}

impl Validate for AccessDoor {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct AccessDoorResponse {}

impl Validate for AccessDoorResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct LockDoor {
    // Token of the Door instance to control.
    #[yaserde(prefix = "tdc", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for LockDoor {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct LockDoorResponse {}

impl Validate for LockDoorResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct UnlockDoor {
    // Token of the Door instance to control.
    #[yaserde(prefix = "tdc", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for UnlockDoor {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct UnlockDoorResponse {}

impl Validate for UnlockDoorResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct BlockDoor {
    // Token of the Door instance to control.
    #[yaserde(prefix = "tdc", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for BlockDoor {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct BlockDoorResponse {}

impl Validate for BlockDoorResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct LockDownDoor {
    // Token of the Door instance to control.
    #[yaserde(prefix = "tdc", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for LockDownDoor {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct LockDownDoorResponse {}

impl Validate for LockDownDoorResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct LockDownReleaseDoor {
    // Token of the Door instance to control.
    #[yaserde(prefix = "tdc", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for LockDownReleaseDoor {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct LockDownReleaseDoorResponse {}

impl Validate for LockDownReleaseDoorResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct LockOpenDoor {
    // Token of the Door instance to control.
    #[yaserde(prefix = "tdc", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for LockOpenDoor {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct LockOpenDoorResponse {}

impl Validate for LockOpenDoorResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct LockOpenReleaseDoor {
    // Token of the Door instance to control.
    #[yaserde(prefix = "tdc", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for LockOpenReleaseDoor {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct LockOpenReleaseDoorResponse {}

impl Validate for LockOpenReleaseDoorResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct DoubleLockDoor {
    // Token of the Door instance to control.
    #[yaserde(prefix = "tdc", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for DoubleLockDoor {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tdc",
    namespace = "tdc: http://www.onvif.org/ver10/doorcontrol/wsdl"
)]
pub struct DoubleLockDoorResponse {}

impl Validate for DoubleLockDoorResponse {}

// This operation returns the capabilities of the service.
// An ONVIF compliant device which provides the Door Control service shall
// implement this method.
pub async fn get_service_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetServiceCapabilities,
) -> Result<GetServiceCapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of all DoorInfo items provided by the device.
// An ONVIF compliant device that provides Door Control service shall implement
// this method.
// A call to this method shall return a StartReference when not all data is
// returned and more data is
// available.
// The reference shall be valid for retrieving the next set of data.
// The number of items returned shall not be greater than Limit parameter.
pub async fn get_door_info_list<T: transport::Transport>(
    transport: &T,
    request: &GetDoorInfoList,
) -> Result<GetDoorInfoListResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of DoorInfo items matching the given tokens.
// An ONVIF-compliant device that provides Door Control service shall implement
// this method.
// The device shall ignore tokens it cannot resolve and shall return an empty
// list
// if there are no items matching specified tokens.
// If the number of requested items is greater than MaxLimit, a TooManyItems
// fault shall be returned.
pub async fn get_door_info<T: transport::Transport>(
    transport: &T,
    request: &GetDoorInfo,
) -> Result<GetDoorInfoResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of all Door items provided by the device.
// A call to this method shall return a StartReference when not all data is
// returned and more data is
// available. The reference shall be valid for retrieving the next set of data.
// Please refer to section 4.8.3 in [Access Control Service Specification] for
// more details.
// The number of items returned shall not be greater than the Limit parameter.
pub async fn get_door_list<T: transport::Transport>(
    transport: &T,
    request: &GetDoorList,
) -> Result<GetDoorListResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of Door items matching the given tokens.
// The device shall ignore tokens it cannot resolve and shall return an empty
// list if there are no items
// matching specified tokens. The device shall not return a fault in this case.
// If the number of requested items is greater than MaxLimit, a TooManyItems
// fault shall be returned.
pub async fn get_doors<T: transport::Transport>(
    transport: &T,
    request: &GetDoors,
) -> Result<GetDoorsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation creates the specified door in the device.
// The token field of the Door structure shall be empty and the device shall
// allocate a token for
// the door. The allocated token shall be returned in the response.
// If the client sends any value in the token field, the device shall return
// InvalidArgVal as a generic fault code.
pub async fn create_door<T: transport::Transport>(
    transport: &T,
    request: &CreateDoor,
) -> Result<CreateDoorResponse, transport::Error> {
    transport::request(transport, request).await
}

// This method is used to synchronize a door in a client with the device.
// If a door with the specified token does not exist in the device, the door is
// created.
// If a door with the specified token exists, then the door is modified.
// A call to this method takes a door structure as input parameter. The token
// field of the Door
// structure shall not be empty.
// A device that signals support for the ClientSuppliedTokenSupported capability
// shall
// implement this command.
// If no token was specified in the request, the device shall return InvalidArgs
// as a generic fault code.
pub async fn set_door<T: transport::Transport>(
    transport: &T,
    request: &SetDoor,
) -> Result<SetDoorResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation modifies the specified door.
// The token of the door to modify is specified in the token field of the Door
// structure and shall
// not be empty. All other fields in the structure shall overwrite the fields in
// the specified door.
// If no token was specified in the request, the device shall return InvalidArgs
// as a generic fault code.
pub async fn modify_door<T: transport::Transport>(
    transport: &T,
    request: &ModifyDoor,
) -> Result<ModifyDoorResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation deletes the specified door.
// If it is associated with one or more entities some devices may not be able to
// delete the door,
// and consequently a ReferenceInUse fault shall be generated.
// If no token was specified in the request, the device shall return InvalidArgs
// as a generic fault code.
pub async fn delete_door<T: transport::Transport>(
    transport: &T,
    request: &DeleteDoor,
) -> Result<DeleteDoorResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests the state of a Door specified by the Token.
// A device implementing the Door Control service shall be capable of reporting
// the status of a door using a DoorState structure available from the
// GetDoorState command.
pub async fn get_door_state<T: transport::Transport>(
    transport: &T,
    request: &GetDoorState,
) -> Result<GetDoorStateResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation allows momentarily accessing a Door.
// It invokes the functionality typically used when a card holder presents a
// card to a card reader at the door and is granted access.
// The DoorMode shall change to Accessed state. Please refer to Accessed mode in
// section [DoorMode] for
// more details.
// The Door shall remain accessible for the defined time. When the time span
// elapses, the DoorMode shall change back to its previous state.
// If the request cannot be fulfilled, a Failure fault shall be returned.
// Please refer to section [DoorMode] for details about Door Modes restrictions.
// A device that signals support for Access capability for a particular Door
// instance shall implement this method. A device that signals support for
// AccessTimingOverride capability for a particular Door instance shall also
// provide optional timing parameters (AccessTime, OpenTooLongTime and
// PreAlarmTime) when performing AccessDoor command.
// The device shall take the best effort approach for parameters not supported,
// it must fallback to preconfigured time or limit the time to the closest
// supported time if the specified time is out of range.
pub async fn access_door<T: transport::Transport>(
    transport: &T,
    request: &AccessDoor,
) -> Result<AccessDoorResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation allows locking a Door.
// The DoorMode shall change to Locked state.
// Please refer to Locked mode in section [DoorMode] for more details.
// A device that signals support for Lock capability for a particular Door
// instance shall implement this method.
// If the request cannot be fulfilled, a Failure fault shall be returned.
// Please refer to section [DoorMode] for more details about Door Modes
// restrictions.
pub async fn lock_door<T: transport::Transport>(
    transport: &T,
    request: &LockDoor,
) -> Result<LockDoorResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation allows unlocking a Door.
// The DoorMode shall change to Unlocked state.
// Please refer to Unlocked mode in section [DoorMode] for more details.
// A device that signals support for Unlock capability for a particular Door
// instance shall implement this method.
// If the request cannot be fulfilled, a Failure fault shall be returned.
// Please refer to section [DoorMode] for more details about Door Modes
// restrictions.
pub async fn unlock_door<T: transport::Transport>(
    transport: &T,
    request: &UnlockDoor,
) -> Result<UnlockDoorResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation allows blocking a Door and preventing momentary access
// (AccessDoor command).
// The DoorMode shall change to Blocked state.
// Please refer to Blocked mode in section [DoorMode] for more details.
// A device that signals support for Block capability for a particular Door
// instance shall implement this method.
// If the request cannot be fulfilled, a Failure fault shall be returned.
// Please refer to section [DoorMode] for more details about Door Modes
// restrictions.
pub async fn block_door<T: transport::Transport>(
    transport: &T,
    request: &BlockDoor,
) -> Result<BlockDoorResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation allows locking and preventing other actions until a
// LockDownRelease command is invoked.
// The DoorMode shall change to LockedDown state.
// Please refer to LockedDown mode in section [DoorMode] for more details.
// The device shall ignore other door control commands until a LockDownRelease
// command is performed.
// A device that signals support for LockDown capability for a particular Door
// instance shall implement this method.
// If a device supports DoubleLock capability for a particular Door instance,
// that operation may be engaged as well.
// If the request cannot be fulfilled, a Failure fault shall be returned.
// Please refer to section [DoorMode] for more details about Door Modes
// restrictions.
pub async fn lock_down_door<T: transport::Transport>(
    transport: &T,
    request: &LockDownDoor,
) -> Result<LockDownDoorResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation allows releasing the LockedDown state of a Door.
// The DoorMode shall change back to its previous/next state.
// It is not defined what the previous/next state shall be, but typically -
// Locked.
// This method shall only succeed if the current DoorMode is LockedDown.
pub async fn lock_down_release_door<T: transport::Transport>(
    transport: &T,
    request: &LockDownReleaseDoor,
) -> Result<LockDownReleaseDoorResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation allows unlocking a Door and preventing other actions until
// LockOpenRelease method is
// invoked.
// The DoorMode shall change to LockedOpen state.
// Please refer to LockedOpen mode in section [DoorMode] for more details.
// The device shall ignore other door control commands until a LockOpenRelease
// command is performed.
// A device that signals support for LockOpen capability for a particular Door
// instance shall implement
// this method.
// If the request cannot be fulfilled, a Failure fault shall be returned.
// Please refer to section [DoorMode] for more details about Door Modes
// restrictions.
pub async fn lock_open_door<T: transport::Transport>(
    transport: &T,
    request: &LockOpenDoor,
) -> Result<LockOpenDoorResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation allows releasing the LockedOpen state of a Door.
// The DoorMode shall change state from the LockedOpen state back to its
// previous/next state.
// It is not defined what the previous/next state shall be, but typically -
// Unlocked.
// A device that signals support for LockOpen capability for a particular Door
// instance shall support
// this command.
// This method shall only succeed if the current DoorMode is LockedOpen.
pub async fn lock_open_release_door<T: transport::Transport>(
    transport: &T,
    request: &LockOpenReleaseDoor,
) -> Result<LockOpenReleaseDoorResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation is used for securely locking a Door.
// A call to this method shall change DoorMode state to DoubleLocked.
// Please refer to DoubleLocked mode in section [DoorMode] for more details.
// A device that signals support for DoubleLock capability for a particular
// Door instance shall implement this method. Otherwise this method can be
// performed as a standard Lock operation (see [LockDoor command]).
// If the door has an extra lock that shall be locked as well.
// If the request cannot be fulfilled, a Failure fault shall be returned.
pub async fn double_lock_door<T: transport::Transport>(
    transport: &T,
    request: &DoubleLockDoor,
) -> Result<DoubleLockDoorResponse, transport::Error> {
    transport::request(transport, request).await
}
