#![allow(clippy::derive_partial_eq_without_eq)]

use types as pt;
use validate::Validate;
use xsd_types::types as xs;
use yaserde_derive::{YaDeserialize, YaSerialize};

// The service capabilities reflect optional functionality of a service.
// The information is static and does not change during device operation.
// The following capabilities are available:
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct ServiceCapabilities {
    // The maximum number of entries returned by a single Get<Entity>List or
    // Get<Entity> request. The device shall never return more than this number
    // of entities in a single response.
    #[yaserde(attribute, rename = "MaxLimit")]
    pub max_limit: pt::PositiveInteger,

    // Indicates the maximum number of schedules the device supports.
    // The device shall support at least one schedule.
    #[yaserde(attribute, rename = "MaxSchedules")]
    pub max_schedules: pt::PositiveInteger,

    // Indicates the maximum number of time periods per day the device supports
    // in a schedule including special days schedule. The device shall support
    // at least one time period per day.
    #[yaserde(attribute, rename = "MaxTimePeriodsPerDay")]
    pub max_time_periods_per_day: pt::PositiveInteger,

    // Indicates the maximum number of special day group entities the device
    // supports.
    // The device shall support at least one ‘SpecialDayGroup’ entity.
    #[yaserde(attribute, rename = "MaxSpecialDayGroups")]
    pub max_special_day_groups: pt::PositiveInteger,

    // Indicates the maximum number of days per ‘SpecialDayGroup’ entity the
    // device
    // supports. The device shall support at least one day per
    // ‘SpecialDayGroup’ entity.
    #[yaserde(attribute, rename = "MaxDaysInSpecialDayGroup")]
    pub max_days_in_special_day_group: pt::PositiveInteger,

    // Indicates the maximum number of ‘SpecialDaysSchedule’ entities
    // referred by a
    // schedule that the device supports.
    #[yaserde(attribute, rename = "MaxSpecialDaysSchedules")]
    pub max_special_days_schedules: pt::PositiveInteger,

    // For schedules:
    // If this capability is supported, then all iCalendar recurrence types
    // shall
    // be supported by the device. The device shall also support the start and
    // end dates (or
    // iCalendar occurrence count) in recurring events (see iCalendar examples
    // in section 3).
    // If this capability is not supported, then only the weekly iCalendar
    // recurrence
    // type shall be supported. Non-recurring events and other recurring types
    // are
    // not supported. The device shall only accept a start date with the year
    // ‘1970’
    // (the month and day is needed to reflect the week day of the recurrence)
    // and will not accept an occurrence count (or iCalendar until date) in
    // recurring events.
    // For special days (only applicable if SpecialDaysSupported is set to
    // true):
    // If this capability is supported, then all iCalendar recurrence types
    // shall
    // be supported by the device. The device shall also support the start and
    // end dates (or occurrence count) in recurring events.
    // If this capability is not supported, then only non-recurring special days
    // are supported.
    #[yaserde(attribute, rename = "ExtendedRecurrenceSupported")]
    pub extended_recurrence_supported: bool,

    // If this capability is supported, then the device shall support special
    // days.
    #[yaserde(attribute, rename = "SpecialDaysSupported")]
    pub special_days_supported: bool,

    // If this capability is set to true, the device shall implement the
    // GetScheduleState command, and shall notify subscribing clients whenever
    // schedules become active or inactive.
    #[yaserde(attribute, rename = "StateReportingSupported")]
    pub state_reporting_supported: bool,

    // Indicates that the client is allowed to supply the token when creating
    // schedules and special day groups.
    // To enable the use of the commands SetSchedule and SetSpecialDayGroup, the
    // value must be set to true.
    #[yaserde(attribute, rename = "ClientSuppliedTokenSupported")]
    pub client_supplied_token_supported: Option<bool>,
}

impl Validate for ServiceCapabilities {}

// pub type Capabilities = ServiceCapabilities;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct ScheduleInfo {
    // A user readable name. It shall be up to 64 characters.
    #[yaserde(prefix = "tsc", rename = "Name")]
    pub name: pt::Name,

    // User readable description for the schedule. It shall be up to 1024
    // characters.
    #[yaserde(prefix = "tsc", rename = "Description")]
    pub description: Option<pt::Description>,

    pub base: pt::DataEntity,
}

