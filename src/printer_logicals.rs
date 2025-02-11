pub mod edit;
pub mod view;

use crate::data_types::{
    EntityId,
    ValidationError,
    Validatable,
};
use crate::Action;
use serde::{Serialize, Deserialize};
use iced::Element;
use iced::widget::{button, column, container, row, text};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Message {
    Edit(edit::Message),
    View(view::Message),
    CreateNew,
    Select(EntityId),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(PrinterLogical),
    StartEdit(EntityId),
    Cancel,
    Back,
    CreateNew(PrinterLogical),
    Select(EntityId),
}

#[derive(Debug, Clone)]
pub enum Mode {
    View,
    Edit,
}

#[derive(Default, Clone)]
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

    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Printer name cannot be empty".to_string()
            ));
        }

        if let Ok(id) = self.id.parse::<EntityId>() {
            if !(1..=999).contains(&id) {
                return Err(ValidationError::InvalidId(
                    "Printer ID must be between 1 and 999".to_string()
                ));
            }
        } else {
            return Err(ValidationError::InvalidId(
                "Invalid ID format".to_string()
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrinterLogical {
    pub id: EntityId,
    pub name: String,
}

impl std::fmt::Display for PrinterLogical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Default for PrinterLogical {
    fn default() -> Self {
        Self {
            id: -1,
            name: String::new(),
        }
    }
}

impl PrinterLogical {

    pub fn new_draft() -> Self {
        Self::default()
    }

    fn validate(&self, other_printers: &[&PrinterLogical]) -> Result<(), ValidationError> {
        // Validate ID range (0-25 based on your screenshot)
        if !(0..=25).contains(&self.id) {
            return Err(ValidationError::InvalidId(
                "Printer Logical ID must be between 0 and 25".to_string()
            ));
        }

        // Check for duplicate IDs
        for other in other_printers {
            if other.id == self.id {
                return Err(ValidationError::DuplicateId(
                    format!("Printer Logical with ID {} already exists", self.id)
                ));
            }
        }

        // Validate name is not empty
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Printer Logical name cannot be empty".to_string()
            ));
        }

        Ok(())
    }
}

pub fn update(
    printer: &mut PrinterLogical,
    message: Message,
    state: &mut EditState,
    other_printers: &[&PrinterLogical]
) -> Action<Operation, Message> {
    match message {
        Message::Edit(msg) => match msg {
            edit::Message::UpdateName(name) => {
                printer.name = name;
                Action::none()
            }
            edit::Message::UpdateId(id) => {
                if let Ok(id) = id.parse() {
                    printer.id = id;
                    Action::none()
                } else {
                    state.validation_error = Some("Invalid ID format".to_string());
                    Action::none()
                }
            }
            edit::Message::Save => {
                if printer.validate(other_printers).is_ok() {
                    Action::operation(Operation::Save(printer.clone()))
                } else {
                    state.validation_error = Some("Validation failed".to_string());
                    Action::none()
                }
            }
            edit::Message::Cancel => Action::operation(Operation::Cancel),
        },
        Message::View(msg) => match msg {
            view::Message::Edit => Action::operation(Operation::StartEdit(printer.id)),
            view::Message::Back => Action::operation(Operation::Back),
        }
        Message::CreateNew => {
            let new_printer_logical = PrinterLogical::default();
            Action::operation(Operation::CreateNew(new_printer_logical))
        },
        Message::Select(id) => {
            Action::operation(Operation::Select(id))
        },
    }
}

pub fn view<'a>(
    printer: &'a PrinterLogical, 
    mode: &'a Mode,
    all_printers: &'a HashMap<EntityId, PrinterLogical>
) -> Element<'a, Message> {

    let printer_list = column(
        all_printers
            .values()
            .map(|printer| {
                button(text(&printer.name))
                    .width(iced::Length::Fill)
                    .on_press(Message::Select(printer.id))
                    .style(if printer.id == printer.id {
                        button::primary
                    } else {
                        button::secondary
                    })
                    .into()
            })
            .collect::<Vec<_>>()
    )
    .spacing(5)
    .width(iced::Length::Fixed(200.0));

    let content = match mode {
        Mode::View => view::view(printer).map(Message::View),
        Mode::Edit => {
            edit::view(
                printer,
                EditState::new(printer),
                all_printers
            ).map(Message::Edit)
        }
    };

    row![
        container(
            column![
                text("Printer Logicals").size(24),
                button("Create New")
                    .on_press(Message::CreateNew)
                    .style(button::primary),
                printer_list,
            ]
            .spacing(10)
            .padding(10)
        )
        .style(container::rounded_box),
        container(content)
            .width(iced::Length::Fill)
            .style(container::rounded_box)
    ]
    .spacing(20)
    .into()
}