use std::fmt::Display;

use async_trait::async_trait;
use futures::Stream;

use crate::{
    common::{
        record::{Offset, Record, RecordSet, RecordStream},
        topic::{OptionalPartition, Partition, TopicPartition, TopicPartitionMetadataMap, TopicPartitionList},
    },
    consumer::common::ConsumerGroupMetadata,
    error::{ProducerError, TransactionError, ProducerMetadataError},
};

use super::record::ProducerRecord;

struct Transactional;
struct NonTransactional;

pub trait Producer<K, V> {}

pub struct Transaction {}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Transaction")
    }
}

impl Transaction {
    pub fn new() -> Self {
        Self {}
    }

    pub fn abort_transaction(self) -> Result<(), TransactionError> {
        Ok(())
    }

    pub fn commit_transaction(self) -> Result<(), TransactionError> {
        Ok(())
    }

    pub fn push_offsets(
        &self,
        offsets: TopicPartitionMetadataMap<Offset>,
        group_metadata: ConsumerGroupMetadata,
    ) -> Result<TopicPartitionMetadataMap<Offset>, TransactionError> {
        Ok(offsets)
    }
}

pub trait TransactionalProducer {
    fn begin_transaction(&self) -> Result<Transaction, TransactionError>;
}

#[cfg(feature = "async_client")]
#[async_trait]
pub trait AsyncProducer<K, V>: Drop {
    async fn send(
        &self,
        record: ProducerRecord<K, V>,
    ) -> Result<ProducerRecord<K, V>, ProducerError>; // TODO: investigate using Ok(RecordMetadata). What are the fields of the Java implementation of RecordMetadata?
    async fn send_all(
        &self,
        records: RecordSet<ProducerRecord<K, V>>,
    ) -> RecordSet<Result<ProducerRecord<K, V>, ProducerError>>;
    async fn send_stream<T>(
        &self,
        records: T,
    ) -> RecordStream<Result<ProducerRecord<K, V>, ProducerError>>
    where
        T: Stream + Send,
        T::Item: Record<K, V>;

    async fn get_partitions_for_topic(&self, topic: String) -> Result<TopicPartitionList<Partition>, ProducerMetadataError>;
}

pub trait SyncProducer<K, V>: Drop {}

mod producer_internal_tests {
    use std::{marker::PhantomData, sync::Arc};

    use async_trait::async_trait;
    use futures::{stream::StreamExt, Stream};
    use log::{error, info};
    use tokio::{join, spawn};

    use crate::{
        common::{
            record::{Record, RecordSet, RecordStream, Offset},
            topic::{OptionalPartition, Partition, TopicPartition, TopicPartitionMetadataMap, TopicPartitionList},
        },
        consumer::common::ConsumerGroupMetadata,
        error::{ProducerError, TransactionError, ProducerMetadataError},
        producer::record::ProducerRecord,
    };

    use super::{
        AsyncProducer, NonTransactional, Transaction, Transactional, TransactionalProducer,
    };

    struct SomeProducer<T>
    where
        T: Sync,
    {
        transactional: PhantomData<T>,
    }

    impl SomeProducer<NonTransactional> {
        fn new() -> Self {
            Self {
                transactional: PhantomData,
            }
        }
    }

    impl SomeProducer<Transactional> {
        fn new_transactional() -> Self {
            Self {
                transactional: PhantomData,
            }
        }
    }

    impl<T> Drop for SomeProducer<T>
    where
        T: Sync,
    {
        fn drop(&mut self) {
            info!("Closing producer");
        }
    }

    #[async_trait]
    impl<T> AsyncProducer<String, String> for SomeProducer<T>
    where
        T: Sync,
    {
        async fn send(
            &self,
            record: ProducerRecord<String, String>,
        ) -> Result<ProducerRecord<String, String>, ProducerError> {
            Err(ProducerError::Unknown)
        }
        async fn send_all(
            &self,
            records: RecordSet<ProducerRecord<String, String>>,
        ) -> RecordSet<Result<ProducerRecord<String, String>, ProducerError>> {
            RecordSet::new()
        }
        async fn send_stream<U>(
            &self,
            records: U,
        ) -> RecordStream<Result<ProducerRecord<String, String>, ProducerError>>
        where
            U: Send,
        {
            RecordStream::new()
        }

        async fn get_partitions_for_topic(&self, topic: String) -> Result<TopicPartitionList<Partition>, ProducerMetadataError> {

            let mut test_list = TopicPartitionList::<Partition>::new();

            test_list.add_partition_range(&topic, 0..10);

            Ok(test_list)
        }
    }

    impl TransactionalProducer for SomeProducer<Transactional> {
        fn begin_transaction(&self) -> Result<Transaction, TransactionError> {
            Ok(Transaction::new())
        }
    }

    #[tokio::test]
    async fn testing_ergonomics() {
        std_logger::Config::logfmt().with_call_location(true).init();

        let s = RecordStream::<ProducerRecord<String, String>>::new().map(|r| r);

        let producer = SomeProducer::new_transactional();

        if let Ok(list) = producer.get_partitions_for_topic("SomeTopic".into()).await {

            for tp in list {
                info!("Topic: {}, Partition: {}", tp.topic(), tp.partition());
            }
        }

        let transaction = match producer.begin_transaction() {
            Ok(t) => {
                info!("{}", t);
                t
            },
            Err(e) => {
                error!("Error: {}", e);
                return;
            }
        };

        let arc = Arc::new(producer);

        let out_producer = arc.clone();
        let out_stream = spawn(async move { out_producer.send_stream(s).await });

        match transaction.push_offsets(
            TopicPartitionMetadataMap::from((TopicPartition::from(("", &5)), 1)),
            ConsumerGroupMetadata::builder()
                .with_group_id("test".into())
                .with_member_id("member_id".into())
                .with_generation_id(1)
                .into_consumer_group_metadata(),
        ) {
            Ok(_) => {}
            Err(e) => {
                error!("Error: {}", e);
            }
        }

        match transaction.abort_transaction() {
            Ok(_) => {}
            Err(e) => {
                error!("Error: {}", e);
            }
        }

        match join!(out_stream) {
            (Ok(r),) => {
                info!("Success: {:?}", r);
            }
            (Err(e),) => {
                error!("Error: {}", e);
            }
        };
    }
}

/*
void 	close()
void 	close(Duration timeout)
void 	flush()
Map<MetricName,? extends Metric> 	metrics()
List<PartitionInfo> 	partitionsFor(String topic)
Future<RecordMetadata> 	send(ProducerRecord<K,V> record, Callback callback)
 */