impl Validate for ScheduleInfo {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct Schedule {
    // An iCalendar structure that defines a number of events. Events
    // can be recurring or non-recurring. The events can, for instance,
    // be used to control when a camera should record or when a facility
    // is accessible. Some devices might not be able to fully support
    // all the features of iCalendar. Setting the service capability
    // ExtendedRecurrenceSupported to false will enable more devices
    // to be ONVIF compliant. Is of type string (but contains an iCalendar
    // structure).
    #[yaserde(prefix = "tsc", rename = "Standard")]
    pub standard: String,

    // For devices that are not able to support all the features of iCalendar,
    // supporting special days is essential. Each SpecialDaysSchedule
    // instance defines an alternate set of time periods that overrides
    // the regular schedule for a specified list of special days.
    // Is of type SpecialDaysSchedule.
    #[yaserde(prefix = "tsc", rename = "SpecialDays")]
    pub special_days: Vec<SpecialDaysSchedule>,

    #[yaserde(prefix = "tsc", rename = "Extension")]
    pub extension: Option<ScheduleExtension>,

    pub base: ScheduleInfo,
}

impl Validate for Schedule {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct ScheduleExtension {}

impl Validate for ScheduleExtension {}

// A override schedule that defines alternate time periods for a group of
// special days.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct SpecialDaysSchedule {
    // Indicates the list of special days in a schedule.
    #[yaserde(prefix = "tsc", rename = "GroupToken")]
    pub group_token: pt::ReferenceToken,

    // Indicates the alternate time periods for the list of special days
    // (overrides the regular schedule). For example, the regular schedule
    // indicates
    // that it is active from 8AM to 5PM on Mondays. However, this particular
    // Monday is a special day, and the alternate time periods state that the
    // schedule is active from 9 AM to 11 AM and 1 PM to 4 PM.
    // If no time periods are defined, then no access is allowed.
    // Is of type TimePeriod.
    #[yaserde(prefix = "tsc", rename = "TimeRange")]
    pub time_range: Vec<TimePeriod>,

    #[yaserde(prefix = "tsc", rename = "Extension")]
    pub extension: Option<SpecialDaysScheduleExtension>,
}

impl Validate for SpecialDaysSchedule {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct SpecialDaysScheduleExtension {}

impl Validate for SpecialDaysScheduleExtension {}

// The ScheduleState contains state information for a schedule.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct ScheduleState {
    // Indicates that the current time is within the boundaries of the schedule
    // or its special days schedules’ time periods. For example, if this
    // schedule is being used for triggering automatic recording on a video
    // source,
    // the Active flag will be true when the schedule-based recording is
    // supposed to record.
    #[yaserde(prefix = "tsc", rename = "Active")]
    pub active: bool,

    // Indicates that the current time is within the boundaries of its special
    // days schedules’ time periods. For example, if this schedule is being
    // used
    // for recording at a lower frame rate on a video source during special
    // days,
    // the SpecialDay flag will be true. If special days are not supported by
    // the device,
    // this field may be omitted and interpreted as false by the client.
    #[yaserde(prefix = "tsc", rename = "SpecialDay")]
    pub special_day: Option<bool>,

    #[yaserde(prefix = "tsc", rename = "Extension")]
    pub extension: Option<ScheduleStateExtension>,
}

impl Validate for ScheduleState {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct ScheduleStateExtension {}

impl Validate for ScheduleStateExtension {}

// A time period defines a start and end time. For full day access, the
// start time ="00:00:00" with no defined end time. For a time period with no
// end time, the schedule runs until midnight. The end time must always be
// greater
// than the start time, otherwise an InvalidArgVal error messages is generated
// by the device.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct TimePeriod {
    // Indicates the start time.
    #[yaserde(prefix = "tsc", rename = "From")]
    pub from: xs::Time,

    // Indicates the end time. Is optional, if omitted, the period ends at
    // midnight.
    // The end time is exclusive, meaning that that exact moment in time is not
    // part of the period. To determine if a moment in time (t) is part of a
    // time period,
    // the formula StartTime ≤ t < EndTime is used.
    #[yaserde(prefix = "tsc", rename = "Until")]
    pub until: Option<xs::Time>,

