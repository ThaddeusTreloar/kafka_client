use crate::{consumer::config::ConsumerProperty, error::{ProducerError, Error}, prelude::Result, config::{raw_config::RawConfig}, producer::config::ProducerProperty};

pub struct StreamsConfig {
    properties: Vec<StreamsProperty>
}

impl TryFrom<RawConfig> for StreamsConfig {
    type Error = Error;

    fn try_from(value: RawConfig) -> std::result::Result<Self, Self::Error> {
        unimplemented!()
    }
}

pub enum StreamsProperty {
    ConsumerProperty(ConsumerProperty),
    ProducerProperty(ProducerProperty),
    ApplicationId(ApplicationId)
}

pub struct ApplicationId {
    id: String
}

impl TryFrom<String> for ApplicationId {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        // Some regex
        unimplemented!()
    }
}