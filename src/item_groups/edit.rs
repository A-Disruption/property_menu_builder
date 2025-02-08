use iced::widget::{
    button, column, container, row, text, text_input,
    horizontal_space,
};
use iced::{Element, Length};
use crate::data_types::EntityId;
use std::collections::HashMap;
use crate::HotKey;
use super::ItemGroup;

/* #[derive(Debug, Clone)]
pub struct EditState {
    pub name: String,
    pub range_start: String,
    pub range_end: String,
    pub validation_error: Option<String>,
}

impl EditState {
    pub fn new(item_group: &ItemGroup) -> Self {
        Self {
            name: item_group.name.clone(),
            range_start: item_group.id_range.start.to_string(),
            range_end: item_group.id_range.end.to_string(),
            validation_error: None,
        }
    }
}

impl EditState {
    pub fn validate(&self, other_groups: &[&ItemGroup]) -> Result<(), ValidationError> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Item group name cannot be empty".to_string()
            ));
        }

        let start: EntityId = self.range_start.parse().map_err(|_| {
            ValidationError::InvalidId("Invalid range start value".to_string())
        })?;

        let end: EntityId = self.range_end.parse().map_err(|_| {
            ValidationError::InvalidId("Invalid range end value".to_string())
        })?;

        if start >= end {
            return Err(ValidationError::InvalidRange(
                "Start ID must be less than end ID".to_string()
            ));
        }

        // Check for range overlap with other groups
        for other in other_groups {
            if ranges_overlap(&(start..=end), &(other.id_range.start..=other.id_range.end)) {
                return Err(ValidationError::RangeOverlap(
                    format!("Range overlaps with group '{}'", other.name)
                ));
            }
        }

        Ok(())
    }
}

fn ranges_overlap<T: Ord>(range1: &std::ops::RangeInclusive<T>, range2: &std::ops::RangeInclusive<T>) -> bool {
    range1.start() <= range2.end() && range2.start() <= range1.end()
} */

#[derive(Debug, Clone)]
pub enum Message {
    UpdateName(String),
    UpdateId(String),
    UpdateRangeStart(String),
    UpdateRangeEnd(String),
    Save,
    Cancel,
}

pub fn view<'a>(
    item_group: &'a ItemGroup,
    state: super::EditState,
    all_groups: &'a HashMap<EntityId, ItemGroup>
) -> Element<'a, Message> {

    let validation_error = &state.validation_error;

    let other_groups: Vec<&ItemGroup> = all_groups.values()
    .filter(|g| g.id != item_group.id)
    .collect();

    let content = container(
        column![
            row![
                text("Name").width(Length::Fixed(150.0)),
                text_input("Item Group Name", &item_group.name)
                    .on_input(Message::UpdateName)
                    .padding(5)
            ],
            row![
                text("ID").width(Length::Fixed(150.0)),
                text_input("ID (1-999)", &item_group.id.to_string())
                    .on_input(Message::UpdateId)
                    .padding(5)
            ],
            row![
                text("ID Range Start").width(Length::Fixed(150.0)),
                text_input("Range Start", &item_group.id_range.start.to_string())
                    .on_input(Message::UpdateRangeStart)
                    .padding(5)
            ],
            row![
                text("ID Range End").width(Length::Fixed(150.0)),
                text_input("Range End", &item_group.id_range.end.to_string())
                    .on_input(Message::UpdateRangeEnd)
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