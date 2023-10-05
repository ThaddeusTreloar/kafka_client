use std::{collections::HashMap, fmt::Display, time::Duration, pin::Pin};

use async_trait::async_trait;
use futures::Stream;

use crate::{
    common::{
        record::{Offset, Record, RecordSet, RecordStream},
        topic::{
            OptionalPartition, Partition, TopicPartition, TopicPartitionList,
            TopicPartitionMetadataMap,
        },
    },
    consumer::common::ConsumerGroupMetadata,
    error::{
        CleanupError, ProducerError, ProducerMetadataError, ProducerSendError, TransactionError,
    },
    metadata::metrics::{Metric, MetricId},
};

use super::record::ProducerRecord;

struct Transactional;
struct NonTransactional;
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

#[async_trait]
pub trait TransactionalProducer<K, V> {
    fn begin_transaction(&self) -> Result<Transaction, TransactionError>;

    // Consider Revising name
    // TODO: revise API
    async fn send_stream_transaction<T>(
        &self,
        records: T,
    ) -> RecordStream<Result<ProducerRecord<K, V>, ProducerError>>
    where
        T: Stream + Send,
        T::Item: Record<K, V>;
}

pub trait Producer<K, V>: Drop {
    fn flush(&self);
}

#[cfg(feature = "async_client")]
#[async_trait]
pub trait AsyncProducer<K, V>: Drop {
    fn send_sync(
        &self,
        record: ProducerRecord<K, V>,
    ) -> Result<ProducerRecord<K, V>, ProducerSendError>; // TODO: investigate using Ok(RecordMetadata). What are the fields of the Java implementation of RecordMetadata?

    async fn send(
        &self,
        record: ProducerRecord<K, V>,
    ) -> Result<ProducerRecord<K, V>, ProducerSendError>; // TODO: investigate using Ok(RecordMetadata). What are the fields of the Java implementation of RecordMetadata?

    async fn send_with_callback<F>(
        &self,
        record: ProducerRecord<K, V>,
        cb: F,
    ) -> Result<ProducerRecord<K, V>, ProducerSendError>
    where
        F: Fn(
                Result<ProducerRecord<K, V>, ProducerSendError>,
            ) -> Result<ProducerRecord<String, String>, ProducerSendError>
            + Send;

    async fn send_all(
        &self,
        records: RecordSet<ProducerRecord<K, V>>,
    ) -> RecordSet<Result<ProducerRecord<K, V>, ProducerError>>;

    async fn send_stream(
        &self,
        records: RecordStream<ProducerRecord<K, V>>,
    ) -> Pin<Box<dyn Stream<Item = Result<ProducerRecord<String, String>, ProducerSendError>>+ Send +'life0>>;
    // Is this the best place for this?
    async fn get_partitions_for_topic(
        &self,
        topic: String,
    ) -> Result<TopicPartitionList<Partition>, ProducerMetadataError>;

    fn close(self, timeout: Duration) -> Result<(), CleanupError>;
    fn metrics(&self) -> Result<HashMap<MetricId, Metric>, CleanupError>;
}

mod producer_internal_tests {
    use std::{
        collections::{HashMap, VecDeque},
        marker::PhantomData,
        sync::Arc,
        thread::sleep,
        time::Duration, pin::Pin,
    };

    use async_trait::async_trait;
    use futures::{stream::StreamExt, Stream, stream::Map, Future};
    use log::{error, info};
    use tokio::{join, spawn};

