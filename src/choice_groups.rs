use crate::data_types::{
    self,
    EntityId,
    Validatable,
};
use crate::Action;
use crate::icon;
use serde::{Serialize, Deserialize};
use iced::widget::{button, row, column, container, text, text_input, scrollable};
use iced::{Element, Length};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Message {
    CreateNew,
    RequestDelete(EntityId),
    CopyChoiceGroup(EntityId),
    EditChoiceGroup(EntityId),
    UpdateId(String),
    UpdateName(String),
    Select(EntityId),
    SaveAll(EntityId, EditState),
    UpdateMultiName(EntityId, String),
    CreateNewMulti,
    CancelEdit(EntityId),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(ChoiceGroup),
    StartEdit(EntityId),
    Cancel,
    Back,
    CreateNew(ChoiceGroup),
    RequestDelete(EntityId),
    CopyChoiceGroup(EntityId),
    EditChoiceGroup(EntityId),
    Select(EntityId),
    SaveAll(EntityId, EditState),
    UpdateMultiName(EntityId, String),
    CreateNewMulti,
    CancelEdit(EntityId),
}

#[derive(Debug, Clone)]
pub enum Mode {
    View,
    Edit,
}

#[derive(Debug, Default, Clone)]
pub struct EditState {
    pub name: String,
    pub original_name: String,
    pub id: String,
    pub validation_error: Option<String>,
    pub next_id: EntityId,
}

impl EditState {
    pub fn new(choice_group: &ChoiceGroup, next_id: EntityId) -> Self {
        Self {
            name: choice_group.name.clone(),
            original_name: choice_group.name.clone(),
            id: choice_group.id.to_string(),
            validation_error: None,
            next_id,
        }
    }

    pub fn reset(& mut self) {
        self.name = self.original_name.clone();
        self.validation_error = None;
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Choice group name cannot be empty".to_string()
            ));
        }

        if let Ok(id) = self.id.parse::<EntityId>() {
            if !(1..=999).contains(&id) {
                return Err(ValidationError::InvalidId(
                    "Choice group ID must be between 1 and 999".to_string()
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

#[derive(Debug, Clone)]
pub enum ValidationError {
    InvalidId(String),
    DuplicateId(String),
    EmptyName(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChoiceGroup {
    pub id: EntityId,
    pub name: String,
}

impl std::fmt::Display for ChoiceGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Default for ChoiceGroup {
    fn default() -> Self {
        Self {
            id: -1,
            name: String::new(),
        }
    }
}

impl ChoiceGroup {

    pub fn new_draft() -> Self {
        Self::default()
    }

    fn validate(&self, other_groups: &[&ChoiceGroup]) -> Result<(), ValidationError> {
        if !(1..=999).contains(&self.id) {
            return Err(ValidationError::InvalidId(
                "Choice group ID must be between 1 and 999".to_string()
            ));
        }

        for other in other_groups {
            if other.id == self.id {
                return Err(ValidationError::DuplicateId(
                    format!("Choice group with ID {} already exists", self.id)
                ));
            }
        }

        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Choice group name cannot be empty".to_string()
            ));
        }

        Ok(())
    }
}

pub fn update(
    choice_group: &mut ChoiceGroup,
    message: Message,
    state: &mut EditState,
    other_groups: &[&ChoiceGroup]
) -> Action<Operation, Message> {
    match message {
        Message::CreateNew => {
            let new_choice_group = ChoiceGroup::default();
            Action::operation(Operation::CreateNew(new_choice_group))
        },
        Message::RequestDelete(id) => {
            Action::operation(Operation::RequestDelete(id))
        },
        Message::CopyChoiceGroup(id) => {
            Action::operation(Operation::CopyChoiceGroup(id))
        },
        Message::EditChoiceGroup(id) => {
            Action::operation(Operation::EditChoiceGroup(id))
        },
        Message::Select(id) => {
            Action::operation(Operation::Select(id))
        },
        Message::UpdateId(id) => {
            if let Ok(id) = id.parse() {
                choice_group.id = id;
                Action::none()
            } else {
                state.validation_error = Some("Invalid ID format".to_string());
                Action::none()
            }
        },
        Message::UpdateName(name) => {
            choice_group.name = name;
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
        Message::CancelEdit(id) => {
            Action::operation(Operation::CancelEdit(id))
        }
    }
}

pub fn view<'a>(
    all_groups: &'a BTreeMap<EntityId, ChoiceGroup>,
    edit_states: &'a Vec<EditState>,
) -> Element<'a, Message> {

    let title_row = container(
        row![
            text("Choice Groups").size(18).style(text::primary),
            iced::widget::horizontal_space(),
            button(icon::new().size(14))
                .on_press(Message::CreateNewMulti)
                .style(button::primary),
        ]
        .width(Length::Fixed(505.0))
        .padding(15)
    )
    .style(container::rounded_box);

    // Header row for columns
    let header_row = container(
        row![
            text("ID").width(Length::Fixed(75.0)),
            text("Name").width(Length::Fixed(250.0)),
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

fn get_next_id(groups: &BTreeMap<EntityId, ChoiceGroup>) -> EntityId {
    groups
        .keys()
        .max()
        .map_or(1, |max_id| max_id + 1)
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
    choice_group: &'a ChoiceGroup,
    edit_states: &'a Vec<EditState>
    ) 
    -> Element<'a, Message> {

        // Find edit state for this choice_group if it exists
        let edit_state = edit_states.iter()
            .find(|state| state.id.parse::<i32>().unwrap() == choice_group.id);

        let editing = edit_state.is_some();

        let display_name = edit_state
            .map(|state| state.name.clone())
            .unwrap_or_else(|| choice_group.name.clone());

        // Check for validation error
        let validation_error = edit_state
        .and_then(|state| state.validation_error.as_ref())
        .cloned();

        let button_content: iced::widget::Button<'a, Message> = button(
            container(
                row![
                    text_input("ID (1-25)", &choice_group.id.to_string())
                        //.on_input(Message::UpdateId)
                        .width(Length::Fixed(75.0)),
                    text_input("Choice Group Name", &display_name)
                        .on_input_maybe(
                            if editing {
                               Some( |a_choice_group| Message::UpdateMultiName(choice_group.id, a_choice_group) )
                             } else {
                                None 
                             }
                        ).style(if validation_error.is_some() { data_types::validated_error } else { text_input::default })
                        .width(Length::Fixed(250.0)),

                    row![

                        button( if editing { icon::save().size(14) } else { icon::edit().size(14) })
                        .on_press( if editing { Message::SaveAll(choice_group.id, edit_state.unwrap().clone()) } else { Message::EditChoiceGroup(choice_group.id) })
                        .style(
                            button::primary
                    ),
                        iced::widget::horizontal_space().width(2),
                    button(icon::copy().size(14))
                        .on_press(Message::CopyChoiceGroup(choice_group.id))
                        .style(
                            button::primary
                    ),
                    iced::widget::horizontal_space().width(2),
                    button(if editing { icon::cancel().size(14) } else { icon::trash().size(14) })
                        .on_press( if editing { Message::CancelEdit(choice_group.id) } else { Message::RequestDelete(choice_group.id) })
                        .style(button::danger),
                    ].width(150),
                ].align_y(iced::Alignment::Center),

            )
        )
        .width(iced::Length::Shrink)
        .on_press(Message::Select(choice_group.id))
        .style(
            button::secondary
        ).into();


        
        button_content.into()
}