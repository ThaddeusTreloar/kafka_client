use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    time::{Duration, Instant},
};

use async_trait::async_trait;
use chrono::NaiveTime;
use serde::Deserialize;

use crate::{
    common::{
        record::{Offset, Position, RecordMetadata, RecordStream, RecordSet, Record},
        topic::{
            OptionalPartition, Partition, TopicPartition, TopicPartitionList,
            TopicPartitionMetadataMap,
        },
    },
    error::{ConsumerError, ConsumerSubscriptionError, ConsumerAssignmentError, ConsumerSyncPollError, ConsumerAsyncPollError},
};

use super::{config::ConsumerConfig, record::ConsumerRecord};

pub struct ConsumerGroupMetadata {
    group_id: String,
    group_instance_id: Option<String>,
    generation_id: i32,
    member_id: String,
}

impl ConsumerGroupMetadata {
    pub fn builder() -> ConsumerGroupMetadataBuilder<Option<String>, Option<i32>, Option<String>> {
        ConsumerGroupMetadataBuilder {
            group_instance_id: None,
            group_id: None,
            generation_id: None,
            member_id: None,
        }
    }
}

impl From<ConsumerGroupMetadataBuilder<String, i32, String>> for ConsumerGroupMetadata {
    fn from(builder: ConsumerGroupMetadataBuilder<String, i32, String>) -> Self {
        Self {
            group_id: builder.group_id,
            group_instance_id: builder.group_instance_id,
            generation_id: builder.generation_id,
            member_id: builder.member_id,
        }
    }
}

pub struct ConsumerGroupMetadataBuilder<GID, GENID, MID> {
    group_id: GID,
    group_instance_id: Option<String>,
    generation_id: GENID,
    member_id: MID,
}

impl <GID, GENID, MID> ConsumerGroupMetadataBuilder<GID, GENID, MID> {
    pub fn new() -> ConsumerGroupMetadataBuilder<Option<String>, Option<i32>, Option<String>> {
        ConsumerGroupMetadataBuilder {
            group_instance_id: None,
            group_id: None,
            generation_id: None,
            member_id: None,
        }
    }

    pub fn with_group_instance_id(self, group_instance_id: String) -> Self {
        Self {
            group_id: self.group_id,
            group_instance_id: Some(group_instance_id),
            generation_id: self.generation_id,
            member_id: self.member_id,
        }
    }

    pub fn with_group_id(self, group_id: String) -> ConsumerGroupMetadataBuilder<String, GENID, MID> {
        ConsumerGroupMetadataBuilder {
            group_id,
            group_instance_id: self.group_instance_id,
            generation_id: self.generation_id,
            member_id: self.member_id,
        }
    }

    pub fn with_generation_id(self, generation_id: i32) -> ConsumerGroupMetadataBuilder<GID, i32, MID> {
        ConsumerGroupMetadataBuilder {
            group_id: self.group_id,
            group_instance_id: self.group_instance_id,
            generation_id,
            member_id: self.member_id,
        }
    }

    pub fn with_member_id(self, member_id: String) -> ConsumerGroupMetadataBuilder<GID, GENID, String> {
        ConsumerGroupMetadataBuilder {
            group_id: self.group_id,
            group_instance_id: self.group_instance_id,
            generation_id: self.generation_id,
            member_id,
        }
    }
}

impl ConsumerGroupMetadataBuilder<String, i32, String> {
    pub fn into_consumer_group_metadata(self) -> ConsumerGroupMetadata {
        self.into()
    }
}

pub struct SubscriberConsumerType;
pub struct AssignedConsumerType;

pub trait SubscriberConsumer<K, V>: Sized {
    fn new(config: ConsumerConfig) -> Result<Self, ConsumerError>;
    fn new_subscribed(config: ConsumerConfig, subscriptions: HashSet<String>) -> Result<Self, ConsumerError>;

