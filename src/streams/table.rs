use futures::{
    future::{join, join_all},
    join,
    stream::{FilterMap, ForEach},
    Future, StreamExt,
};
use std::{
    collections::HashMap,
    hash::Hash,
    marker::PhantomData,
    process::Output,
    sync::{Arc, RwLock},
    thread::{sleep, JoinHandle},
    time::Duration,
};
use tokio::spawn;

use crate::{
    common::record::Record,
    consumer::{common::AsyncConsumer, config::ConsumerConfig},
    error::KafkaError,
};

use super::store::{common::StateStore, in_memory::InMemoryStateStore};

pub struct IdleStreamTable;
pub struct InitialisedStreamTable;

pub struct StreamTable<'a, S, K, V>
where
    K: From<Vec<u8>> + Into<Vec<u8>> + std::cmp::Eq + PartialEq + Hash,
    V: From<Vec<u8>> + Into<Vec<u8>>,
{
    input_stream: Arc<dyn AsyncConsumer<'a, K, V>>,
    state_store: Arc<dyn StateStore<K, V> + Send + Sync>,
    commit_store: Arc<dyn StateStore<K, V> + Send + Sync>,
    topic: String,
    state: PhantomData<S>,
}

impl<'a, K, V> StreamTable<'a, IdleStreamTable, K, V>
where
    K: From<Vec<u8>> + Into<Vec<u8>> + std::cmp::Eq + PartialEq + Hash,
    V: From<Vec<u8>> + Into<Vec<u8>> + Clone,
{
    pub fn new(
        input: &ConsumerConfig,
        topic: &str,
    ) -> Result<StreamTable<'a, IdleStreamTable, K, V>, KafkaError> {
        unimplemented!()
    }

    pub fn into_running(self) -> StreamTable<'a, InitialisedStreamTable, K, V> {
        unimplemented!()
    }

    pub fn start(self) {}
}

fn process_table_message<K, V>(store: Arc<RwLock<HashMap<String, String>>>, message: impl Record<K, V>) {
    unimplemented!()
}
/*
async fn start_table<K, V>(table: Arc<StreamTable<InitialisedStreamTable, K, V>>) -> Result<()>
where
    K: From<Vec<u8>> + Into<Vec<u8>> + std::cmp::Eq + PartialEq + Hash,
    V: From<Vec<u8>> + Into<Vec<u8>>
{
    table.input_stream.stream().filter_map(
        |item| async {
            match item {
                Ok(i) => Some(i),
                Err(_) => None
            }
        }
    ).for_each(
        |message| {
            // TODO : remove this
            let state_store_ref = table.state_store.clone();

            return async move {
                let msg = message.detach();

                state_store_ref.put(
                    msg.key().unwrap().to_vec().into(),
                    msg.payload().unwrap().to_vec().into()
                );


            }
        }
    ).await;

    Ok(())
} */
