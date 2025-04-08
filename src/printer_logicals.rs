use crate::data_types::{EntityId, ValidationError};
use crate::Action;
use crate::entity_component::{self, Entity, EditState};
use serde::{Serialize, Deserialize};
use iced::Element;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Message {
    CreateNew,
    RequestDelete(EntityId),
    CopyPrinterLogical(EntityId),
    EditPrinterLogical(EntityId),
    UpdateId(String),
    UpdateName(String),
    Select(EntityId),
    SaveMultiTest(EntityId, EditState),
    UpdateMultiName(EntityId, String),
    CreateNewMulti,
    CancelEdit(EntityId),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(PrinterLogical),
    StartEdit(EntityId),
    Cancel,
    Back,
    CreateNew(PrinterLogical),
    RequestDelete(EntityId),
    CopyPrinterLogical(EntityId),
    EditPrinterLogical(EntityId),
    Select(EntityId),
    SaveMultiTest(EntityId, EditState),
    UpdateMultiName(EntityId, String),
    CreateNewMulti,
    CancelEdit(EntityId),
}

#[derive(Debug, Clone)]
pub enum Mode {
    View,
    Edit,
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

impl Entity for PrinterLogical {
    fn id(&self) -> EntityId {
        self.id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn with_id(&self, id: EntityId) -> Self {
        let mut clone = self.clone();
        clone.id = id;
        clone
    }
    
    fn with_name(&self, name: String) -> Self {
        let mut clone = self.clone();
        clone.name = name;
        clone
    }
    
    fn default_new() -> Self {
        Self::default()
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

        // Validate name is not more than 16 Characters
        if self.name.len() > 16 {
            return Err(ValidationError::NameTooLong(
                "Printer Logical name cannot be more than 16 Characters".to_string()
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
        Message::CreateNew => {
            let new_printer_logical = PrinterLogical::default();
            Action::operation(Operation::CreateNew(new_printer_logical))
        },
        Message::RequestDelete(id) => {
            Action::operation(Operation::RequestDelete(id))
        },
        Message::CopyPrinterLogical(id) => {
            Action::operation(Operation::CopyPrinterLogical(id))
        },
        Message::EditPrinterLogical(id) => {
            println!("Editing ID: {}", id);
            Action::operation(Operation::EditPrinterLogical(id))
        }
        Message::Select(id) => {
            Action::operation(Operation::Select(id))
        },
        Message::UpdateId(id) => {
            if let Ok(id) = id.parse() {
                printer.id = id;
                Action::none()
            } else {
                state.id_validation_error = Some("Invalid ID format".to_string());
                Action::none()
            }
        }
        Message::UpdateName(name) => {
            printer.name = name;
            Action::none()
        }
        Message::CreateNewMulti => {
            Action::operation(Operation::CreateNewMulti)
        }
        Message::SaveMultiTest(id, edit_state) => {
            Action::operation(Operation::SaveMultiTest(id, edit_state))
        }
        Message::UpdateMultiName(id, new_name) => {
            Action::operation(Operation::UpdateMultiName(id, new_name))
        }
        Message::CancelEdit(id) => {
            Action::operation(Operation::CancelEdit(id))
        }
    }
}

pub fn view<'a>(
    all_printers: &'a BTreeMap<EntityId, PrinterLogical>,
    edit_states: &'a Vec<EditState>,
) -> Element<'a, Message> {
    entity_component::entity_view(
        "Printer Logicals",
        Message::CreateNewMulti,
        all_printers,
        edit_states,
        |printer, edit_states| render_printer_row(printer, edit_states),
    )
}

fn render_printer_row<'a>(
    printer: &'a PrinterLogical,
    edit_states: &'a Vec<EditState>
) -> Element<'a, Message> {
    entity_component::entity_quick_edit_view(
        printer,
        edit_states,
        Message::EditPrinterLogical,
        Message::SaveMultiTest,
        Message::CopyPrinterLogical,
        Message::RequestDelete,
        Message::CancelEdit,
        Message::UpdateMultiName,
        "Printer Name"
    )
}