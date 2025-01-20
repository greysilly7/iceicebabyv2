use std::sync::Arc;
use chorus::types::GatewayReady;
use iced::futures::{StreamExt, channel::mpsc, SinkExt, Stream};
use iced::{stream, Subscription};
use crate::types::message::Message;
use crate::App;

pub fn ready_event(app: App) -> Subscription<Message> {
    Subscription::run_with_id(std::any::TypeId::of::<GatewayReady>(),
    stream::channel(100, |mut output| async move {
        // Create a channel for forwarding GatewayReady events
        let (sender, mut receiver) = mpsc::channel::<GatewayReady>(100);

        // Set up the ReadyEventObserver with the sender part of the channel
        let gateway_ready_observer = Arc::new(crate::gw_observers::ready::ReadyEventObserver {
            queue: sender.clone(),
        });

        // Subscribe the observer to the session.ready events
        if let Some(user) = app.user.clone() {
            tokio::spawn(async move {
                user.gateway
                    .events
                    .lock()
                    .await
                    .session
                    .ready
                    .subscribe(gateway_ready_observer);
            });
        } else {
            log::error!("No user object - cannot subscribe ReadyEventObserver");
        }

        // Forward GatewayReady events from the receiver to the output stream
        while let Some(event) = receiver.next().await {
            if let Err(err) = output.send(Message::ReadyRecieved(event)).await {
                log::error!("Failed to send ReadyReceived message: {}", err);
                break; // Stop stream processing on error
            }
        }
    }))
}