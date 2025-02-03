use iced::widget::{
    button, column, container, row, text, text_input,
    horizontal_space,
};
use iced::{Element, Length};

use crate::HotKey;
use super::PrinterLogical;
use crate::data_types::{EntityId, ValidationError};
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
    pub fn new(printer: &PrinterLogical) -> Self {
        Self {
            name: printer.name.clone(),
            id: printer.id.to_string(),
            validation_error: None,
        }
    }
}

impl EditState {
    pub fn validate(&self, other_printers: &[&PrinterLogical]) -> Result<(), ValidationError> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Printer logical name cannot be empty".to_string()
            ));
        }

        let id: EntityId = self.id.parse().map_err(|_| {
            ValidationError::InvalidId("Invalid ID format".to_string())
        })?;

        if !(0..=25).contains(&id) {
            return Err(ValidationError::InvalidId(
                "Printer Logical ID must be between 0 and 25".to_string()
            ));
        }

        for other in other_printers {
            if id == other.id {
                return Err(ValidationError::DuplicateId(
                    format!("Printer Logical with ID {} already exists", id)
                ));
            }
        }

        Ok(())
    }
}

pub fn view<'a>(
    printer: &'a PrinterLogical,
    state: EditState,
    other_printers: &'a [&'a PrinterLogical],
) -> Element<'a, Message> {

    let name = state.name.clone();
    let id = state.id.clone();
    let error_message = state.validation_error.clone();

    let content = container(
        column![
            row![
                text("Name").width(Length::Fixed(150.0)),
                text_input("Printer Logical Name", &name)
                    .on_input(Message::UpdateName)
                    .padding(5)
            ],
            row![
                text("ID").width(Length::Fixed(150.0)),
                text_input("ID (0-25)", &id)
                    .on_input(Message::UpdateId)
                    .padding(5)
            ],
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

    let mut col = column![content, controls].spacing(20);

    if let Some(error) = error_message {
        col = col.push(
            container(
                text(error)
                    .style(text::danger)
            )
            .padding(10)
        );
    }

    container(col)
        .padding(20)
        .into()
}

pub fn handle_hotkey(hotkey: HotKey) -> crate::Action<super::Operation, Message> {
    match hotkey {
        HotKey::Escape => crate::Action::operation(super::Operation::Cancel),
        _ => crate::Action::none(),
    }
}