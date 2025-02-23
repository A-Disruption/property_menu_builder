use iced::widget::{
    button, column, container, row, text, text_input,
    horizontal_space,
};
use iced::{Element, Length};
use std::collections::BTreeMap;
use crate::data_types::EntityId;
use crate::icon;
use crate::HotKey;
use super::TaxGroup;

#[derive(Debug, Clone)]
pub enum Message {
    UpdateName(String),
    UpdateId(String),
    UpdateRate(String),
    Save,
    Cancel,
}

pub fn view<'a>(
    tax_group: &'a TaxGroup,
    state: super::EditState,
    all_groups: &'a BTreeMap<EntityId, TaxGroup>
) -> Element<'a, Message> {

    let header = row![
        horizontal_space().width(10),
        text(&tax_group.name).size(18).style(text::primary),
        horizontal_space(),
        button(icon::save().size(14)).on_press(Message::Save).style(button::primary),
        button(icon::cancel().size(14)).on_press(Message::Cancel).style(button::danger),
        horizontal_space().width(4),
    ]
    .spacing(10)
    .padding(20)
    .align_y(iced::Alignment::Center);

    let validation_error = &state.validation_error;

    let other_groups: Vec<&TaxGroup> = all_groups.values()
    .filter(|g| g.id != tax_group.id)
    .collect();

    let content = container(
        column![
            row![
                text("Name").width(Length::Fixed(150.0)),
                text_input("Tax Group Name", &tax_group.name)
                    .on_input(Message::UpdateName)
                    .padding(5)
            ],
            row![
                text("ID").width(Length::Fixed(150.0)),
                text_input("ID (1-99)", &tax_group.id.to_string())
                    .on_input(Message::UpdateId)
                    .padding(5)
            ],
            row![
                text("Tax Rate (%)").width(Length::Fixed(150.0)),
                text_input("Rate", &tax_group.rate_percentage().to_string())
                    .on_input(Message::UpdateRate)
                    .padding(5),
                text("%").width(Length::Fixed(30.0))
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