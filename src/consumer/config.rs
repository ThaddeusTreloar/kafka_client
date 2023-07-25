use crate::config::{KafkaProperty, raw_config::RawConfig};

pub struct ConsumerConfig {
    properties: Vec<ConsumerProperty>
}

impl ConsumerConfig {
    pub fn new() -> ConsumerConfig {
        ConsumerConfig {
            properties: vec![]
        }
    }
}

impl From<RawConfig> for ConsumerConfig {
    fn from(value: RawConfig) -> Self {
        ConsumerConfig { properties: vec![] }
    }
}

pub enum ConsumerProperty {
    KafkaProperty(KafkaProperty)
}