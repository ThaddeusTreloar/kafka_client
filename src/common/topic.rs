use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    ops::Range, fmt::{Display, Formatter, self},
};

use super::record::{Offset, RecordMetadata};

pub type Partition = i32;
pub type OptionalPartition = Option<Partition>;
// TODO: Solve the problem where OptionalPartition doesn't implement Display
// but impl Display for TopicPartition<OptionalPartition> conflicts with impl <T> Display for TopicPartition<T>
pub fn fmt_optional_partition(op: OptionalPartition) -> String {
    match op {
        Some(partition) => format!("{}", partition),
        None => "None".into(),
    }
}

pub enum Metadata {
    Offset(Offset),
    OffsetAbstract((Offset, RecordMetadata)),
    Timestamp(NaiveTime),
    Abstract(RecordMetadata),
}

pub struct TopicPartitionMetadataMap<T> {
    map: HashMap<TopicPartition<Partition>, T>,
}

pub struct OffsetAndMetadata(Offset, RecordMetadata);

impl From<(TopicPartition<Partition>, OffsetAndMetadata)>
    for TopicPartitionMetadataMap<OffsetAndMetadata>
{
    fn from(value: (TopicPartition<Partition>, OffsetAndMetadata)) -> Self {
        let mut map = HashMap::new();
        map.insert(value.0, value.1);

        TopicPartitionMetadataMap { map }
    }
}

impl From<(TopicPartition<Partition>, Offset)>
    for TopicPartitionMetadataMap<Offset>
{
    fn from(value: (TopicPartition<Partition>, Offset)) -> Self {
        let mut map = HashMap::new();
        map.insert(value.0, value.1);

        TopicPartitionMetadataMap { map }
    }
}

impl TopicPartitionMetadataMap<Offset> {
    pub fn new() -> TopicPartitionMetadataMap<Offset> {
        TopicPartitionMetadataMap {
            map: HashMap::new(),
        }
    }
}

pub struct TopicPartitionList<T> {
    set: HashSet<TopicPartition<T>>,
}

impl IntoIterator for TopicPartitionList<Partition> {
    type Item = TopicPartition<Partition>;
    type IntoIter = std::collections::hash_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.into_iter()
    }
}

impl <T> TopicPartitionList<T> {
    pub fn new() -> Self {
        TopicPartitionList {
            set: HashSet::new(),
        }
    }

    pub fn get_topic<'a>(&self, topic: impl Into<&'a str>) -> TopicPartitionList<T> {
        let s_ref: &str = topic.into();

        self.set
            .iter()
            .filter(|tp| s_ref.eq(tp.topic.as_str()))
            .collect()
    }
}

impl TopicPartitionList<Partition> {
    pub fn add_partition(&mut self, tp: TopicPartition<Partition>) {

        self.set.insert((tp.topic(), tp.partition()).into());
    }

    pub fn add_partition_range(&mut self, topic: &str, range: Range<Partition>) {
        range
            .into_iter()
            .for_each(|partition| self.add_partition((topic, &partition).into()));
    }
}

// TODO: This is a very odd type. It doesn't really make sense to have a TopicPartitionList with
// OptionalPartition.
impl TopicPartitionList<OptionalPartition> {
    fn add_topic(&mut self, topic: &str) -> TopicPartitionList<OptionalPartition> {
        let (mut retain, out): (
            HashSet<TopicPartition<OptionalPartition>>,
            HashSet<TopicPartition<OptionalPartition>>,
        ) = self
            .set
            .clone()
            .into_iter()
            .partition(|tp| topic.eq(tp.topic.as_str()));

        retain.insert(topic.into());

        self.set = retain;

        out.into()
    }
}

impl <PT> FromIterator<TopicPartition<PT>> for TopicPartitionList<PT> {
    fn from_iter<T: IntoIterator<Item = TopicPartition<PT>>>(iter: T) -> Self {
        iter.into_iter().collect()
    }
}

impl<'a, PT> FromIterator<&'a TopicPartition<PT>> for TopicPartitionList<PT> {
    fn from_iter<T: IntoIterator<Item = &'a TopicPartition<PT>>>(iter: T) -> Self {
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

impl From<HashSet<TopicPartition<OptionalPartition>>> for TopicPartitionList<OptionalPartition> {
    fn from(value: HashSet<TopicPartition<OptionalPartition>>) -> Self {
        TopicPartitionList { set: value }
    }
}

impl From<TopicPartition<OptionalPartition>> for TopicPartitionList<OptionalPartition> {
    fn from(value: TopicPartition<OptionalPartition>) -> Self {
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

impl <T> Display for TopicPartition<T> 
where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.topic, self.partition)
    }
}

impl <T> TopicPartition<T> {
    pub fn topic(&self) -> &str {
        self.topic.as_str()
    }

    pub fn with_partition(self, partition: Partition) -> TopicPartition<Partition> {
        TopicPartition {
            topic: self.topic,
            partition,
        }
    }

    pub fn with_partition_option(self, partition: Option<Partition>) -> TopicPartition<OptionalPartition> {
        TopicPartition {
            topic: self.topic,
            partition: partition,
        }
    }

    pub fn partition(&self) -> &T {
        &self.partition
    }
}

impl TopicPartition<Partition> {
    pub fn new_partitioned(topic: &str, partition: Partition) -> Self {
        TopicPartition {
            topic: String::from(topic),
            partition,
        }
    }
}

impl TopicPartition<OptionalPartition> {
    pub fn new(topic: &str, partition: Option<Partition>) -> TopicPartition<OptionalPartition> {
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

impl From<(&str, &Partition)> for TopicPartition<OptionalPartition> {
    fn from(value: (&str, &Partition)) -> Self {
        TopicPartition {
            topic: String::from(value.0),
            partition: Some(*value.1),
        }
    }
}

impl From<(String, Partition)> for TopicPartition<OptionalPartition> {
    fn from(value: (String, Partition)) -> Self {
        TopicPartition {
            topic: value.0,
            partition: Some(value.1),
        }
    }
}

impl From<(String, OptionalPartition)> for TopicPartition<OptionalPartition> {
    fn from(value: (String, OptionalPartition)) -> Self {
        TopicPartition {
            topic: value.0,
            partition: value.1,
        }
    }
}

impl From<&str> for TopicPartition<OptionalPartition> {
    fn from(value: &str) -> Self {
        TopicPartition {
            topic: String::from(value),
            partition: None,
        }
    }
}

impl From<TopicPartition<Partition>> for TopicPartition<OptionalPartition> {
    fn from(value: TopicPartition<Partition>) -> Self {
        TopicPartition {
            topic: value.topic,
            partition: Some(value.partition),
        }
    }
}

impl TryFrom<TopicPartition<OptionalPartition>> for TopicPartition<Partition> {
    type Error = (); // TODO: Make Error type

    fn try_from(value: TopicPartition<OptionalPartition>) -> Result<Self, Self::Error> {
        match value.partition {
            Some(partition) => Ok(TopicPartition {
                topic: value.topic,
                partition,
            }),
            None => unimplemented!(),
        }
    }
}

impl From<String> for TopicPartition<OptionalPartition> {
    fn from(value: String) -> Self {
        TopicPartition {
            topic: value,
            partition: None,
        }
    }
}