    fn subscribe(&mut self, subscriptions: HashSet<String>);
    //fn subscribe_with_callback(&mut self, subscriptions: HashSet<String>, callback: ()); // callback type
    fn subscription(&self) -> HashSet<String>;
    fn unsubscribe(&mut self) -> Result<(), ConsumerSubscriptionError>;
    //fn subscribe_to_pattern_with_callback(Pattern pattern, ConsumerRebalanceListener callback);
    //fn subscribe_to_pattern(Pattern pattern);
}

pub trait AssignedConsumer<K, V>: Sized {
    fn new(config: ConsumerConfig) -> Result<Self, ConsumerError>;
    fn new_assigned(config: ConsumerConfig, subscriptions: HashSet<String>) -> Result<Self, ConsumerError>;


    fn assign(
        &mut self,
        assignments: TopicPartitionList<OptionalPartition>,
    ) -> TopicPartitionList<Partition>;
    fn assignment(&self) -> TopicPartitionList<Partition>;
    fn unassign(&mut self) -> Result<(), ConsumerAssignmentError>;
}

#[cfg(feature = "async_client")]
#[async_trait(?Send)]
pub trait AsyncConsumer<'a, K, V>: Drop 
where K: Deserialize<'a>,
      V: Deserialize<'a>,
{
    async fn poll(&self) -> Result<RecordSet<ConsumerRecord<K, V>>, ConsumerAsyncPollError>;
    fn stream(&self) -> RecordStream<ConsumerRecord<K, V>>;
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
        topic_partition: TopicPartition<OptionalPartition>,
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
        partition: TopicPartitionList<OptionalPartition>,
        timeout: Option<Duration>,
    ) -> Result<Offset, ConsumerError>;
    async fn committed(
        &self,
        partition: TopicPartitionList<OptionalPartition>,
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
        partitions: TopicPartitionList<OptionalPartition>,
        timeout: Option<Duration>,
    ) -> Result<TopicPartitionMetadataMap<Offset>, ConsumerError>;
    async fn end(
        &self,
        partitions: TopicPartitionList<OptionalPartition>,
        timeout: Option<Duration>,
    ) -> Result<TopicPartitionMetadataMap<Offset>, ConsumerError>;
    async fn metric(&self) -> HashSet<(), ()>; // determine type
    async fn pause(
        &self,
        partitions: TopicPartitionList<OptionalPartition>,
    ) -> Result<(), ConsumerError>;
    async fn paused(&self) -> TopicPartitionList<OptionalPartition>;
    async fn resume(
        &self,
        partitions: TopicPartitionList<OptionalPartition>,
    ) -> TopicPartitionList<Partition>;
    async fn offset_at_timestamp(
        &self,
        partition_times: TopicPartitionMetadataMap<NaiveTime>,
        timeout: Duration,
    ) -> TopicPartitionMetadataMap<(Offset, NaiveTime)>;
    async fn current_lag(
        &self,
        partitions: TopicPartitionList<OptionalPartition>,
    ) -> HashMap<TopicPartition<Partition>, Duration>;
    async fn group_metadata(&self) -> Result<ConsumerGroupMetadata, ConsumerError>;
    async fn enforce_rebalance(&self, reason: Option<&str>);
}

