

pub struct UnallocatedConsumer;
pub struct SubscriberConsumer;
pub struct AssignedConsumer;

pub trait UnallocatedConsumerTrait {
    fn subscribe(self) -> Box<dyn SubscriberConsumerTrait>;
    fn assign(self) -> Box<dyn AssignedConsumerTrait>;
}
pub trait SubscriberConsumerTrait {
    fn subscribe(&mut self);
}
pub trait AssignedConsumerTrait {
    fn assign(&mut self);
}