    #[yaserde(prefix = "tsc", rename = "Extension")]
    pub extension: Option<TimePeriodExtension>,
}

impl Validate for TimePeriod {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct TimePeriodExtension {}

impl Validate for TimePeriodExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct SpecialDayGroupInfo {
    // User readable name. It shall be up to 64 characters.
    #[yaserde(prefix = "tsc", rename = "Name")]
    pub name: pt::Name,

    // User readable description for the special days. It shall be up to 1024
    // characters.
    #[yaserde(prefix = "tsc", rename = "Description")]
    pub description: Option<pt::Description>,

    pub base: pt::DataEntity,
}

impl Validate for SpecialDayGroupInfo {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct SpecialDayGroup {
    // An iCalendar structure that contains a group of special days.
    // Is of type string (containing an iCalendar structure).
    #[yaserde(prefix = "tsc", rename = "Days")]
    pub days: Option<String>,

    #[yaserde(prefix = "tsc", rename = "Extension")]
    pub extension: Option<SpecialDayGroupExtension>,

    pub base: SpecialDayGroupInfo,
}

impl Validate for SpecialDayGroup {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct SpecialDayGroupExtension {}

impl Validate for SpecialDayGroupExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct GetServiceCapabilitiesResponse {
    // The capability response message contains the requested schedule service
    // capabilities using a hierarchical XML capability structure.
    #[yaserde(prefix = "tsc", rename = "Capabilities")]
    pub capabilities: ServiceCapabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct GetScheduleState {
    // Token of schedule instance to get ScheduleState.
    #[yaserde(prefix = "tsc", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for GetScheduleState {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct GetScheduleStateResponse {
    // ScheduleState item.
    #[yaserde(prefix = "tsc", rename = "ScheduleState")]
    pub schedule_state: ScheduleState,
}

impl Validate for GetScheduleStateResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct GetScheduleInfo {
    // Tokens of ScheduleInfo items to get.
    #[yaserde(prefix = "tsc", rename = "Token")]
    pub token: Vec<pt::ReferenceToken>,
}

impl Validate for GetScheduleInfo {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct GetScheduleInfoResponse {
    // List of ScheduleInfo items.
    #[yaserde(prefix = "tsc", rename = "ScheduleInfo")]
    pub schedule_info: Vec<ScheduleInfo>,
}

impl Validate for GetScheduleInfoResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct GetScheduleInfoList {
    // Maximum number of entries to return. If not specified, less than one
    // or higher than what the device supports, the number of items is
    // determined by the device.
    #[yaserde(prefix = "tsc", rename = "Limit")]
    pub limit: Option<i32>,

    // Start returning entries from this start reference.
    // If not specified, entries shall start from the beginning of the dataset.
    #[yaserde(prefix = "tsc", rename = "StartReference")]
    pub start_reference: Option<String>,
}

impl Validate for GetScheduleInfoList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct GetScheduleInfoListResponse {
    // StartReference to use in next call to get the following items.
    // If absent, no more items to get.
    #[yaserde(prefix = "tsc", rename = "NextStartReference")]
    pub next_start_reference: Option<String>,

    // List of ScheduleInfo items.
    #[yaserde(prefix = "tsc", rename = "ScheduleInfo")]
    pub schedule_info: Vec<ScheduleInfo>,
}

impl Validate for GetScheduleInfoListResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct GetSchedules {
    // Tokens of Schedule items to get
    #[yaserde(prefix = "tsc", rename = "Token")]
    pub token: Vec<pt::ReferenceToken>,
}

impl Validate for GetSchedules {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct GetSchedulesResponse {
    // List of schedule items.
    #[yaserde(prefix = "tsc", rename = "Schedule")]
    pub schedule: Vec<Schedule>,
}

impl Validate for GetSchedulesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct GetScheduleList {
    // Maximum number of entries to return.
    // If not specified, less than one or higher than what the device supports,
    // the number of items is determined by the device.
    #[yaserde(prefix = "tsc", rename = "Limit")]
    pub limit: Option<i32>,

