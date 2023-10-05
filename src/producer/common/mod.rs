use std::{collections::HashMap, fmt::Display, time::Duration, pin::Pin};

use async_trait::async_trait;
use futures::{Stream, Sink, SinkExt};

use crate::{
    common::{
        record::{Offset, Record, RecordSet, RecordStream},
        topic::{
            OptionalPartition, Partition, TopicPartition, TopicPartitionList,
            TopicPartitionMetadataMap,
        },
    },
    consumer::common::{ConsumerGroupMetadata, AssignedConsumer, SubscriberConsumer},
    error::{
        CleanupError, ProducerError, ProducerMetadataError, ProducerSendError, TransactionError,
    },
    metadata::metrics::{Metric, MetricId},
};

use super::record::ProducerRecord;

mod ergo_test;

struct Transactional;
struct NonTransactional;
pub struct Transaction {}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Transaction")
    }
}

impl Transaction {
    pub fn new() -> Self {
        Self {}
    }

    pub fn abort_transaction(self) -> Result<(), TransactionError> {
        Ok(())
    }

    pub fn commit_transaction(self) -> Result<(), TransactionError> {
        Ok(())
    }

    pub fn push_offsets(
        &self,
        offsets: TopicPartitionMetadataMap<Offset>,
        group_metadata: ConsumerGroupMetadata,
    ) -> Result<TopicPartitionMetadataMap<Offset>, TransactionError> {
        Ok(offsets)
    }
}

#[async_trait]
pub trait TransactionalProducer<K, V> {

    // Returns an error if a transaction is already in progress
    fn begin_transaction(&self) -> Result<Transaction, TransactionError>;
    //fn begin_transaction<CK, CV>(&self, consumer: &impl SubscriberConsumer<CK, CV>) -> Result<(Transaction, &mut Self), TransactionError>;
}



pub trait Producer<K, V>: Drop {
    fn flush(&self);

    fn send_sync(
        &self,
        record: ProducerRecord<K, V>,
    ) -> Result<ProducerRecord<K, V>, ProducerSendError>; // TODO: investigate using Ok(RecordMetadata). What are the fields of the Java implementation of RecordMetadata?

}

#[cfg(feature = "async_client")]
#[async_trait]
pub trait AsyncProducer<K, V>: Drop + Sink<ProducerRecord<K, V>> {

    //async fn send(
    //    &self,
    //    record: ProducerRecord<K, V>,
    //) -> Result<ProducerRecord<K, V>, ProducerSendError>; // TODO: investigate using Ok(RecordMetadata). What are the fields of the Java implementation of RecordMetadata?

    async fn send_with_callback<F>(
        &self,
        record: ProducerRecord<K, V>,
        cb: F,
    ) -> Result<ProducerRecord<K, V>, ProducerSendError>
    where
        F: Fn(
                Result<ProducerRecord<K, V>, ProducerSendError>,
            ) -> Result<ProducerRecord<String, String>, ProducerSendError>
            + Send;

    async fn send_set(
        &self,
        records: RecordSet<ProducerRecord<K, V>>,
    ) -> RecordSet<Result<ProducerRecord<K, V>, ProducerError>>;

    //async fn send_stream(
    //    &self,
    //    records: RecordStream<ProducerRecord<K, V>>,
    //) -> Pin<Box<dyn Stream<Item = Result<ProducerRecord<String, String>, ProducerSendError>>+ Send +'life0>>;

    // Is this the best place for this?
    async fn get_partitions_for_topic(
        &self,
        topic: String,
    ) -> Result<TopicPartitionList<Partition>, ProducerMetadataError>;

    fn close(self, timeout: Duration) -> Result<(), CleanupError>;
    fn metrics(&self) -> Result<HashMap<MetricId, Metric>, CleanupError>;
}

