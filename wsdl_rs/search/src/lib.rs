#![allow(clippy::derive_partial_eq_without_eq)]

use onvif as tt;
use validate::Validate;
use xsd_types::types as xs;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct GetServiceCapabilitiesResponse {
    // The capabilities for the search service is returned in the Capabilities
    // element.
    #[yaserde(prefix = "tse", rename = "Capabilities")]
    pub capabilities: Capabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct Capabilities {
    #[yaserde(attribute, rename = "MetadataSearch")]
    pub metadata_search: Option<bool>,

    // Indicates support for general virtual property events in the FindEvents
    // method.
    #[yaserde(attribute, rename = "GeneralStartEvents")]
    pub general_start_events: Option<bool>,
}

impl Validate for Capabilities {}

// pub type Capabilities = Capabilities;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct GetRecordingSummary {}

impl Validate for GetRecordingSummary {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct GetRecordingSummaryResponse {
    #[yaserde(prefix = "tse", rename = "Summary")]
    pub summary: tt::RecordingSummary,
}

impl Validate for GetRecordingSummaryResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct GetRecordingInformation {
    #[yaserde(prefix = "tse", rename = "RecordingToken")]
    pub recording_token: tt::RecordingReference,
}

impl Validate for GetRecordingInformation {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct GetRecordingInformationResponse {
    #[yaserde(prefix = "tse", rename = "RecordingInformation")]
    pub recording_information: tt::RecordingInformation,
}

impl Validate for GetRecordingInformationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct GetMediaAttributes {
    #[yaserde(prefix = "tse", rename = "RecordingTokens")]
    pub recording_tokens: Vec<tt::RecordingReference>,

    #[yaserde(prefix = "tse", rename = "Time")]
    pub time: xs::DateTime,
}

impl Validate for GetMediaAttributes {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct GetMediaAttributesResponse {
    #[yaserde(prefix = "tse", rename = "MediaAttributes")]
    pub media_attributes: Vec<tt::MediaAttributes>,
}

impl Validate for GetMediaAttributesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct FindRecordings {
    // Scope defines the dataset to consider for this search.
    #[yaserde(prefix = "tse", rename = "Scope")]
    pub scope: tt::SearchScope,

    // The search will be completed after this many matches. If not specified,
    // the search will continue until reaching the endpoint or until the session
    // expires.
    #[yaserde(prefix = "tse", rename = "MaxMatches")]
    pub max_matches: Option<i32>,

    // The time the search session will be kept alive after responding to this
    // and subsequent requests. A device shall support at least values up to ten
    // seconds.
    #[yaserde(prefix = "tse", rename = "KeepAliveTime")]
    pub keep_alive_time: xs::Duration,
}

impl Validate for FindRecordings {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct FindRecordingsResponse {
    #[yaserde(prefix = "tse", rename = "SearchToken")]
    pub search_token: tt::JobToken,
}

impl Validate for FindRecordingsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct GetRecordingSearchResults {
    // The search session to get results from.
    #[yaserde(prefix = "tse", rename = "SearchToken")]
    pub search_token: tt::JobToken,

    // The minimum number of results to return in one response.
    #[yaserde(prefix = "tse", rename = "MinResults")]
    pub min_results: Option<i32>,

    // The maximum number of results to return in one response.
    #[yaserde(prefix = "tse", rename = "MaxResults")]
    pub max_results: Option<i32>,

    // The maximum time before responding to the request, even if the MinResults
    // parameter is not fulfilled.
    #[yaserde(prefix = "tse", rename = "WaitTime")]
    pub wait_time: Option<xs::Duration>,
}

impl Validate for GetRecordingSearchResults {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct GetRecordingSearchResultsResponse {
    #[yaserde(prefix = "tse", rename = "ResultList")]
    pub result_list: tt::FindRecordingResultList,
}

impl Validate for GetRecordingSearchResultsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct FindEvents {
    // The point of time where the search will start.
    #[yaserde(prefix = "tse", rename = "StartPoint")]
    pub start_point: xs::DateTime,

    // The point of time where the search will stop. This can be a time before
    // the StartPoint, in which case the search is performed backwards in time.
    #[yaserde(prefix = "tse", rename = "EndPoint")]
    pub end_point: Option<xs::DateTime>,

    #[yaserde(prefix = "tse", rename = "Scope")]
    pub scope: tt::SearchScope,

    #[yaserde(prefix = "tse", rename = "SearchFilter")]
    pub search_filter: tt::EventFilter,

    // Setting IncludeStartState to true means that the server should return
    // virtual events representing the start state for any recording included in
    // the scope. Start state events are limited to the topics defined in the
    // SearchFilter that have the IsProperty flag set to true.
    #[yaserde(prefix = "tse", rename = "IncludeStartState")]
    pub include_start_state: bool,

    // The search will be completed after this many matches. If not specified,
    // the search will continue until reaching the endpoint or until the session
    // expires.
    #[yaserde(prefix = "tse", rename = "MaxMatches")]
    pub max_matches: Option<i32>,

    // The time the search session will be kept alive after responding to this
    // and subsequent requests. A device shall support at least values up to ten
    // seconds.
    #[yaserde(prefix = "tse", rename = "KeepAliveTime")]
    pub keep_alive_time: xs::Duration,
}

impl Validate for FindEvents {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct FindEventsResponse {
    // A unique reference to the search session created by this request.
    #[yaserde(prefix = "tse", rename = "SearchToken")]
    pub search_token: tt::JobToken,
}

impl Validate for FindEventsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct GetEventSearchResults {
    // The search session to get results from.
    #[yaserde(prefix = "tse", rename = "SearchToken")]
    pub search_token: tt::JobToken,

    // The minimum number of results to return in one response.
    #[yaserde(prefix = "tse", rename = "MinResults")]
    pub min_results: Option<i32>,

    // The maximum number of results to return in one response.
    #[yaserde(prefix = "tse", rename = "MaxResults")]
    pub max_results: Option<i32>,

    // The maximum time before responding to the request, even if the MinResults
    // parameter is not fulfilled.
    #[yaserde(prefix = "tse", rename = "WaitTime")]
    pub wait_time: Option<xs::Duration>,
}

impl Validate for GetEventSearchResults {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct GetEventSearchResultsResponse {
    #[yaserde(prefix = "tse", rename = "ResultList")]
    pub result_list: tt::FindEventResultList,
}

impl Validate for GetEventSearchResultsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct FindPTZPosition {
    // The point of time where the search will start.
    #[yaserde(prefix = "tse", rename = "StartPoint")]
    pub start_point: xs::DateTime,

    // The point of time where the search will stop. This can be a time before
    // the StartPoint, in which case the search is performed backwards in time.
    #[yaserde(prefix = "tse", rename = "EndPoint")]
    pub end_point: Option<xs::DateTime>,

    #[yaserde(prefix = "tse", rename = "Scope")]
    pub scope: tt::SearchScope,

    #[yaserde(prefix = "tse", rename = "SearchFilter")]
    pub search_filter: tt::PtzpositionFilter,

    // The search will be completed after this many matches. If not specified,
    // the search will continue until reaching the endpoint or until the session
    // expires.
    #[yaserde(prefix = "tse", rename = "MaxMatches")]
    pub max_matches: Option<i32>,

    // The time the search session will be kept alive after responding to this
    // and subsequent requests. A device shall support at least values up to ten
    // seconds.
    #[yaserde(prefix = "tse", rename = "KeepAliveTime")]
    pub keep_alive_time: xs::Duration,
}

impl Validate for FindPTZPosition {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct FindPTZPositionResponse {
    // A unique reference to the search session created by this request.
    #[yaserde(prefix = "tse", rename = "SearchToken")]
    pub search_token: tt::JobToken,
}

impl Validate for FindPTZPositionResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct GetPTZPositionSearchResults {
    // The search session to get results from.
    #[yaserde(prefix = "tse", rename = "SearchToken")]
    pub search_token: tt::JobToken,

    // The minimum number of results to return in one response.
    #[yaserde(prefix = "tse", rename = "MinResults")]
    pub min_results: Option<i32>,

    // The maximum number of results to return in one response.
    #[yaserde(prefix = "tse", rename = "MaxResults")]
    pub max_results: Option<i32>,

    // The maximum time before responding to the request, even if the MinResults
    // parameter is not fulfilled.
    #[yaserde(prefix = "tse", rename = "WaitTime")]
    pub wait_time: Option<xs::Duration>,
}

impl Validate for GetPTZPositionSearchResults {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct GetPTZPositionSearchResultsResponse {
    #[yaserde(prefix = "tse", rename = "ResultList")]
    pub result_list: tt::FindPTZPositionResultList,
}

impl Validate for GetPTZPositionSearchResultsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct FindMetadata {
    // The point of time where the search will start.
    #[yaserde(prefix = "tse", rename = "StartPoint")]
    pub start_point: xs::DateTime,

    // The point of time where the search will stop. This can be a time before
    // the StartPoint, in which case the search is performed backwards in time.
    #[yaserde(prefix = "tse", rename = "EndPoint")]
    pub end_point: Option<xs::DateTime>,

    #[yaserde(prefix = "tse", rename = "Scope")]
    pub scope: tt::SearchScope,

    #[yaserde(prefix = "tse", rename = "MetadataFilter")]
    pub metadata_filter: tt::MetadataFilter,

    // The search will be completed after this many matches. If not specified,
    // the search will continue until reaching the endpoint or until the session
    // expires.
    #[yaserde(prefix = "tse", rename = "MaxMatches")]
    pub max_matches: Option<i32>,

    // The time the search session will be kept alive after responding to this
    // and subsequent requests. A device shall support at least values up to ten
    // seconds.
    #[yaserde(prefix = "tse", rename = "KeepAliveTime")]
    pub keep_alive_time: xs::Duration,
}

impl Validate for FindMetadata {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct FindMetadataResponse {
    // A unique reference to the search session created by this request.
    #[yaserde(prefix = "tse", rename = "SearchToken")]
    pub search_token: tt::JobToken,
}

impl Validate for FindMetadataResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct GetMetadataSearchResults {
    // The search session to get results from.
    #[yaserde(prefix = "tse", rename = "SearchToken")]
    pub search_token: tt::JobToken,

    // The minimum number of results to return in one response.
    #[yaserde(prefix = "tse", rename = "MinResults")]
    pub min_results: Option<i32>,

    // The maximum number of results to return in one response.
    #[yaserde(prefix = "tse", rename = "MaxResults")]
    pub max_results: Option<i32>,

    // The maximum time before responding to the request, even if the MinResults
    // parameter is not fulfilled.
    #[yaserde(prefix = "tse", rename = "WaitTime")]
    pub wait_time: Option<xs::Duration>,
}

impl Validate for GetMetadataSearchResults {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct GetMetadataSearchResultsResponse {
    #[yaserde(prefix = "tse", rename = "ResultList")]
    pub result_list: tt::FindMetadataResultList,
}

impl Validate for GetMetadataSearchResultsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct GetSearchState {
    // The search session to get the state from.
    #[yaserde(prefix = "tse", rename = "SearchToken")]
    pub search_token: tt::JobToken,
}

impl Validate for GetSearchState {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct GetSearchStateResponse {
    #[yaserde(prefix = "tse", rename = "State")]
    pub state: tt::SearchState,
}

impl Validate for GetSearchStateResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct EndSearch {
    // The search session to end.
    #[yaserde(prefix = "tse", rename = "SearchToken")]
    pub search_token: tt::JobToken,
}

impl Validate for EndSearch {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tse",
    namespace = "tse: http://www.onvif.org/ver10/search/wsdl"
)]
pub struct EndSearchResponse {
    // The point of time the search had reached when it was ended. It is equal
    // to the EndPoint specified in Find-operation if the search was completed.
    #[yaserde(prefix = "tse", rename = "Endpoint")]
    pub endpoint: xs::DateTime,
}

impl Validate for EndSearchResponse {}

// Returns the capabilities of the search service. The result is returned in a
// typed answer.
pub async fn get_service_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetServiceCapabilities,
) -> Result<GetServiceCapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// GetRecordingSummary is used to get a summary description of all recorded
// data. This
// operation is mandatory to support for a device implementing the recording
// search service.
pub async fn get_recording_summary<T: transport::Transport>(
    transport: &T,
    request: &GetRecordingSummary,
) -> Result<GetRecordingSummaryResponse, transport::Error> {
    transport::request(transport, request).await
}

// Returns information about a single Recording specified by a RecordingToken.
// This operation
// is mandatory to support for a device implementing the recording search
// service.
pub async fn get_recording_information<T: transport::Transport>(
    transport: &T,
    request: &GetRecordingInformation,
) -> Result<GetRecordingInformationResponse, transport::Error> {
    transport::request(transport, request).await
}

// Returns a set of media attributes for all tracks of the specified recordings
// at a specified point
// in time. Clients using this operation shall be able to use it as a non
// blocking operation. A
// device shall set the starttime and endtime of the MediaAttributes structure
// to equal values if
// calculating this range would causes this operation to block. See
// MediaAttributes structure for
// more information. This operation is mandatory to support for a device
// implementing the
// recording search service.
pub async fn get_media_attributes<T: transport::Transport>(
    transport: &T,
    request: &GetMediaAttributes,
) -> Result<GetMediaAttributesResponse, transport::Error> {
    transport::request(transport, request).await
}

// FindRecordings starts a search session, looking for recordings that matches
// the scope (See
// 20.2.4) defined in the request. Results from the search are acquired using
// the
// GetRecordingSearchResults request, specifying the search token returned from
// this request.
// The device shall continue searching until one of the following occurs:
pub async fn find_recordings<T: transport::Transport>(
    transport: &T,
    request: &FindRecordings,
) -> Result<FindRecordingsResponse, transport::Error> {
    transport::request(transport, request).await
}

// GetRecordingSearchResults acquires the results from a recording search
// session previously
// initiated by a FindRecordings operation. The response shall not include
// results already
// returned in previous requests for the same session. If MaxResults is
// specified, the response
// shall not contain more than MaxResults results. The number of results relates
// to the number of recordings.
// For viewing individual recorded data for a signal track use the FindEvents
// method.
pub async fn get_recording_search_results<T: transport::Transport>(
    transport: &T,
    request: &GetRecordingSearchResults,
) -> Result<GetRecordingSearchResultsResponse, transport::Error> {
    transport::request(transport, request).await
}

// FindEvents starts a search session, looking for recording events (in the
// scope that
// matches the search filter defined in the request. Results from the search are
// acquired using the GetEventSearchResults request, specifying the search token
// returned from
// this request.
pub async fn find_events<T: transport::Transport>(
    transport: &T,
    request: &FindEvents,
) -> Result<FindEventsResponse, transport::Error> {
    transport::request(transport, request).await
}

// GetEventSearchResults acquires the results from a recording event search
// session previously
// initiated by a FindEvents operation. The response shall not include results
// already returned in
// previous requests for the same session. If MaxResults is specified, the
// response shall not
// contain more than MaxResults results.
pub async fn get_event_search_results<T: transport::Transport>(
    transport: &T,
    request: &GetEventSearchResults,
) -> Result<GetEventSearchResultsResponse, transport::Error> {
    transport::request(transport, request).await
}

// FindPTZPosition starts a search session, looking for ptz positions in the
// scope (See 20.2.4)
// that matches the search filter defined in the request. Results from the
// search are acquired
// using the GetPTZPositionSearchResults request, specifying the search token
// returned from
// this request.
pub async fn find_ptz_position<T: transport::Transport>(
    transport: &T,
    request: &FindPTZPosition,
) -> Result<FindPTZPositionResponse, transport::Error> {
    transport::request(transport, request).await
}

// GetPTZPositionSearchResults acquires the results from a ptz position search
// session
// previously initiated by a FindPTZPosition operation. The response shall not
// include results
// already returned in previous requests for the same session. If MaxResults is
// specified, the
// response shall not contain more than MaxResults results.
pub async fn get_ptz_position_search_results<T: transport::Transport>(
    transport: &T,
    request: &GetPTZPositionSearchResults,
) -> Result<GetPTZPositionSearchResultsResponse, transport::Error> {
    transport::request(transport, request).await
}

// GetSearchState returns the current state of the specified search session.
// This command is deprecated .
pub async fn get_search_state<T: transport::Transport>(
    transport: &T,
    request: &GetSearchState,
) -> Result<GetSearchStateResponse, transport::Error> {
    transport::request(transport, request).await
}

// EndSearch stops and ongoing search session, causing any blocking result
// request to return
// and the SearchToken to become invalid. If the search was interrupted before
// completion, the
// point in time that the search had reached shall be returned. If the search
// had not yet begun,
// the StartPoint shall be returned. If the search was completed the original
// EndPoint supplied
// by the Find operation shall be returned. When issuing EndSearch on a
// FindRecordings request the
// EndPoint is undefined and shall not be used since the FindRecordings request
// doesn't have
// StartPoint/EndPoint. This operation is mandatory to support for a device
// implementing the
// recording search service.
pub async fn end_search<T: transport::Transport>(
    transport: &T,
    request: &EndSearch,
) -> Result<EndSearchResponse, transport::Error> {
    transport::request(transport, request).await
}

// FindMetadata starts a search session, looking for metadata in the scope (See
// 20.2.4) that
// matches the search filter defined in the request. Results from the search are
// acquired using
// the GetMetadataSearchResults request, specifying the search token returned
// from this
// request.
pub async fn find_metadata<T: transport::Transport>(
    transport: &T,
    request: &FindMetadata,
) -> Result<FindMetadataResponse, transport::Error> {
    transport::request(transport, request).await
}

// GetMetadataSearchResults acquires the results from a recording search session
// previously
// initiated by a FindMetadata operation. The response shall not include results
// already returned
// in previous requests for the same session. If MaxResults is specified, the
// response shall not
// contain more than MaxResults results.
pub async fn get_metadata_search_results<T: transport::Transport>(
    transport: &T,
    request: &GetMetadataSearchResults,
) -> Result<GetMetadataSearchResultsResponse, transport::Error> {
    transport::request(transport, request).await
}