    // Start returning entries from this start reference.
    // If not specified, entries shall start from the beginning of the dataset.
    #[yaserde(prefix = "tsc", rename = "StartReference")]
    pub start_reference: Option<String>,
}

impl Validate for GetScheduleList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct GetScheduleListResponse {
    // StartReference to use in next call to get the following items.
    // If absent, no more items to get.
    #[yaserde(prefix = "tsc", rename = "NextStartReference")]
    pub next_start_reference: Option<String>,

    // List of Schedule items.
    #[yaserde(prefix = "tsc", rename = "Schedule")]
    pub schedule: Vec<Schedule>,
}

impl Validate for GetScheduleListResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct CreateSchedule {
    // The Schedule to create
    #[yaserde(prefix = "tsc", rename = "Schedule")]
    pub schedule: Schedule,
}

impl Validate for CreateSchedule {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct CreateScheduleResponse {
    // The token of created Schedule
    #[yaserde(prefix = "tsc", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for CreateScheduleResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct SetSchedule {
    // The Schedule to modify/create
    #[yaserde(prefix = "tsc", rename = "Schedule")]
    pub schedule: Schedule,
}

impl Validate for SetSchedule {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct SetScheduleResponse {}

impl Validate for SetScheduleResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct ModifySchedule {
    // The Schedule to modify/update
    #[yaserde(prefix = "tsc", rename = "Schedule")]
    pub schedule: Schedule,
}

impl Validate for ModifySchedule {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct ModifyScheduleResponse {}

impl Validate for ModifyScheduleResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct DeleteSchedule {
    // The token of the schedule to delete.
    #[yaserde(prefix = "tsc", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for DeleteSchedule {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct DeleteScheduleResponse {}

impl Validate for DeleteScheduleResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct GetSpecialDayGroupInfo {
    // Tokens of SpecialDayGroupInfo items to get.
    #[yaserde(prefix = "tsc", rename = "Token")]
    pub token: Vec<pt::ReferenceToken>,
}

impl Validate for GetSpecialDayGroupInfo {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct GetSpecialDayGroupInfoResponse {
    // List of SpecialDayGroupInfo items.
    #[yaserde(prefix = "tsc", rename = "SpecialDayGroupInfo")]
    pub special_day_group_info: Vec<SpecialDayGroupInfo>,
}

impl Validate for GetSpecialDayGroupInfoResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct GetSpecialDayGroupInfoList {
    // Maximum number of entries to return. If not specified, less than
    // one or higher than what the device supports, the number
    // of items is determined by the device.
    #[yaserde(prefix = "tsc", rename = "Limit")]
    pub limit: Option<i32>,

    // Start returning entries from this start reference.
    // If not specified, entries shall start from the beginning of the dataset.
    #[yaserde(prefix = "tsc", rename = "StartReference")]
    pub start_reference: Option<String>,
}

impl Validate for GetSpecialDayGroupInfoList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct GetSpecialDayGroupInfoListResponse {
    // StartReference to use in next call to get the following items.
    // If absent, no more items to get.
    #[yaserde(prefix = "tsc", rename = "NextStartReference")]
    pub next_start_reference: Option<String>,

    // List of SpecialDayGroupInfo items.
    #[yaserde(prefix = "tsc", rename = "SpecialDayGroupInfo")]
    pub special_day_group_info: Vec<SpecialDayGroupInfo>,
}

impl Validate for GetSpecialDayGroupInfoListResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct GetSpecialDayGroups {
    // Tokens of the SpecialDayGroup items to get
    #[yaserde(prefix = "tsc", rename = "Token")]
    pub token: Vec<pt::ReferenceToken>,
}

impl Validate for GetSpecialDayGroups {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct GetSpecialDayGroupsResponse {
    // List of SpecialDayGroup items.
    #[yaserde(prefix = "tsc", rename = "SpecialDayGroup")]
    pub special_day_group: Vec<SpecialDayGroup>,
}

impl Validate for GetSpecialDayGroupsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct GetSpecialDayGroupList {
    // Maximum number of entries to return. If not specified, less than
    // one or higher than what the device supports, the number of
    // items is determined by the device.
    #[yaserde(prefix = "tsc", rename = "Limit")]
    pub limit: Option<i32>,

