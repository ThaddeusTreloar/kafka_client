use std::{
    collections::{HashMap, VecDeque},
    marker::PhantomData,
    sync::Arc,
    thread::sleep,
    time::Duration, pin::Pin,
};

use async_trait::async_trait;
use futures::{stream::StreamExt, Stream, stream::Map, Future, Sink, sink::SinkExt, TryStream};
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
    transaction_state: PhantomData<T>,
}

impl SomeProducer<NonTransactional> {
    fn new() -> Self {
        Self {
            transaction_state: PhantomData,
        }
    }
}

impl SomeProducer<Transactional> {
    fn new_transactional() -> Self {
        Self {
            transaction_state: PhantomData,
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

impl <T> Sink<ProducerRecord<String, String>> for SomeProducer<T>
where
    T: Sync,
{
    type Error = ProducerSendError;

    fn poll_ready(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn start_send(
        self: Pin<&mut Self>,
        item: ProducerRecord<String, String>,
    ) -> Result<(), Self::Error> {
        info!("ProducerRecord: {}", item);
        Ok(())
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn poll_close(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }
}

#[async_trait]
impl<T> AsyncProducer<String, String> for SomeProducer<T>
where
    T: Sync,
{
    //fn send_sync(
    //    &self,
    //    record: ProducerRecord<String, String>,
    //) -> Result<ProducerRecord<String, String>, ProducerSendError> {
    //    Ok(record)
    //}
//
    //async fn send(
    //    &self,
    //    record: ProducerRecord<String, String>,
    //) -> Result<ProducerRecord<String, String>, ProducerSendError> {
    //    Ok(record)
    //}

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

    async fn send_set(
        &self,
        records: RecordSet<ProducerRecord<String, String>>,
    ) -> RecordSet<Result<ProducerRecord<String, String>, ProducerError>> {
        RecordSet::new()
    }
    
    //async fn send_stream(
    //    &self,
    //    records: RecordStream<ProducerRecord<String, String>>,
    //) -> Pin<Box<dyn Stream<Item = Result<ProducerRecord<String, String>, ProducerSendError>> + Send +'life0>>
    //{
    //    let f = |r: ProducerRecord<String, String>| {
    //        self.send_sync(r.into())
    //    };
    //    let m = records.map(f);
    //    m.boxed()
    //}

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

    //async fn send_stream_transaction<U>(
    //    &self,
    //    records: U,
    //) -> RecordStream<Result<ProducerRecord<String, String>, ProducerError>>
    //where
    //    U: Stream + Send,
    //    U::Item: Record<String, String>,
    //{
    //    RecordStream::new()
    //}
}

#[tokio::test]
async fn testing_ergonomics() {
    std_logger::Config::logfmt().with_call_location(true).init();

    let mut producer = SomeProducer::new_transactional();

    let rs_records: Vec<Result<ProducerRecord<String, String>, ProducerSendError>> = vec![
        ProducerRecord::<String, String>::from_key_value("one".into(), "SomeData".into())
            .with_topic("SomeTopic")
            .into_record(),
        ProducerRecord::<String, String>::from_key_value("two".into(), "OtherData".into())
            .with_topic("SomeTopic")
            .into_record(),
        ProducerRecord::<String, String>::from_key_value("three".into(), "MoreData".into())
            .with_topic("SomeTopic")
            .into_record(),
    ].into_iter()
    .map(
        |r| Ok(r)
    ).collect();

    //let producer_ref = Arc::new(producer);
    let mut rs = RecordStream::from(rs_records);
    let rs_channel = rs.get_channel();

    let rs_join_handle = spawn(
        async move {
            producer
            .send_all(&mut rs)
            .await
            //.filter_map(
            //    |r| async {
            //        match r {
            //            Ok(pr) => Some(pr),
            //            Err(e) => {
            //                error!("Error: {}", e);
            //                None
            //            }
            //        }
            //    }
            //)
            //.for_each(|r| async move {
            //    info!("Successful Stream Item: {:?}", r);
            //}).await;
        }
    );

    for n in 0..20 {
        match rs_channel.send(
            Ok(ProducerRecord::from_key_value(format!("key{}", n), format!("value{}", n))
                .with_topic("AnotherTopic")
                .into_record()),
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
