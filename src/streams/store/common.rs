use std::{sync::RwLock, collections::HashMap, hash::Hash};

use crate::{prelude::Result};

use super::in_memory::InMemoryStateStore;

pub trait StateStore<T, U>
{
    fn get(&self, k: &T) -> Result<Option<U>>;
    fn put(& self, k: T, v: U) -> Result<Option<U>>;
}

pub enum StateStores<T, U>
where 
    T: From<Vec<u8>> + Into<Vec<u8>> + std::cmp::Eq + PartialEq + Hash,
    U: From<Vec<u8>> + Into<Vec<u8>> + Clone
{
    InMemory(InMemoryStateStore<T, U>),
    SurrealDb,
    Redis
}
