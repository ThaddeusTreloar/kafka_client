use std::marker::PhantomData;

use chrono::offset;
use futures::Stream;
use serde::{Deserialize, Serialize};

use crate::common::{
    record::{Record, Offset},
    topic::{OptionalPartition, Partition, TopicPartition},
};

pub type RecordMetadata = ();

#[derive(Debug)]
pub struct ConsumerRecord<K, V> {
    headers: Vec<(String, String)>,
    key: Option<K>,
    offset: Offset,
    timestamp: i64,
    topic_partition: TopicPartition<Partition>,
    value: Option<V>,
}

impl<K, V> ConsumerRecord<K, V> {
    /* TODO: Explore changing these to impl Into<T> instead.
    Originally had it set out this way however the compiler was then unable to infer
    types for the record. eg:

       // Unable to infer type
       let record = ConsumerRecordBuilder::from_key_value("key", "value")
           .with_topic("topic")
           .with_partition(0)
           .with_offset(0)
           .with_timestamp(0)
           .into_record();

    you would then have to either define the type of the consumer record:

       let record: ConsumerRecord<String, String> = ConsumerRecordBuilder::from_key_value("key", "value")
           .with_topic("topic")
           .with_partition(0)
           .with_offset(0)
           .with_timestamp(0)
           .into_record();

    or, give the full type when declaring the builder, having to provide all type args at once
       let record = ConsumerRecordBuilder<
           String,
           String,
           Option<TopicPartition<OptionalPartition>>,
           Option<Offset>,
           Option<i64>,
       >::from_key_value("key", "value")
               .with_topic("topic")
               .with_partition(0)
               .with_offset(0)
               .with_timestamp(0)
               .into_record();

    We will have to wait
    */

    // TODO: Check if this is appropriate. May lead to confusion as the from_* function is returning a type, different from the associated struct
    pub fn from_key(key: K) -> ConsumerRecordBuilder<
        K,
        V,
        Option<TopicPartition<OptionalPartition>>,
        Option<Offset>,
        Option<i64>,
    > {
        ConsumerRecordBuilder {
            headers: Vec::new(),
            key: Some(key),
            offset: None,
            timestamp: None,
            topic_partition: None,
            value: None,
        }
    }

    // TODO: Check if this is appropriate. May lead to confusion as the from_* function is returning a type, different from the associated struct
    pub fn from_value(value: V) -> ConsumerRecordBuilder<
        K,
        V,
        Option<TopicPartition<OptionalPartition>>,
        Option<Offset>,
        Option<i64>,
    > {
        ConsumerRecordBuilder {
            headers: Vec::new(),
            key: None,
            offset: None,
            timestamp: None,
            topic_partition: None,
            value: Some(value),
        }
    }

    // TODO: Check if this is appropriate. May lead to confusion as the from_* function is returning a type, different from the associated struct
    pub fn from_key_value(key: K, value: V) -> ConsumerRecordBuilder<
        K,
        V,
        Option<TopicPartition<OptionalPartition>>,
        Option<Offset>,
        Option<i64>,
    > {
        ConsumerRecordBuilder {
            headers: Vec::new(),
            key: Some(key),
            offset: None,
            timestamp: None,
            topic_partition: None,
            value: Some(value),
        }
    }
}

impl <K, V> Record<K, V> for ConsumerRecord<K, V> {
    fn key(&self) -> Option<&K> {
        if let Some(key) = &self.key {
            Some(&key)
        } else {
            None
        }
    }

    fn value(&self) -> Option<&V> {
        if let Some(val) = &self.value {
            Some(&val)
        } else {
            None
        }
    }

    fn timestamp(&self) -> Option<i64> {
        Some(self.timestamp)
    }

    fn topic(&self) -> &str {
        self.topic_partition.topic()
    }
}

impl<K, V> From<ConsumerRecordBuilder<K, V, TopicPartition<Partition>, Offset, i64>>
    for ConsumerRecord<K, V>
{
    fn from(builder: ConsumerRecordBuilder<K, V, TopicPartition<Partition>, Offset, i64>) -> Self {
        ConsumerRecord {
            headers: builder.headers,
            key: builder.key,
            offset: builder.offset,
            timestamp: builder.timestamp,
            topic_partition: builder.topic_partition,
            value: builder.value,
        }
    }
}

