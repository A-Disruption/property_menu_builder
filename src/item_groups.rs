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
use std::ops::Range;

#[derive(Debug, Clone)]
pub enum Message {
    Edit(edit::Message),
    View(view::Message),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(ItemGroup),
    StartEdit(EntityId),
    Cancel,
    Back,
}

#[derive(Debug, Clone)]
pub enum Mode {
    View,
    Edit,
}

#[derive(Debug, Clone)]
pub struct ItemGroup {
    pub id: EntityId,
    pub name: String,
    pub id_range: Range<EntityId>,
}

impl std::fmt::Display for ItemGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl ItemGroup {
    pub fn validate_range(&self) -> Result<(), ValidationError> {
        if self.id_range.start >= self.id_range.end {
            return Err(ValidationError::InvalidRange(
                "Start ID must be less than End ID".to_string()
            ));
        }
        Ok(())
    }

    pub fn validate_no_overlap(&self, other_groups: &[&ItemGroup]) -> Result<(), ValidationError> {
        for other in other_groups {
            if self.id == other.id {
                continue; // Skip comparing with self
            }
            
            if (self.id_range.start..=self.id_range.end)
                .overlaps(other.id_range.start..=other.id_range.end)
            {
                return Err(ValidationError::RangeOverlap(
                    format!("Range overlaps with group '{}'", other.name)
                ));
            }
        }
        Ok(())
    }

    pub fn validate(&self, other_groups: &[&ItemGroup]) -> Result<(), ValidationError> {
        self.validate_range()?;
        self.validate_no_overlap(other_groups)?;
        Ok(())
    }
}

pub fn update(
    item_group: &mut ItemGroup,
    state: &mut edit::EditState,
    other_groups: &[&ItemGroup],
    message: Message,
) -> Action<Operation, Message> {
    match message {
        Message::Edit(msg) => match msg {
            edit::Message::UpdateName(name) => {
                state.name = name;
                state.validation_error = None;
                Action::none()
            }
            edit::Message::UpdateRangeStart(start) => {
                state.range_start = start;
                state.validation_error = None;
                Action::none()
            }
            edit::Message::UpdateRangeEnd(end) => {
                state.range_end = end;
                state.validation_error = None;
                Action::none()
            }
            edit::Message::ValidateRange => {
                let start: EntityId = match state.range_start.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        state.validation_error = Some("Invalid start ID".to_string());
                        return Action::none();
                    }
                };
                let end: EntityId = match state.range_end.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        state.validation_error = Some("Invalid end ID".to_string());
                        return Action::none();
                    }
                };

                let temp_group = ItemGroup {
                    id: item_group.id,
                    name: state.name.clone(),
                    id_range: start..end,
                };

                match temp_group.validate(other_groups) {
                    Ok(_) => {
                        state.validation_error = None;
                        Action::none()
                    }
                    Err(e) => {
                        state.validation_error = Some(e.to_string());
                        Action::none()
                    }
                }
            }
            edit::Message::Save => {
                // Validate before saving
                let start: EntityId = match state.range_start.parse() {
                    Ok(n) => n,
                    Err(_) => return Action::none(),
                };
                let end: EntityId = match state.range_end.parse() {
                    Ok(n) => n,
                    Err(_) => return Action::none(),
                };

                let temp_group = ItemGroup {
                    id: item_group.id,
                    name: state.name.clone(),
                    id_range: start..end,
                };

                match temp_group.validate(other_groups) {
                    Ok(_) => {
                        *item_group = temp_group;
                        Action::operation(Operation::Save(item_group.clone()))
                    }
                    Err(e) => {
                        state.validation_error = Some(e.to_string());
                        Action::none()
                    }
                }
            }
            edit::Message::Cancel => Action::operation(Operation::Cancel),
        },
        // ... rest of message handling
    }
}

pub fn view(item_group: &ItemGroup, mode: Mode) -> Element<Message> {
    match mode {
        Mode::View => view::view(item_group).map(Message::View),
        Mode::Edit => edit::view(item_group).map(Message::Edit),
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InvalidRange(msg) => write!(f, "Invalid range: {}", msg),
            ValidationError::RangeOverlap(msg) => write!(f, "Range overlap: {}", msg),
        }
    }
}