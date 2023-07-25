pub enum Error {
    Generic(String),
    Configuration(ConfigurationError),
    Consumer(ConsumerError),
    Producer(ProducerError),
    Kafka(KafkaError),
    StateStore(StateStoreError),
}

impl From<ConfigurationError> for Error {
    fn from(value: ConfigurationError) -> Self {
        Error::Configuration(value)
    }
}

impl From<StateStoreError> for Error {
    fn from(value: StateStoreError) -> Self {
        Error::StateStore(value)
    }
}

impl From<ConsumerError> for Error {
    fn from(value: ConsumerError) -> Self {
        Error::Consumer(value)
    }
}

impl From<ProducerError> for Error {
    fn from(value: ProducerError) -> Self {
        Error::Producer(value)
    }
}

impl From<KafkaError> for Error {
    fn from(value: KafkaError) -> Self {
        Error::Kafka(value)
    }
}

pub enum ConfigurationError {
    MalformedOption(String),
    MalformedOptions(Vec<String>),
}

pub enum StateStoreError {
    StateStoreFailed(String),
}

pub enum ConsumerError {}

pub enum ProducerError {}

pub enum KafkaError {
    ProducerError(ProducerError),
    ConsumerError(ConsumerError),
}

impl From<ConsumerError> for KafkaError {
    fn from(value: ConsumerError) -> Self {
        KafkaError::ConsumerError(value)
    }
}

impl From<ProducerError> for KafkaError {
    fn from(value: ProducerError) -> Self {
        KafkaError::ProducerError(value)
    }
}
