use crate::validate::Validate;
use std::str::FromStr;
use xsd_macro_utils::*;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wstop",
    namespace = "wstop: http://docs.oasis-open.org/wsn/t-1"
)]
pub struct Documentation {}

impl Validate for Documentation {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wstop",
    namespace = "wstop: http://docs.oasis-open.org/wsn/t-1"
)]
pub struct ExtensibleDocumented {
    #[yaserde(prefix = "wstop", rename = "documentation")]
    pub documentation: Option<Documentation>,
}

impl Validate for ExtensibleDocumented {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wstop",
    namespace = "wstop: http://docs.oasis-open.org/wsn/t-1"
)]
pub struct QueryExpressionType {
    #[yaserde(attribute, rename = "Dialect")]
    pub dialect: String,
}

impl Validate for QueryExpressionType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wstop",
    namespace = "wstop: http://docs.oasis-open.org/wsn/t-1"
)]
pub struct TopicNamespaceType {
    #[yaserde(prefix = "wstop", rename = "Topic")]
    pub topic: Vec<topic_namespace_type::TopicType>,

    #[yaserde(attribute, rename = "name")]
    pub name: Option<String>,

    #[yaserde(attribute, rename = "targetNamespace")]
    pub target_namespace: String,

    #[yaserde(attribute, rename = "final")]
    pub _final: Option<bool>,

    #[yaserde(prefix = "wstop", rename = "documentation")]
    pub documentation: Option<Documentation>,
}

impl Validate for TopicNamespaceType {}

pub mod topic_namespace_type {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "wstop",
        namespace = "wstop: http://docs.oasis-open.org/wsn/t-1"
    )]
    pub struct TopicType {
        #[yaserde(attribute, rename = "parent")]
        pub parent: Option<ConcreteTopicExpression>,

        #[yaserde(prefix = "wstop", rename = "MessagePattern")]
        pub message_pattern: QueryExpressionType,

        #[yaserde(prefix = "wstop", rename = "Topic")]
        pub topic: Vec<TopicType>,

        #[yaserde(attribute, rename = "name")]
        pub name: String,

        #[yaserde(attribute, rename = "messageTypes")]
        pub message_types: Option<String>,

        #[yaserde(attribute, rename = "final")]
        pub _final: Option<bool>,
    }

    impl Validate for TopicType {}
}

// pub type TopicNamespace = TopicNamespaceType;
// pub type TopicNamespaceLocation = String;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wstop",
    namespace = "wstop: http://docs.oasis-open.org/wsn/t-1"
)]
pub struct TopicType {
    #[yaserde(prefix = "wstop", rename = "MessagePattern")]
    pub message_pattern: QueryExpressionType,

    #[yaserde(prefix = "wstop", rename = "Topic")]
    pub topic: Vec<TopicType>,

    #[yaserde(attribute, rename = "name")]
    pub name: String,

    #[yaserde(attribute, rename = "messageTypes")]
    pub message_types: Option<String>,

    #[yaserde(attribute, rename = "final")]
    pub _final: Option<bool>,

    #[yaserde(prefix = "wstop", rename = "documentation")]
    pub documentation: Option<Documentation>,
}

impl Validate for TopicType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "wstop",
    namespace = "wstop: http://docs.oasis-open.org/wsn/t-1"
)]
pub struct TopicSetType {
    #[yaserde(prefix = "wstop", rename = "documentation")]
    pub documentation: Option<Documentation>,
}

impl Validate for TopicSetType {}

pub type TopicSet = TopicSetType;
// pub type Topic = bool;
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct FullTopicExpression(pub String);

impl Validate for FullTopicExpression {}
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct ConcreteTopicExpression(pub String);

impl Validate for ConcreteTopicExpression {}
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct SimpleTopicExpression(pub String);

impl Validate for SimpleTopicExpression {}
