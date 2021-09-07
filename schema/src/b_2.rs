pub use crate::common::*;
use crate::{t_1 as wstop, validate::Validate, ws_addr as wsa};
use xsd_macro_utils::*;
use xsd_types::types as xs;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct QueryExpressionType {
    #[yaserde(attribute, rename = "Dialect")]
    pub dialect: String,
}

impl Validate for QueryExpressionType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct TopicExpressionType {
    #[yaserde(attribute, rename = "Dialect")]
    pub dialect: String,
}

impl Validate for TopicExpressionType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct FilterType {}

impl Validate for FilterType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct SubscriptionPolicyType {}

impl Validate for SubscriptionPolicyType {}

pub type TopicExpression = TopicExpressionType;
// pub type FixedTopicSet = bool;
// pub type TopicExpressionDialect = String;

// TODO: replace FixedTopicSet and TopicExpressionDialect with actual types generated from .xsd

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
pub struct FixedTopicSet {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
pub struct TopicExpressionDialect {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct NotificationProducerRP {
    #[yaserde(prefix = "wsnt", rename = "TopicExpression")]
    pub topic_expression: Vec<TopicExpression>,

    #[yaserde(prefix = "wsnt", rename = "FixedTopicSet")]
    pub fixed_topic_set: FixedTopicSet,

    #[yaserde(prefix = "wsnt", rename = "TopicExpressionDialect")]
    pub topic_expression_dialect: Vec<TopicExpressionDialect>,

    #[yaserde(prefix = "wstop", rename = "TopicSet")]
    pub topic_set: wstop::TopicSet,
}

impl Validate for NotificationProducerRP {}

pub type ConsumerReference = wsa::EndpointReferenceType;
pub type Filter = FilterType;
pub type SubscriptionPolicy = SubscriptionPolicyType;
pub type CreationTime = xs::DateTime;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct SubscriptionManagerRP {
    #[yaserde(prefix = "wsnt", rename = "ConsumerReference")]
    pub consumer_reference: ConsumerReference,

    #[yaserde(prefix = "wsnt", rename = "Filter")]
    pub filter: Filter,

    #[yaserde(prefix = "wsnt", rename = "SubscriptionPolicy")]
    pub subscription_policy: SubscriptionPolicy,

    #[yaserde(prefix = "wsnt", rename = "CreationTime")]
    pub creation_time: CreationTime,
}

impl Validate for SubscriptionManagerRP {}

pub type SubscriptionReference = wsa::EndpointReferenceType;
pub type Topic = TopicExpressionType;
pub type ProducerReference = wsa::EndpointReferenceType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct NotificationMessageHolderType {
    #[yaserde(prefix = "wsnt", rename = "SubscriptionReference")]
    pub subscription_reference: SubscriptionReference,

    #[yaserde(prefix = "wsnt", rename = "Topic")]
    pub topic: Topic,

    #[yaserde(prefix = "wsnt", rename = "ProducerReference")]
    pub producer_reference: ProducerReference,

    #[yaserde(prefix = "wsnt", rename = "Message")]
    pub message: notification_message_holder_type::MessageType,
}

impl Validate for NotificationMessageHolderType {}

pub mod notification_message_holder_type {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "wsnt",
        namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
    )]
    pub struct MessageType {}

    impl Validate for MessageType {}
}

pub type NotificationMessage = NotificationMessageHolderType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct Notify {
    #[yaserde(prefix = "wsnt", rename = "NotificationMessage")]
    pub notification_message: Vec<NotificationMessage>,
}

impl Validate for Notify {}

#[derive(PartialEq, Debug, UtilsUnionSerDe)]
pub enum AbsoluteOrRelativeTimeType {
    DateTime(xs::DateTime),
    Duration(xs::Duration),
    __Unknown__(String),
}

impl Default for AbsoluteOrRelativeTimeType {
    fn default() -> AbsoluteOrRelativeTimeType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for AbsoluteOrRelativeTimeType {}

pub type CurrentTime = xs::DateTime;
pub type TerminationTime = xs::DateTime;
pub type ProducerProperties = QueryExpressionType;
pub type MessageContent = QueryExpressionType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct UseRaw {}

impl Validate for UseRaw {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct Subscribe {
    #[yaserde(prefix = "wsnt", rename = "ConsumerReference")]
    pub consumer_reference: wsa::EndpointReferenceType,

