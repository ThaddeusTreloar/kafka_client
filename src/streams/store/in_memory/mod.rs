use std::{collections::HashMap, hash::Hash, sync::RwLock};

use crate::{error::StateStoreError, prelude::Result};

use super::common::StateStore;

pub struct InMemoryStateStore<T, U>
where
    T: From<Vec<u8>> + Into<Vec<u8>> + std::cmp::Eq + PartialEq + Hash,
    U: From<Vec<u8>> + Into<Vec<u8>> + Clone,
{
    store: RwLock<HashMap<T, U>>,
}

impl<T, U> InMemoryStateStore<T, U>
where
    T: From<Vec<u8>> + Into<Vec<u8>> + std::cmp::Eq + PartialEq + Hash,
    U: From<Vec<u8>> + Into<Vec<u8>> + Clone,
{
    pub fn new() -> Self {
        InMemoryStateStore {
            store: RwLock::new(HashMap::new()),
        }
    }
}
// TODO: Change this so that the input type implement Send and Sync
unsafe impl<T, U> Send for InMemoryStateStore<T, U>
where
    T: From<Vec<u8>> + Into<Vec<u8>> + std::cmp::Eq + PartialEq + Hash,
    U: From<Vec<u8>> + Into<Vec<u8>> + Clone,
{
}
// TODO: Change this so that the input type implement Send and Sync
unsafe impl<T, U> Sync for InMemoryStateStore<T, U>
where
    T: From<Vec<u8>> + Into<Vec<u8>> + std::cmp::Eq + PartialEq + Hash,
    U: From<Vec<u8>> + Into<Vec<u8>> + Clone,
{
}

impl<T, U> StateStore<T, U> for InMemoryStateStore<T, U>
where
    T: From<Vec<u8>> + Into<Vec<u8>> + std::cmp::Eq + PartialEq + Hash,
    U: From<Vec<u8>> + Into<Vec<u8>> + Clone,
{
    fn get(&self, k: &T) -> Result<Option<U>> {
        match self.store.read() {
            Ok(lock) => Ok(match lock.get(k) {
                Some(v) => Some(v.clone()),
                None => None,
            }),
            Err(e) => unimplemented!()//Err(StateStoreError::StateStoreFailed(e.to_string()).into()),
        }
    }

    fn put(&self, k: T, v: U) -> Result<Option<U>> {
        match self.store.write() {
            Ok(mut lock) => Ok(lock.insert(k, v)),
            Err(e) => unimplemented!()//Err(StateStoreError::StateStoreFailed(e.to_string()).into()),
        }
    }
}
