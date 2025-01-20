use std::sync::Arc;
use chorus::types::{GatewayReady, MessageCreate};
use iced::futures::{StreamExt, channel::mpsc, SinkExt};
use iced::{stream, Subscription};
use crate::types::message::Message;
use crate::App;

pub fn message_create_event(app: App) -> Subscription<Message> {
    Subscription::run_with_id(std::any::TypeId::of::<MessageCreate>(),
    stream::channel(100, |mut output| async move {
        // Create a channel for forwarding GatewayReady events
        let (sender, mut receiver) = mpsc::channel::<MessageCreate>(100);

        // Set up the ReadyEventObserver with the sender part of the channel
        let message_create_observer = Arc::new(crate::gw_observers::message_create::MessageCreateObserver {
            queue: sender.clone(),
        });

        // Subscribe the observer to the session.ready events
        if let Some(user) = app.user.clone() {
            tokio::spawn(async move {
                user.gateway
                    .events
                    .lock()
                    .await
                    .message
                    .create
                    .subscribe(message_create_observer);
            });
        } else {
            log::error!("No user object - cannot subscribe MessageCreateObserver");
        }

        // Forward GatewayReady events from the receiver to the output stream
        while let Some(event) = receiver.next().await {
            if let Err(err) = output.send(Message::MessageCreateReceived(event)).await {
                log::error!("Failed to send MessageCreate message: {}", err);
                break; // Stop stream processing on error
            }
        }
    }))
}