use std::{marker::PhantomData, vec};

use async_trait::async_trait;

use crate::common::record::RecordStream;

use super::{config::ConsumerConfig, common::{SubscriberConsumer, AssignedConsumer, SubscriberConsumerTrait, AssignedConsumerTrait, UnallocatedConsumer, UnallocatedConsumerTrait, Consumer, AsyncConsumer}};


pub struct KafkaConsumer<K, V, T>
{
    key: Option<K>,
    value: Option<V>,
    consumption_strategy: PhantomData<T>
}

type UnallocatedKafkaConsumer<K, V> = KafkaConsumer<K, V, UnallocatedConsumer>;
type SubscriberKafkaConsumer<K, V> = KafkaConsumer<K, V, SubscriberConsumer>;
type AssignedKakfaConsumer<K, V> = KafkaConsumer<K, V, AssignedConsumer>;

impl <K, V> UnallocatedKafkaConsumer<K, V> {

}

impl <K, V> From<ConsumerConfig> for UnallocatedKafkaConsumer<K, V> {
    fn from(value: ConsumerConfig) -> Self {
        UnallocatedKafkaConsumer {
            key: None,
            value: None,
            consumption_strategy: PhantomData
        }
    }
}

impl <K, V> UnallocatedConsumerTrait<K, V> for UnallocatedKafkaConsumer<K, V>
where K: 'static, V: 'static // TODO : Refactor to remove static
{
    fn subscribe(self) -> Box<dyn SubscriberConsumerTrait<K, V>> {
        Box::new(SubscriberKafkaConsumer {
            key: None::<K>,
            value: None::<V>,
            consumption_strategy: PhantomData
        })
    }

    fn assign(self) -> Box<dyn AssignedConsumerTrait<K, V>> {
        Box::new(AssignedKakfaConsumer {
            key: None::<K>,
            value: None::<V>,
            consumption_strategy: PhantomData
        })
    }
}


impl <K, V> SubscriberKafkaConsumer<K, V> {
    
}

impl<K, V> From<ConsumerConfig> for SubscriberKafkaConsumer<K, V> {
    fn from(value: ConsumerConfig) -> Self {
        SubscriberKafkaConsumer {
            key: None::<K>,
            value: None::<V>,
            consumption_strategy: PhantomData
        }
    }
}

impl<K, V> SubscriberConsumerTrait<K, V> for SubscriberKafkaConsumer<K, V> {
    fn subscribe(&mut self) {
        
    }
}

impl<K, V> Consumer<K, V> for SubscriberKafkaConsumer<K, V> {
    fn poll(&self) -> Vec<(K, V)> {
        vec![]
    }
}

impl<K, V> AssignedKakfaConsumer<K, V> {
    
}


impl<K, V> AssignedConsumerTrait<K, V> for AssignedKakfaConsumer<K, V> {
    fn assign(&mut self) {
        
    }
}

impl<K, V> Consumer<K, V> for AssignedKakfaConsumer<K, V> {
    fn poll(&self) -> Vec<(K, V)> {
        vec![]
    }
}

#[cfg(feature = "async_client")]
#[async_trait(?Send)]
impl <K, V> AsyncConsumer<K, V> for AssignedKakfaConsumer<K, V> {
    async fn poll_async(&self) -> RecordStream<K, V> {
        unimplemented!()
    }
}

impl <K, V> From<ConsumerConfig> for AssignedKakfaConsumer<K, V> {
    fn from(value: ConsumerConfig) -> Self {
        AssignedKakfaConsumer {
            key: None::<K>,
            value: None::<V>,
            consumption_strategy: PhantomData
        }
    }
}