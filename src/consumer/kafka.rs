use std::marker::PhantomData;

use async_trait::async_trait;

use super::{config::ConsumerConfig, internal::{SubscriberConsumer, AssignedConsumer, SubscriberConsumerTrait, AssignedConsumerTrait, UnallocatedConsumer, UnallocatedConsumerTrait, Consumer, AsyncConsumer}};


pub struct KafkaConsumer<T> {
    consumption_strategy: PhantomData<T>
}

type UnallocatedKafkaConsumer = KafkaConsumer<UnallocatedConsumer>;
type SubscriberKafkaConsumer = KafkaConsumer<SubscriberConsumer>;
type AssignedKakfaConsumer = KafkaConsumer<AssignedConsumer>;

impl UnallocatedKafkaConsumer {

}

impl From<ConsumerConfig> for UnallocatedKafkaConsumer {
    fn from(value: ConsumerConfig) -> Self {
        UnallocatedKafkaConsumer {
            consumption_strategy: PhantomData
        }
    }
}

impl UnallocatedConsumerTrait for UnallocatedKafkaConsumer {
    fn subscribe(self) -> Box<dyn SubscriberConsumerTrait> {
        Box::new(SubscriberKafkaConsumer {
            consumption_strategy: PhantomData
        })
    }

    fn assign(self) -> Box<dyn AssignedConsumerTrait> {
        Box::new(AssignedKakfaConsumer {
            consumption_strategy: PhantomData
        })
    }
}


impl SubscriberKafkaConsumer {
    
}

impl From<ConsumerConfig> for SubscriberKafkaConsumer {
    fn from(value: ConsumerConfig) -> Self {
        SubscriberKafkaConsumer {
            consumption_strategy: PhantomData
        }
    }
}

impl SubscriberConsumerTrait for SubscriberKafkaConsumer {
    fn subscribe(&mut self) {
        
    }
}

impl Consumer for SubscriberKafkaConsumer {
    fn poll(&self) {
        
    }
}

impl AssignedKakfaConsumer {
    
}


impl AssignedConsumerTrait for AssignedKakfaConsumer {
    fn assign(&mut self) {
        
    }
}

impl Consumer for AssignedKakfaConsumer {
    fn poll(&self) {
        
    }
}

#[cfg(feature = "async_components")]
#[async_trait]
impl AsyncConsumer for AssignedKakfaConsumer {
    async fn poll_async(&self) -> dyn Iterator<Item = bool> {
        unimplemented!()
    }
}

impl From<ConsumerConfig> for AssignedKakfaConsumer {
    fn from(value: ConsumerConfig) -> Self {
        AssignedKakfaConsumer {
            consumption_strategy: PhantomData
        }
    }
}