use chorus::types::{Channel, GetChannelMessagesSchema, Snowflake};
use iced::{futures, Alignment, Border, Color, Length, Vector};
use iced::widget::{button, column, row, container, scrollable, text, text_input};
use log::info;
use crate::App;
use crate::types::message::Message;

pub fn main_view(app: &App) -> iced::widget::Container<Message> {
    let guilds_column = build_guilds_column(app);
    let channels_column = build_channels_column(app);
    let sidebar = row![guilds_column, channels_column]
        .spacing(10)
        .height(Length::Fill)
        .width(Length::Fixed(240.0));

    let chat_area = build_chat_area(app);

    let body = row![sidebar, chat_area]
        .spacing(10)
        .width(Length::Fill)
        .height(Length::Fill);

    container(body)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
}

fn build_guilds_column(app: &App) -> iced::widget::Column<Message> {
    column![
        text("Guilds").size(30),
        app.guilds.as_ref().map_or_else(
            || column![text("No guilds available").size(20)],
            |guilds| {
                column(
                    guilds.iter().map(|guild| {
                        button(text(guild.name.clone().unwrap_or_else(|| "Unnamed Guild".to_string())))
                            .padding(10)
                            .on_press(Message::SwitchGuild(guild.id))
                            .into()
                    }).collect::<Vec<_>>()
                )
            }
        )
    ]
        .spacing(10)
        .width(Length::Fixed(120.0))
        .height(Length::Fill)
        .align_x(Alignment::Start)
}

// Build columns for channels
fn build_channels_column(app: &App) -> iced::widget::Column<Message> {
    app.current_guild
        .and_then(|current_guild_id| {
            app.guilds.as_ref().and_then(|guilds| {
                guilds.iter().find(|guild| guild.id == current_guild_id).map(|guild| {
                    column(
                        futures::executor::block_on(guild.channels(&mut app.user.clone().unwrap()))
                            .unwrap_or_default()
                            .iter()
                            .map(|channel| {
                                button(text(channel.name.clone().unwrap_or_else(|| "Unnamed Channel".to_string())))
                                    .padding(5)
                                    .on_press(Message::SwitchChannel(channel.id))
                                    .into()
                            })
                            .collect::<Vec<_>>()
                    )
                })
            })
        })
        .unwrap_or_else(|| column![text("No guild selected").size(15)])
        .spacing(5)
        .width(Length::Fixed(120.0))
        .height(Length::Fill)
        .align_x(Alignment::Start)
}

fn build_chat_area(app: &App) -> iced::widget::Column<Message> {
    if let Some(current_channel) = app.current_channel {
        let messages_result = futures::executor::block_on(
            Channel::messages(
                GetChannelMessagesSchema::before(Snowflake::generate()),
                current_channel,
                &mut app.user.clone().unwrap(),
            )
        );

        let messages = match messages_result {
            Ok(mut msgs) => {
                msgs.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
                msgs
            }
            Err(err) => {
                info!("Failed to fetch messages for channel {:?}: {:?}", current_channel, err);
                Vec::new()
            }
        };

        column![
            scrollable(
                column(
                    messages.into_iter()
                        .map(|message| {
                            let username = message
                                .author
                                .and_then(|author| Some(author.username))
                                .unwrap_or_else(|| Option::from("Unknown".to_string()));
                            let content = message.content.unwrap_or_else(|| "No Content".to_string());

                            text(format!("{:?}: {}", username, content))
                                .size(10)
                                .into()
                        })
                        .collect::<Vec<_>>()
                )
            )
            .height(Length::Fill),

            row![
                text_input("Type a message...", &app.message_input)
                    .padding(10)
                    .on_input(Message::MessageInputUpdate),
                button("Send")
                    .padding(10)
                    .on_press(Message::SendMessage),
            ]
            .spacing(10)
            .padding(10)
            .width(Length::Fill),
        ]
            .spacing(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Start)
    } else {
        column![
            text("No guild or channel selected!").size(15)
        ]
            .align_x(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill)
    }
}
