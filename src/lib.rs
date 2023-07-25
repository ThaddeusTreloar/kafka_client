#![allow(dead_code, unused_imports, unused_variables)]

mod acl;
mod admin;
mod common;
mod config;
mod consumer;
mod error;
mod metadata;
mod prelude;
mod producer;
#[cfg(feature = "schema_registry")]
mod schema_registry;
mod security;
#[cfg(feature = "streams")]
mod streams;
