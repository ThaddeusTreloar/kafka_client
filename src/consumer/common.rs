use std::{collections::HashSet, time::Duration};

use async_trait::async_trait;

use crate::{
    common::{
        record::{Offset, PartitionOffset, RecordStream},
        topic::{Partition, TopicPartition, TopicPartitionList, TopicPartitionOffsetMap},
    },
    error::ConsumerError,
};

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
    fn assign(&mut self, assignments: TopicPartitionList);
    fn assignment(&self) -> TopicPartitionList;
}

#[cfg(feature = "async_client")]
#[async_trait(?Send)]
pub trait AsyncConsumer<K, V>: Drop {
    async fn poll(&self) -> Result<RecordStream<K, V>, ConsumerError>;
    async fn commit(
        &self,
        commit: ConsumerCommit,
    ) -> Result<TopicPartitionOffsetMap, ConsumerError>;
    async fn set_callback(&mut self, callback: ()); // callback type
    async fn close(self, timeout: Duration);
    async fn seek(&self, seek: ConsumerSeek) -> Result<Offset, ConsumerError>;
}

pub trait SyncConsumer<K, V>: Drop {
    fn poll(&self) -> Result<RecordStream<K, V>, ConsumerError>;
    fn commit(&self, commit: ConsumerCommit) -> Result<TopicPartitionOffsetMap, ConsumerError>;
    fn set_callback(&mut self, callback: ()); // callback type
    fn close(self, timeout: Duration);
    fn wakeup(&self);
    fn seek(&self, seek: ConsumerSeek) -> Result<Offset, ConsumerError>;
}

pub enum CommitBounds {
    All,
    Topics(HashSet<String>),
    Partitions(TopicPartitionList),
    Offsets(TopicPartitionOffsetMap),
}

pub struct ConsumerCommit {
    timeout: Option<Duration>,
    bounds: CommitBounds,
}

impl ConsumerCommit {
    fn new() -> ConsumerCommit {
        ConsumerCommit {
            timeout: None,
            bounds: CommitBounds::All,
        }
    }

    fn with_timeout(self, timeout: Duration) -> Self {
        ConsumerCommit {
            timeout: Some(timeout),
            bounds: self.bounds,
        }
    }

    fn with_topics(self, bounds: HashSet<String>) -> Self {
        ConsumerCommit {
            timeout: self.timeout,
            bounds: CommitBounds::Topics(bounds),
        }
    }

    fn with_partitions(self, bounds: TopicPartitionList) -> Self {
        ConsumerCommit {
            timeout: self.timeout,
            bounds: CommitBounds::Partitions(bounds),
        }
    }

    fn with_offsets(self, bounds: TopicPartitionOffsetMap) -> Self {
        ConsumerCommit {
            timeout: self.timeout,
            bounds: CommitBounds::Offsets(bounds),
        }
    }
}

pub struct ConsumerSeek {
    topic_partition: TopicPartition,
    offset: PartitionOffset,
    metadata: Option<()>, // TODO: Figure out type
}

impl ConsumerSeek {
    fn new(topic_partition: TopicPartition, offset: PartitionOffset) -> Self {
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

/*

long position(TopicPartition partition);


long position(TopicPartition partition, final Duration timeout);

OffsetAndMetadata committed(TopicPartition partition);


OffsetAndMetadata committed(TopicPartition partition, final Duration timeout);


Map<TopicPartition, OffsetAndMetadata> committed(Set<TopicPartition> partitions);


Map<TopicPartition, OffsetAndMetadata> committed(Set<TopicPartition> partitions, final Duration timeout);


Map<MetricName, ? extends Metric> metrics();


List<PartitionInfo> partitionsFor(String topic);


List<PartitionInfo> partitionsFor(String topic, Duration timeout);


Map<String, List<PartitionInfo>> listTopics();


Map<String, List<PartitionInfo>> listTopics(Duration timeout);


Set<TopicPartition> paused();


void pause(Collection<TopicPartition> partitions);


void resume(Collection<TopicPartition> partitions);


Map<TopicPartition, OffsetAndTimestamp> offsetsForTimes(Map<TopicPartition, Long> timestampsToSearch);


Map<TopicPartition, OffsetAndTimestamp> offsetsForTimes(Map<TopicPartition, Long> timestampsToSearch, Duration timeout);


Map<TopicPartition, Long> beginningOffsets(Collection<TopicPartition> partitions);


Map<TopicPartition, Long> beginningOffsets(Collection<TopicPartition> partitions, Duration timeout);


Map<TopicPartition, Long> endOffsets(Collection<TopicPartition> partitions);


Map<TopicPartition, Long> endOffsets(Collection<TopicPartition> partitions, Duration timeout);


OptionalLong currentLag(TopicPartition topicPartition);


ConsumerGroupMetadata groupMetadata();


void enforceRebalance();


void enforceRebalance(final String reason);

*/
