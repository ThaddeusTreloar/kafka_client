use std::{marker::PhantomData, vec};

use async_trait::async_trait;

use crate::common::record::RecordStream;

use super::{
    config::ProducerConfig,
    internal::{
        AssignedProducer, AssignedProducerTrait, AsyncProducer, Producer, SubscriberProducer,
        SubscriberProducerTrait, UnallocatedProducer, UnallocatedProducerTrait,
    },
};

pub struct KafkaProducer<K, V, T> {
    key: Option<K>,
    value: Option<V>,
    consumption_strategy: PhantomData<T>,
}

type UnallocatedKafkaProducer<K, V> = KafkaProducer<K, V, UnallocatedProducer>;
type SubscriberKafkaProducer<K, V> = KafkaProducer<K, V, SubscriberProducer>;
type AssignedKakfaProducer<K, V> = KafkaProducer<K, V, AssignedProducer>;

impl<K, V> UnallocatedKafkaProducer<K, V> {}

impl<K, V> From<ProducerConfig> for UnallocatedKafkaProducer<K, V> {
    fn from(value: ProducerConfig) -> Self {
        UnallocatedKafkaProducer {
            key: None,
            value: None,
            consumption_strategy: PhantomData,
        }
    }
}

impl<K, V> UnallocatedProducerTrait<K, V> for UnallocatedKafkaProducer<K, V>
where
    K: 'static,
    V: 'static, // TODO : Refactor to remove static
{
    fn subscribe(self) -> Box<dyn SubscriberProducerTrait<K, V>> {
        Box::new(SubscriberKafkaProducer {
            key: None::<K>,
            value: None::<V>,
            consumption_strategy: PhantomData,
        })
    }

    fn assign(self) -> Box<dyn AssignedProducerTrait<K, V>> {
        Box::new(AssignedKakfaProducer {
            key: None::<K>,
            value: None::<V>,
            consumption_strategy: PhantomData,
        })
    }
}

impl<K, V> SubscriberKafkaProducer<K, V> {}

impl<K, V> From<ProducerConfig> for SubscriberKafkaProducer<K, V> {
    fn from(value: ProducerConfig) -> Self {
        SubscriberKafkaProducer {
            key: None::<K>,
            value: None::<V>,
            consumption_strategy: PhantomData,
        }
    }
}

impl<K, V> SubscriberProducerTrait<K, V> for SubscriberKafkaProducer<K, V> {
    fn subscribe(&mut self) {}
}

impl<K, V> Producer<K, V> for SubscriberKafkaProducer<K, V> {
    fn poll(&self) -> Vec<(K, V)> {
        vec![]
    }
}

impl<K, V> AssignedKakfaProducer<K, V> {}

impl<K, V> AssignedProducerTrait<K, V> for AssignedKakfaProducer<K, V> {
    fn assign(&mut self) {}
}

impl<K, V> Producer<K, V> for AssignedKakfaProducer<K, V> {
    fn poll(&self) -> Vec<(K, V)> {
        vec![]
    }
}

#[cfg(feature = "async_client")]
#[async_trait(?Send)]
impl<K, V> AsyncProducer<K, V> for AssignedKakfaProducer<K, V> {
    async fn poll_async(&self) -> RecordStream<K, V> {
        unimplemented!()
    }
}

impl<K, V> From<ProducerConfig> for AssignedKakfaProducer<K, V> {
    fn from(value: ProducerConfig) -> Self {
        AssignedKakfaProducer {
            key: None::<K>,
            value: None::<V>,
            consumption_strategy: PhantomData,
        }
    }
}
