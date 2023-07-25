use async_trait::async_trait;

use crate::common::record::RecordStream;

pub struct UnallocatedProducer;
pub struct SubscriberProducer;
pub struct AssignedProducer;

pub trait UnallocatedProducerTrait<K, V> {
    fn subscribe(self) -> Box<dyn SubscriberProducerTrait<K, V>>;
    fn assign(self) -> Box<dyn AssignedProducerTrait<K, V>>;
}
pub trait SubscriberProducerTrait<K, V>: Producer<K, V> {
    fn subscribe(&mut self);
}
pub trait AssignedProducerTrait<K, V>: Producer<K, V> {
    fn assign(&mut self);
}

#[cfg(feature = "async_client")]
#[async_trait(?Send)]
pub trait AsyncProducer<K, V> {
    async fn poll_async(&self) -> RecordStream<K, V>;
}

pub trait Producer<K, V> {
    fn poll(&self) -> Vec<(K, V)>;
}
