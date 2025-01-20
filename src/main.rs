mod types;
mod views;
mod gw_observers;
mod subscriptions;

use std::string::String;
use chorus::instance::ChorusUser;
use chorus::instance::Instance;
use chorus::types::{Channel, GetChannelMessagesSchema, Guild, LoginSchema, MessageSendSchema, Snowflake};
use log::{debug, info, warn, LevelFilter};
use std::default::Default;
use iced::{Subscription, Task, Theme};
use crate::types::message::Message;

#[derive(Clone)]
struct App {
    current_view: View,
    username: String,
    password: String,
    instance_url: String,
    instance: Option<Instance>,
    user: Option<ChorusUser>,
    current_guild: Option<Snowflake>,
    current_channel: Option<Snowflake>,
    message_input: String,
    guilds: Option<Vec<Guild>>,
    messages: Option<Vec<chorus::types::Message>>
}

#[derive(Debug, Clone, Copy)]
enum View {
    Login,
    Register,
    MainView,
}



impl App {
    fn new() -> Self {
        Self {
            current_view: View::Login,
            // username: String::default(),
            // password: String::default(),
            // instance_url: String::default(),
            username: "greysilly7@gmail.com".to_string(),
            password: "#Hogan1975".to_string(),
            instance_url: "https://spacebar.greysilly7.xyz".to_string(),
            instance: None,
            user: None,
            current_channel: None,
            current_guild: None,
            message_input: String::default(),
            guilds: None,
            messages: None,
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Login => self.current_view = View::Login,
            Message::Register => self.current_view = View::Register,
            Message::MainView => self.current_view = View::MainView,
            Message::UsernameUpdate(username) => {
                self.username = username;
            }
            Message::PasswordUpdate(password) => {
                self.password = password;
            }
            Message::InstanceUrlUpdate(instance_url) => {
                self.instance_url = instance_url;
            }
            Message::SubmitLogin => {
                let instance_url = self.instance_url.clone();
                let username = self.username.clone();
                let password = self.password.clone();
                return Task::perform(
                    {
                        async move {
                            let mut instance = Instance::new(&instance_url, None)
                                .await
                                .expect("Failed to connect to the Spacebar server");
                            let login_schema = LoginSchema {
                                login: username,
                                password,
                                ..Default::default()
                            };

                            let mut chorus_user = instance
                                .login_account(login_schema)
                                .await
                                .expect("An error occurred during the login process");

                            let guilds = ChorusUser::get_guilds(&mut chorus_user, None).await;

                            (instance, chorus_user, guilds.ok())
                        }
                    },
                    |(instance, user, guilds)| Message::LoginSuccess(instance, user, guilds),
                );
            }
            Message::LoginSuccess(instance, user, guilds) => {
                self.instance = Some(instance);
                self.user = Some(user);
                self.guilds = guilds;
                self.current_view = View::MainView;
            }
            Message::Logout => {
                self.instance = None;
                self.user = None;
                self.current_view = View::Login;
            }
            Message::SwitchGuild(guild_id) => {
                info!("Switching Guilds");
                self.current_guild = Some(guild_id)
            },
            Message::SwitchChannel(channel_id) => {
                info!("Switching Channel");
                self.current_channel = Some(channel_id);
            },
            Message::SendMessage => {
                let message = self.message_input.clone();
                let current_channel = self.current_channel;
                let user = self.user.clone();
                let send_message = async move {
                    if let (Some(current_channel), Some(mut user)) = (current_channel, user) {
                        user.send_message(
                            MessageSendSchema {
                                content: Some(message.clone()),
                                ..Default::default()
                            },
                            current_channel,
                        ).await.unwrap();
                    }
                };
                return Task::perform(send_message, |_| Message::MessageSent);
            },
            Message::MessageInputUpdate(message) => {
                self.message_input = message;
            },
            Message::ReadyReceived(ready_event) => {
                // self.guilds = Some(ready_event.guilds.clone());
                info!("Ready event processed: ${:?}", ready_event);
            },
            Message::LogoutCleanup => {
                self.instance = None;
                self.user = None;
                self.guilds = None;
                self.current_view = View::Login;
            }
            Message::UpdateGuilds(guilds) => {
                self.guilds = Some(guilds);
            }
            Message::UpdateChannels(channels) => {
                // Optional: Add logic to update channels
                info!("Updated channels: {:?}", channels);
            }
            Message::UpdateMessages(messages) => {
                // Optional: Add logic to update messages
                self.messages = Some(messages.clone());
                info!("Updated messages: {:?}", messages);
            }
            Message::MessageSent => {
                self.message_input = String::default();
            }
            Message::MessageCreateReceived(message_create) => {
                info!("Message received: {:?}", message_create);

                // Step 1: Check if the user is logged in
                let current_user = match self.user.clone() {
                    Some(user) => user,
                    None => {
                        warn!("Received a message, but no user is logged in.");
                        return Task::none(); // Skip if no user is logged in
                    }
                };

                // Step 2: Check if we're in a guild and channel
                let (current_guild, current_channel) = match (self.current_guild, self.current_channel) {
                    (Some(guild), Some(channel)) => (guild, channel),
                    _ => {
                        warn!("Received a message, but the current guild or channel is missing.");
                        return Task::none(); // Skip if guild or channel is missing
                    }
                };

                // Step 3: Check if the message came from another user
                let message_author_id_result = message_create
                    .member
                    .as_ref()
                    .and_then(|member| member.user.as_ref())
                    .and_then(|user| user.read().ok())
                    .map(|user| user.id);

                let message_author_id = match message_author_id_result {
                    Some(id) => id,
                    None => {
                        warn!("Could not retrieve the message author's ID.");
                        return Task::none(); // Skip if author information is invalid
                    }
                };

                if message_author_id == current_user.object.read().unwrap().id {
                    debug!("Received a message from the current user. Ignoring.");
                    return Task::none(); // Skip if the message is from the current user
                }

                // Step 4: Fetch messages asynchronously and fire the update
                return Task::chain(
                    {
                        let user = self.user.clone();
                        Task::perform(
                            async move {
                                if let Some(mut chorus_user) = user {
                                    // Fetch channel messages
                                    Channel::messages(
                                        GetChannelMessagesSchema::before(Snowflake::generate()),
                                        current_channel,
                                        &mut chorus_user,
                                    )
                                        .await
                                        .unwrap_or_else(|err| {
                                            warn!("Failed to fetch messages: {:?}", err);
                                            Vec::new() // Default to an empty message list on failure
                                        })
                                } else {
                                    warn!("User context disappeared while processing messages.");
                                    Vec::new()
                                }
                            },
                            |messages| Message::UpdateMessages(messages), // Pass messages to the `UpdateMessages` event
                        )
                    },
                    Task::none()
                );
            }
    }

    Task::none()
    }

    fn view(&self) -> iced::Element<Message> {
        match self.current_view {
            View::Login => views::login::login_view(&self).into(),
            View::Register => todo!(),
            View::MainView => views::main_view::main_view(&self).into(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.user.is_some() {
            return Subscription::batch(vec![
                subscriptions::ready_event::ready_event(self.clone()),
                subscriptions::message_create::message_create_event(
                    self.clone())
            ]);
        } else {
            iced::Subscription::none()
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), iced::Error> {
    env_logger::builder()
        .default_format()
        .filter_module("iceicebabyv2", LevelFilter::Trace)
        .filter_module("chorus", LevelFilter::Trace)
        .try_init()
        .unwrap();

    iced::application("iceicebabyv2", App::update, App::view)
        .theme(|_state| Theme::CatppuccinMocha)
        .subscription(App::subscription)
        .run_with(|| (App::new(), Task::none()))

}

/*
async fn setup_observers(gateway: GatewayHandle) {
    let gateway_ready_observer = Arc::new(ReadyEventObserver {});
    gateway.events.lock().await.session.ready.subscribe(gateway_ready_observer);
}

 */
