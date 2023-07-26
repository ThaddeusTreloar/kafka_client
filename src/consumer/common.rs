use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    time::{Duration, Instant},
};

use async_trait::async_trait;
use chrono::NaiveTime;

use crate::{
    common::{
        record::{Offset, Position, RecordMetadata, RecordStream},
        topic::{
            MaybePartition, Partition, TopicPartition, TopicPartitionList,
            TopicPartitionMetadataMap,
        },
    },
    error::ConsumerError,
};

pub struct ConsumerGroupMetadata {}

pub struct UnallocatedConsumer;
pub struct SubscriberConsumer;
pub struct AssignedConsumer;

pub trait UnallocatedConsumerTrait<K, V> {
    fn subscribe(self) -> Box<dyn SubscriberConsumerTrait<K, V>>;
    fn assign(self) -> Box<dyn AssignedConsumerTrait<K, V>>;
}
pub trait SubscriberConsumerTrait<K, V>: SyncConsumer<K, V> {
    fn subscribe(&mut self, subscriptions: HashSet<String>);
    //fn subscribe_with_callback(&mut self, subscriptions: HashSet<String>, callback: ()); // callback type
    fn subscription(&self) -> HashSet<String>;
    fn unsubscribe(&mut self) -> Result<(), ConsumerError>;
    //fn subscribe_to_pattern_with_callback(Pattern pattern, ConsumerRebalanceListener callback);
    //fn subscribe_to_pattern(Pattern pattern);
}
pub trait AssignedConsumerTrait<K, V>: SyncConsumer<K, V> {
    fn assign(
        &mut self,
        assignments: TopicPartitionList<MaybePartition>,
    ) -> TopicPartitionList<Partition>;
    fn assignment(&self) -> TopicPartitionList<Partition>;
}

#[cfg(feature = "async_client")]
#[async_trait(?Send)]
pub trait AsyncConsumer<K, V>: Drop {
    async fn poll(&self) -> Result<RecordStream<K, V>, ConsumerError>;
    async fn commit(
        &self,
        commit: ConsumerCommit,
        timeout: Option<Duration>,
    ) -> Result<TopicPartitionMetadataMap<Offset>, ConsumerError>;
    async fn set_callback(&mut self, callback: ()); // callback type
    async fn wakeup(&self);
    async fn close(self, timeout: Duration);
    // Find our whether seek or seek_obj is ore ergonomic
    async fn seek(
        &self,
        topic_partition: TopicPartition<MaybePartition>,
        offset: Position,
        metadata: Option<RecordMetadata>,
    ) -> Result<Offset, ConsumerError>;
    async fn seek_map(
        &self,
        map: TopicPartitionMetadataMap<Offset>,
    ) -> Result<Offset, ConsumerError>; // Is this type unweildy or complicated?
    async fn seek_obj(&self, seek: ConsumerSeek) -> Result<Offset, ConsumerError>;
    async fn position(
        &self,
        partition: TopicPartitionList<MaybePartition>,
        timeout: Option<Duration>,
    ) -> Result<Offset, ConsumerError>;
    async fn committed(
        &self,
        partition: TopicPartitionList<MaybePartition>,
        timeout: Option<Duration>,
    ) -> Result<(Offset, RecordMetadata), ConsumerError>;
    async fn partitions(
        &self,
        partition: HashSet<String>,
        timeout: Option<Duration>,
    ) -> Result<TopicPartitionList<Partition>, ConsumerError>;
    async fn topics(&self, timeout: Option<Duration>) -> HashSet<String>;
    async fn beginning(
        &self,
        partitions: TopicPartitionList<MaybePartition>,
        timeout: Option<Duration>,
    ) -> Result<TopicPartitionMetadataMap<Offset>, ConsumerError>;
    async fn end(
        &self,
        partitions: TopicPartitionList<MaybePartition>,
        timeout: Option<Duration>,
    ) -> Result<TopicPartitionMetadataMap<Offset>, ConsumerError>;
    async fn metric(&self) -> HashSet<(), ()>; // determine type
    async fn pause(
        &self,
        partitions: TopicPartitionList<MaybePartition>,
    ) -> Result<(), ConsumerError>;
    async fn paused(&self) -> TopicPartitionList<MaybePartition>;
    async fn resume(
        &self,
        partitions: TopicPartitionList<MaybePartition>,
    ) -> TopicPartitionList<Partition>;
    async fn offset_at_timestamp(
        &self,
        partition_times: TopicPartitionMetadataMap<NaiveTime>,
        timeout: Duration,
    ) -> TopicPartitionMetadataMap<(Offset, NaiveTime)>;
    async fn current_lag(
        &self,
        partitions: TopicPartitionList<MaybePartition>,
    ) -> HashMap<TopicPartition<Partition>, Duration>;
    async fn group_metadata(&self) -> Result<ConsumerGroupMetadata, ConsumerError>;
    async fn enforce_rebalance(&self, reason: Option<&str>);
}

