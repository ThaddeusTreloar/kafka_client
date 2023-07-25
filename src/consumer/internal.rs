use async_trait::async_trait;
use futures::{Stream, StreamExt};

use crate::internal::KafkaMessage;



pub struct UnallocatedConsumer;
pub struct SubscriberConsumer;
pub struct AssignedConsumer;

pub trait UnallocatedConsumerTrait {
    fn subscribe(self) -> Box<dyn SubscriberConsumerTrait>;
    fn assign(self) -> Box<dyn AssignedConsumerTrait>;
}
pub trait SubscriberConsumerTrait: Consumer {
    fn subscribe(&mut self);
}
pub trait AssignedConsumerTrait: Consumer {
    fn assign(&mut self);
}

#[cfg(feature = "async_components")]
#[async_trait]
pub trait AsyncConsumer {
    async fn poll_async(&self) -> dyn Iterator<Item = bool>;
}

pub trait Consumer {
    fn poll(&self);
}

pub struct MessageStream {

}

impl Stream for MessageStream {
    type Item = KafkaMessage;

    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        unimplemented!()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unimplemented!()
    }
}