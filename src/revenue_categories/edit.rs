use iced::widget::{
    button, column, container, row, text, text_input,
    horizontal_space,
};
use iced::{Element, Length};
use std::collections::HashMap;
use crate::data_types::EntityId;
use crate::HotKey;
use super::RevenueCategory;

#[derive(Debug, Clone)]
pub enum Message {
    UpdateName(String),
    UpdateId(String),
    Save,
    Cancel,
}

pub fn view<'a>(
    revenue_category: &'a RevenueCategory,
    state: super::EditState,
    all_categories: &'a HashMap<EntityId, RevenueCategory>
) -> Element<'a, Message> {

    let validation_error = &state.validation_error;

    let other_categories: Vec<&RevenueCategory> = all_categories.values()
    .filter(|c| c.id != revenue_category.id)
    .collect();

    let content = container(
        column![
            row![
                text("Name").width(Length::Fixed(150.0)),
                text_input("Revenue Category Name", &revenue_category.name)
                    .on_input(Message::UpdateName)
                    .padding(5)
            ],
            row![
                text("ID").width(Length::Fixed(150.0)),
                text_input("ID (1-999)", &revenue_category.id.to_string())
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