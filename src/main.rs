mod types;
mod views;

use std::string::String;
use chorus::instance::ChorusUser;
use chorus::instance::Instance;
use chorus::types::{Guild, LoginSchema};
use log::{info, LevelFilter};
use std::default::Default;
use chorus::errors::ChorusResult;
use iced::Task;
use crate::types::message::Message;

struct App {
    current_view: View,
    username: String,
    password: String,
    instance_url: String,
    instance: Option<Instance>,
    user: Option<ChorusUser>,
    current_channel: String,
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
            username: "".to_string(),
            password: "".to_string(),
            instance_url: "".to_string(),
            instance: None,
            user: None,
            current_channel: "General".to_string(),
            message_input: "".to_string(),
            guilds: None,
        }
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Login => self.current_view = View::Login,
            Message::Register => self.current_view = View::Register,
            Message::MainView => self.current_view = View::MainView,
            Message::UsernameUpdate(username) => {
                self.username = username;
                info!("Username updated");
            }
            Message::PasswordUpdate(password) => {
                self.password = password;
                info!("Password updated");
            }
            Message::InstanceUrlUpdate(instance_url) => {
                self.instance_url = instance_url;
                info!("Instance URL updated");
            }
            Message::SubmitLogin => {
                let instance_url = self.instance_url.clone();
                let username = self.username.clone();
                let password = self.password.clone();
                return iced::Task::perform(
                    async move {
                        let mut instance = Instance::new(&instance_url, None)
                            .await
                            .expect("Failed to connect to the Spacebar server");
                        let login_schema = LoginSchema {
                            login: username.clone(),
                            password: password.clone(),
                            ..Default::default()
                        };

                        let chorus_user = instance
                            .login_account(login_schema)
                            .await
                            .expect("An error occurred during the login process");



                        (instance, chorus_user)
                    },
                    |(instance, user)| Message::LoginSuccess(instance, user),
                );
            }
            Message::LoginSuccess(instance, user) => {
                self.instance = Some(instance);
                self.user = Some(user);
                self.current_view = View::MainView;
            }
            Message::Logout => {
                self.instance = None;
                self.user = None;
                self.current_view = View::Login;
            }
            Message::SwitchChannel(_) => todo!(),
            Message::SendMessage => todo!(),
            Message::GuildsFetched(guilds) => {
                info!("Guilds fetched: {:?}", guilds);
                self.guilds = guilds.ok();
            }
            Message::FetchGuilds => {
                let mut chorus_user = self.user.clone().unwrap();
                return iced::Task::perform(async move { fetch_guilds(chorus_user).await }, Message::GuildsFetched);
            }
            Message::MessageInputUpdate(_) => {}
        }

        iced::Task::none()
    }

    fn view(&self) -> iced::Element<Message> {
        match self.current_view {
            View::Login => views::login::login_view(&self).into(),
            View::Register => todo!(),
            View::MainView => views::main_view::main_view(&self).into(),
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

    iced::application("Discord-like App", App::update, App::view)
        .run_with(|| (App::new(), iced::Task::none()))
}

async fn fetch_guilds(mut chorus_user: ChorusUser) -> ChorusResult<Vec<Guild>> {
    return ChorusUser::get_guilds(&mut chorus_user, None).await;
}