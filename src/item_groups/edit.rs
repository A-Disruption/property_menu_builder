use iced::widget::{
    button, column, container, row, text, text_input,
    horizontal_space,
};
use iced::{Element, Length};
use crate::data_types::EntityId;
use crate::icon;
use std::collections::BTreeMap;
use crate::HotKey;
use super::ItemGroup;

#[derive(Debug, Clone)]
pub enum Message {
    UpdateName(String),
    UpdateId(String),
    UpdateRangeStart(String),
    UpdateRangeEnd(String),
    Save,
    Cancel,
}

pub fn view<'a>(
    item_group: &'a ItemGroup,
    state: super::EditState,
    all_groups: &'a BTreeMap<EntityId, ItemGroup>
) -> Element<'a, Message> {

    let header = row![
        horizontal_space().width(10),
        text(&item_group.name).size(18).style(text::primary),
        horizontal_space(),
        button(icon::save().size(14)).on_press(Message::Save).style(button::primary),
        button(icon::cancel().size(14)).on_press(Message::Cancel).style(button::danger),
        horizontal_space().width(4),
    ]
    .spacing(10)
    .padding(20)
    .align_y(iced::Alignment::Center);

    let validation_error = &state.validation_error;

    let other_groups: Vec<&ItemGroup> = all_groups.values()
    .filter(|g| g.id != item_group.id)
    .collect();

    let content = container(
        column![
            row![
                text("Name").width(Length::Fixed(150.0)),
                text_input("Item Group Name", &item_group.name)
                    .on_input(Message::UpdateName)
                    .padding(5)
            ],
            row![
                text("ID").width(Length::Fixed(150.0)),
                text_input("ID (1-999)", &item_group.id.to_string())
                    .on_input(Message::UpdateId)
                    .padding(5)
            ],
            row![
                text("ID Range Start").width(Length::Fixed(150.0)),
                text_input("Range Start", &item_group.id_range.start.to_string())
                    .on_input(Message::UpdateRangeStart)
                    .padding(5)
            ],
            row![
                text("ID Range End").width(Length::Fixed(150.0)),
                text_input("Range End", &item_group.id_range.end.to_string())
                    .on_input(Message::UpdateRangeEnd)
                    .padding(5)
            ],
            // Show validation error if any
            if let Some(error) = validation_error {
                text(error.to_string()).style(text::danger)
            } else {
                text("".to_string())
            },
        ]
        .spacing(10)
    )
    .padding(20);

    container(column![header, content].padding(10)).into()
}

pub fn handle_hotkey(hotkey: HotKey) -> crate::Action<super::Operation, Message> {
    match hotkey {
        HotKey::Escape => crate::Action::operation(super::Operation::Cancel),
        _ => crate::Action::none(),
    }
}