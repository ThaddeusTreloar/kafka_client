use async_trait::async_trait;
use futures::Stream;

use crate::{common::{record::{RecordStream, ProducerRecord, RecordSet, Record}, topic::{TopicPartition, OptionalPartition}}, error::ProducerError};

pub trait Producer<K, V> {
}

#[cfg(feature = "async_client")]
#[async_trait(?Send)]
pub trait AsyncProducer<K, V> {
    async fn send(&self, record: ProducerRecord<K, V>) -> Result<ProducerRecord<K, V>, ProducerError>;
    async fn send_all(&self, records: RecordSet<ProducerRecord<K, V>>) -> RecordSet<Result<ProducerRecord<K, V>, ProducerError>>;
    async fn send_stream<T>(&self, records: T)
    -> RecordStream<Result<ProducerRecord<K, V>, ProducerError>>
    where T: Stream,
          T::Item: Record<K, V>;
}

pub trait SyncProducer<K, V> {

}


mod producer_internal_tests {
    use async_trait::async_trait;
    use futures::{stream::StreamExt, Stream};

    use crate::{common::record::{RecordStream, ProducerRecord, RecordSet, Record}, error::ProducerError};

    use super::AsyncProducer;

    struct SomeProducer {
        
    }

    impl SomeProducer {
        fn new() -> Self {
            Self {}
        }
    }

    #[async_trait(?Send)]
    impl AsyncProducer<String, String> for SomeProducer {
        async fn send(&self, record: ProducerRecord<String, String>) -> Result<ProducerRecord<String, String>, ProducerError> {
            unimplemented!()
        }
        async fn send_all(&self, records: RecordSet<ProducerRecord<String, String>>) -> RecordSet<Result<ProducerRecord<String, String>, ProducerError>> {
            unimplemented!()
        }
        async fn send_stream<T>(&self, records: T) -> RecordStream<Result<ProducerRecord<String, String>, ProducerError>> {
            unimplemented!()
        }
    }

    #[tokio::test]
    async fn do_stuff() {
        let s = RecordStream::<ProducerRecord<String, String>>::new().map(|r|r);

        let producer = SomeProducer::new();

        let out_stream = producer.send_stream(s);
    }
}