#[derive(Debug)]
pub struct ConsumerRecordBuilder<K, V, TP, OFS, TS> {
    headers: Vec<(String, String)>,
    key: Option<K>,
    offset: OFS,
    timestamp: TS,
    topic_partition: TP,
    value: Option<V>,
}

impl<K, V> ConsumerRecordBuilder<K, V, TopicPartition<Partition>, Offset, i64> {
    pub fn into_record(self) -> ConsumerRecord<K, V> {
        self.into()
    }
}

impl<K, V, TP, OFS, TS> ConsumerRecordBuilder<K, V, TP, OFS, TS> {
    pub fn key(&self) -> Option<&K> {
        if let Some(key) = &self.key {
            Some(&key)
        } else {
            None
        }
    }

    pub fn value(&self) -> Option<&V> {
        if let Some(val) = &self.value {
            Some(&val)
        } else {
            None
        }
    }

    pub fn with_topic<T>(
        self,
        topic: impl Into<TopicPartition<OptionalPartition>>,
    ) -> ConsumerRecordBuilder<K, V, TopicPartition<OptionalPartition>, OFS, TS> {
        ConsumerRecordBuilder {
            headers: self.headers,
            key: self.key,
            offset: self.offset,
            timestamp: self.timestamp,
            topic_partition: topic.into(),
            value: self.value,
        }
    }

    pub fn with_topic_partition(
        self,
        tp: impl Into<TopicPartition<Partition>>,
    ) -> ConsumerRecordBuilder<K, V, TopicPartition<Partition>, OFS, TS> {
        ConsumerRecordBuilder {
            headers: self.headers,
            key: self.key,
            offset: self.offset,
            timestamp: self.timestamp,
            topic_partition: tp.into(),
            value: self.value,
        }
    }

    pub fn with_timestamp(
        self,
        timestamp: impl Into<i64>,
    ) -> ConsumerRecordBuilder<K, V, TP, OFS, i64> {
        ConsumerRecordBuilder {
            headers: self.headers,
            key: self.key,
            offset: self.offset,
            timestamp: timestamp.into(),
            topic_partition: self.topic_partition,
            value: self.value,
        }
    }

    pub fn with_key(self, key: impl Into<K>) -> Self {
        ConsumerRecordBuilder {
            headers: self.headers,
            key: Some(key.into()),
            offset: self.offset,
            timestamp: self.timestamp,
            topic_partition: self.topic_partition,
            value: self.value,
        }
    }

    pub fn with_value(self, value: impl Into<V>) -> Self {
        ConsumerRecordBuilder {
            headers: self.headers,
            key: self.key,
            offset: self.offset,
            timestamp: self.timestamp,
            topic_partition: self.topic_partition,
            value: Some(value.into()),
        }
    }
    // TODO: Header types
    pub fn with_header(mut self, header_key: String, header_val: String) -> Self {
        self.headers.push((header_key, header_val));
        ConsumerRecordBuilder {
            headers: self.headers,
            key: self.key,
            offset: self.offset,
            timestamp: self.timestamp,
            topic_partition: self.topic_partition,
            value: self.value,
        }
    }
    // TODO: Header types
    pub fn with_headers(self, headers: Vec<(String, String)>) -> Self {
        ConsumerRecordBuilder {
            headers,
            key: self.key,
            offset: self.offset,
            timestamp: self.timestamp,
            topic_partition: self.topic_partition,
            value: self.value,
        }
    }

    pub fn with_offset(
        self,
        offset: impl Into<Offset>,
    ) -> ConsumerRecordBuilder<K, V, TP, Offset, TS> {
        ConsumerRecordBuilder {
            headers: self.headers,
            key: self.key,
            offset: offset.into(),
            timestamp: self.timestamp,
            topic_partition: self.topic_partition,
            value: self.value,
        }
    }
}

impl<K, V, OFS, TS> ConsumerRecordBuilder<K, V, TopicPartition<OptionalPartition>, OFS, TS> {
    pub fn with_partition(
        self,
        partition: impl Into<Partition>,
    ) -> ConsumerRecordBuilder<K, V, TopicPartition<Partition>, OFS, TS> {
        ConsumerRecordBuilder {
            headers: self.headers,
            key: self.key,
            offset: self.offset,
            timestamp: self.timestamp,
            topic_partition: self.topic_partition.with_partition(partition.into()),
            value: self.value,
        }
    }
}
