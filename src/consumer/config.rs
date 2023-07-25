use crate::{
    config::{raw_config::RawConfig, KafkaProperty},
    error::Error,
};

pub struct ConsumerConfig {
    properties: Vec<ConsumerProperty>,
}

impl ConsumerConfig {
    pub fn new() -> ConsumerConfig {
        ConsumerConfig { properties: vec![] }
    }
}

impl TryFrom<RawConfig> for ConsumerConfig {
    type Error = Error;

    fn try_from(value: RawConfig) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

pub enum ConsumerProperty {
    KafkaProperty(KafkaProperty),
}
