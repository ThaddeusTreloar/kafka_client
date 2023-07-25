use std::marker::PhantomData;

use futures::Stream;

pub struct RecordSet<K, V> {
    a: K,
    b: V
}

pub struct RecordStream<K, V> {
    a: K,
    b: V
}

impl <K, V> Stream for RecordStream<K, V> {
    // TODO
    type Item = Record<K, V>;

    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        unimplemented!()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unimplemented!()
    }
}

pub struct Record<K, V> {
    a: K,
    b: V
}

impl<K, V> Record<K, V> {
    fn from_key_value(key: K, value: V) -> Record<K, V> {
        Record { 
            a: key,
            b: value
        }
    }

    fn with_partition(self, parition: usize) -> Self {
        self
    }

    fn with_topic(self, topic: String) -> Self {
        self
    }

    fn with_topic_partition(self, topic: String, partition: usize) -> Self {
        self
    }

                                    // TODO: Bytes?
    fn with_header(self, header_key: String, header_val: String) -> Self {
        self
    }
                                    // TODO: Bytes?
    fn with_headers(self, headers: Vec<(String, String)>) -> Self {
        self
    }

    fn from_value(key: K, value: V) -> Record<K, V> {
        Record { 
            a: key,
            b: value
        }
    }
}

pub struct Headers<'a, K, V> 
where K: From<&'a [u8]>, V: From<&'a [u8]>
{
    headers: &'a Vec<(K, V)>
}