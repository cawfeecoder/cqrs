use anyhow::anyhow;
use async_trait::async_trait;
use crossbeam_channel::{unbounded, Receiver};

use crate::context::common::application::ports::outbound::event_bus::EventBus;

pub struct ChannelBus<T> {
    receiver: crossbeam_channel::Receiver<T>,
    sender: crossbeam_channel::Sender<T>,
}

impl<T> ChannelBus<T> {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        return Self {
            receiver: rx,
            sender: tx,
        };
    }
}

#[async_trait]
impl<T> EventBus<T, T> for ChannelBus<T> {
    fn send_event(&self, event: T) -> Result<(), anyhow::Error> {
        self.sender.try_send(event).map_err(|_e| anyhow!("Unknown"))
    }

    fn receive_events(&self) -> Receiver<T> {
        return self.receiver.clone();
    }
}
