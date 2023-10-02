use futures::Stream;

use super::topic::Partition;

pub type Offset = i64;
pub type RecordMetadata = ();

pub enum Position {
    Beginning,
    End,
    Offset(Offset),
}

pub struct RecordSet<K, V> {
    a: K,
    b: V,
}

pub struct RecordStream<K, V> {
    a: K,
    b: V,
}

impl<K, V> Iterator for RecordSet<K, V> {
    type Item = Record<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

// Some way to guarentee key ordering needs to be developed.
impl<K, V> Stream for RecordStream<K, V> {
    // TODO
    type Item = Record<K, V>;

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

#[derive(Debug)]
pub struct Record<K, V> {
    key: K,
    value: Option<V>,
}

impl<K, V> Record<K, V> {
    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn value(&self) -> Option<&V> {
        if let Some(val) = self.value {
            Some(&val)
        } else {
            None
        }
    }

    fn from_key_value(key: K, value: V) -> Record<K, V> {
        Record { key, value: Some(value) }
    }

    fn with_partition(self, parition: Partition) -> Self {
        self
    }

    fn with_topic(self, topic: String) -> Self {
        self
    }

    fn with_topic_partition(self, topic: String, partition: Partition) -> Self {
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
        Record { key, value: Some(value) }
    }
}

pub struct Headers<'a, K, V>
where
    K: From<&'a [u8]>,
    V: From<&'a [u8]>,
{
    headers: &'a Vec<(K, V)>,
}
