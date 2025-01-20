use chorus::types::{Channel, GetChannelMessagesSchema, Snowflake};
use iced::{futures, Alignment, Length};
use iced::widget::{button, column, row, container, scrollable, text, text_input};
use log::info;
use crate::App;
use crate::types::message::Message;

pub fn main_view(app: &App) -> iced::widget::Container<Message> {
    let mut chorus_user = &mut app.user.clone().unwrap();
    let guilds_column = column![
        text("Guilds").size(30),
        app.guilds.as_ref().map_or_else(
            || column![text("No guilds available").size(20)], // Handle when no guilds are available
            |guilds| {
                column(
                    guilds.iter().map(|guild| {
                        button(text(guild.name.clone().unwrap_or_else(|| "Unnamed Guild".to_string())))
                            .padding(10)
                            .on_press(Message::SwitchGuild(guild.id.clone()))
                            .into()
                    }).collect::<Vec<_>>()
                )
            }
        )
    ]
        .spacing(10)
        .width(Length::Fixed(120.0))
        .height(Length::Fill)
        .align_x(Alignment::Start);

    let channels_column = app.current_guild.and_then(|current_guild_id| {
        app.guilds.as_ref().and_then(|guilds| {
            info!("Debug 1: Guilds are present");

            // Find the currently selected guild
            guilds.iter().find(|guild| guild.id == current_guild_id).map(|guild| {
                info!("Debug 2: Found selected guild with ID {:?}", guild.id);

                // Build the column with the channels
                column(
                    futures::executor::block_on(guild.channels(&mut chorus_user)).unwrap_or_default().iter().map(|channel| {
                        info!("Debug 3: Listing Channel: {:?}", channel);

                        button(
                            text(channel.name.clone().unwrap_or_else(|| "Unnamed Channel".to_string()))
                        )
                            .padding(5)
                            .on_press(Message::SwitchChannel(channel.id.clone()))
                            .into()
                    }).collect::<Vec<_>>()
                )
                    .spacing(5)
                    .height(Length::Fill)
            })
        })
    }).unwrap_or_else(|| {
        info!("Debug 4: No guild selected or no channels available");
        column![text("No guild selected").size(15)]
    })
        .width(Length::Fixed(120.0))  // Adjust the width for channels
        .height(Length::Fill)
        .align_x(Alignment::Start);

    let sidebar = row![
        guilds_column,
        channels_column
    ]
        .spacing(10)
        .height(Length::Fill)
        .width(Length::Fixed(240.0));

    let chat_area = if let Some(current_channel) = app.current_channel {
        // Only fetch messages if the current channel and guild exist
        let messages = futures::executor::block_on(
            Channel::messages(
                GetChannelMessagesSchema::before(Snowflake::generate()),
                current_channel,
                &mut chorus_user,
            )
        ).unwrap_or_default();

        column![
        // Scrollable area for messages
scrollable(
    column(
        {
            // Sort the messages by timestamp before processing
            let mut messages = messages;
            messages.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

            // Process and map the sorted messages to UI elements
            messages.into_iter().filter_map(|message| {
                // Safely unwrap the author and message content
                let username = message.author.and_then(|author| Some(author.username)).unwrap_or_else(|| Option::from("Unknown".to_string()));
                let content = message.content.unwrap_or_else(|| "No Content".to_string());

                Some(text(format!("{:?}: {}", username, content)))
                    .map(|t| t.size(10).into()) // Apply styling to text
            }).collect::<Vec<_>>()
        }
    )
)        .height(Length::Fill),
        // Input row for message input and send button
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
        // Show a fallback if no channel or guild is selected
        column![
        text("No guild or channel selected!").size(15)
    ]
            .align_x(Alignment::Center)
            // .align_y(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill)
    };

    let body = row![
        sidebar, // Sidebar (guilds and channels)
        chat_area
    ]
        .spacing(10)
        .width(Length::Fill)
        .height(Length::Fill);

    // Wrap the main layout in a container
    container(body)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
}