    #[yaserde(prefix = "wsnt", rename = "Filter")]
    pub filter: FilterType,

    #[yaserde(prefix = "wsnt", rename = "InitialTerminationTime")]
    pub initial_termination_time: AbsoluteOrRelativeTimeType,

    #[yaserde(prefix = "wsnt", rename = "SubscriptionPolicy")]
    pub subscription_policy: subscribe::SubscriptionPolicyType,
}

impl Validate for Subscribe {}

pub mod subscribe {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "wsnt",
        namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
    )]
    pub struct SubscriptionPolicyType {}

    impl Validate for SubscriptionPolicyType {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct SubscribeResponse {
    #[yaserde(prefix = "wsnt", rename = "SubscriptionReference")]
    pub subscription_reference: wsa::EndpointReferenceType,

    #[yaserde(prefix = "wsnt", rename = "CurrentTime")]
    pub current_time: CurrentTime,

    #[yaserde(prefix = "wsnt", rename = "TerminationTime")]
    pub termination_time: TerminationTime,
}

impl Validate for SubscribeResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct GetCurrentMessage {
    #[yaserde(prefix = "wsnt", rename = "Topic")]
    pub topic: TopicExpressionType,
}

impl Validate for GetCurrentMessage {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct GetCurrentMessageResponse {}

impl Validate for GetCurrentMessageResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct SubscribeCreationFailedFaultType {}

impl Validate for SubscribeCreationFailedFaultType {}

pub type SubscribeCreationFailedFault = SubscribeCreationFailedFaultType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct InvalidFilterFaultType {
    #[yaserde(prefix = "wsnt", rename = "UnknownFilter")]
    pub unknown_filter: Vec<String>,
}

impl Validate for InvalidFilterFaultType {}

pub type InvalidFilterFault = InvalidFilterFaultType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct TopicExpressionDialectUnknownFaultType {}

impl Validate for TopicExpressionDialectUnknownFaultType {}

pub type TopicExpressionDialectUnknownFault = TopicExpressionDialectUnknownFaultType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct InvalidTopicExpressionFaultType {}

impl Validate for InvalidTopicExpressionFaultType {}

pub type InvalidTopicExpressionFault = InvalidTopicExpressionFaultType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct TopicNotSupportedFaultType {}

impl Validate for TopicNotSupportedFaultType {}

pub type TopicNotSupportedFault = TopicNotSupportedFaultType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct MultipleTopicsSpecifiedFaultType {}

impl Validate for MultipleTopicsSpecifiedFaultType {}

pub type MultipleTopicsSpecifiedFault = MultipleTopicsSpecifiedFaultType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct InvalidProducerPropertiesExpressionFaultType {}

impl Validate for InvalidProducerPropertiesExpressionFaultType {}

pub type InvalidProducerPropertiesExpressionFault = InvalidProducerPropertiesExpressionFaultType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct InvalidMessageContentExpressionFaultType {}

impl Validate for InvalidMessageContentExpressionFaultType {}

pub type InvalidMessageContentExpressionFault = InvalidMessageContentExpressionFaultType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct UnrecognizedPolicyRequestFaultType {
    #[yaserde(prefix = "wsnt", rename = "UnrecognizedPolicy")]
    pub unrecognized_policy: Vec<String>,
}

impl Validate for UnrecognizedPolicyRequestFaultType {}

pub type UnrecognizedPolicyRequestFault = UnrecognizedPolicyRequestFaultType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct UnsupportedPolicyRequestFaultType {
    #[yaserde(prefix = "wsnt", rename = "UnsupportedPolicy")]
    pub unsupported_policy: Vec<String>,
}

impl Validate for UnsupportedPolicyRequestFaultType {}

pub type UnsupportedPolicyRequestFault = UnsupportedPolicyRequestFaultType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct NotifyMessageNotSupportedFaultType {}

impl Validate for NotifyMessageNotSupportedFaultType {}

pub type NotifyMessageNotSupportedFault = NotifyMessageNotSupportedFaultType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct UnacceptableInitialTerminationTimeFaultType {
    #[yaserde(prefix = "wsnt", rename = "MinimumTime")]
    pub minimum_time: xs::DateTime,

    #[yaserde(prefix = "wsnt", rename = "MaximumTime")]
    pub maximum_time: Option<xs::DateTime>,
}

impl Validate for UnacceptableInitialTerminationTimeFaultType {}

pub type UnacceptableInitialTerminationTimeFault = UnacceptableInitialTerminationTimeFaultType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct NoCurrentMessageOnTopicFaultType {}

impl Validate for NoCurrentMessageOnTopicFaultType {}

pub type NoCurrentMessageOnTopicFault = NoCurrentMessageOnTopicFaultType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct GetMessages {
    #[yaserde(prefix = "wsnt", rename = "MaximumNumber")]
    pub maximum_number: Option<xs::Integer>,
}

impl Validate for GetMessages {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct GetMessagesResponse {
    #[yaserde(prefix = "wsnt", rename = "NotificationMessage")]
    pub notification_message: Vec<NotificationMessage>,
}

impl Validate for GetMessagesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct DestroyPullPoint {}

impl Validate for DestroyPullPoint {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct DestroyPullPointResponse {}

impl Validate for DestroyPullPointResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct UnableToGetMessagesFaultType {}

impl Validate for UnableToGetMessagesFaultType {}

pub type UnableToGetMessagesFault = UnableToGetMessagesFaultType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct UnableToDestroyPullPointFaultType {}

impl Validate for UnableToDestroyPullPointFaultType {}

pub type UnableToDestroyPullPointFault = UnableToDestroyPullPointFaultType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct CreatePullPoint {}

impl Validate for CreatePullPoint {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct CreatePullPointResponse {
    #[yaserde(prefix = "wsnt", rename = "PullPoint")]
    pub pull_point: wsa::EndpointReferenceType,
}

impl Validate for CreatePullPointResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct UnableToCreatePullPointFaultType {}

impl Validate for UnableToCreatePullPointFaultType {}

pub type UnableToCreatePullPointFault = UnableToCreatePullPointFaultType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct Renew {
    #[yaserde(prefix = "wsnt", rename = "TerminationTime")]
    pub termination_time: AbsoluteOrRelativeTimeType,
}

impl Validate for Renew {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct RenewResponse {
    #[yaserde(prefix = "wsnt", rename = "TerminationTime")]
    pub termination_time: TerminationTime,

