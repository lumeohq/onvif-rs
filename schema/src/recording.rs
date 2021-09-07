use crate::onvif as tt;
use crate::transport;
use crate::validate::Validate;
use std::str::FromStr;
use xsd_macro_utils::*;
use xsd_types::types as xs;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct GetServiceCapabilitiesResponse {
    // The capabilities for the recording service is returned in the
    // Capabilities element.
    #[yaserde(prefix = "trc", rename = "Capabilities")]
    pub capabilities: Capabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct Capabilities {
    // Indication if the device supports dynamic creation and deletion of
    // recordings
    #[yaserde(attribute, rename = "DynamicRecordings")]
    pub dynamic_recordings: Option<bool>,

    // Indication if the device supports dynamic creation and deletion of tracks
    #[yaserde(attribute, rename = "DynamicTracks")]
    pub dynamic_tracks: Option<bool>,

    // Indication which encodings are supported for recording. The list may
    // contain one or more enumeration values of tt:VideoEncoding and
    // tt:AudioEncoding. For encodings that are neither defined in
    // tt:VideoEncoding nor tt:AudioEncoding the device shall use the
    #[yaserde(attribute, rename = "Encoding")]
    pub encoding: Option<EncodingTypes>,

    // Maximum supported bit rate for all tracks of a recording in kBit/s.
    #[yaserde(attribute, rename = "MaxRate")]
    pub max_rate: Option<f64>,

    // Maximum supported bit rate for all recordings in kBit/s.
    #[yaserde(attribute, rename = "MaxTotalRate")]
    pub max_total_rate: Option<f64>,

    // Maximum number of recordings supported. (Integer values only.)
    #[yaserde(attribute, rename = "MaxRecordings")]
    pub max_recordings: Option<f64>,

    // Maximum total number of supported recording jobs by the device.
    #[yaserde(attribute, rename = "MaxRecordingJobs")]
    pub max_recording_jobs: Option<i32>,

    // Indication if the device supports the GetRecordingOptions command.
    #[yaserde(attribute, rename = "Options")]
    pub options: Option<bool>,

    // Indication if the device supports recording metadata.
    #[yaserde(attribute, rename = "MetadataRecording")]
    pub metadata_recording: Option<bool>,

    // Indication that the device supports ExportRecordedData command for the
    // listed export file formats.
    // The list shall return at least one export file format value. The value of
    // 'ONVIF' refers to
    // ONVIF Export File Format specification.
    #[yaserde(attribute, rename = "SupportedExportFileFormats")]
    pub supported_export_file_formats: Option<tt::StringAttrList>,
}

impl Validate for Capabilities {}

#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct EncodingTypes(pub Vec<String>);

impl Validate for EncodingTypes {}
// pub type Capabilities = Capabilities;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct CreateRecording {
    // Initial configuration for the recording.
    #[yaserde(prefix = "trc", rename = "RecordingConfiguration")]
    pub recording_configuration: tt::RecordingConfiguration,
}

impl Validate for CreateRecording {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct CreateRecordingResponse {
    // The reference to the created recording.
    #[yaserde(prefix = "trc", rename = "RecordingToken")]
    pub recording_token: tt::RecordingReference,
}

impl Validate for CreateRecordingResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct DeleteRecording {
    // The reference of the recording to be deleted.
    #[yaserde(prefix = "trc", rename = "RecordingToken")]
    pub recording_token: tt::RecordingReference,
}

impl Validate for DeleteRecording {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct DeleteRecordingResponse {}

impl Validate for DeleteRecordingResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct GetRecordings {}

impl Validate for GetRecordings {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct GetRecordingsResponse {
    // List of recording items.
    #[yaserde(prefix = "trc", rename = "RecordingItem")]
    pub recording_item: Vec<tt::GetRecordingsResponseItem>,
}

impl Validate for GetRecordingsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct SetRecordingConfiguration {
    // Token of the recording that shall be changed.
    #[yaserde(prefix = "trc", rename = "RecordingToken")]
    pub recording_token: tt::RecordingReference,

