use iced::widget::{
    button, column, container, row, text, text_input,
    horizontal_space,
};
use iced::{Alignment, Element, Length, Color};
use rust_decimal::Decimal;
use crate::data_types::EntityId;
use crate::HotKey;
use super::{TaxGroup, ValidationError};

#[derive(Debug, Clone)]
pub enum Message {
    UpdateName(String),
    UpdateId(String),
    UpdateRate(String),
    Save,
    Cancel,
}

pub struct EditState {
    name: String,
    id: String,
    rate: String,
    validation_error: Option<String>,
}

impl EditState {
    pub fn new(tax_group: &TaxGroup) -> Self {
        Self {
            name: tax_group.name.clone(),
            id: tax_group.id.to_string(),
            rate: tax_group.rate_percentage().to_string(),
            validation_error: None,
        }
    }
}

pub fn view(state: &EditState) -> Element<Message> {
    let content = container(
        column![
            row![
                text("Name").width(Length::Fixed(150.0)),
                text_input("Tax Group Name", &state.name)
                    .on_input(Message::UpdateName)
                    .padding(5)
            ],
            row![
                text("ID").width(Length::Fixed(150.0)),
                text_input("ID (1-99)", &state.id)
                    .on_input(Message::UpdateId)
                    .padding(5)
            ],
            row![
                text("Tax Rate (%)").width(Length::Fixed(150.0)),
                text_input("Rate", &state.rate)
                    .on_input(Message::UpdateRate)
                    .padding(5),
                text("%").width(Length::Fixed(30.0))
            ],
            if let Some(error) = &state.validation_error {
                container(
                    text(error)
                        .style(iced::widget::text::danger)
                )
                .padding(10)
            } else {
                container(
                    text("")
                        .style(iced::widget::text::danger)
                )
                .padding(10)
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