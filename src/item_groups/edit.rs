use iced::widget::{
    button, column, container, row, text, text_input,
    horizontal_space,
};
use iced::{Element, Length};
use std::iter::empty;
use crate::HotKey;
use super::ItemGroup;

#[derive(Debug, Clone)]
pub struct EditState {
    name: String,
    range_start: String,
    range_end: String,
    validation_error: Option<String>,
}

impl EditState {
    pub fn new(item_group: &ItemGroup) -> Self {
        Self {
            name: item_group.name.clone(),
            range_start: item_group.id_range.start.to_string(),
            range_end: item_group.id_range.end.to_string(),
            validation_error: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    UpdateName(String),
    UpdateRangeStart(String),
    UpdateRangeEnd(String),
    ValidateRange,
    Save,
    Cancel,
}

pub fn view(item_group: &ItemGroup, state: &EditState, other_groups: &[&ItemGroup]) -> Element<Message> {
    let content = container(
        column![
            // Name input
            row![
                text("Name").width(Length::Fixed(150.0)),
                text_input("Group Name", &state.name)
                    .on_input(Message::UpdateName)
                    .on_submit(Message::ValidateRange)
                    .padding(5)
            ],
            // Range inputs
            row![
                text("ID Range Start").width(Length::Fixed(150.0)),
                text_input("Start ID", &state.range_start)
                    .on_input(Message::UpdateRangeStart)
                    .on_submit(Message::ValidateRange)
                    .padding(5)
            ],
            row![
                text("ID Range End").width(Length::Fixed(150.0)),
                text_input("End ID", &state.range_end)
                    .on_input(Message::UpdateRangeEnd)
                    .on_submit(Message::ValidateRange)
                    .padding(5)
            ],
            // Validation error message (if any)
            if let Some(error) = &state.validation_error {
                container(
                    text(error)
                        .style(iced::widget::text::danger)
                )
                .padding(10)
                .into()
            } else {
                empty().into()
            }
        ]
        .spacing(10)
    )
    .style(container::rounded_box)
    .padding(20);

    let controls = row![
        horizontal_space(),
        button("Cancel")
            .on_press(Message::Cancel)
            .style(button::danger),
        button("Save")
            .on_press(Message::Save)
            .style(button::success),
    ]
    .spacing(10)
    .padding(20);

    container(
        column![
            content,
            controls,
        ]
        .spacing(20)
    )
    .padding(20)
    .into()
}

pub fn handle_hotkey(hotkey: HotKey) -> crate::Action<super::Operation, Message> {
    match hotkey {
        HotKey::Escape => crate::Action::operation(super::Operation::Cancel),
        _ => crate::Action::none(),
    }
}