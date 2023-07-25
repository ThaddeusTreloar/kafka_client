mod consumer;
mod error;
mod prelude;
mod producer;
mod client;
mod config;
mod admin;
mod metadata;
mod common;
#[cfg(feature="streams")]
mod streams;
#[cfg(feature="schema_registry")]
mod schema_registry;