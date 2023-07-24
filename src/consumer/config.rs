




pub struct ConsumerConfig {
    properties: Vec<ConsumerProperty>
}

impl ConsumerConfig {
    fn new() -> ConsumerConfig {
        ConsumerConfig {
            properties: vec![]
        }
    }
}

pub enum ConsumerProperty {

}