    #[yaserde(prefix = "wsnt", rename = "CurrentTime")]
    pub current_time: CurrentTime,
}

impl Validate for RenewResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct UnacceptableTerminationTimeFaultType {
    #[yaserde(prefix = "wsnt", rename = "MinimumTime")]
    pub minimum_time: xs::DateTime,

    #[yaserde(prefix = "wsnt", rename = "MaximumTime")]
    pub maximum_time: Option<xs::DateTime>,
}

impl Validate for UnacceptableTerminationTimeFaultType {}

pub type UnacceptableTerminationTimeFault = UnacceptableTerminationTimeFaultType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct Unsubscribe {}

impl Validate for Unsubscribe {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct UnsubscribeResponse {}

impl Validate for UnsubscribeResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct UnableToDestroySubscriptionFaultType {}

impl Validate for UnableToDestroySubscriptionFaultType {}

pub type UnableToDestroySubscriptionFault = UnableToDestroySubscriptionFaultType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct PauseSubscription {}

impl Validate for PauseSubscription {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct PauseSubscriptionResponse {}

impl Validate for PauseSubscriptionResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct ResumeSubscription {}

impl Validate for ResumeSubscription {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct ResumeSubscriptionResponse {}

impl Validate for ResumeSubscriptionResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct PauseFailedFaultType {}

impl Validate for PauseFailedFaultType {}

pub type PauseFailedFault = PauseFailedFaultType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wsnt",
    namespace = "wsnt: http://docs.oasis-open.org/wsn/b-2"
)]
pub struct ResumeFailedFaultType {}

impl Validate for ResumeFailedFaultType {}

pub type ResumeFailedFault = ResumeFailedFaultType;