pub trait SyncConsumer<'a, K, V>: Drop
where K: Deserialize<'a>,
      V: Deserialize<'a>,
{
    fn poll(&self, timeout: Duration) -> Result<RecordStream<ConsumerRecord<K, V>>, ConsumerSyncPollError>;
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
        topic_partition: TopicPartition<OptionalPartition>,
        offset: Position,
        metadata: Option<RecordMetadata>,
    ) -> Result<Offset, ConsumerError>;
    fn seek_map(&self, map: TopicPartitionMetadataMap<Offset>) -> Result<Offset, ConsumerError>; // Is this type unweildy or complicated?
    fn seek_obj(&self, seek: ConsumerSeek) -> Result<Offset, ConsumerError>;
    fn position(
        &self,
        partition: TopicPartitionList<OptionalPartition>,
        timeout: Option<Duration>,
    ) -> Result<Offset, ConsumerError>;
    fn committed(
        &self,
        partition: TopicPartitionList<OptionalPartition>,
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
        partitions: TopicPartitionList<OptionalPartition>,
        timeout: Option<Duration>,
    ) -> Result<TopicPartitionMetadataMap<Offset>, ConsumerError>;
    fn end(
        &self,
        partitions: TopicPartitionList<OptionalPartition>,
        timeout: Option<Duration>,
    ) -> Result<TopicPartitionMetadataMap<Offset>, ConsumerError>;
    fn metric(&self) -> HashSet<(), ()>; // determine type
    fn pause(&self, partitions: TopicPartitionList<OptionalPartition>) -> Result<(), ConsumerError>;
    fn paused(&self) -> TopicPartitionList<OptionalPartition>;
    fn resume(
        &self,
        partitions: TopicPartitionList<OptionalPartition>,
    ) -> TopicPartitionList<Partition>;
    fn offset_at_timestamp(
        &self,
        partition_times: TopicPartitionMetadataMap<NaiveTime>,
        timeout: Duration,
    ) -> TopicPartitionMetadataMap<(Offset, NaiveTime)>;
    fn current_lag(
        &self,
        partitions: TopicPartitionList<OptionalPartition>,
    ) -> HashMap<TopicPartition<Partition>, Duration>;
    fn group_metadata(&self) -> Result<ConsumerGroupMetadata, ConsumerError>;
    fn enforce_rebalance(&self, reason: Option<&str>);
}

pub enum ConsumerCommit {
    All,
    Topics(HashSet<String>),
    Partitions(TopicPartitionList<OptionalPartition>),
    Offsets(TopicPartitionMetadataMap<Offset>),
}

pub struct ConsumerSeek {
    topic_partition: TopicPartition<OptionalPartition>,
    offset: Position,
    metadata: Option<()>, // TODO: Figure out type
}

