use crate::data_types::{self, EntityId, ValidationError, Validatable};
use crate::Action;
use crate::entity_component::{self, Entity, EditState as BaseEditState};
use crate::icon;
use iced_modern_theme::Modern;
use serde::{Serialize, Deserialize};
use iced::{Element, Length};
use iced::widget::{button, row, column, container, text, text_input, scrollable, tooltip};
use std::ops::Range;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Message {
    CreateNew,
    RequestDelete(EntityId),
    CopyItemGroup(EntityId),
    EditItemGroup(EntityId),
    UpdateId(String),
    UpdateName(String),
    Select(EntityId),
    SaveAll(EntityId, ItemGroupEditState),
    UpdateMultiName(EntityId, String),
    UpdateIdRangeStart(EntityId, String),
    UpdateIdRangeEnd(EntityId, String),
    CreateNewMulti,
    CancelEdit(EntityId),
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
    EditItemGroup(EntityId),
    Select(EntityId),
    SaveAll(EntityId, ItemGroupEditState),
    UpdateMultiName(EntityId, String),
    UpdateIdRangeStart(EntityId, String),
    UpdateIdRangeEnd(EntityId, String),
    CreateNewMulti,
    CancelEdit(EntityId),
}

#[derive(Debug, Clone)]
pub enum Mode {
    View,
    Edit,
}

#[derive(Default, Debug, Clone)]
pub struct ItemGroupEditState {
    pub base: BaseEditState,
    pub id_range_start: String,
    pub original_id_range_start: String,
    pub id_range_end: String,
    pub original_id_range_end: String,
    pub range_validation_error: Option<String>,
}

impl ItemGroupEditState {
    pub fn new(item_group: &ItemGroup) -> Self {
        Self {
            base: BaseEditState::new(item_group),
            id_range_start: item_group.id_range.start.to_string(),
            original_id_range_start: item_group.id_range.start.to_string(),
            id_range_end: item_group.id_range.end.to_string(),
            original_id_range_end: item_group.id_range.end.to_string(),
            range_validation_error: None,
        }
    }

    pub fn reset(&mut self) {
        self.base.reset();
        self.id_range_start = self.original_id_range_start.clone();
        self.id_range_end = self.original_id_range_end.clone();
        self.range_validation_error = None;
    }
 
    pub fn validate(&self) -> Result<(), ValidationError> {
        // First validate the base fields
        self.base.validate(1..=99999)?;
 
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

impl Entity for ItemGroup {
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
    state: &mut ItemGroupEditState,
    other_groups: &[&ItemGroup]
) -> Action<Operation, Message> {
    match message {
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
        Message::EditItemGroup(id) => {
            Action::operation(Operation::EditItemGroup(id))
        },
        Message::Select(id) => {
            Action::operation(Operation::Select(id))
        },
        Message::UpdateId(id) => {
            if let Ok(id) = id.parse() {
                item_group.id = id;
                Action::none()
            } else {
                state.base.id_validation_error = Some("Invalid ID format".to_string());
                Action::none()
            }
        },
        Message::UpdateName(name) => {
            item_group.name = name;
            Action::none()
        },
        Message::CreateNewMulti => {
            Action::operation(Operation::CreateNewMulti)
        },
        Message::SaveAll(id, edit_state) => {
            Action::operation(Operation::SaveAll(id, edit_state))
        }
        Message::UpdateMultiName(id, new_name) => {
            Action::operation(Operation::UpdateMultiName(id, new_name))
        }
        Message::UpdateIdRangeStart(id, new_start) => {
            Action::operation(Operation::UpdateIdRangeStart(id, new_start))
        }
        Message::UpdateIdRangeEnd(id, new_end) => {
            Action::operation(Operation::UpdateIdRangeEnd(id, new_end))
        }
        Message::CancelEdit(id) => {
            Action::operation(Operation::CancelEdit(id))
        }
    }
}

pub fn view<'a>(
    all_groups: &'a BTreeMap<EntityId, ItemGroup>,
    edit_states: &'a Vec<ItemGroupEditState>,
) -> Element<'a, Message> {
    let title_row = entity_component::render_title_row(
        "Item Groups", 
        Message::CreateNewMulti,
        805.0 // view width
    );

    // Custom header row for columns including range fields
    let header_row = row![
        text("ID").width(Length::Fixed(75.0)),
        text("Name").width(Length::Fixed(250.0)),
        text("Range Start").width(Length::Fixed(150.0)),
        text("Range End").width(Length::Fixed(150.0)),
        text("Actions").width(Length::Fixed(150.0)),
    ]
    .padding(15);

    // List of item groups
    let groups_list = scrollable(
        column(
            all_groups
                .values()
                .map(|group| 
                    row![
                        render_item_group_row(group, edit_states)
                    ]
                    .padding(5)
                    .into()
                )
                .collect::<Vec<_>>()
        )
    ).height(Length::Fill);

