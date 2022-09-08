#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::large_enum_variant)]

use b_2 as wsnt;
use common::*;
use validate::Validate;
use xsd_types::types as xs;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Appearance {
    #[yaserde(prefix = "tt", rename = "Transformation")]
    pub transformation: Option<Transformation>,

    #[yaserde(prefix = "tt", rename = "Shape")]
    pub shape: Option<ShapeDescriptor>,

    #[yaserde(prefix = "tt", rename = "Color")]
    pub color: Option<ColorDescriptor>,

    #[yaserde(prefix = "tt", rename = "Class")]
    pub class: Option<ClassDescriptor>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<AppearanceExtension>,

    #[yaserde(prefix = "tt", rename = "GeoLocation")]
    pub geo_location: Option<GeoLocation>,

    #[yaserde(prefix = "tt", rename = "VehicleInfo")]
    pub vehicle_info: Option<VehicleInfo>,

    #[yaserde(prefix = "tt", rename = "LicensePlateInfo")]
    pub license_plate_info: Option<LicensePlateInfo>,
}

impl Validate for Appearance {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AppearanceExtension {}

impl Validate for AppearanceExtension {}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum VehicleType {
    Bus,
    Car,
    Truck,
    Bicycle,
    Motorcycle,
    __Unknown__(String),
}

impl Default for VehicleType {
    fn default() -> VehicleType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for VehicleType {}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum PlateType {
    Normal,
    Police,
    Diplomat,
    Temporary,
    __Unknown__(String),
}

impl Default for PlateType {
    fn default() -> PlateType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for PlateType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VehicleInfo {
    #[yaserde(prefix = "tt", rename = "Type")]
    pub _type: StringLikelihood,

    #[yaserde(prefix = "tt", rename = "Brand")]
    pub brand: StringLikelihood,

    #[yaserde(prefix = "tt", rename = "Model")]
    pub model: StringLikelihood,
}

impl Validate for VehicleInfo {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct LicensePlateInfo {
    // A string of vehicle license plate number.
    #[yaserde(prefix = "tt", rename = "PlateNumber")]
    pub plate_number: StringLikelihood,

    // A description of the vehicle license plate, e.g., "Normal", "Police",
    // "Diplomat"
    #[yaserde(prefix = "tt", rename = "PlateType")]
    pub plate_type: StringLikelihood,

    // Describe the country of the license plate, in order to avoid the same
    // license plate number.
    #[yaserde(prefix = "tt", rename = "CountryCode")]
    pub country_code: StringLikelihood,

    // State province or authority that issue the license plate.
    #[yaserde(prefix = "tt", rename = "IssuingEntity")]
    pub issuing_entity: StringLikelihood,
}

impl Validate for LicensePlateInfo {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ShapeDescriptor {
    #[yaserde(prefix = "tt", rename = "BoundingBox")]
    pub bounding_box: Rectangle,

    #[yaserde(prefix = "tt", rename = "CenterOfGravity")]
    pub center_of_gravity: Vector,

    #[yaserde(prefix = "tt", rename = "Polygon")]
    pub polygon: Vec<Polygon>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ShapeDescriptorExtension>,
}

impl Validate for ShapeDescriptor {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ShapeDescriptorExtension {}

impl Validate for ShapeDescriptorExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ColorDescriptor {
    #[yaserde(prefix = "tt", rename = "ColorCluster")]
    pub color_cluster: Vec<color_descriptor::ColorClusterType>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ColorDescriptorExtension>,
}

impl Validate for ColorDescriptor {}

pub mod color_descriptor {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
    pub struct ColorClusterType {
        #[yaserde(prefix = "tt", rename = "Color")]
        pub color: Color,

        #[yaserde(prefix = "tt", rename = "Weight")]
        pub weight: Option<f64>,

        #[yaserde(prefix = "tt", rename = "Covariance")]
        pub covariance: Option<ColorCovariance>,
    }

    impl Validate for ColorClusterType {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ColorDescriptorExtension {}

impl Validate for ColorDescriptorExtension {}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum ClassType {
    Animal,
    Face,
    Human,
    Vehical,
    Other,
    __Unknown__(String),
}

impl Default for ClassType {
    fn default() -> ClassType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for ClassType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct StringLikelihood {
    #[yaserde(attribute, rename = "Likelihood")]
    pub likelihood: Option<f64>,
}

impl Validate for StringLikelihood {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ClassDescriptor {
    #[yaserde(prefix = "tt", rename = "ClassCandidate")]
    pub class_candidate: Vec<class_descriptor::ClassCandidateType>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ClassDescriptorExtension>,

