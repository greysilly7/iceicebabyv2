use chorus::instance::ChorusUser;
use chorus::types::GetUserGuildSchema;
use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Alignment, Length, Task};
use log::info;
use crate::{fetch_guilds, types, App};
use crate::types::message::Message;

pub fn main_view(app: &App) -> iced::widget::Container<types::message::Message> {
    let user_clone = app.user.clone();
    let _ = Task::perform(async move { fetch_guilds(user_clone.unwrap()).await }, Message::GuildsFetched);

    for guild in app.guilds.clone() {
        info!("{:?}", guild);
    }
    let sidebar = column![
        text("Channels").size(30).size(10),
        button("General")
            .padding(10)
            .on_press(types::message::Message::SwitchChannel(
                "General".to_string()
            )),
        button("Random")
            .padding(10)
            .on_press(types::message::Message::SwitchChannel("Random".to_string())),
        // Add more channels as needed
    ]
    .spacing(10)
    .width(Length::Fixed(200.0))
    .height(Length::Fill)
    .align_x(Alignment::Start);

    let chat_area = column![
        scrollable(column![
            // Add chat messages here
            text("User1: Hello!").size(10),
            text("User2: Hi there!").size(10),
            // Add more messages as needed
        ])
        .height(Length::Fill),
        // .padding(10),
        row![
            text_input("Type a message...", &app.message_input)
                .padding(10)
                .on_input(types::message::Message::MessageInputUpdate),
            button("Send")
                .padding(10)
                .on_press(types::message::Message::SendMessage),
        ]
        .spacing(10)
        .padding(10)
        .width(Length::Fill)
    ]
    .spacing(10)
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Alignment::Start);

    let body = row![sidebar, chat_area,]
        .spacing(10)
        .width(Length::Fill)
        .height(Length::Fill);

    container(body)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
}