    // The new configuration.
    #[yaserde(prefix = "trc", rename = "RecordingConfiguration")]
    pub recording_configuration: tt::RecordingConfiguration,
}

impl Validate for SetRecordingConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct SetRecordingConfigurationResponse {}

impl Validate for SetRecordingConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct GetRecordingConfiguration {
    // Token of the configuration to be retrieved.
    #[yaserde(prefix = "trc", rename = "RecordingToken")]
    pub recording_token: tt::RecordingReference,
}

impl Validate for GetRecordingConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct GetRecordingConfigurationResponse {
    // Configuration of the recording.
    #[yaserde(prefix = "trc", rename = "RecordingConfiguration")]
    pub recording_configuration: tt::RecordingConfiguration,
}

impl Validate for GetRecordingConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct CreateTrack {
    // Identifies the recording to which a track shall be added.
    #[yaserde(prefix = "trc", rename = "RecordingToken")]
    pub recording_token: tt::RecordingReference,

    // The configuration of the new track.
    #[yaserde(prefix = "trc", rename = "TrackConfiguration")]
    pub track_configuration: tt::TrackConfiguration,
}

impl Validate for CreateTrack {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct CreateTrackResponse {
    // The TrackToken shall identify the newly created track. The
    // TrackToken shall be unique within the recoding to which
    // the new track belongs.
    #[yaserde(prefix = "trc", rename = "TrackToken")]
    pub track_token: tt::TrackReference,
}

impl Validate for CreateTrackResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct DeleteTrack {
    // Token of the recording the track belongs to.
    #[yaserde(prefix = "trc", rename = "RecordingToken")]
    pub recording_token: tt::RecordingReference,

    // Token of the track to be deleted.
    #[yaserde(prefix = "trc", rename = "TrackToken")]
    pub track_token: tt::TrackReference,
}

impl Validate for DeleteTrack {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct DeleteTrackResponse {}

impl Validate for DeleteTrackResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct GetTrackConfiguration {
    // Token of the recording the track belongs to.
    #[yaserde(prefix = "trc", rename = "RecordingToken")]
    pub recording_token: tt::RecordingReference,

    // Token of the track.
    #[yaserde(prefix = "trc", rename = "TrackToken")]
    pub track_token: tt::TrackReference,
}

impl Validate for GetTrackConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct GetTrackConfigurationResponse {
    // Configuration of the track.
    #[yaserde(prefix = "trc", rename = "TrackConfiguration")]
    pub track_configuration: tt::TrackConfiguration,
}

impl Validate for GetTrackConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct SetTrackConfiguration {
    // Token of the recording the track belongs to.
    #[yaserde(prefix = "trc", rename = "RecordingToken")]
    pub recording_token: tt::RecordingReference,

    // Token of the track to be modified.
    #[yaserde(prefix = "trc", rename = "TrackToken")]
    pub track_token: tt::TrackReference,

    // New configuration for the track.
    #[yaserde(prefix = "trc", rename = "TrackConfiguration")]
    pub track_configuration: tt::TrackConfiguration,
}

impl Validate for SetTrackConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct SetTrackConfigurationResponse {}

impl Validate for SetTrackConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct CreateRecordingJob {
    // The initial configuration of the new recording job.
    #[yaserde(prefix = "trc", rename = "JobConfiguration")]
    pub job_configuration: tt::RecordingJobConfiguration,
}

impl Validate for CreateRecordingJob {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct CreateRecordingJobResponse {
    // The JobToken shall identify the created recording job.
    #[yaserde(prefix = "trc", rename = "JobToken")]
    pub job_token: tt::RecordingJobReference,

    // The JobConfiguration structure shall be the configuration as it is used
    // by the device. This may be different from the
    // JobConfiguration passed to CreateRecordingJob.
    #[yaserde(prefix = "trc", rename = "JobConfiguration")]
    pub job_configuration: tt::RecordingJobConfiguration,
}

impl Validate for CreateRecordingJobResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct DeleteRecordingJob {
    // The token of the job to be deleted.
    #[yaserde(prefix = "trc", rename = "JobToken")]
    pub job_token: tt::RecordingJobReference,
}

impl Validate for DeleteRecordingJob {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct DeleteRecordingJobResponse {}

impl Validate for DeleteRecordingJobResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct GetRecordingJobs {}

impl Validate for GetRecordingJobs {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct GetRecordingJobsResponse {
    // List of recording jobs.
    #[yaserde(prefix = "trc", rename = "JobItem")]
    pub job_item: Vec<tt::GetRecordingJobsResponseItem>,
}

impl Validate for GetRecordingJobsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct SetRecordingJobConfiguration {
    // Token of the job to be modified.
    #[yaserde(prefix = "trc", rename = "JobToken")]
    pub job_token: tt::RecordingJobReference,

