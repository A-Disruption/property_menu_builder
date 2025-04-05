use crate::data_types::{
    self,
    EntityId,
    ValidationError,
    Validatable,
};
use crate::Action;
use crate::icon;
use serde::{Serialize, Deserialize};
use iced::{Element, Length};
use iced::widget::{row, column, text, button, container, text_input, scrollable};
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
    SaveAll(EntityId, EditState),
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
    SaveAll(EntityId, EditState),
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
pub struct EditState {
    pub name: String,
    pub original_name: String,
    pub id: String,
    pub id_range_start: String,
    pub original_id_range_start: String,
    pub id_range_end: String,
    pub original_id_range_end: String,
    pub validation_error: Option<String>,
}

impl EditState {
    pub fn new(item_group: &ItemGroup) -> Self {
        Self {
            name: item_group.name.clone(),
            original_name: item_group.name.clone(),
            id: item_group.id.to_string(),
            id_range_start: item_group.id_range.start.to_string(),
            original_id_range_start: item_group.id_range.start.to_string(),
            id_range_end: item_group.id_range.end.to_string(),
            original_id_range_end: item_group.id_range.end.to_string(),
            validation_error: None,
        }
    }

    pub fn reset(& mut self) {
        self.name = self.original_name.clone();
        self.id_range_start = self.original_id_range_start.clone();
        self.id_range_end = self.original_id_range_end.clone();
        self.validation_error = None;
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
                state.validation_error = Some("Invalid ID format".to_string());
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
        Message::UpdateIdRangeStart(id, new_name) => {
            Action::operation(Operation::UpdateIdRangeStart(id, new_name))
        }
        Message::UpdateIdRangeEnd(id, new_name) => {
            Action::operation(Operation::UpdateIdRangeEnd(id, new_name))
        }
        Message::CancelEdit(id) => {
            Action::operation(Operation::CancelEdit(id))
        }
    }
 }

 pub fn view<'a>(
    all_groups: &'a BTreeMap<EntityId, ItemGroup>,
    edit_states: &'a Vec<EditState>,
) -> Element<'a, Message> {

    let title_row = container(
        row![
            text("Item Groups").size(18).style(text::primary),
            iced::widget::horizontal_space(),
            button(icon::new().size(14))
                .on_press(Message::CreateNewMulti)
                .style(button::primary),
        ]
        .width(Length::Fixed(805.0))
        .padding(15)
    )
    .style(container::rounded_box);

    // Header row for columns
    let header_row = container(
        row![
            text("ID").width(Length::Fixed(75.0)),
            text("Name").width(Length::Fixed(250.0)),
            text("Range Start").width(Length::Fixed(150.0)),
            text("Range End").width(Length::Fixed(150.0)),
            text("Actions").width(Length::Fixed(150.0)),
        ]
        .padding(15)
    )
    .style(container::rounded_box);

    let groups_list = scrollable(
        column(
            all_groups
                .values()
                .map(|group| 
                    container(
                        logical_quick_edit_view(
                            group,
                            edit_states
                        )
                    )
                    .style(container::bordered_box)
                    .padding(5)
                    .into()
                )
                .collect::<Vec<_>>()
        )
    ).height(Length::Fill);

    column![
        title_row,
        header_row,
        container(groups_list)
            .height(Length::Fill)
            .style(container::rounded_box)
    ].into()
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


fn logical_quick_edit_view<'a>(
    item_group: &'a ItemGroup,
    edit_states: &'a Vec<EditState>
    ) 
    -> Element<'a, Message> {

        // Find edit state for this item_group if it exists
        let edit_state = edit_states.iter()
            .find(|state| state.id.parse::<i32>().unwrap() == item_group.id);

        let editing = edit_state.is_some();

        let display_name = edit_state
            .map(|state| state.name.clone())
            .unwrap_or_else(|| item_group.name.clone());

        let range_start = edit_state
            .map(|state| state.id_range_start.clone())
            .unwrap_or_else(|| item_group.id_range.start.to_string());

        let range_end = edit_state
            .map(|state| state.id_range_end.clone())
            .unwrap_or_else(|| item_group.id_range.end.to_string());

        // Check for validation error
        let validation_error = edit_state
        .and_then(|state| state.validation_error.as_ref())
        .cloned();

        let button_content: iced::widget::Button<'a, Message> = button(
            container(
                row![
                    text_input("ID (1-25)", &item_group.id.to_string())
                        //.on_input(Message::UpdateId)
                        .width(Length::Fixed(75.0)),
                    text_input("Item Group Name", &display_name)
                        .on_input_maybe(
                            if editing {
                               Some( |a_item_group| Message::UpdateMultiName(item_group.id, a_item_group) )
                             } else {
                                None 
                             }
                        ).style(if validation_error.is_some() { data_types::validated_error } else { text_input::default })
                        .width(Length::Fixed(250.0)),
                    
                    text_input("Item ID Range Start", &range_start)
                        .on_input_maybe(
                            if editing {
                               Some( |a_item_group| Message::UpdateIdRangeStart(item_group.id, a_item_group) )
                             } else {
                                None 
                             }
                        ).style(if validation_error.is_some() { data_types::validated_error } else { text_input::default })
                        .width(Length::Fixed(150.0)),
                    text_input("Item ID Range End", &range_end)
                        .on_input_maybe(
                            if editing {
                               Some( |a_item_group| Message::UpdateIdRangeEnd(item_group.id, a_item_group) )
                             } else {
                                None 
                             }
                        ).style(if validation_error.is_some() { data_types::validated_error } else { text_input::default })
                        .width(Length::Fixed(150.0)),

                    row![

                        button( if editing { icon::save().size(14) } else { icon::edit().size(14) })
                        .on_press( if editing { Message::SaveAll(item_group.id, edit_state.unwrap().clone()) } else { Message::EditItemGroup(item_group.id) })
                        .style(
                            button::primary
                    ),
                        iced::widget::horizontal_space().width(2),
                    button(icon::copy().size(14))
                        .on_press(Message::CopyItemGroup(item_group.id))
                        .style(
                            button::primary
                    ),
                    iced::widget::horizontal_space().width(2),
                    button(if editing { icon::cancel().size(14) } else { icon::trash().size(14) })
                        .on_press( if editing { Message::CancelEdit(item_group.id) } else { Message::RequestDelete(item_group.id) })
                        .style(button::danger),
                    ].width(150),
                ].align_y(iced::Alignment::Center),

            )
        )
        .width(iced::Length::Shrink)
        .on_press(Message::Select(item_group.id))
        .style(
            button::secondary
        ).into();


        
        button_content.into()
}