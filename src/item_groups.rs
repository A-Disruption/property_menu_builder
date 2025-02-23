pub mod edit;
pub mod view;

use crate::data_types::{
    EntityId,
    ValidationError,
    Validatable,
};
use crate::Action;
use crate::icon;
use serde::{Serialize, Deserialize};
use iced::Element;
use iced::widget::{row, column, text, button, container};
use std::ops::Range;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Message {
    Edit(edit::Message),
    View(view::Message),
    CreateNew,
    RequestDelete(EntityId),
    CopyItemGroup(EntityId),
    Select(EntityId),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(ItemGroup),
    StartEdit(EntityId),
    Cancel,
    Back,
    CreateNew(ItemGroup),
    RequestDelete(EntityId),
    CopyItemGroup(EntityId),
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
            id: -1,
            name: String::new(),
            id_range: Range { start: 1, end: 1000 }
        }
    }
 }

 impl ItemGroup {

    pub fn new_draft() -> Self {
        Self::default()
    }

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
                    if item_group.id < 0 {
                        item_group.id = id;
                    }
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
        Message::CreateNew => {
            let new_item_group = ItemGroup::default();
            Action::operation(Operation::CreateNew(new_item_group))
        },
        Message::RequestDelete(id) => {
            Action::operation(Operation::RequestDelete(id))
        },
        Message::CopyItemGroup(id) => {
            Action::operation(Operation::CopyItemGroup(id))
        },
        Message::Select(id) => {
            Action::operation(Operation::Select(id))
        },
    }
 }

 pub fn view<'a>(
    item_group: &'a ItemGroup, 
    mode: &'a Mode,
    all_groups: &'a BTreeMap<EntityId, ItemGroup>
) -> Element<'a, Message> {

    let groups_list = column(
        all_groups
            .values()
            .map(|group| {
                button(
                    list_item(
                        &group.name.as_str(), 
                        button(icon::copy().size(14))
                            .on_press(Message::CopyItemGroup(group.id))
                            .style(
                                if group.id == item_group.id {
                                    button::secondary
                                } else {
                                    button::primary
                                }
                            ), 
                        button(icon::trash().size(14)).on_press(Message::RequestDelete(group.id)),
                    )
                )
                .width(iced::Length::Fill)
                .on_press(Message::Select(group.id))
                .style(if group.id == item_group.id {
                    button::primary
                } else {
                    button::secondary
                })
                .into()
            })
            .collect::<Vec<_>>()
    )
    .spacing(5)
    .width(iced::Length::Fixed(250.0));

    let content = match mode {
        Mode::View => view::view(item_group).map(Message::View),
        Mode::Edit => {
            edit::view(
                item_group,
                EditState::new(item_group),
                all_groups
            ).map(Message::Edit)
        }
    };

    row![
        container(
            column![
                row![
                    text("Item Groups").size(18),
                    iced::widget::horizontal_space(),
                    button(icon::new().size(14))
                        .on_press(Message::CreateNew)
                        .style(button::primary),
                ].width(250),
                groups_list,
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

pub fn list_item<'a>(list_text: &'a str, copy_button: iced::widget::Button<'a, Message>,delete_button: iced::widget::Button<'a, Message>) -> Element<'a, Message> {
    let button_content = container (
        row![
            text(list_text),
            iced::widget::horizontal_space(),
            copy_button,
            delete_button.style(button::danger)
        ].align_y(iced::Alignment::Center),
    );
    
    button_content.into()
}
