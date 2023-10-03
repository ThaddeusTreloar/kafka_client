use std::marker::PhantomData;

use chrono::offset;
use futures::Stream;
use serde::{Deserialize, Serialize};

use super::topic::{OptionalPartition, Partition, TopicPartition};

pub type Offset = i64;
pub type RecordMetadata = ();

pub enum Position {
    Beginning,
    End,
    Offset(Offset),
}

pub struct RecordSet<T> {
    records: Vec<T>,
}

impl<T> Iterator for RecordSet<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

pub struct RecordStream<T> {
    a: Vec<T>,
}

impl<T> RecordStream<T> {
    pub fn new() -> Self {
        Self { a: Vec::new() }
    }
}

// Some way to guarentee key ordering needs to be developed.
impl<T> Stream for RecordStream<T> {
    // TODO
    type Item = T;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        unimplemented!()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unimplemented!()
    }
}

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

    // TODO: Bytes?
    pub fn with_header(self, header_key: String, header_val: String) -> Self {
        self
    }
    // TODO: Bytes?
    pub fn with_headers(self, headers: Vec<(String, String)>) -> Self {
        self
    }

    pub fn with_offset(self, offset: Offset) -> ConsumerRecordBuilder<K, V, TP, Offset, TS> {
        ConsumerRecordBuilder {
            headers: self.headers,
            key: self.key,
            offset,
            timestamp: self.timestamp,
            topic_partition: self.topic_partition,
            value: self.value,
        }
    }

    pub fn with_timestamp(self, timestamp: i64) -> ConsumerRecordBuilder<K, V, TP, OFS, i64> {
        ConsumerRecordBuilder {
            headers: self.headers,
            key: self.key,
            offset: self.offset,
            timestamp,
            topic_partition: self.topic_partition,
            value: self.value,
        }
    }

    pub fn with_topic<T>(
        self,
        topic: String,
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
        tp: TopicPartition<Partition>,
    ) -> ConsumerRecordBuilder<K, V, TopicPartition<Partition>, OFS, TS> {
        ConsumerRecordBuilder {
            headers: self.headers,
            key: self.key,
            offset: self.offset,
            timestamp: self.timestamp,
            topic_partition: tp,
            value: self.value,
        }
    }
}