    // Combine all elements
    let all_content = column![title_row, header_row, groups_list];

    column![
        container(all_content)
            .height(Length::Shrink)
            .style(Modern::card_container())
    ]
    .into()
}

fn render_item_group_row<'a>(
    item_group: &'a ItemGroup,
    edit_states: &'a Vec<ItemGroupEditState>
) -> Element<'a, Message> {
    // Find edit state for this item_group if it exists
    let edit_state = edit_states.iter()
        .find(|state| state.base.id.parse::<i32>().unwrap_or(-999) == item_group.id);

    let editing = edit_state.is_some();

    // Get display values
    let display_name = edit_state
        .map(|state| state.base.name.clone())
        .unwrap_or_else(|| item_group.name.clone());

    let range_start = edit_state
        .map(|state| state.id_range_start.clone())
        .unwrap_or_else(|| item_group.id_range.start.to_string());

    let range_end = edit_state
        .map(|state| state.id_range_end.clone())
        .unwrap_or_else(|| item_group.id_range.end.to_string());

    // Check for validation errors
    let id_validation_error = edit_state
        .and_then(|state| state.base.id_validation_error.as_ref());

    let name_validation_error = edit_state
        .and_then(|state| state.base.name_validation_error.as_ref());

    let range_validation_error = edit_state
        .and_then(|state| state.range_validation_error.as_ref());

    // ID input with validation
    let id_input: Element<'_, Message> = {
        let input = text_input("ID (1-999)", &item_group.id.to_string())
            .style(Modern::validated_text_input(id_validation_error.is_some()))
            .width(Length::Fixed(75.0));

        if let Some(error) = id_validation_error {
            tooltip(
                input,
                container(error.as_str()).padding(10).style(Modern::danger_tooltip_container()),
                tooltip::Position::Top,
            ).into()
        } else {
            input.into()
        }
    };

    // Name input with validation
    let name_input: Element<'_, Message> = {
        let input = text_input("Item Group Name", &display_name)
            .on_input_maybe(
                if editing {
                    Some(|name| Message::UpdateMultiName(item_group.id, name))
                } else {
                    None
                }
            )
            .style(Modern::validated_text_input(name_validation_error.is_some()))
            .width(Length::Fixed(250.0));

        if let Some(error) = name_validation_error {
            tooltip(
                input,
                container(error.as_str()).padding(10).style(Modern::danger_tooltip_container()),
                tooltip::Position::Top,
            ).into()
        } else {
            input.into()
        }
    };

    // Range start input with validation
    let range_start_input: Element<'_, Message> = {
        let input = text_input("Range Start", &range_start)
            .on_input_maybe(
                if editing {
                    Some(|start| Message::UpdateIdRangeStart(item_group.id, start))
                } else {
                    None
                }
            )
            .style(Modern::validated_text_input(range_validation_error.is_some()))
            .width(Length::Fixed(150.0));

        if let Some(error) = range_validation_error {
            tooltip(
                input,
                container(error.as_str()).padding(10).style(Modern::danger_tooltip_container()),
                tooltip::Position::Top,
            ).into()
        } else {
            input.into()
        }
    };

    // Range end input with validation
    let range_end_input: Element<'_, Message> = {
        let input = text_input("Range End", &range_end)
            .on_input_maybe(
                if editing {
                    Some(|end| Message::UpdateIdRangeEnd(item_group.id, end))
                } else {
                    None
                }
            )
            .style(Modern::validated_text_input(range_validation_error.is_some()))
            .width(Length::Fixed(150.0));

        if let Some(error) = range_validation_error {
            tooltip(
                input,
                container(error.as_str()).padding(10).style(Modern::danger_tooltip_container()),
                tooltip::Position::Top,
            ).into()
        } else {
            input.into()
        }
    };

    // Action buttons
    let action_row = row![
        button(if editing { icon::save().size(14) } else { icon::edit().size(14) })
            .on_press(
                if editing { 
                    Message::SaveAll(item_group.id, edit_state.unwrap().clone()) 
                } else { 
                    Message::EditItemGroup(item_group.id) 
                }
            )
            .style(Modern::primary_button()),
        iced::widget::horizontal_space().width(2),
        button(icon::copy().size(14))
            .on_press(Message::CopyItemGroup(item_group.id))
            .style(Modern::primary_button()),
        iced::widget::horizontal_space().width(2),
        button(if editing { icon::cancel().size(14) } else { icon::trash().size(14) })
            .on_press(
                if editing { 
                    Message::CancelEdit(item_group.id) 
                } else { 
                    Message::RequestDelete(item_group.id) 
                }
            )
            .style(Modern::danger_button()),
    ].width(150);


    row![
        iced::widget::horizontal_space().width(3),
        id_input,
        name_input,
        range_start_input,
        range_end_input,
        iced::widget::horizontal_space().width(5),
        action_row,
    ]
    .align_y(iced::Alignment::Center)
    .width(Length::Fixed(795.0))
    .into()
}