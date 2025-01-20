mod types;
mod views;
mod gw_observers;
mod subscriptions;

use std::string::String;
use chorus::instance::ChorusUser;
use chorus::instance::Instance;
use chorus::types::{Guild, LoginSchema, MessageSendSchema, Snowflake};
use log::{info, LevelFilter};
use std::default::Default;
use iced::{Task, Theme};
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
    guilds: Option<Vec<Guild>>
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
                let current_guild = self.current_guild;
                let user = self.user.clone();
                let send_message = async move {
                    if let (Some(current_channel), Some(current_guild), Some(mut user)) = (current_channel, current_guild, user) {
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
            Message::ReadyRecieved(ready_event) => {
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
                info!("Updated messages: {:?}", messages);
            }
            Message::MessageSent => {
                self.message_input = String::default();
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

    fn subscription(&self) -> iced::Subscription<Message> {
        // Only activate the ready_event subscription if the user is logged in
        if self.user.is_some() {
            subscriptions::ready_event::ready_event(self.clone())
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
        .run_with(|| (App::new(), iced::Task::none()))

}

/*
async fn setup_observers(gateway: GatewayHandle) {
    let gateway_ready_observer = Arc::new(ReadyEventObserver {});
    gateway.events.lock().await.session.ready.subscribe(gateway_ready_observer);
}

 */
