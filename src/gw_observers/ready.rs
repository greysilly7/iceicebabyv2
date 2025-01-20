use async_trait::async_trait;
use iced::futures::channel::mpsc;
use log::{info, error};
use pubserve::Subscriber;
use chorus::types::GatewayReady;

#[derive(Debug)]
pub struct ReadyEventObserver {
    pub queue: mpsc::Sender<GatewayReady>, // Sender to push events into the app
}

#[async_trait]
impl Subscriber<GatewayReady> for ReadyEventObserver {
    async fn update(&self, data: &GatewayReady) {        // Send the `GatewayReady` event to the channel
        if let Err(err) = self.queue.clone().try_send(data.clone()) {
            error!("Failed to send GatewayReady event: {}", err);
        } else {
            // Successful event logging
            info!("Ready event received and forwarded to the app");
        }
    }
}