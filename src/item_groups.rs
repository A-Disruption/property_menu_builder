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
use std::collections::HashMap;

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

#[derive(Default, Clone)]
pub struct EditState {
    pub name: String,
    pub id: String,
    pub id_range_start: String,
    pub id_range_end: String,
    pub validation_error: Option<String>,
}

impl EditState {
    pub fn new(item_group: &ItemGroup) -> Self {
        Self {
            name: item_group.name.clone(),
            id: item_group.id.to_string(),
            id_range_start: item_group.id_range.start.to_string(),
            id_range_end: item_group.id_range.end.to_string(),
            validation_error: None,
        }
    }
 
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Item group name cannot be empty".to_string()
            ));
        }
 
        if let Ok(id) = self.id.parse::<EntityId>() {
            if !(1..=99999).contains(&id) {
                return Err(ValidationError::InvalidId(
                    "Item group ID must be between 1 and 99999".to_string()
                ));
            }
        } else {
            return Err(ValidationError::InvalidId(
                "Invalid ID format".to_string()
            ));
        }
 
        let start = self.id_range_start.parse::<EntityId>().map_err(|_| {
            ValidationError::InvalidId("Invalid range start format".to_string())
        })?;
 
        let end = self.id_range_end.parse::<EntityId>().map_err(|_| {
            ValidationError::InvalidId("Invalid range end format".to_string())
        })?;
 
        if start >= end {
            return Err(ValidationError::InvalidValue(
                "Range start must be less than range end".to_string()
            ));
        }
 
        Ok(())
    }
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

impl Default for ItemGroup {
    fn default() -> Self {
        Self {
            id: 1,
            name: String::new(),
            id_range: Range { start: 1, end: 1000 }
        }
    }
 }

 impl ItemGroup {
    fn validate(&self, other_groups: &[&ItemGroup]) -> Result<(), ValidationError> {
        if !(1..=999).contains(&self.id) {
            return Err(ValidationError::InvalidId(
                "Item group ID must be between 1 and 999".to_string()
            ));
        }
 
        // Check for duplicate IDs
        for other in other_groups {
            if other.id == self.id {
                return Err(ValidationError::DuplicateId(
                    format!("Item group with ID {} already exists", self.id)
                ));
            }
        }
 
        // Check for overlapping ranges
        for other in other_groups {
            if self.id == other.id {
                continue; 
            }

            if ranges_overlap(&(self.id_range.start..=self.id_range.end), &(other.id_range.start..=other.id_range.end)) {
                return Err(ValidationError::RangeOverlap(
                    format!("Range overlaps with group '{}'", other.name)
                ));
            }
        }
 
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Item group name cannot be empty".to_string()
            ));
        }
 
        Ok(())
    }
 }


fn ranges_overlap<T: Ord>(range1: &std::ops::RangeInclusive<T>, range2: &std::ops::RangeInclusive<T>) -> bool {
    range1.start() <= range2.end() && range2.start() <= range1.end()
}

pub fn update(
    item_group: &mut ItemGroup,
    message: Message,
    state: &mut EditState,
    other_groups: &[&ItemGroup]
 ) -> Action<Operation, Message> {
    match message {
        Message::Edit(msg) => match msg {
            edit::Message::UpdateName(name) => {
                item_group.name = name;
                Action::none()
            }
            edit::Message::UpdateId(id) => {
                if let Ok(id) = id.parse() {
                    item_group.id = id;
                    Action::none()
                } else {
                    state.validation_error = Some("Invalid ID format".to_string());
                    Action::none()
                }
            }
            edit::Message::UpdateRangeStart(start) => {
                if let Ok(start) = start.parse() {
                    item_group.id_range.start = start;
                    Action::none()
                } else {
                    state.validation_error = Some("Invalid range start format".to_string());
                    Action::none()
                }
            }
            edit::Message::UpdateRangeEnd(end) => {
                if let Ok(end) = end.parse() {
                    item_group.id_range.end = end;
                    Action::none()
                } else {
                    state.validation_error = Some("Invalid range end format".to_string());
                    Action::none()
                }
            }
            edit::Message::Save => {
                if item_group.validate(other_groups).is_ok() {
                    Action::operation(Operation::Save(item_group.clone()))
                } else {
                    state.validation_error = Some("Validation failed".to_string());
                    Action::none()
                }
            }
            edit::Message::Cancel => Action::operation(Operation::Cancel),
        },
        Message::View(msg) => match msg {
            view::Message::Edit => Action::operation(Operation::StartEdit(item_group.id)),
            view::Message::Back => Action::operation(Operation::Back),
        }
    }
 }

 pub fn view<'a>(
    item_group: &'a ItemGroup, 
    mode: &'a Mode,
    all_groups: &'a HashMap<EntityId, ItemGroup>
) -> Element<'a, Message> {
    match mode {
        Mode::View => view::view(item_group).map(Message::View),
        Mode::Edit => {
            edit::view(
                item_group,
                EditState::new(item_group),
                all_groups
            ).map(Message::Edit)
        }
    }
}

/*  
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
            ValidationError::InvalidPrice(msg)=> write!(f, "Invalid Price: {}", msg),
            ValidationError::MissingItemGroup(msg)=> write!(f, "Missing Item Group: {}", msg),
            ValidationError::MissingRevenueCategory(msg)=> write!(f, "Missing Revenue Group: {}", msg),
        }
    }
}
 */