use async_trait::async_trait;
use iced::futures::channel::mpsc;
use log::{info, error};
use pubserve::Subscriber;
use chorus::types::MessageCreate;

#[derive(Debug)]
pub struct MessageCreateObserver {
    pub queue: mpsc::Sender<MessageCreate>, // Sender to push events into the app
}

#[async_trait]
impl Subscriber<MessageCreate> for MessageCreateObserver {
    async fn update(&self, data: &MessageCreate) {        // Send the `GatewayReady` event to the channel
        if let Err(err) = self.queue.clone().try_send(data.clone()) {
            error!("Failed to send MessageCreate event: {}", err);
        } else {
            // Successful event logging
            info!("MessageCreate event received and forwarded to the app");
        }
    }
}