use iced::widget::{
    button, column, container, row, text, text_input,
    horizontal_space,
};
use iced::{Alignment, Element, Length, Color};
use crate::data_types::{EntityId, ValidationError};
use crate::HotKey;
use super::RevenueCategory;

#[derive(Debug, Clone)]
pub enum Message {
    UpdateName(String),
    UpdateId(String),
    Save,
    Cancel,
}

pub struct EditState {
    pub name: String,
    pub id: String,
    pub validation_error: Option<String>,
}

impl EditState {
    pub fn new(category: &RevenueCategory) -> Self {
        Self {
            name: category.name.clone(),
            id: category.id.to_string(),
            validation_error: None,
        }
    }
}

impl EditState {
    pub fn validate(&self, other_categories: &[&RevenueCategory]) -> Result<(), ValidationError> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Revenue category name cannot be empty".to_string()
            ));
        }

        let id: EntityId = self.id.parse().map_err(|_| {
            ValidationError::InvalidId("Invalid ID format".to_string())
        })?;

        if !(1..=99).contains(&id) {
            return Err(ValidationError::InvalidId(
                "Revenue Category ID must be between 1 and 99".to_string()
            ));
        }

        for other in other_categories {
            if id == other.id {
                return Err(ValidationError::DuplicateId(
                    format!("Revenue Category with ID {} already exists", id)
                ));
            }
        }

        Ok(())
    }
}

pub fn view(state: &EditState) -> Element<Message> {
    let content = container(
        column![
            row![
                text("Name").width(Length::Fixed(150.0)),
                text_input("Revenue Category Name", &state.name)
                    .on_input(Message::UpdateName)
                    .padding(5)
            ],
            row![
                text("ID").width(Length::Fixed(150.0)),
                text_input("ID (1-99)", &state.id)
                    .on_input(Message::UpdateId)
                    .padding(5)
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