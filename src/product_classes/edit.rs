use iced::widget::{
    button, column, container, row, text, text_input,
    horizontal_space,
};
use iced::{Element, Length};
use std::collections::HashMap;

use crate::HotKey;
use super::ProductClass;
use crate::data_types::EntityId;


#[derive(Debug, Clone)]
pub enum Message {
    UpdateName(String),
    UpdateId(String),
    Save,
    Cancel,
}

pub fn view<'a>(
    product_class: &'a ProductClass,
    state: super::EditState,
    all_classes: &'a HashMap<EntityId, ProductClass>
) -> Element<'a, Message> {

    let validation_error = &state.validation_error;

    let other_classes: Vec<&ProductClass> = all_classes.values()
    .filter(|c| c.id != product_class.id)
    .collect();

    let content = container(
        column![
            row![
                text("Name").width(Length::Fixed(150.0)),
                text_input("Product Class Name", &product_class.name)
                    .on_input(Message::UpdateName)
                    .padding(5)
            ],
            row![
                text("ID").width(Length::Fixed(150.0)),
                text_input("ID (1-999)", &product_class.id.to_string())
                    .on_input(Message::UpdateId)
                    .padding(5)
            ],
            // Show validation error if any
            if let Some(error) = validation_error {
                text(error.to_string()).style(text::danger)
            } else {
                text("".to_string())
            },
            row![
                horizontal_space(),
                button("Cancel")
                    .on_press(Message::Cancel)
                    .style(button::danger),
                button("Save")
                    .on_press(Message::Save)
                    .style(button::success)
            ].spacing(10)
        ]
        .spacing(10)
    )
    .padding(20);

    container(content).into()
}

pub fn handle_hotkey(hotkey: HotKey) -> crate::Action<super::Operation, Message> {
    match hotkey {
        HotKey::Escape => crate::Action::operation(super::Operation::Cancel),
        _ => crate::Action::none(),
    }
}