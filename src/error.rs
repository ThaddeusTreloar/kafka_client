use chrono::NaiveTime;
use serde::de;
use serde_json::error;

use crate::common::topic::{TopicPartition, OptionalPartition, Partition};

// TODO: The not so glamorous job of designing informative errors.... 

#[derive(Debug, thiserror::Error) ]
pub enum Error {
    #[error("Generic: {0}")]
    Generic(String),
    #[error(transparent)]
    Configuration(#[from] ConfigurationError),
    //#[error("Consumer: {0}")]
    //Consumer(ConsumerError),
    //#[error("Producer: {0}")]
    //Producer(ProducerError),
    //#[error("Kafka: {0}")]
    //Kafka(KafkaError),
    //#[error("StateStore: {0}")]
    //StateStore(StateStoreError),
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigurationError {
    #[error("Malformed Option: {0}")]
    MalformedOption(String),
    #[error("Malformed Options: {0:?}")]
    MalformedOptions(Vec<String>),
    #[error("Missing Option: {0}")]
    InvalidClientDnsLookup(String),
    #[error("Unrecognised Key: {0}")]
    UnrecognisedKey(String),
    #[error("Key Missing: {0}")]
    MissingKey(String),
    #[error("Invalid Key: {0}")]
    InvalidKey(String),
    #[error("Missing Value: {0}")]
    MissingValue(String),
    #[error("Invalid Value: {0}")]
    InvalidValue(String),
    #[error("Invalid Invalid Value: {0} for: {1}")]
    InvalidValueFor(String, String),
}

pub enum StateStoreError {
    StateStoreFailed(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ConsumerError {

}

pub enum ConsumerSubscriptionError {
    InvalidTopic(String),
    //InvalidPartition(TopicPartition<Partition>),
    //InvalidOffset(i64),
    //InvalidTimestamp(i64),
    //InvalidTime(NaiveTime),
    //InvalidMetadata(String),
}

pub enum ConsumerAssignmentError {
    InvalidTopic(String),
    InvalidPartition(TopicPartition<Partition>),
    InvalidOffset(i64),
    //InvalidTimestamp(i64),
    //InvalidTime(NaiveTime),
    //InvalidMetadata(String),
}

#[derive(Debug)]
pub enum ConsumerAsyncPollError {
    InvalidOffset(i64),
    AuthorisationFailed,
    AuthenticationFailed,
    FencedInstance,
}

pub enum ConsumerSyncPollError {
    InvalidOffset(i64),
    Wakeup,
    AuthorisationFailed,
    AuthenticationFailed,
    FencedInstance,
}

#[derive(Debug, thiserror::Error)]
pub enum ProducerError {
    #[error("Unknown Error")]
    Unknown, // TODO: This is a placeholder. MUST be removed
    #[error(transparent)]
    Cleanup(CleanupError),
}

#[derive(Debug, thiserror::Error)]
pub enum ProducerSendError {
    #[error("Unknown Error")]
    Unknown, // TODO: This is a placeholder. MUST be removed
}

#[derive(Debug, thiserror::Error)]
pub enum ProducerFlushError {
    #[error("Unknown Error")]
    Unknown, // TODO: This is a placeholder. MUST be removed
}

#[derive(Debug, thiserror::Error)]
pub enum CleanupError {
    #[error("Unknown Error")]
    Unknown // TODO: This is a placeholder. MUST be removed
}

#[derive(Debug, thiserror::Error)]
pub enum ProducerMetadataError {
    #[error("Invalid Topic: {0}")]
    InvalidTopic(String),
    #[error("Invalid Partition: {0}")]
    InvalidPartition(TopicPartition<Partition>),
    #[error("Invalid Offset: {0}")]
    InvalidOffset(i64),
    //InvalidTimestamp(i64),
    //InvalidTime(NaiveTime),
    //InvalidMetadata(String),
}

#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
    #[error("Producer Fenced: {0}")]
    ProducerFenced(String)
}

pub enum KafkaError {
    ProducerError(ProducerError),
    ConsumerError(ConsumerError),
}

impl From<ConsumerError> for KafkaError {
    fn from(value: ConsumerError) -> Self {
        KafkaError::ConsumerError(value)
    }
}

impl From<ProducerError> for KafkaError {
    fn from(value: ProducerError) -> Self {
        KafkaError::ProducerError(value)
    }
}
