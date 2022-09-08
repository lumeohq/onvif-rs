#![allow(clippy::derive_partial_eq_without_eq)]

use b_2 as wsnt;
use t_1 as wstop;
use validate::Validate;
use ws_addr as wsa;
use xsd_types::types as xs;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tev",
    namespace = "tev: http://www.onvif.org/ver10/events/wsdl"
)]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tev",
    namespace = "tev: http://www.onvif.org/ver10/events/wsdl"
)]
pub struct GetServiceCapabilitiesResponse {
    // The capabilities for the event service is returned in the Capabilities
    // element.
    #[yaserde(prefix = "tev", rename = "Capabilities")]
    pub capabilities: Capabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tev",
    namespace = "tev: http://www.onvif.org/ver10/events/wsdl"
)]
pub struct Capabilities {
    // Indicates that the WS Subscription policy is supported.
    #[yaserde(attribute, rename = "WSSubscriptionPolicySupport")]
    pub ws_subscription_policy_support: Option<bool>,

    // Indicates that the WS Pull Point is supported.
    #[yaserde(attribute, rename = "WSPullPointSupport")]
    pub ws_pull_point_support: Option<bool>,

    // Indicates that the WS Pausable Subscription Manager Interface is
    // supported.
    #[yaserde(attribute, rename = "WSPausableSubscriptionManagerInterfaceSupport")]
    pub ws_pausable_subscription_manager_interface_support: Option<bool>,

    // Maximum number of supported notification producers as defined by
    // WS-BaseNotification.
    #[yaserde(attribute, rename = "MaxNotificationProducers")]
    pub max_notification_producers: Option<i32>,

    // Maximum supported number of notification pull points.
    #[yaserde(attribute, rename = "MaxPullPoints")]
    pub max_pull_points: Option<i32>,

    // Indication if the device supports persistent notification storage.
    #[yaserde(attribute, rename = "PersistentNotificationStorage")]
    pub persistent_notification_storage: Option<bool>,
}

impl Validate for Capabilities {}

// pub type Capabilities = Capabilities;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tev",
    namespace = "tev: http://www.onvif.org/ver10/events/wsdl"
)]
pub struct CreatePullPointSubscription {
    // Optional XPATH expression to select specific topics.
    #[yaserde(prefix = "tev", rename = "Filter")]
    pub filter: Option<wsnt::FilterType>,

    // Initial termination time.
    #[yaserde(prefix = "tev", rename = "InitialTerminationTime")]
    pub initial_termination_time: Option<wsnt::AbsoluteOrRelativeTimeType>,

    // Refer to
    #[yaserde(prefix = "tev", rename = "SubscriptionPolicy")]
    pub subscription_policy: Option<create_pull_point_subscription::SubscriptionPolicyType>,
}

impl Validate for CreatePullPointSubscription {}

pub mod create_pull_point_subscription {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tev",
        namespace = "tev: http://www.onvif.org/ver10/events/wsdl"
    )]
    pub struct SubscriptionPolicyType {}

    impl Validate for SubscriptionPolicyType {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tev",
    namespace = "tev: http://www.onvif.org/ver10/events/wsdl"
)]
pub struct CreatePullPointSubscriptionResponse {
    // Endpoint reference of the subscription to be used for pulling the
    // messages.
    #[yaserde(prefix = "tev", rename = "SubscriptionReference")]
    pub subscription_reference: wsa::EndpointReferenceType,

    // Current time of the server for synchronization purposes.
    #[yaserde(prefix = "wsnt", rename = "CurrentTime")]
    pub current_time: wsnt::CurrentTime,

    // Date time when the PullPoint will be shut down without further pull
    // requests.
    #[yaserde(prefix = "wsnt", rename = "TerminationTime")]
    pub termination_time: wsnt::TerminationTime,
}

impl Validate for CreatePullPointSubscriptionResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tev",
    namespace = "tev: http://www.onvif.org/ver10/events/wsdl"
)]
pub struct PullMessages {
    // Maximum time to block until this method returns.
    #[yaserde(prefix = "tev", rename = "Timeout")]
    pub timeout: xs::Duration,

    // Upper limit for the number of messages to return at once. A server
    // implementation may decide to return less messages.
    #[yaserde(prefix = "tev", rename = "MessageLimit")]
    pub message_limit: i32,
}

impl Validate for PullMessages {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tev",
    namespace = "tev: http://www.onvif.org/ver10/events/wsdl"
)]
pub struct PullMessagesResponse {
    // The date and time when the messages have been delivered by the web server
    // to the client.
    #[yaserde(prefix = "tev", rename = "CurrentTime")]
    pub current_time: xs::DateTime,

    // Date time when the PullPoint will be shut down without further pull
    // requests.
    #[yaserde(prefix = "tev", rename = "TerminationTime")]
    pub termination_time: xs::DateTime,

    // List of messages. This list shall be empty in case of a timeout.
    #[yaserde(prefix = "wsnt", rename = "NotificationMessage")]
    pub notification_message: Vec<wsnt::NotificationMessage>,
}

impl Validate for PullMessagesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tev",
    namespace = "tev: http://www.onvif.org/ver10/events/wsdl"
)]
pub struct PullMessagesFaultResponse {
    // Maximum timeout supported by the device.
    #[yaserde(prefix = "tev", rename = "MaxTimeout")]
    pub max_timeout: xs::Duration,

    // Maximum message limit supported by the device.
    #[yaserde(prefix = "tev", rename = "MaxMessageLimit")]
    pub max_message_limit: i32,
}

impl Validate for PullMessagesFaultResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tev",
    namespace = "tev: http://www.onvif.org/ver10/events/wsdl"
)]
pub struct Seek {
    // The date and time to match against stored messages.
    #[yaserde(prefix = "tev", rename = "UtcTime")]
    pub utc_time: xs::DateTime,

    // Reverse the pull direction of PullMessages.
    #[yaserde(prefix = "tev", rename = "Reverse")]
    pub reverse: bool,
}

impl Validate for Seek {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tev",
    namespace = "tev: http://www.onvif.org/ver10/events/wsdl"
)]
pub struct SeekResponse {}

impl Validate for SeekResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tev",
    namespace = "tev: http://www.onvif.org/ver10/events/wsdl"
)]
pub struct SetSynchronizationPoint {}

impl Validate for SetSynchronizationPoint {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tev",
    namespace = "tev: http://www.onvif.org/ver10/events/wsdl"
)]
pub struct SetSynchronizationPointResponse {}

impl Validate for SetSynchronizationPointResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tev",
    namespace = "tev: http://www.onvif.org/ver10/events/wsdl"
)]
pub struct GetEventProperties {}

impl Validate for GetEventProperties {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tev",
    namespace = "tev: http://www.onvif.org/ver10/events/wsdl"
)]
pub struct GetEventPropertiesResponse {
    // List of topic namespaces supported.
    #[yaserde(prefix = "tev", rename = "TopicNamespaceLocation")]
    pub topic_namespace_location: Vec<String>,

    // True when topicset is fixed for all times.
    #[yaserde(prefix = "wsnt", rename = "FixedTopicSet")]
    pub fixed_topic_set: wsnt::FixedTopicSet,

    // Set of topics supported.
    #[yaserde(prefix = "wstop", rename = "TopicSet")]
    pub topic_set: wstop::TopicSet,

    // Defines the XPath expression syntax supported for matching topic
    // expressions.
    #[yaserde(prefix = "wsnt", rename = "TopicExpressionDialect")]
    pub topic_expression_dialect: Vec<wsnt::TopicExpressionDialect>,

    // Defines the XPath function set supported for message content filtering.
    #[yaserde(prefix = "tev", rename = "MessageContentFilterDialect")]
    pub message_content_filter_dialect: Vec<String>,

