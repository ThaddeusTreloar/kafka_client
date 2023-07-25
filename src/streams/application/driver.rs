use crate::streams::config::StreamsConfig;

pub trait StreamDriver: TryFrom<StreamsConfig> {
}