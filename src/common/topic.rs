use std::{collections::{HashSet, HashMap}, ops::Range};
use serde::{Serialize, Deserialize};

use super::record::Offset;

pub type Partition = i32;

pub struct TopicPartitionOffsetMap {
    map: HashMap<(String, Partition), Offset>
}

impl TopicPartitionOffsetMap {
    fn new() -> TopicPartitionOffsetMap {
        TopicPartitionOffsetMap {
            map: HashMap::new()
        }
    }
}

pub struct TopicPartitionList {
    set: HashSet<TopicPartition>
}

impl TopicPartitionList {
    fn new() -> Self {
        TopicPartitionList { set: HashSet::new() }
    }

    fn add_topic<'a>(&mut self, topic: impl Into<&'a str>) -> TopicPartitionList {
        let s_ref: &str = topic.into();

        let (mut retain, out): (HashSet<TopicPartition>, HashSet<TopicPartition>) = self.set
            .into_iter()
            .partition(
                |tp| s_ref.eq(tp.topic.as_str())
            );

        retain.insert(s_ref.into());

        self.set = retain;

        out.into()
    }

    fn add_partition<'a>(&mut self, topic: impl Into<&'a str>, partition: impl Into<Partition>) {
        let t = topic.into();
        let p = partition.into();
        let tp = (t, p).into();

        self.set.insert(tp);
    }

    fn add_partition_range<'a>(&mut self, topic: impl Into<&'a str>, range: Range<Partition>) {
        range.into_iter().for_each(|partition| self.add_partition(topic, partition));
    }

    fn get_topic<'a>(&self, topic: impl Into<&'a str>) -> TopicPartitionList {
        let s_ref: &str = topic.into();

        self.set.iter().filter(|tp|s_ref.eq(tp.topic.as_str())).map(|tp| tp.clone()).collect()
    }
}

impl FromIterator<TopicPartition> for TopicPartitionList {
    fn from_iter<T: IntoIterator<Item = TopicPartition>>(iter: T) -> Self {
        iter.into_iter().map(
            |item| item.clone()
        ).collect()
    }
}

impl <'a> FromIterator<&'a TopicPartition> for TopicPartitionList {
    fn from_iter<T: IntoIterator<Item = &'a TopicPartition>>(iter: T) -> Self {
        iter.into_iter().map(
            |item| item.clone()
        ).collect()
    }
}

impl From<TopicPartitionOffsetMap> for TopicPartitionList {
    fn from(value: TopicPartitionOffsetMap) -> Self {
        TopicPartitionList {
            set: value.map.into_iter().map(
                |(tp, o)| tp.into()
            ).collect()
        }
    }
}

impl From<HashSet<TopicPartition>> for TopicPartitionList { 
    fn from(value: HashSet<TopicPartition>) -> Self {
        TopicPartitionList { set: value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct TopicPartition {
    topic: String,
    partition: Option<Partition>
}

impl TopicPartition {
    fn new(topic: &str, partition: Partition) -> TopicPartition {
        TopicPartition { 
            topic: String::from(topic), 
            partition: Some(partition)
        }
    }
}

impl From<(&str, Partition)> for TopicPartition {
    fn from(value: (&str, Partition)) -> Self {
        TopicPartition { 
            topic: String::from(value.0), 
            partition: Some(value.1)
        }
    }
}

impl From<(String, Partition)> for TopicPartition {
    fn from(value: (String, Partition)) -> Self {
        TopicPartition { 
            topic: value.0,
            partition: Some(value.1)
        }
    }
}
impl From<&str> for TopicPartition {
    fn from(value: &str) -> Self {
        TopicPartition { 
            topic: String::from(value), 
            partition: None
        }
    }
}

impl From<String> for TopicPartition {
    fn from(value: String) -> Self {
        TopicPartition { 
            topic: value,
            partition: None
        }
    }
}