    // New configuration of the recording job.
    #[yaserde(prefix = "trc", rename = "JobConfiguration")]
    pub job_configuration: tt::RecordingJobConfiguration,
}

impl Validate for SetRecordingJobConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct SetRecordingJobConfigurationResponse {
    // The JobConfiguration structure shall be the configuration
    // as it is used by the device. This may be different from the
    // JobConfiguration passed to SetRecordingJobConfiguration.
    #[yaserde(prefix = "trc", rename = "JobConfiguration")]
    pub job_configuration: tt::RecordingJobConfiguration,
}

impl Validate for SetRecordingJobConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct GetRecordingJobConfiguration {
    // Token of the recording job.
    #[yaserde(prefix = "trc", rename = "JobToken")]
    pub job_token: tt::RecordingJobReference,
}

impl Validate for GetRecordingJobConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct GetRecordingJobConfigurationResponse {
    // Current configuration of the recording job.
    #[yaserde(prefix = "trc", rename = "JobConfiguration")]
    pub job_configuration: tt::RecordingJobConfiguration,
}

impl Validate for GetRecordingJobConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct SetRecordingJobMode {
    // Token of the recording job.
    #[yaserde(prefix = "trc", rename = "JobToken")]
    pub job_token: tt::RecordingJobReference,

    // The new mode for the recording job.
    #[yaserde(prefix = "trc", rename = "Mode")]
    pub mode: tt::RecordingJobMode,
}

impl Validate for SetRecordingJobMode {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct SetRecordingJobModeResponse {}

impl Validate for SetRecordingJobModeResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct GetRecordingJobState {
    // Token of the recording job.
    #[yaserde(prefix = "trc", rename = "JobToken")]
    pub job_token: tt::RecordingJobReference,
}

impl Validate for GetRecordingJobState {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct GetRecordingJobStateResponse {
    // The current state of the recording job.
    #[yaserde(prefix = "trc", rename = "State")]
    pub state: tt::RecordingJobStateInformation,
}

impl Validate for GetRecordingJobStateResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct GetRecordingOptions {
    // Token of the recording.
    #[yaserde(prefix = "trc", rename = "RecordingToken")]
    pub recording_token: tt::RecordingReference,
}

impl Validate for GetRecordingOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct GetRecordingOptionsResponse {
    // Configuration of the recording.
    #[yaserde(prefix = "trc", rename = "Options")]
    pub options: RecordingOptions,
}

impl Validate for GetRecordingOptionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct ExportRecordedData {
    // Optional parameter that specifies start time for the exporting.
    #[yaserde(prefix = "trc", rename = "StartPoint")]
    pub start_point: Option<xs::DateTime>,

    // Optional parameter that specifies end time for the exporting.
    #[yaserde(prefix = "trc", rename = "EndPoint")]
    pub end_point: Option<xs::DateTime>,

    // Indicates the selection criterion on the existing recordings. .
    #[yaserde(prefix = "trc", rename = "SearchScope")]
    pub search_scope: tt::SearchScope,

    // Indicates which export file format to be used.
    #[yaserde(prefix = "trc", rename = "FileFormat")]
    pub file_format: String,

    // Indicates the target storage and relative directory path.
    #[yaserde(prefix = "trc", rename = "StorageDestination")]
    pub storage_destination: tt::StorageReferencePath,
}

impl Validate for ExportRecordedData {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct ExportRecordedDataResponse {
    // Unique operation token for client to associate the relevant events.
    #[yaserde(prefix = "trc", rename = "OperationToken")]
    pub operation_token: tt::ReferenceToken,

    // List of exported file names. The device can also use
    // AsyncronousOperationStatus event to publish this list.
    #[yaserde(prefix = "trc", rename = "FileNames")]
    pub file_names: Vec<String>,

    #[yaserde(prefix = "trc", rename = "Extension")]
    pub extension: Option<export_recorded_data_response::ExtensionType>,
}

impl Validate for ExportRecordedDataResponse {}

pub mod export_recorded_data_response {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "trc",
        namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
    )]
    pub struct ExtensionType {}