    // ONVIF recommends to use this 'Type' element instead of 'ClassCandidate'
    // and 'Extension' above for new design.
    #[yaserde(prefix = "tt", rename = "Type")]
    pub _type: Vec<StringLikelihood>,
}

impl Validate for ClassDescriptor {}

pub mod class_descriptor {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
    pub struct ClassCandidateType {
        #[yaserde(prefix = "tt", rename = "Type")]
        pub _type: ClassType,

        #[yaserde(prefix = "tt", rename = "Likelihood")]
        pub likelihood: f64,
    }

    impl Validate for ClassCandidateType {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ClassDescriptorExtension {
    #[yaserde(prefix = "tt", rename = "OtherTypes")]
    pub other_types: Vec<OtherType>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ClassDescriptorExtension2>,
}

impl Validate for ClassDescriptorExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ClassDescriptorExtension2 {}

impl Validate for ClassDescriptorExtension2 {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct OtherType {
    // Object Class Type
    #[yaserde(prefix = "tt", rename = "Type")]
    pub _type: String,

    // A likelihood/probability that the corresponding object belongs to this
    // class. The sum of the likelihoods shall NOT exceed 1
    #[yaserde(prefix = "tt", rename = "Likelihood")]
    pub likelihood: f64,
}

impl Validate for OtherType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Object {
    #[yaserde(prefix = "tt", rename = "Appearance")]
    pub appearance: Option<Appearance>,

    #[yaserde(prefix = "tt", rename = "Behaviour")]
    pub behaviour: Option<Behaviour>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ObjectExtension>,

    #[yaserde(attribute, rename = "ObjectId")]
    pub object_id: Option<xs::Integer>,
}

impl Validate for Object {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ObjectExtension {}

impl Validate for ObjectExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Frame {
    #[yaserde(prefix = "tt", rename = "PTZStatus")]
    pub ptz_status: Option<Ptzstatus>,

    #[yaserde(prefix = "tt", rename = "Transformation")]
    pub transformation: Option<Transformation>,

    #[yaserde(prefix = "tt", rename = "Object")]
    pub object: Vec<Object>,

    #[yaserde(prefix = "tt", rename = "ObjectTree")]
    pub object_tree: Option<ObjectTree>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<FrameExtension>,

    #[yaserde(attribute, rename = "UtcTime")]
    pub utc_time: xs::DateTime,
}

impl Validate for Frame {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct FrameExtension {
    #[yaserde(prefix = "tt", rename = "MotionInCells")]
    pub motion_in_cells: Option<MotionInCells>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<FrameExtension2>,
}

impl Validate for FrameExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct FrameExtension2 {}

impl Validate for FrameExtension2 {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Merge {
    #[yaserde(prefix = "tt", rename = "from")]
    pub from: Vec<ObjectId>,

    #[yaserde(prefix = "tt", rename = "to")]
    pub to: ObjectId,
}

impl Validate for Merge {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Split {
    #[yaserde(prefix = "tt", rename = "from")]
    pub from: ObjectId,

    #[yaserde(prefix = "tt", rename = "to")]
    pub to: Vec<ObjectId>,
}

impl Validate for Split {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Rename {
    #[yaserde(prefix = "tt", rename = "from")]
    pub from: ObjectId,

    #[yaserde(prefix = "tt", rename = "to")]
    pub to: ObjectId,
}

impl Validate for Rename {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ObjectId {
    #[yaserde(attribute, rename = "ObjectId")]
    pub object_id: Option<xs::Integer>,
}

impl Validate for ObjectId {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Behaviour {
    #[yaserde(prefix = "tt", rename = "Removed")]
    pub removed: Option<behaviour::RemovedType>,

    #[yaserde(prefix = "tt", rename = "Idle")]
    pub idle: Option<behaviour::IdleType>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<BehaviourExtension>,

    #[yaserde(prefix = "tt", rename = "Speed")]
    pub speed: Option<f64>,
}

impl Validate for Behaviour {}

pub mod behaviour {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
    pub struct RemovedType {}

    impl Validate for RemovedType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
    pub struct IdleType {}

    impl Validate for IdleType {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct BehaviourExtension {}

impl Validate for BehaviourExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ObjectTree {
    #[yaserde(prefix = "tt", rename = "Rename")]
    pub rename: Vec<Rename>,

    #[yaserde(prefix = "tt", rename = "Split")]
    pub split: Vec<Split>,

    #[yaserde(prefix = "tt", rename = "Merge")]
    pub merge: Vec<Merge>,

