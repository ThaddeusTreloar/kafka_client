use std::net::SocketAddr;

use self::raw_config::RawConfig;

pub mod raw_config;

pub enum KafkaProperty { 
    BootstrapServers(Vec<SocketAddr>)
}