impl ConsumerSeek {
    fn new(topic_partition: TopicPartition<OptionalPartition>, offset: Position) -> Self {
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

mod consumer_test {
    use std::{time::Duration, collections::{HashSet, HashMap}, fmt::Debug};

    use chrono::NaiveTime;
    use futures::{StreamExt, Future, task::SpawnExt};
    use serde::Deserialize;

    use crate::{common::{record::{RecordStream, Position, RecordMetadata, Offset, RecordSet, Record}, topic::{TopicPartitionMetadataMap, TopicPartition, OptionalPartition, TopicPartitionList, Partition}}, error::{ConsumerAsyncPollError, ConsumerError, ConsumerSubscriptionError, ConsumerAssignmentError, self}, consumer::{config::{ConsumerConfig, ConsumerProperty}, record::ConsumerRecord}, config::{ClientProperty, ClientDnsLookup}, producer::{self, record::{ProducerRecordBuilder, ProducerRecord}}};

    use super::{ConsumerCommit, ConsumerSeek, ConsumerGroupMetadata, SubscriberConsumer, AssignedConsumer, AssignedConsumerType, SubscriberConsumerType, AsyncConsumer};

    struct TestConsumer<K, V, M> {
        key_type: std::marker::PhantomData<K>,
        value_type: std::marker::PhantomData<V>,
        partition_method: std::marker::PhantomData<M>,
    }

    impl <K, V, M> Drop for TestConsumer<K, V, M> {
        fn drop(&mut self) {
            println!("Closing TestConsumer");
        }
    }

    #[async_trait::async_trait(?Send)]
    impl <'a, K, V, M> AsyncConsumer<'a, K, V> for TestConsumer<K, V, M>
    where K: Deserialize<'a>,
          V: Deserialize<'a>,
    {
        async fn poll(&self) -> Result<RecordSet<ConsumerRecord<K, V>>, ConsumerAsyncPollError> {
            let key = serde_json::from_str("").unwrap();
            let val = serde_json::from_str("").unwrap();

            let new_record = ConsumerRecord::from_key_value(key, val)
                .with_topic_partition(TopicPartition::new_partitioned("", 0))
                .with_header(String::new(), String::new())
                .with_offset(0)
                .with_timestamp(0)
                .into_record();

            Ok((vec![new_record]).into())
        }
        
        fn stream(&self) -> RecordStream<ConsumerRecord<K, V>> {
            unimplemented!()
        }

        async fn commit(
            &self,
            commit: ConsumerCommit,
            timeout: Option<Duration>,
        ) -> Result<TopicPartitionMetadataMap<Offset>, ConsumerError>  {
            unimplemented!()
        }

        async fn set_callback(&mut self, callback: ()) {
            unimplemented!()
        } // callback type
        async fn wakeup(&self) {
            unimplemented!()
        }
        async fn close(self, timeout: Duration) {
            unimplemented!()
        }
        // Find our whether seek or seek_obj is ore ergonomic
        async fn seek(
            &self,
            topic_partition: TopicPartition<OptionalPartition>,
            offset: Position,
            metadata: Option<RecordMetadata>,
        ) -> Result<Offset, ConsumerError> {
            unimplemented!()
        }
        async fn seek_map(
            &self,
            map: TopicPartitionMetadataMap<Offset>,
        ) -> Result<Offset, ConsumerError> {
            unimplemented!()
        } // Is this type unweildy or complicated?
        async fn seek_obj(&self, seek: ConsumerSeek) -> Result<Offset, ConsumerError> {
            unimplemented!()
        }
        async fn position(
            &self,
            partition: TopicPartitionList<OptionalPartition>,
            timeout: Option<Duration>,
        ) -> Result<Offset, ConsumerError> {
            unimplemented!()
        }
        async fn committed(
            &self,
            partition: TopicPartitionList<OptionalPartition>,
            timeout: Option<Duration>,
        ) -> Result<(Offset, RecordMetadata), ConsumerError> {
            unimplemented!()
        }
        async fn partitions(
            &self,
            partition: HashSet<String>,
            timeout: Option<Duration>,
        ) -> Result<TopicPartitionList<Partition>, ConsumerError> {
            unimplemented!()
        }
        async fn topics(&self, timeout: Option<Duration>) -> HashSet<String> {
            unimplemented!()
        }
        async fn beginning(
            &self,
            partitions: TopicPartitionList<OptionalPartition>,
            timeout: Option<Duration>,
        ) -> Result<TopicPartitionMetadataMap<Offset>, ConsumerError> {
            unimplemented!()
        }
        async fn end(
            &self,
            partitions: TopicPartitionList<OptionalPartition>,
            timeout: Option<Duration>,
        ) -> Result<TopicPartitionMetadataMap<Offset>, ConsumerError> {
            unimplemented!()
        }
        async fn metric(&self) -> HashSet<(), ()> {
            unimplemented!()
        } // determine type
        async fn pause(
            &self,
            partitions: TopicPartitionList<OptionalPartition>,
        ) -> Result<(), ConsumerError> {
            unimplemented!()
        }
        async fn paused(&self) -> TopicPartitionList<OptionalPartition> {
            unimplemented!()
        }
        async fn resume(
            &self,
            partitions: TopicPartitionList<OptionalPartition>,
        ) -> TopicPartitionList<Partition> {
            unimplemented!()
        }
        async fn offset_at_timestamp(
            &self,
            partition_times: TopicPartitionMetadataMap<NaiveTime>,
            timeout: Duration,
        ) -> TopicPartitionMetadataMap<(Offset, NaiveTime)> {
            unimplemented!()
        }
        async fn current_lag(
            &self,
            partitions: TopicPartitionList<OptionalPartition>,
        ) -> HashMap<TopicPartition<Partition>, Duration> {
            unimplemented!()
        }
        async fn group_metadata(&self) -> Result<ConsumerGroupMetadata, ConsumerError> {
            unimplemented!()
        }
        async fn enforce_rebalance(&self, reason: Option<&str>) {
            unimplemented!()
        }
    }

    impl <K, V> SubscriberConsumer<K, V> for TestConsumer<K, V, SubscriberConsumerType> {
        fn new(config: crate::consumer::config::ConsumerConfig) -> Result<Self, ConsumerError> {
            Ok(Self {
                key_type: std::marker::PhantomData,
                value_type: std::marker::PhantomData,
                partition_method: std::marker::PhantomData,
            })
        }

        fn new_subscribed(config: crate::consumer::config::ConsumerConfig, subscriptions: HashSet<String>) -> Result<Self, ConsumerError> {
            unimplemented!()
        }

        fn subscribe(&mut self, subscriptions: HashSet<String>) {
            unimplemented!()
        }
        //fn subscribe_with_callback(&mut self, subscriptions: HashSet<String>, callback: ()); // callback type
        fn subscription(&self) -> HashSet<String> {
            unimplemented!()
        }

        fn unsubscribe(&mut self) -> Result<(), ConsumerSubscriptionError> {
            unimplemented!()
        }
        //fn subscribe_to_pattern_with_callback(Pattern pattern, ConsumerRebalanceListener callback);
        //fn subscribe_to_pattern(Pattern pattern);
    }

    impl <K, V> AssignedConsumer<K, V> for TestConsumer<K, V, AssignedConsumerType> {
        fn new(config: crate::consumer::config::ConsumerConfig) -> Result<Self, ConsumerError> {
            unimplemented!()
        }

        fn new_assigned(config: ConsumerConfig, subscriptions: HashSet<String>) -> Result<Self, ConsumerError> {
            unimplemented!()
        }

        fn assign(
            &mut self,
            assignments: TopicPartitionList<OptionalPartition>,
        ) -> TopicPartitionList<Partition> {
            unimplemented!()
        }

        fn assignment(&self) -> TopicPartitionList<Partition> {
            unimplemented!()
        }

        fn unassign(&mut self) -> Result<(), ConsumerAssignmentError> {
            unimplemented!()
        }
    }

    async fn return_val<K, V>(r: ConsumerRecord<K, V>) -> Option<V> 
    where V: Clone
    {
        match r.value() {
            Some(val) => Some(val.clone()),
            None => None,
        }
    }

    async fn print_val<V>(v: V) 
    where V: Debug
    {
        println!("Value: {:?}", v);
    }

    #[tokio::test]
    async fn test_consumer_drop() {
        let mut config = ConsumerConfig::default();

        config.push_client_prop(
            ClientProperty::ClientDnsLookup(
                ClientDnsLookup::ResolveCanonicalBootstrapServersOnly
            )
        );

        let consumer = match TestConsumer::<String, String, SubscriberConsumerType>::new(config.clone()) {
            Ok(consumer) => consumer,
            Err(e) => panic!("Error creating consumer: {}", e),
        };
    }

    async fn test_consumer() {
        let mut config = ConsumerConfig::default();

        config.push_client_prop(
            ClientProperty::ClientDnsLookup(
                ClientDnsLookup::ResolveCanonicalBootstrapServersOnly
            )
        );

        let mut some_consumer = match TestConsumer::<String, String, SubscriberConsumerType>::new(config.clone()) {
            Ok(consumer) => consumer,
            Err(_) => panic!("Error creating consumer"),
        };

        some_consumer.subscribe(HashSet::new());

        let immutable_consumer: TestConsumer<String, String, SubscriberConsumerType> 
        = match TestConsumer::new_subscribed(config, HashSet::new()) {
            Ok(consumer) => consumer,
            Err(_) => panic!("Error creating consumer"),
        };

        loop {
            match some_consumer.poll().await {
                Ok(records) => for record in records {
                    println!("Record: {:?}", record);
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }

        immutable_consumer.stream()
            .filter_map(return_val)
            .for_each(print_val)
            .await;

        let x = ConsumerRecord::from_key_value("", "")
            .with_topic_partition(TopicPartition::new_partitioned("", 0))
            .with_header(String::new(), String::new());

        let consumer_record = ConsumerRecord::from_key_value("", "")
            .with_topic_partition(TopicPartition::new_partitioned("", 0))
            .with_header(String::new(), String::new())
            .with_offset(0)
            .with_timestamp(0)
            .into_record();

        let producer_record = ProducerRecord::from_key_value("", "")
            .with_topic_partition(TopicPartition::new_partitioned("", 0))
            .with_header(String::new(), String::new())
            .with_timestamp(0)
            .into_record();
    }
}