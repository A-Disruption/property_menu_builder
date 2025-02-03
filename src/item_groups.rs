pub mod edit;
pub mod view;

use crate::data_types::{
    EntityId,
    ValidationError,
    Validatable,
    IdRange,
};
use rangemap::RangeInclusiveSet;
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

#[derive(Debug, Clone, PartialEq)]
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

            if ranges_overlap(&(self.id_range.start..=self.id_range.end), &(other.id_range.start..=other.id_range.end)) {
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

fn ranges_overlap<T: Ord>(range1: &std::ops::RangeInclusive<T>, range2: &std::ops::RangeInclusive<T>) -> bool {
    range1.start() <= range2.end() && range2.start() <= range1.end()
}

pub fn update(
    group: &mut ItemGroup,
    message: Message,
    state: &mut edit::EditState,
    other_groups: &[&ItemGroup],
) -> Action<Operation, Message> {
    match message {
        Message::Edit(msg) => match msg {
            edit::Message::UpdateName(name) => {
                state.name = name;
                state.validation_error = None;
                Action::none()
            }
            edit::Message::UpdateRangeStart(start) => {
                if let Ok(start_val) = start.parse::<EntityId>() {
                    state.range_start = start;
                    state.validation_error = None;
                } else {
                    state.validation_error = Some("Invalid range start value".to_string());
                }
                Action::none()
            }
            edit::Message::UpdateRangeEnd(end) => {
                if let Ok(end_val) = end.parse::<EntityId>() {
                    state.range_end = end;
                    state.validation_error = None;
                } else {
                    state.validation_error = Some("Invalid range end value".to_string());
                }
                Action::none()
            }
            edit::Message::ValidateRange => {
                if let (Ok(start), Ok(end)) = (
                    state.range_start.parse::<EntityId>(),
                    state.range_end.parse::<EntityId>(),
                ) {
                    let current_range = start..=end;
                    for other in other_groups {
                        let other_range = other.id_range.start..=other.id_range.end;
                        if ranges_overlap(&current_range, &other_range) {
                            state.validation_error = Some(format!(
                                "Range overlaps with group '{}'",
                                other.name
                            ));
                            return Action::none();
                        }
                    }
                    state.validation_error = None;
                } else {
                    state.validation_error = Some("Invalid range values".to_string());
                }
                Action::none()
            }
            edit::Message::Save => {
                match state.validate(other_groups) {
                    Ok(_) => Action::operation(Operation::Save(group.clone())),
                    Err(e) => {
                        state.validation_error = Some(e.to_string());
                        Action::none()
                    }
                }
            }
            edit::Message::Cancel => Action::operation(Operation::Cancel),
        },
        Message::View(msg) => match msg {
            view::Message::Edit => Action::operation(Operation::StartEdit(group.id)),
            view::Message::Back => Action::operation(Operation::Back),
        },
    }
}

pub fn view<'a>(
    group: &'a ItemGroup, 
    mode: &'a Mode, 
    other_groups: &'a [&'a ItemGroup]
) -> Element<'a, Message> {
    match mode {
        Mode::View => view::view(group).map(Message::View),
        Mode::Edit => {
            edit::view(group, edit::EditState::new(group), other_groups).map(Message::Edit)
        }
    }
}

// Update Display implementation for ValidationError
impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InvalidRange(msg) => write!(f, "Invalid range: {}", msg),
            ValidationError::RangeOverlap(msg) => write!(f, "Range overlap: {}", msg),
            ValidationError::InvalidId(msg) => write!(f, "Invalid ID: {}", msg),
            ValidationError::DuplicateId(msg) => write!(f, "Duplicate ID: {}", msg),
            ValidationError::EmptyName(msg) => write!(f, "Empty name: {}", msg),
            ValidationError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
            ValidationError::InvalidReference(msg) => write!(f, "Invalid refernce: {}", msg),
            ValidationError::InvalidRate(msg) => write!(f, "Invalid rate: {}", msg),
        }
    }
}