    impl Validate for ExtensionType {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct StopExportRecordedData {
    // Unique ExportRecordedData operation token
    #[yaserde(prefix = "trc", rename = "OperationToken")]
    pub operation_token: tt::ReferenceToken,
}

impl Validate for StopExportRecordedData {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct StopExportRecordedDataResponse {
    // Progress percentage of ExportRecordedData operation.
    #[yaserde(prefix = "trc", rename = "Progress")]
    pub progress: f64,

    #[yaserde(prefix = "trc", rename = "FileProgressStatus")]
    pub file_progress_status: tt::ArrayOfFileProgress,
}

impl Validate for StopExportRecordedDataResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct GetExportRecordedDataState {
    // Unique ExportRecordedData operation token
    #[yaserde(prefix = "trc", rename = "OperationToken")]
    pub operation_token: tt::ReferenceToken,
}

impl Validate for GetExportRecordedDataState {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct GetExportRecordedDataStateResponse {
    // Progress percentage of ExportRecordedData operation.
    #[yaserde(prefix = "trc", rename = "Progress")]
    pub progress: f64,

    #[yaserde(prefix = "trc", rename = "FileProgressStatus")]
    pub file_progress_status: tt::ArrayOfFileProgress,
}

impl Validate for GetExportRecordedDataStateResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct RecordingOptions {
    #[yaserde(prefix = "trc", rename = "Job")]
    pub job: JobOptions,

    #[yaserde(prefix = "trc", rename = "Track")]
    pub track: TrackOptions,
}

impl Validate for RecordingOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct JobOptions {
    // Number of spare jobs that can be created for the recording.
    #[yaserde(attribute, rename = "Spare")]
    pub spare: Option<i32>,

    // A device that supports recording of a restricted set of Media/Media2
    // Service Profiles returns the list of profiles that can be recorded on the
    // given Recording.
    #[yaserde(attribute, rename = "CompatibleSources")]
    pub compatible_sources: Option<tt::StringAttrList>,
}

impl Validate for JobOptions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "trc",
    namespace = "trc: http://www.onvif.org/ver10/recording/wsdl"
)]
pub struct TrackOptions {
    // Total spare number of tracks that can be added to this recording.
    #[yaserde(attribute, rename = "SpareTotal")]
    pub spare_total: Option<i32>,

    // Number of spare Video tracks that can be added to this recording.
    #[yaserde(attribute, rename = "SpareVideo")]
    pub spare_video: Option<i32>,

    // Number of spare Aduio tracks that can be added to this recording.
    #[yaserde(attribute, rename = "SpareAudio")]
    pub spare_audio: Option<i32>,