    #[yaserde(prefix = "tt", rename = "Delete")]
    pub delete: Vec<ObjectId>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<ObjectTreeExtension>,
}

impl Validate for ObjectTree {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct ObjectTreeExtension {}

impl Validate for ObjectTreeExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MotionInCells {
    // Number of columns of the cell grid (x dimension)
    #[yaserde(attribute, rename = "Columns")]
    pub columns: xs::Integer,

    // Number of rows of the cell grid (y dimension)
    #[yaserde(attribute, rename = "Rows")]
    pub rows: xs::Integer,

    // A “1” denotes a cell where motion is detected and a “0” an empty
    // cell. The first cell is in the upper left corner. Then the cell order
    // goes first from left to right and then from up to down. If the number of
    // cells is not a multiple of 8 the last byte is filled with zeros. The
    // information is run length encoded according to Packbit coding in ISO
    // 12369 (TIFF, Revision 6.0).
    #[yaserde(attribute, rename = "Cells")]
    pub cells: String,
}

impl Validate for MotionInCells {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MetadataStream {
    #[yaserde(prefix = "tt", rename = "MetadataStreamChoice")]
    pub metadata_stream_choice: metadata_stream::MetadataStreamChoice,
}

impl Validate for MetadataStream {}

pub mod metadata_stream {
    use super::*;

    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    pub enum MetadataStreamChoice {
        VideoAnalytics(VideoAnalyticsStream),
        #[yaserde(rename = "PTZ")]
        Ptz(Ptzstream),
        Event(EventStream),
        Extension(MetadataStreamExtension),
        __Unknown__(String),
    }

    impl Default for MetadataStreamChoice {
        fn default() -> MetadataStreamChoice {
            Self::__Unknown__("No valid variants".into())
        }
    }

    impl Validate for MetadataStreamChoice {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MetadataStreamExtension {
    #[yaserde(prefix = "tt", rename = "AudioAnalyticsStream")]
    pub audio_analytics_stream: Option<AudioAnalyticsStream>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<MetadataStreamExtension2>,
}

impl Validate for MetadataStreamExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct MetadataStreamExtension2 {}

impl Validate for MetadataStreamExtension2 {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AudioAnalyticsStream {
    #[yaserde(prefix = "tt", rename = "AudioDescriptor")]
    pub audio_descriptor: Vec<AudioDescriptor>,

    #[yaserde(prefix = "tt", rename = "Extension")]
    pub extension: Option<AudioAnalyticsStreamExtension>,
}

impl Validate for AudioAnalyticsStream {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AudioDescriptor {
    #[yaserde(attribute, rename = "UtcTime")]
    pub utc_time: xs::DateTime,
}

impl Validate for AudioDescriptor {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct AudioAnalyticsStreamExtension {}

impl Validate for AudioAnalyticsStreamExtension {}

// pub type MetadataStream = MetadataStream;
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum VideoAnalyticsStreamChoice {
    Frame(Frame),
    Extension(VideoAnalyticsStreamExtension),
    __Unknown__(String),
}

impl Default for VideoAnalyticsStreamChoice {
    fn default() -> VideoAnalyticsStreamChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for VideoAnalyticsStreamChoice {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoAnalyticsStream {
    #[yaserde(flatten)]
    pub video_analytics_stream_choice: VideoAnalyticsStreamChoice,
}

impl Validate for VideoAnalyticsStream {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct VideoAnalyticsStreamExtension {}

impl Validate for VideoAnalyticsStreamExtension {}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum PtzstreamChoice {
    #[yaserde(rename = "PTZStatus")]
    Ptzstatus(Ptzstatus),
    Extension(PtzstreamExtension),
    __Unknown__(String),
}

impl Default for PtzstreamChoice {
    fn default() -> PtzstreamChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for PtzstreamChoice {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct Ptzstream {
    #[yaserde(flatten)]
    pub ptz_stream_choice: PtzstreamChoice,
}

impl Validate for Ptzstream {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct PtzstreamExtension {}

impl Validate for PtzstreamExtension {}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum EventStreamChoice {
    #[yaserde(prefix = "wsnt", rename = "NotificationMessage")]
    NotificationMessage(wsnt::NotificationMessage),
    Extension(EventStreamExtension),
    __Unknown__(String),
}

impl Default for EventStreamChoice {
    fn default() -> EventStreamChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for EventStreamChoice {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct EventStream {
    #[yaserde(flatten)]
    pub event_stream_choice: EventStreamChoice,
}

impl Validate for EventStream {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tt", namespace = "tt: http://www.onvif.org/ver10/schema")]
pub struct EventStreamExtension {}

impl Validate for EventStreamExtension {}
