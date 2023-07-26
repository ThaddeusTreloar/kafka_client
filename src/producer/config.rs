use crate::config::{raw_config::RawConfig, ClientPropertyValue};

pub struct ProducerConfig {
    properties: Vec<ProducerProperty>,
}

impl ProducerConfig {
    pub fn new() -> ProducerConfig {
        ProducerConfig { properties: vec![] }
    }
}

impl From<RawConfig> for ProducerConfig {
    fn from(value: RawConfig) -> Self {
        ProducerConfig { properties: vec![] }
    }
}

pub enum ProducerProperty {
    KafkaProperty(ClientPropertyValue),
}
