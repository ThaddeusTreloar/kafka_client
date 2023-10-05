use std::{marker::PhantomData, collections::VecDeque, pin::Pin, task::{Context, Poll, Waker}, ops::{DerefMut, Deref}, sync::{Arc, Mutex, mpsc}, thread::{spawn, JoinHandle}};

use async_trait::async_trait;
use chrono::{offset, DateTime};
use futures::{Stream, StreamExt, TryStream};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::pin::pin;

use crate::error;

use super::topic::{OptionalPartition, Partition, TopicPartition};

pub type Offset = i64;
pub type RecordMetadata = ();

pub enum Position {
    Beginning,
    End,
    Offset(Offset),
}

pub struct RecordSet<T> {
    records: Vec<T>,
}

impl<T> RecordSet<T> {
    pub fn new() -> Self {
        Self { records: Vec::new() }
    }
}

impl <T> From<Vec<T>> for RecordSet<T> {
    fn from(records: Vec<T>) -> Self {
        Self { records }
    }
}

impl<T> Iterator for RecordSet<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct RecordStream<T> {
    a: Arc<Mutex<VecDeque<T>>>,
    sender: mpsc::Sender<T>,
    //reciever: mpsc::Receiver<T>,
    waker: Arc<Mutex<Option<Waker>>>,
    //channel_thread: JoinHandle<()>, T
}

/*impl <T> TryStream for RecordStream<T> {
    type Ok = T;
    type Error = error::Error;

    fn try_poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Ok, Self::Error>>> {
        let mut_ref = self.get_mut();
        if let Some(item) = mut_ref.pop() {
            std::task::Poll::Ready(Some(Ok(item)))
        } else {
            mut_ref.set_waker(cx.waker());
            std::task::Poll::Pending
        }
    }
}*/

impl <T> Unpin for RecordStream<T> {}

impl<T> RecordStream<T>
where T: Send + 'static
{
    pub fn new() -> Self {
        let (sender, reciever) = mpsc::channel();
        let queue = Arc::new(Mutex::new(VecDeque::new()));
        let waker: Arc<Mutex<Option<Waker>>> = Arc::new(Mutex::new(None));

        let queue_ref = queue.clone();
        let waker_ref = waker.clone();

        spawn(move || {
            let reciever = reciever;
            let queue = queue_ref;
            let waker = waker_ref;

            loop {
                let item = match reciever.recv() {
                    Ok(item) => item,
                    Err(e) => {
                        info!("Channel Closed: {}", e);
                        break;
                    }
                };

                let mut queue_lock = queue.lock().unwrap();
                queue_lock.push_back(item);

                let mut waker_lock = waker.lock().unwrap();

                if let Some(waker) = waker_lock.take() {
                    info!("Waking up stream");
                    waker.wake_by_ref();
                }
            }
        });
        
        Self { a: queue, sender, waker: waker,}// channel_thread: thread }
    }
    
    // Preferably replace this with a channel
    pub fn get_channel(&self) -> mpsc::Sender<T> {
        self.sender.clone()
    }

    pub fn set_waker(&mut self, waker: &Waker) {
        let mut lock = self.waker.lock().unwrap();
        lock.replace(waker.clone());
        info!("Waker set");
    }

    fn push(&mut self, item: T) {
        let mut lock = self.a.lock().unwrap();
        lock.push_back(item);
    }

    fn pop(&mut self) -> Option<T> {
        let mut lock = self.a.lock().unwrap();
        lock.pop_front()
    }
}

impl <T> From<Vec<T>> for RecordStream<T>
where T: Send + 'static
{
    fn from(a: Vec<T>) -> Self {
        let s = Self::new();
        let sender = s.get_channel();

        for item in a {
            match sender.send(item) {
                Ok(_) => {},
                Err(e) => {
                    error!("Error sending item to channel: {}", e);
                }
            };
        }

        s
    }
}

// Some way to guarentee key ordering needs to be developed.
impl<T> Stream for RecordStream<T>
where T: Send + 'static
{
    // TODO
    type Item = T;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let mut_ref = self.get_mut();
        if let Some(item) = mut_ref.pop() {
            std::task::Poll::Ready(Some(item))
        } else {
            mut_ref.set_waker(cx.waker());
            std::task::Poll::Pending
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unimplemented!()
    }
}

/*
    TODO: The generic trait that allows streams to be comopose of record streams.
*/
pub trait Record<K, V>
{
    fn key(&self) -> Option<&K>;
    fn value(&self) -> Option<&V>;
    fn timestamp(&self) -> Option<i64>;
    fn topic(&self) -> &str;
}

pub struct Headers<'a, K, V>
where
    K: Serialize + Deserialize<'a>,
    V: Serialize + Deserialize<'a>,
{
    headers: &'a Vec<(K, V)>,
}
