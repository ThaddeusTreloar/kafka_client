use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use super::record::{Offset, RecordMetadata};

pub type Partition = i32;
pub type MaybePartition = Option<Partition>;

pub enum Metadata {
    Offset(Offset),
    OffsetAbstract((Offset, RecordMetadata)),
    Timestamp(NaiveTime),
    Abstract(RecordMetadata),
}

pub struct TopicPartitionMetadataMap<T> {
    map: HashMap<TopicPartition<Partition>, T>,
}

impl From<(TopicPartition<Partition>, (Offset, RecordMetadata))>
    for TopicPartitionMetadataMap<(Offset, RecordMetadata)>
{
    fn from(value: (TopicPartition<Partition>, (Offset, RecordMetadata))) -> Self {
        let mut map = HashMap::new();
        map.insert(value.0, value.1);

        TopicPartitionMetadataMap { map }
    }
}

impl TopicPartitionMetadataMap<Offset> {
    fn new() -> TopicPartitionMetadataMap<Offset> {
        TopicPartitionMetadataMap {
            map: HashMap::new(),
        }
    }
}

pub struct TopicPartitionList<T> {
    set: HashSet<TopicPartition<T>>,
}

impl TopicPartitionList<MaybePartition> {
    fn new() -> Self {
        TopicPartitionList {
            set: HashSet::new(),
        }
    }

    fn add_topic(&mut self, topic: &str) -> TopicPartitionList<MaybePartition> {
        let (mut retain, out): (
            HashSet<TopicPartition<MaybePartition>>,
            HashSet<TopicPartition<MaybePartition>>,
        ) = self
            .set
            .clone()
            .into_iter()
            .partition(|tp| topic.eq(tp.topic.as_str()));

        retain.insert(topic.into());

        self.set = retain;

        out.into()
    }

    fn add_partition(&mut self, topic: &str, partition: &Partition) {
        self.set.insert((topic, partition).into());
    }

    fn add_partition_range(&mut self, topic: &str, range: Range<Partition>) {
        range
            .into_iter()
            .for_each(|partition| self.add_partition(topic, &partition));
    }

    fn get_topic<'a>(&self, topic: impl Into<&'a str>) -> TopicPartitionList<MaybePartition> {
        let s_ref: &str = topic.into();

        self.set
            .iter()
            .filter(|tp| s_ref.eq(tp.topic.as_str()))
            .collect()
    }
}

impl FromIterator<TopicPartition<MaybePartition>> for TopicPartitionList<MaybePartition> {
    fn from_iter<T: IntoIterator<Item = TopicPartition<MaybePartition>>>(iter: T) -> Self {
        iter.into_iter().collect()
    }
}

impl<'a> FromIterator<&'a TopicPartition<MaybePartition>> for TopicPartitionList<MaybePartition> {
    fn from_iter<T: IntoIterator<Item = &'a TopicPartition<MaybePartition>>>(iter: T) -> Self {
        iter.into_iter().collect()
    }
}

impl<T> From<TopicPartitionMetadataMap<T>> for TopicPartitionList<Partition> {
    fn from(value: TopicPartitionMetadataMap<T>) -> Self {
        TopicPartitionList {
            set: value.map.into_iter().map(|(tp, o)| tp.into()).collect(),
        }
    }
}

impl From<HashSet<TopicPartition<MaybePartition>>> for TopicPartitionList<MaybePartition> {
    fn from(value: HashSet<TopicPartition<MaybePartition>>) -> Self {
        TopicPartitionList { set: value }
    }
}

impl From<TopicPartition<MaybePartition>> for TopicPartitionList<MaybePartition> {
    fn from(value: TopicPartition<MaybePartition>) -> Self {
        let mut set = HashSet::new();
        set.insert(value);
        TopicPartitionList { set }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct TopicPartition<T> {
    topic: String,
    partition: T,
}

impl TopicPartition<Partition> {
    fn new(topic: &str, partition: Partition) -> TopicPartition<Partition> {
        TopicPartition {
            topic: String::from(topic),
            partition,
        }
    }
}

impl TopicPartition<MaybePartition> {
    fn new(topic: &str, partition: Option<Partition>) -> TopicPartition<MaybePartition> {
        TopicPartition {
            topic: String::from(topic),
            partition,
        }
    }
}

impl From<(&str, &Partition)> for TopicPartition<Partition> {
    fn from(value: (&str, &Partition)) -> Self {
        TopicPartition {
            topic: String::from(value.0),
            partition: *value.1,
        }
    }
}

impl From<(String, Partition)> for TopicPartition<Partition> {
    fn from(value: (String, Partition)) -> Self {
        TopicPartition {
            topic: value.0,
            partition: value.1,
        }
    }
}

impl From<(&str, &Partition)> for TopicPartition<MaybePartition> {
    fn from(value: (&str, &Partition)) -> Self {
        TopicPartition {
            topic: String::from(value.0),
            partition: Some(*value.1),
        }
    }
}

impl From<(String, Partition)> for TopicPartition<MaybePartition> {
    fn from(value: (String, Partition)) -> Self {
        TopicPartition {
            topic: value.0,
            partition: Some(value.1),
        }
    }
}

impl From<&str> for TopicPartition<MaybePartition> {
    fn from(value: &str) -> Self {
        TopicPartition {
            topic: String::from(value),
            partition: None,
        }
    }
}

impl From<String> for TopicPartition<MaybePartition> {
    fn from(value: String) -> Self {
        TopicPartition {
            topic: value,
            partition: None,
        }
    }
}
