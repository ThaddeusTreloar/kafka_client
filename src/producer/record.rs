use std::{marker::PhantomData, fmt::Display};

use chrono::offset;
use futures::Stream;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::common::{
    record::Record,
    topic::{OptionalPartition, Partition, TopicPartition, fmt_optional_partition},
};

pub type RecordMetadata = ();

#[derive(Debug, Serialize, Deserialize)]
pub struct ProducerRecord<K, V> 
where K: Sized, V: Sized
{
    headers: Vec<(String, String)>,
    key: Option<K>,
    topic_partition: TopicPartition<OptionalPartition>,
    timestamp: Option<i64>,
    value: Option<V>,
}

impl <K, V> Display for ProducerRecord<K, V>
where K: Display + Serialize, V: Display + Serialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Consider rewriting this function to just serialise the struct and write
        let mut headers = String::new();
        for (key, value) in &self.headers {
            headers.push_str(&format!("{}: {}\n", key, value));
        }

        let json = match serde_json::to_string(&self) {
            Ok(json) => json,
            Err(e) => format!("Error serialising ProducerRecord: {}", e),
        };
        
        write!(f, "ProducerRecord: {}", json)
    }
}

impl<K, V> From<ProducerRecordBuilder<K, V, TopicPartition<OptionalPartition>>> for ProducerRecord<K, V> {
    fn from(builder: ProducerRecordBuilder<K, V, TopicPartition<OptionalPartition>>) -> Self {
        ProducerRecord {
            headers: builder.headers,
            key: builder.key,
            topic_partition: builder.topic_partition,
            timestamp: builder.timestamp,
            value: builder.value,
        }
    }
}


impl<K, V> ProducerRecord<K, V> {
    // TODO: Check if this is appropriate. May lead to confusion as the from_* function is returning a type, different from the associated struct
    pub fn from_key(
        key: K,
    ) -> ProducerRecordBuilder<K, V, Option<TopicPartition<OptionalPartition>>> {
        ProducerRecordBuilder {
            headers: Vec::new(),
            key: Some(key),
            timestamp: None,
            topic_partition: None,
            value: None,
        }
    }

    // TODO: Check if this is appropriate. May lead to confusion as the from_* function is returning a type, different from the associated struct
    pub fn from_value(
        value: V,
    ) -> ProducerRecordBuilder<K, V, Option<TopicPartition<OptionalPartition>>> {
        ProducerRecordBuilder {
            headers: Vec::new(),
            key: None,
            timestamp: None,
            topic_partition: None,
            value: Some(value),
        }
    }

    // TODO: Check if this is appropriate. May lead to confusion as the from_* function is returning a type, different from the associated struct
    pub fn from_key_value(
        key: K,
        value: V,
    ) -> ProducerRecordBuilder<K, V, Option<TopicPartition<OptionalPartition>>> {
        ProducerRecordBuilder {
            headers: Vec::new(),
            key: Some(key),
            timestamp: None,
            topic_partition: None,
            value: Some(value),
        }
    }
}

impl <K, V> ProducerRecord<K, V>
where K: Clone, V: Clone {
    fn from_record(record: impl Record<K, V>) -> Self {
        ProducerRecord {
            headers: Vec::new(),
            key: record.key().cloned(),
            topic_partition: TopicPartition::from(String::from(record.topic())),
            timestamp: None,
            value: record.value().cloned(),
        }
    }
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

    fn timestamp(&self) -> Option<i64> {
        self.timestamp
    }

    fn topic(&self) -> &str {
        self.topic_partition.topic()
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

impl<K, V> ProducerRecordBuilder<K, V, Option<TopicPartition<OptionalPartition>>> {}

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

    pub fn with_topic(
        self,
        topic: impl Into<TopicPartition<OptionalPartition>>,
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
    // TODO: Header types
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
    // TODO: Header types
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
        self.into()
    }
}
