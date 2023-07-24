




pub struct ConsumerConfig {
    properties: Vec<ConsumerProperty>
}

impl ConsumerConfig {
    pub fn new() -> ConsumerConfig {
        ConsumerConfig {
            properties: vec![]
        }
    }
}

pub enum ConsumerProperty {

}