impl<K, V>
    ConsumerRecordBuilder<
        K,
        V,
        Option<TopicPartition<OptionalPartition>>,
        Option<Offset>,
        Option<i64>,
    >
{
    pub fn from_key(key: K) -> Self {
        ConsumerRecordBuilder {
            headers: Vec::new(),
            key: Some(key),
            offset: None,
            timestamp: None,
            topic_partition: None,
            value: None,
        }
    }

    pub fn from_value(value: V) -> Self {
        ConsumerRecordBuilder {
            headers: Vec::new(),
            key: None,
            offset: None,
            timestamp: None,
            topic_partition: None,
            value: Some(value),
        }
    }
    pub fn from_key_value(key: K, value: V) -> Self {
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

impl<K, V, OFS, TS> ConsumerRecordBuilder<K, V, TopicPartition<OptionalPartition>, OFS, TS> {
    pub fn with_partition(
        self,
        partition: Partition,
    ) -> ConsumerRecordBuilder<K, V, TopicPartition<Partition>, OFS, TS> {
        ConsumerRecordBuilder {
            headers: self.headers,
            key: self.key,
            offset: self.offset,
            timestamp: self.timestamp,
            topic_partition: self.topic_partition.with_partition(partition),
            value: self.value,
        }
    }
}

/*
    TODO: The generic trait that allows streams to be comopose of record streams.
*/
pub trait Record<K, V> {
    fn key(&self) -> Option<&K>;
    fn value(&self) -> Option<&V>;
}

#[derive(Debug)]
pub struct ProducerRecord<K, V> {
    headers: Vec<(String, String)>,
    key: Option<K>,
    topic_partition: TopicPartition<OptionalPartition>,
    timestamp: Option<i64>,
    value: Option<V>,
}



impl<K, V> Record<K, V> for ProducerRecord<K, V> {
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
}

#[derive(Debug)]
pub struct ProducerRecordBuilder<K, V, TP> {
    headers: Vec<(String, String)>,
    key: Option<K>,
    topic_partition: TP,
    timestamp: Option<i64>,
    value: Option<V>,
}

impl<K, V> ProducerRecordBuilder<K, V, Option<TopicPartition<OptionalPartition>>> {
    pub fn from_key(key: K) -> Self {
        ProducerRecordBuilder {
            headers: Vec::new(),
            key: Some(key),
            timestamp: None,
            topic_partition: None,
            value: None,
        }
    }

    pub fn from_value(value: V) -> Self {
        ProducerRecordBuilder {
            headers: Vec::new(),
            key: None,
            timestamp: None,
            topic_partition: None,
            value: Some(value),
        }
    }

    pub fn from_key_value(key: K, value: V) -> Self {
        ProducerRecordBuilder {
            headers: Vec::new(),
            key: Some(key),
            timestamp: None,
            topic_partition: None,
            value: Some(value),
        }
    }
}

impl<K, V, TP> ProducerRecordBuilder<K, V, TP> {
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
        topic: String,
    ) -> ProducerRecordBuilder<K, V, TopicPartition<OptionalPartition>> {
        ProducerRecordBuilder {
            headers: self.headers,
            key: self.key,
            timestamp: self.timestamp,
            topic_partition: topic.into(),
            value: self.value,
        }
    }

    pub fn with_topic_partition(
        self,
        tp: TopicPartition<Partition>,
    ) -> ProducerRecordBuilder<K, V, TopicPartition<OptionalPartition>> {
        ProducerRecordBuilder {
            headers: self.headers,
            key: self.key,
            timestamp: self.timestamp,
            topic_partition: tp.into(),
            value: self.value,
        }
    }

    pub fn with_timestamp(self, timestamp: i64) -> Self {
        ProducerRecordBuilder {
            headers: self.headers,
            key: self.key,
            timestamp: Some(timestamp),
            topic_partition: self.topic_partition,
            value: self.value,
        }
    }

    pub fn with_key(self, key: K) -> Self {
        ProducerRecordBuilder {
            headers: self.headers,
            key: Some(key),
            timestamp: self.timestamp,
            topic_partition: self.topic_partition,
            value: self.value,
        }
    }

    pub fn with_value(self, value: V) -> Self {
        ProducerRecordBuilder {
            headers: self.headers,
            key: self.key,
            timestamp: self.timestamp,
            topic_partition: self.topic_partition,
            value: Some(value),
        }
    }

    pub fn with_header(mut self, header_key: String, header_val: String) -> Self {
        self.headers.push((header_key, header_val));
        ProducerRecordBuilder {
            headers: self.headers,
            key: self.key,
            timestamp: self.timestamp,
            topic_partition: self.topic_partition,
            value: self.value,
        }
    }

    pub fn with_headers(self, headers: Vec<(String, String)>) -> Self {
        ProducerRecordBuilder {
            headers,
            key: self.key,
            timestamp: self.timestamp,
            topic_partition: self.topic_partition,
            value: self.value,
        }
    }
}

impl<K, V> ProducerRecordBuilder<K, V, TopicPartition<OptionalPartition>> {
    pub fn with_partition(self, partition: OptionalPartition) -> Self {
        ProducerRecordBuilder {
            headers: self.headers,
            key: self.key,
            timestamp: self.timestamp,
            topic_partition: self.topic_partition.with_partition_option(partition),
            value: self.value,
        }
    }

    pub fn into_record(self) -> ProducerRecord<K, V> {
        ProducerRecord {
            headers: self.headers,
            key: self.key,
            topic_partition: self.topic_partition,
            timestamp: self.timestamp,
            value: self.value,
        }
    }
}

pub struct Headers<'a, K, V>
where
    K: Serialize + Deserialize<'a>,
    V: Serialize + Deserialize<'a>,
{
    headers: &'a Vec<(K, V)>,
}
