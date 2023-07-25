use async_trait::async_trait;

use crate::common::record::RecordStream;

pub struct UnallocatedConsumer;
pub struct SubscriberConsumer;
pub struct AssignedConsumer;

pub trait UnallocatedConsumerTrait<K, V> {
    fn subscribe(self) -> Box<dyn SubscriberConsumerTrait<K, V>>;
    fn assign(self) -> Box<dyn AssignedConsumerTrait<K, V>>;
}
pub trait SubscriberConsumerTrait<K, V>: Consumer<K, V> {
    fn subscribe(&mut self);
}
pub trait AssignedConsumerTrait<K, V>: Consumer<K, V> {
    fn assign(&mut self);
}

#[cfg(feature = "async_client")]
#[async_trait(?Send)]
pub trait AsyncConsumer<K, V> {
    async fn poll_async(&self) -> RecordStream<K, V>;
}

pub trait Consumer<K, V> {
    fn poll(&self) -> Vec<(K, V)>;
}