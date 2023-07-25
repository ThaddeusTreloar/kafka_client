use std::net::SocketAddr;

pub mod raw_config;

pub enum KafkaProperty { 
    BootstrapServers(Vec<SocketAddr>)
}

