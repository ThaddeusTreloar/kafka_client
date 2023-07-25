use crate::config::{KafkaProperty, raw_config::RawConfig};


pub struct ProducerConfig {
    properties: Vec<ProducerProperty>
}

impl ProducerConfig {
    pub fn new() -> ProducerConfig {
        ProducerConfig {
            properties: vec![]
        }
    }
}

impl From<RawConfig> for ProducerConfig {
    fn from(value: RawConfig) -> Self {
        ProducerConfig { properties: vec![] }
    }
}

pub enum ProducerProperty {
    KafkaProperty(KafkaProperty)
}