    use crate::{
        common::{
            record::{Offset, Record, RecordSet, RecordStream},
            topic::{
                OptionalPartition, Partition, TopicPartition, TopicPartitionList,
                TopicPartitionMetadataMap,
            },
        },
        consumer::common::ConsumerGroupMetadata,
        error::{ProducerError, ProducerMetadataError, ProducerSendError, TransactionError},
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
        fn send_sync(
            &self,
            record: ProducerRecord<String, String>,
        ) -> Result<ProducerRecord<String, String>, ProducerSendError> {
            Ok(record)
        }

        async fn send(
            &self,
            record: ProducerRecord<String, String>,
        ) -> Result<ProducerRecord<String, String>, ProducerSendError> {
            Ok(record)
        }

        async fn send_with_callback<F>(
            &self,
            record: ProducerRecord<String, String>,
            cb: F,
        ) -> Result<ProducerRecord<String, String>, ProducerSendError>
        where
            F: Fn(
                    Result<ProducerRecord<String, String>, ProducerSendError>,
                ) -> Result<ProducerRecord<String, String>, ProducerSendError>
                + Send,
        {
            cb(Ok(record))
        }

        async fn send_all(
            &self,
            records: RecordSet<ProducerRecord<String, String>>,
        ) -> RecordSet<Result<ProducerRecord<String, String>, ProducerError>> {
            RecordSet::new()
        }
        
        async fn send_stream(
            &self,
            records: RecordStream<ProducerRecord<String, String>>,
        ) -> Pin<Box<dyn Stream<Item = Result<ProducerRecord<String, String>, ProducerSendError>> + Send +'life0>>
        {
            let f = |r: ProducerRecord<String, String>| {
                self.send_sync(r.into())
            };
            let m = records.map(f);
            m.boxed()
        }

        async fn get_partitions_for_topic(
            &self,
            topic: String,
        ) -> Result<TopicPartitionList<Partition>, ProducerMetadataError> {
            let mut test_list = TopicPartitionList::<Partition>::new();

            test_list.add_partition_range(&topic, 0..10);

            Ok(test_list)
        }

        fn close(self, timeout: std::time::Duration) -> Result<(), crate::error::CleanupError> {
            Ok(())
        }

        fn metrics(
            &self,
        ) -> Result<
            HashMap<crate::producer::common::MetricId, crate::producer::common::Metric>,
            crate::error::CleanupError,
        > {
            Ok(HashMap::new())
        }
    }

    #[async_trait]
    impl TransactionalProducer<String, String> for SomeProducer<Transactional> {
        fn begin_transaction(&self) -> Result<Transaction, TransactionError> {
            Ok(Transaction::new())
        }

        async fn send_stream_transaction<U>(
            &self,
            records: U,
        ) -> RecordStream<Result<ProducerRecord<String, String>, ProducerError>>
        where
            U: Stream + Send,
            U::Item: Record<String, String>,
        {
            RecordStream::new()
        }
    }

    #[tokio::test]
    async fn testing_ergonomics() {
        std_logger::Config::logfmt().with_call_location(true).init();

        let producer = SomeProducer::new_transactional();

        let rs_records = vec![
            ProducerRecord::<String, String>::from_key_value("one".into(), "SomeData".into())
                .with_topic("SomeTopic")
                .into_record(),
            ProducerRecord::<String, String>::from_key_value("two".into(), "OtherData".into())
                .with_topic("SomeTopic")
                .into_record(),
            ProducerRecord::<String, String>::from_key_value("three".into(), "MoreData".into())
                .with_topic("SomeTopic")
                .into_record(),
        ];

        let producer_ref = Arc::new(producer);
        let rs = RecordStream::from(rs_records);
        let rs_channel = rs.get_channel();

        let rs_join_handle = spawn(
            async move {
                producer_ref
                .send_stream(rs)
                .await
                .for_each(|r| async {
                    match r {
                        Ok(r) => {
                            info!("Successful Stream Item: {:?}", r);
                        }
                        Err(e) => {
                            error!("Failed Stream Item: {}", e);
                        }
                    }
                }).await;
            }
        );

        for n in 0..20 {
            match rs_channel.send(
                ProducerRecord::from_key_value(format!("key{}", n), format!("value{}", n))
                    .with_topic("AnotherTopic")
                    .into_record(),
            ) {
                Ok(_) => {
                    info!("Success");
                }
                Err(e) => {
                    error!("Error: {}", e);
                }
            };
        }

        match rs_join_handle.await {
            Ok(_) => {
                info!("Success");
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        }
    }
}