    // Number of spare Metadata tracks that can be added to this recording.
    #[yaserde(attribute, rename = "SpareMetadata")]
    pub spare_metadata: Option<i32>,
}

impl Validate for TrackOptions {}

// Returns the capabilities of the recording service. The result is returned in
// a typed answer.
pub async fn get_service_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetServiceCapabilities,
) -> Result<GetServiceCapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// CreateRecording shall create a new recording. The new recording shall be
// created with a track
// for each supported TrackType see Recording Control Spec.
pub async fn create_recording<T: transport::Transport>(
    transport: &T,
    request: &CreateRecording,
) -> Result<CreateRecordingResponse, transport::Error> {
    transport::request(transport, request).await
}

// DeleteRecording shall delete a recording object. Whenever a recording is
// deleted, the device
// shall delete all the tracks that are part of the recording, and it shall
// delete all the Recording
// Jobs that record into the recording. For each deleted recording job, the
// device shall also
// delete all the receiver objects associated with the recording job that are
// automatically created
// using the AutoCreateReceiver field of the recording job configuration
// structure and are not
// used in any other recording job.
pub async fn delete_recording<T: transport::Transport>(
    transport: &T,
    request: &DeleteRecording,
) -> Result<DeleteRecordingResponse, transport::Error> {
    transport::request(transport, request).await
}

// GetRecordings shall return a description of all the recordings in the device.
// This description
// shall include a list of all the tracks for each recording.
pub async fn get_recordings<T: transport::Transport>(
    transport: &T,
    request: &GetRecordings,
) -> Result<GetRecordingsResponse, transport::Error> {
    transport::request(transport, request).await
}

// SetRecordingConfiguration shall change the configuration of a recording.
pub async fn set_recording_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetRecordingConfiguration,
) -> Result<SetRecordingConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// GetRecordingConfiguration shall retrieve the recording configuration for a
// recording.
pub async fn get_recording_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetRecordingConfiguration,
) -> Result<GetRecordingConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// GetRecordingOptions returns information for a recording identified by the
// RecordingToken. The information includes the number of additonal tracks as
// well as recording jobs that can be configured.
pub async fn get_recording_options<T: transport::Transport>(
    transport: &T,
    request: &GetRecordingOptions,
) -> Result<GetRecordingOptionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This method shall create a new track within a recording.
pub async fn create_track<T: transport::Transport>(
    transport: &T,
    request: &CreateTrack,
) -> Result<CreateTrackResponse, transport::Error> {
    transport::request(transport, request).await
}

// DeleteTrack shall remove a track from a recording. All the data in the track
// shall be deleted.
pub async fn delete_track<T: transport::Transport>(
    transport: &T,
    request: &DeleteTrack,
) -> Result<DeleteTrackResponse, transport::Error> {
    transport::request(transport, request).await
}

// GetTrackConfiguration shall retrieve the configuration for a specific track.
pub async fn get_track_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetTrackConfiguration,
) -> Result<GetTrackConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// SetTrackConfiguration shall change the configuration of a track.
pub async fn set_track_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetTrackConfiguration,
) -> Result<SetTrackConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// CreateRecordingJob shall create a new recording job.
pub async fn create_recording_job<T: transport::Transport>(
    transport: &T,
    request: &CreateRecordingJob,
) -> Result<CreateRecordingJobResponse, transport::Error> {
    transport::request(transport, request).await
}

// DeleteRecordingJob removes a recording job. It shall also implicitly delete
// all the receiver
// objects associated with the recording job that are automatically created
// using the
// AutoCreateReceiver field of the recording job configuration structure and are
// not used in any
// other recording job.
pub async fn delete_recording_job<T: transport::Transport>(
    transport: &T,
    request: &DeleteRecordingJob,
) -> Result<DeleteRecordingJobResponse, transport::Error> {
    transport::request(transport, request).await
}

// GetRecordingJobs shall return a list of all the recording jobs in the device.
pub async fn get_recording_jobs<T: transport::Transport>(
    transport: &T,
    request: &GetRecordingJobs,
) -> Result<GetRecordingJobsResponse, transport::Error> {
    transport::request(transport, request).await
}

// SetRecordingJobConfiguration shall change the configuration for a recording
// job.
pub async fn set_recording_job_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetRecordingJobConfiguration,
) -> Result<SetRecordingJobConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// GetRecordingJobConfiguration shall return the current configuration for a
// recording job.
pub async fn get_recording_job_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetRecordingJobConfiguration,
) -> Result<GetRecordingJobConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// SetRecordingJobMode shall change the mode of the recording job. Using this
// method shall be
// equivalent to retrieving the recording job configuration, and writing it back
// with a different
// mode.
pub async fn set_recording_job_mode<T: transport::Transport>(
    transport: &T,
    request: &SetRecordingJobMode,
) -> Result<SetRecordingJobModeResponse, transport::Error> {
    transport::request(transport, request).await
}

// GetRecordingJobState returns the state of a recording job. It includes an
// aggregated state,
// and state for each track of the recording job.
pub async fn get_recording_job_state<T: transport::Transport>(
    transport: &T,
    request: &GetRecordingJobState,
) -> Result<GetRecordingJobStateResponse, transport::Error> {
    transport::request(transport, request).await
}

// Exports the selected recordings (from existing recorded data) to the given
// storage target based on the requested file format.
pub async fn export_recorded_data<T: transport::Transport>(
    transport: &T,
    request: &ExportRecordedData,
) -> Result<ExportRecordedDataResponse, transport::Error> {
    transport::request(transport, request).await
}

// Stops the selected ExportRecordedData operation.
pub async fn stop_export_recorded_data<T: transport::Transport>(
    transport: &T,
    request: &StopExportRecordedData,
) -> Result<StopExportRecordedDataResponse, transport::Error> {
    transport::request(transport, request).await
}

// Retrieves the status of selected ExportRecordedData operation.
pub async fn get_export_recorded_data_state<T: transport::Transport>(
    transport: &T,
    request: &GetExportRecordedDataState,
) -> Result<GetExportRecordedDataStateResponse, transport::Error> {
    transport::request(transport, request).await
}