    // Optional ProducerPropertiesDialects. Refer to
    #[yaserde(prefix = "tev", rename = "ProducerPropertiesFilterDialect")]
    pub producer_properties_filter_dialect: Vec<String>,

    // The Message Content Description Language allows referencing
    // of vendor-specific types. In order to ease the integration of such types
    // into a client application,
    // the GetEventPropertiesResponse shall list all URI locations to schema
    // files whose types are
    // used in the description of notifications, with
    // MessageContentSchemaLocation elements.
    #[yaserde(prefix = "tev", rename = "MessageContentSchemaLocation")]
    pub message_content_schema_location: Vec<String>,
}

impl Validate for GetEventPropertiesResponse {}

// When this element is included in the SubscriptionPolicy, the pullpoint shall
// not provide Initialized or Deleted events for Properties.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tev",
    namespace = "tev: http://www.onvif.org/ver10/events/wsdl"
)]
pub struct ChangedOnly {}

impl Validate for ChangedOnly {}

// Returns the capabilities of the event service. The result is returned in a
// typed answer.
pub async fn get_service_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetServiceCapabilities,
) -> Result<GetServiceCapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This method returns a PullPointSubscription that can be polled using
// PullMessages.
// This message contains the same elements as the SubscriptionRequest of the
// WS-BaseNotification without the ConsumerReference.
pub async fn create_pull_point_subscription<T: transport::Transport>(
    transport: &T,
    request: &CreatePullPointSubscription,
) -> Result<CreatePullPointSubscriptionResponse, transport::Error> {
    transport::request(transport, request).await
}

// The WS-BaseNotification specification defines a set of OPTIONAL
// WS-ResouceProperties.
// This specification does not require the implementation of the
// WS-ResourceProperty interface.
// Instead, the subsequent direct interface shall be implemented by an ONVIF
// compliant device
// in order to provide information about the FilterDialects, Schema files and
// topics supported by
// the device.
pub async fn get_event_properties<T: transport::Transport>(
    transport: &T,
    request: &GetEventProperties,
) -> Result<GetEventPropertiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This method pulls one or more messages from a PullPoint.
// The device shall provide the following PullMessages command for all
// SubscriptionManager
// endpoints returned by the CreatePullPointSubscription command. This method
// shall not wait until
// the requested number of messages is available but return as soon as at least
// one message is available.
pub async fn pull_messages<T: transport::Transport>(
    transport: &T,
    request: &PullMessages,
) -> Result<PullMessagesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This method readjusts the pull pointer into the past.
// A device supporting persistent notification storage shall provide the
// following Seek command for all SubscriptionManager endpoints returned by
// the CreatePullPointSubscription command. The optional Reverse argument can
// be used to reverse the pull direction of the PullMessages command.
pub async fn seek<T: transport::Transport>(
    transport: &T,
    request: &Seek,
) -> Result<SeekResponse, transport::Error> {
    transport::request(transport, request).await
}

// Properties inform a client about property creation, changes and
// deletion in a uniform way. When a client wants to synchronize its properties
// with the
// properties of the device, it can request a synchronization point which
// repeats the current
// status of all properties to which a client has subscribed. The
// PropertyOperation of all
// produced notifications is set to “Initialized”. The Synchronization Point
// is
// requested directly from the SubscriptionManager which was returned in either
// the
// SubscriptionResponse or in the CreatePullPointSubscriptionResponse. The
// property update is
// transmitted via the notification transportation of the notification
// interface. This method is mandatory.
pub async fn set_synchronization_point<T: transport::Transport>(
    transport: &T,
    request: &SetSynchronizationPoint,
) -> Result<SetSynchronizationPointResponse, transport::Error> {
    transport::request(transport, request).await
}

// The device shall provide the following Unsubscribe command for all
// SubscriptionManager endpoints returned by the CreatePullPointSubscription
// command.
// pub async fn unsubscribe<T: transport::Transport>(
//     transport: &T
// ) -> Result<, transport::Error> {
// transport::request(transport, request).await
// }
