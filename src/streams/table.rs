use std::{collections::HashMap, sync::{RwLock, Arc}, marker::PhantomData, thread::{JoinHandle, sleep}, process::Output, time::Duration, hash::Hash};
use futures::{StreamExt, stream::{ForEach, FilterMap}, Future, future::{join_all, join}, join};
use tokio::{spawn};

use crate::{consumer::{common::AsyncConsumer, config::ConsumerConfig}, error::KafkaError, common::record::Record};

use super::store::{common::StateStore, in_memory::InMemoryStateStore};


pub struct IdleStreamTable;
pub struct InitialisedStreamTable;

pub struct StreamTable<S, K, V>
where 
    K: From<Vec<u8>> + Into<Vec<u8>> + std::cmp::Eq + PartialEq + Hash,
    V: From<Vec<u8>> + Into<Vec<u8>>
{
    input_stream: Arc<dyn AsyncConsumer<K, V>>,
    state_store: Arc<dyn StateStore<K, V> + Send + Sync>,
    commit_store: Arc<dyn StateStore<K, V> + Send + Sync>,
    topic: String,
    state: PhantomData<S>
}

impl <K, V> StreamTable<IdleStreamTable, K, V>
where 
    K: From<Vec<u8>> + Into<Vec<u8>> + std::cmp::Eq + PartialEq + Hash,
    V: From<Vec<u8>> + Into<Vec<u8>> + Clone
{
    pub fn new(input: &ConsumerConfig, topic: &str) -> Result<StreamTable<IdleStreamTable, K, V>, KafkaError> {
        unimplemented!()
    }

    pub fn into_running(self) -> StreamTable<InitialisedStreamTable, K, V> {
        unimplemented!()
    }

    pub fn start(self)
    {
    }
}

fn process_table_message<K, V>(store: Arc<RwLock<HashMap<String, String>>>, message: Record<K, V>) {
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