    // Start returning entries from this start reference.
    // If not specified, entries shall start from the beginning of the dataset.
    #[yaserde(prefix = "tsc", rename = "StartReference")]
    pub start_reference: Option<String>,
}

impl Validate for GetSpecialDayGroupList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct GetSpecialDayGroupListResponse {
    // StartReference to use in next call to get the following items. If
    // absent, no more items to get.
    #[yaserde(prefix = "tsc", rename = "NextStartReference")]
    pub next_start_reference: Option<String>,

    // List of SpecialDayGroup items.
    #[yaserde(prefix = "tsc", rename = "SpecialDayGroup")]
    pub special_day_group: Vec<SpecialDayGroup>,
}

impl Validate for GetSpecialDayGroupListResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct CreateSpecialDayGroup {
    // The special day group to create.
    #[yaserde(prefix = "tsc", rename = "SpecialDayGroup")]
    pub special_day_group: SpecialDayGroup,
}

impl Validate for CreateSpecialDayGroup {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct CreateSpecialDayGroupResponse {
    // The token of created special day group.
    #[yaserde(prefix = "tsc", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for CreateSpecialDayGroupResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct SetSpecialDayGroup {
    // The SpecialDayGroup to modify/create
    #[yaserde(prefix = "tsc", rename = "SpecialDayGroup")]
    pub special_day_group: SpecialDayGroup,
}

impl Validate for SetSpecialDayGroup {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct SetSpecialDayGroupResponse {}

impl Validate for SetSpecialDayGroupResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct ModifySpecialDayGroup {
    // The special day group to modify/update.
    #[yaserde(prefix = "tsc", rename = "SpecialDayGroup")]
    pub special_day_group: SpecialDayGroup,
}

impl Validate for ModifySpecialDayGroup {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct ModifySpecialDayGroupResponse {}

impl Validate for ModifySpecialDayGroupResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct DeleteSpecialDayGroup {
    // The token of the special day group item to delete.
    #[yaserde(prefix = "tsc", rename = "Token")]
    pub token: pt::ReferenceToken,
}

impl Validate for DeleteSpecialDayGroup {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tsc",
    namespace = "tsc: http://www.onvif.org/ver10/schedule/wsdl"
)]
pub struct DeleteSpecialDayGroupResponse {}

impl Validate for DeleteSpecialDayGroupResponse {}

// This operation returns the capabilities of the schedule service.
pub async fn get_service_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetServiceCapabilities,
) -> Result<GetServiceCapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests the ScheduleState for the schedule instance specified
// by the given token.
pub async fn get_schedule_state<T: transport::Transport>(
    transport: &T,
    request: &GetScheduleState,
) -> Result<GetScheduleStateResponse, transport::Error> {
    transport::request(transport, request).await
}

// This method returns a list of schedule info items, specified in the request.
// Only found schedules shall be returned, i.e., the returned numbers of
// elements can
// differ from the requested element.
// The device shall ignore tokens it cannot resolve and shall return an empty
// list if
// there are no items matching the specified tokens.
// If the number of requested items is greater than MaxLimit, a TooManyItems
// fault shall be returned.
pub async fn get_schedule_info<T: transport::Transport>(
    transport: &T,
    request: &GetScheduleInfo,
) -> Result<GetScheduleInfoResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of all of ScheduleInfo items provided by the
// device.
// A call to this method shall return a StartReference when not all data is
// returned
// and more data is available. The reference shall be valid for retrieving the
// next set of data.
// Please refer Access Control Service Specification for more details.
// The number of items returned shall not be greater the Limit parameter.
pub async fn get_schedule_info_list<T: transport::Transport>(
    transport: &T,
    request: &GetScheduleInfoList,
) -> Result<GetScheduleInfoListResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns the specified schedule item matching the given tokens.
// The device shall ignore tokens it cannot resolve and shall return an empty
// list
// if there are no items matching the specified tokens.
// If the number of requested items is greater than MaxLimit, a TooManyItems
// fault shall be returned
pub async fn get_schedules<T: transport::Transport>(
    transport: &T,
    request: &GetSchedules,
) -> Result<GetSchedulesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of all of Schedule items provided by the
// device.
// A call to this method shall return a StartReference when not all data is
// returned
// and more data is available. The reference shall be valid for retrieving the
// next set of data.
// Please refer Access Control Service Specification for more details.
// The number of items returned shall not be greater the Limit parameter.
pub async fn get_schedule_list<T: transport::Transport>(
    transport: &T,
    request: &GetScheduleList,
) -> Result<GetScheduleListResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation creates the specified schedule. The token field of the
// schedule structure
// shall be empty, the device shall allocate a token for the schedule. The
// allocated token
// shall be returned in the response. If the client sends any value in the token
// field,
// the device shall return InvalidArgVal as generic fault code.
pub async fn create_schedule<T: transport::Transport>(
    transport: &T,
    request: &CreateSchedule,
) -> Result<CreateScheduleResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation modifies or creates the specified schedule.
pub async fn set_schedule<T: transport::Transport>(
    transport: &T,
    request: &SetSchedule,
) -> Result<SetScheduleResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation modifies or updates the specified schedule.
pub async fn modify_schedule<T: transport::Transport>(
    transport: &T,
    request: &ModifySchedule,
) -> Result<ModifyScheduleResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation will delete the specified schedule.
// If it is associated with one or more entities some devices may not be able to
// delete the schedule,
// and consequently a ReferenceInUse fault shall be generated.
pub async fn delete_schedule<T: transport::Transport>(
    transport: &T,
    request: &DeleteSchedule,
) -> Result<DeleteScheduleResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of SpecialDayGroupInfo items matching the
// given tokens.
// The device shall ignore tokens it cannot resolve and shall return an empty
// list if
// there are no items matching specified tokens. The device shall not return a
// fault in this case.
// If the number of requested items is greater than MaxLimit, a TooManyItems
// fault shall be returned.
pub async fn get_special_day_group_info<T: transport::Transport>(
    transport: &T,
    request: &GetSpecialDayGroupInfo,
) -> Result<GetSpecialDayGroupInfoResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of all of SpecialDayGroupInfo items provided
// by the device.
// A call to this method shall return a StartReference when not all data is
// returned and
// more data is available. The reference shall be valid for retrieving the next
// set of data.
// The number of items returned shall not be greater than Limit parameter.
pub async fn get_special_day_group_info_list<T: transport::Transport>(
    transport: &T,
    request: &GetSpecialDayGroupInfoList,
) -> Result<GetSpecialDayGroupInfoListResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns the specified special day group item matching the
// given token.
pub async fn get_special_day_groups<T: transport::Transport>(
    transport: &T,
    request: &GetSpecialDayGroups,
) -> Result<GetSpecialDayGroupsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation requests a list of all of SpecialDayGroupList items provided
// by the device.
// A call to this method shall return a StartReference when not all data is
// returned and
// more data is available. The reference shall be valid for retrieving the next
// set of data.
// Please refer Access Control Service Specification for more details.
// The number of items returned shall not be greater the Limit parameter.
pub async fn get_special_day_group_list<T: transport::Transport>(
    transport: &T,
    request: &GetSpecialDayGroupList,
) -> Result<GetSpecialDayGroupListResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation creates the specified special day group. The token field of
// the
// SpecialDayGroup structure shall be empty, the device shall allocate a token
// for the
// special day group. The allocated token shall be returned in the response.
// If there is any value in the token field, the device shall return
// InvalidArgVal as generic fault code.
pub async fn create_special_day_group<T: transport::Transport>(
    transport: &T,
    request: &CreateSpecialDayGroup,
) -> Result<CreateSpecialDayGroupResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation modifies or creates the specified special day group.
pub async fn set_special_day_group<T: transport::Transport>(
    transport: &T,
    request: &SetSpecialDayGroup,
) -> Result<SetSpecialDayGroupResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation updates the specified special day group.
pub async fn modify_special_day_group<T: transport::Transport>(
    transport: &T,
    request: &ModifySpecialDayGroup,
) -> Result<ModifySpecialDayGroupResponse, transport::Error> {
    transport::request(transport, request).await
}

// This method deletes the specified special day group.
// If it is associated with one or more schedules some devices may not be able
// to delete
// the special day group, and consequently a ReferenceInUse fault must be
// generated.
pub async fn delete_special_day_group<T: transport::Transport>(
    transport: &T,
    request: &DeleteSpecialDayGroup,
) -> Result<DeleteSpecialDayGroupResponse, transport::Error> {
    transport::request(transport, request).await
}
