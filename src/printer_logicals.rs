pub mod edit;
pub mod view;

use crate::data_types::{
    EntityId,
    ValidationError,
    Validatable,
    IdRange,
};
use crate::Action;
use iced::Element;

#[derive(Debug, Clone)]
pub enum Message {
    Edit(edit::Message),
    View(view::Message),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(PrinterLogical),
    StartEdit(EntityId),
    Cancel,
    Back,
}

#[derive(Debug, Clone)]
pub enum Mode {
    View,
    Edit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrinterLogical {
    pub id: EntityId,
    pub name: String,
}

impl std::fmt::Display for PrinterLogical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PrinterLogical {
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
    state: &mut edit::EditState,
    other_printers: &[&PrinterLogical],
) -> Action<Operation, Message> {
    match message {
        Message::Edit(msg) => match msg {
            edit::Message::UpdateName(name) => {
                state.name = name;
                state.validation_error = None;
                Action::none()
            }
            edit::Message::UpdateId(id) => {
                state.id = id;
                state.validation_error = None;
                Action::none()
            }
            edit::Message::Save => {
                match state.validate(other_printers) {
                    Ok(_) => Action::operation(Operation::Save(printer.clone())),
                    Err(e) => {
                        state.validation_error = Some(e.to_string());
                        Action::none()
                    }
                }
            }
            edit::Message::Cancel => Action::operation(Operation::Cancel),
        },
        Message::View(msg) => match msg {
            view::Message::Edit => Action::operation(Operation::StartEdit(printer.id)),
            view::Message::Back => Action::operation(Operation::Back),
        },
    }
}

pub fn view<'a>(
    printer: &'a PrinterLogical, 
    mode: &'a Mode,
    other_printers: &'a [&'a PrinterLogical]
) -> Element<'a, Message> {
    match mode {
        Mode::View => view::view(printer).map(Message::View),
        Mode::Edit => {
            edit::view(
                printer,
                edit::EditState::new(printer),
                other_printers
            ).map(Message::Edit)
        }
    }
}