use iced::widget::{button, column, container, text, text_input, Space};
use iced::{Alignment, Length};

use crate::{types, App};

pub fn login_view(app: &App) -> iced::widget::Container<types::message::Message> {
    let body = column![
        text("Login")
            .size(40)
            .align_x(iced::alignment::Horizontal::Center),
        Space::with_height(Length::Fixed(20.0)),
        text_input("Email", &app.username)
            .padding(10)
            .on_input(types::message::Message::UsernameUpdate),
        Space::with_height(Length::Fixed(10.0)),
        text_input("Password", &app.password)
            .padding(10)
            .on_input(types::message::Message::PasswordUpdate),
        Space::with_height(Length::Fixed(10.0)),
        text_input("Instance URL", &app.instance_url)
            .padding(10)
            .on_input(types::message::Message::InstanceUrlUpdate),
        Space::with_height(Length::Fixed(20.0)),
        button("Login")
            .padding(10)
            .on_press(types::message::Message::SubmitLogin),
    ]
    .spacing(20)
    .align_x(Alignment::Center);

    container(body)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
}
