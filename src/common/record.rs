use std::marker::PhantomData;

use chrono::offset;
use futures::Stream;
use serde::{Deserialize, Serialize};

use super::topic::{OptionalPartition, Partition, TopicPartition};

pub type Offset = i64;
pub type RecordMetadata = ();

pub enum Position {
    Beginning,
    End,
    Offset(Offset),
}

pub struct RecordSet<T> {
    records: Vec<T>,
}

impl<T> RecordSet<T> {
    pub fn new() -> Self {
        Self { records: Vec::new() }
    }
}

impl <T> From<Vec<T>> for RecordSet<T> {
    fn from(records: Vec<T>) -> Self {
        Self { records }
    }
}

impl<T> Iterator for RecordSet<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

pub struct RecordStream<T> {
    a: Vec<T>,
}

impl<T> RecordStream<T> {
    pub fn new() -> Self {
        Self { a: Vec::new() }
    }
}

// Some way to guarentee key ordering needs to be developed.
impl<T> Stream for RecordStream<T> {
    // TODO
    type Item = T;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        unimplemented!()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unimplemented!()
    }
}

/*
    TODO: The generic trait that allows streams to be comopose of record streams.
*/
pub trait Record<K, V> {
    fn key(&self) -> Option<&K>;
    fn value(&self) -> Option<&V>;
}

pub struct Headers<'a, K, V>
where
    K: Serialize + Deserialize<'a>,
    V: Serialize + Deserialize<'a>,
{
    headers: &'a Vec<(K, V)>,
}