pub trait SyncConsumer<K, V>: Drop {
    fn poll(&self) -> Result<RecordStream<K, V>, ConsumerError>;
    fn commit(
        &self,
        commit: ConsumerCommit,
        timeout: Option<Duration>,
    ) -> Result<TopicPartitionMetadataMap<Offset>, ConsumerError>;
    fn set_callback(&mut self, callback: ()); // callback type
    fn close(self, timeout: Duration);
    fn wakeup(&self);
    fn seek(
        &self,
        topic_partition: TopicPartition<MaybePartition>,
        offset: Position,
        metadata: Option<RecordMetadata>,
    ) -> Result<Offset, ConsumerError>;
    fn seek_map(&self, map: TopicPartitionMetadataMap<Offset>) -> Result<Offset, ConsumerError>; // Is this type unweildy or complicated?
    fn seek_obj(&self, seek: ConsumerSeek) -> Result<Offset, ConsumerError>;
    fn position(
        &self,
        partition: TopicPartitionList<MaybePartition>,
        timeout: Option<Duration>,
    ) -> Result<Offset, ConsumerError>;
    fn committed(
        &self,
        partition: TopicPartitionList<MaybePartition>,
        timeout: Option<Duration>,
    ) -> Result<(Offset, RecordMetadata), ConsumerError>;
    fn partitions(
        &self,
        partition: HashSet<String>,
        timeout: Option<Duration>,
    ) -> Result<TopicPartitionList<Partition>, ConsumerError>;
    fn topics(&self, timeout: Option<Duration>) -> HashSet<String>;
    fn beginning(
        &self,
        partitions: TopicPartitionList<MaybePartition>,
        timeout: Option<Duration>,
    ) -> Result<TopicPartitionMetadataMap<Offset>, ConsumerError>;
    fn end(
        &self,
        partitions: TopicPartitionList<MaybePartition>,
        timeout: Option<Duration>,
    ) -> Result<TopicPartitionMetadataMap<Offset>, ConsumerError>;
    fn metric(&self) -> HashSet<(), ()>; // determine type
    fn pause(&self, partitions: TopicPartitionList<MaybePartition>) -> Result<(), ConsumerError>;
    fn paused(&self) -> TopicPartitionList<MaybePartition>;
    fn resume(
        &self,
        partitions: TopicPartitionList<MaybePartition>,
    ) -> TopicPartitionList<Partition>;
    fn offset_at_timestamp(
        &self,
        partition_times: TopicPartitionMetadataMap<NaiveTime>,
        timeout: Duration,
    ) -> TopicPartitionMetadataMap<(Offset, NaiveTime)>;
    fn current_lag(
        &self,
        partitions: TopicPartitionList<MaybePartition>,
    ) -> HashMap<TopicPartition<Partition>, Duration>;
    fn group_metadata(&self) -> Result<ConsumerGroupMetadata, ConsumerError>;
    fn enforce_rebalance(&self, reason: Option<&str>);
}

pub enum ConsumerCommit {
    All,
    Topics(HashSet<String>),
    Partitions(TopicPartitionList<MaybePartition>),
    Offsets(TopicPartitionMetadataMap<Offset>),
}

pub struct ConsumerSeek {
    topic_partition: TopicPartition<MaybePartition>,
    offset: Position,
    metadata: Option<()>, // TODO: Figure out type
}

impl ConsumerSeek {
    fn new(topic_partition: TopicPartition<MaybePartition>, offset: Position) -> Self {
        ConsumerSeek {
            topic_partition,
            offset,
            metadata: None,
        }
    }

    fn with_metadata(self, metadata: ()) -> Self {
        ConsumerSeek {
            topic_partition: self.topic_partition,
            offset: self.offset,
            metadata: Some(metadata),
        }
    }
}
