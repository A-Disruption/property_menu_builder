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
use iced::widget::{button, container, column, row, text, text_input, scrollable};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Message {
    CreateNew,
    RequestDelete(EntityId),
    CopySecurityLevel(EntityId),
    EditSecurityLevel(EntityId),
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
    Save(SecurityLevel),
    StartEdit(EntityId),
    Cancel,
    Back,
    CreateNew(SecurityLevel),
    RequestDelete(EntityId),
    CopySecurityLevel(EntityId),
    EditSecurityLevel(EntityId),
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

#[derive(Default, Debug, Clone)]
pub struct EditState {
    pub name: String,
    pub original_name: String,
    pub id: String,
    pub validation_error: Option<String>,
}

impl EditState {
    pub fn new(security_level: &SecurityLevel) -> Self {
        Self {
            name: security_level.name.clone(),
            original_name: security_level.name.clone(),
            id: security_level.id.to_string(),
            validation_error: None,
        }
    }

    pub fn reset(& mut self) {
        self.name = self.original_name.clone();
        self.validation_error = None;
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Security level name cannot be empty".to_string()
            ));
        }

        if let Ok(id) = self.id.parse::<EntityId>() {
            if !(1..=999).contains(&id) {
                return Err(ValidationError::InvalidId(
                    "Security level ID must be between 1 and 999".to_string()
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityLevel {
    pub id: EntityId,
    pub name: String,
}

impl std::fmt::Display for SecurityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Default for SecurityLevel {
    fn default() -> Self {
        Self {
            id: -1,
            name: String::new(),
        }
    }
}

impl SecurityLevel {

    pub fn new_draft() -> Self {
        Self::default()
    }

    fn validate(&self, other_levels: &[&SecurityLevel]) -> Result<(), ValidationError> {
        if !(1..=999).contains(&self.id) {
            return Err(ValidationError::InvalidId(
                "Security level ID must be between 1 and 999".to_string()
            ));
        }

        for other in other_levels {
            if other.id == self.id {
                return Err(ValidationError::DuplicateId(
                    format!("Security level with ID {} already exists", self.id)
                ));
            }
        }

        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Security level name cannot be empty".to_string()
            ));
        }

        Ok(())
    }
}

pub fn update(
    security_level: &mut SecurityLevel,
    message: Message,
    state: &mut EditState,
    other_levels: &[&SecurityLevel]
) -> Action<Operation, Message> {
    match message {
        Message::CreateNew => {
            println!("CreateNew message received in security_levels");
            let new_security_level = SecurityLevel::default();
            Action::operation(Operation::CreateNew(new_security_level))
        },
        Message::RequestDelete(id) => {
            Action::operation(Operation::RequestDelete(id))
        },
        Message::CopySecurityLevel(id) => {
            Action::operation(Operation::CopySecurityLevel(id))
        },
        Message::EditSecurityLevel(id) => {
            Action::operation(Operation::EditSecurityLevel(id))
        },
        Message::Select(id) => {
            Action::operation(Operation::Select(id))
        },
        Message::UpdateId(id) => {
            if let Ok(id) = id.parse() {
                security_level.id = id;
                Action::none()
            } else {
                state.validation_error = Some("Invalid ID format".to_string());
                Action::none()
            }
        },
        Message::UpdateName(name) => {
            security_level.name = name;
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
    all_levels: &'a BTreeMap<EntityId, SecurityLevel>,
    edit_states: &'a Vec<EditState>,
) -> Element<'a, Message> {

    let title_row = container(
        row![
            text("Security Levels").size(18).style(text::primary),
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

    let levels_list = scrollable(
        column(
            all_levels
                .values()
                .map(|level| 
                    container(
                        logical_quick_edit_view(
                            level,
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
        container(levels_list)
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
    security_level: &'a SecurityLevel,
    edit_states: &'a Vec<EditState>
    ) 
    -> Element<'a, Message> {

        // Find edit state for this choice_group if it exists
        let edit_state = edit_states.iter()
            .find(|state| state.id.parse::<i32>().unwrap() == security_level.id);

        let editing = edit_state.is_some();

        let display_name = edit_state
            .map(|state| state.name.clone())
            .unwrap_or_else(|| security_level.name.clone());

        // Check for validation error
        let validation_error = edit_state
        .and_then(|state| state.validation_error.as_ref())
        .cloned();

        let button_content: iced::widget::Button<'a, Message> = button(
            container(
                row![
                    text_input("ID (1-25)", &security_level.id.to_string())
                        //.on_input(Message::UpdateId)
                        .width(Length::Fixed(75.0)),
                    text_input("Security Level Name", &display_name)
                        .on_input_maybe(
                            if editing {
                               Some( |a_security_level| Message::UpdateMultiName(security_level.id, a_security_level) )
                             } else {
                                None 
                             }
                        ).style(if validation_error.is_some() { data_types::validated_error } else { text_input::default })
                        .width(Length::Fixed(250.0)),

                    row![

                        button( if editing { icon::save().size(14) } else { icon::edit().size(14) })
                        .on_press( if editing { Message::SaveAll(security_level.id, edit_state.unwrap().clone()) } else { Message::EditSecurityLevel(security_level.id) })
                        .style(
                            button::primary
                    ),
                        iced::widget::horizontal_space().width(2),
                    button(icon::copy().size(14))
                        .on_press(Message::CopySecurityLevel(security_level.id))
                        .style(
                            button::primary
                    ),
                    iced::widget::horizontal_space().width(2),
                    button(if editing { icon::cancel().size(14) } else { icon::trash().size(14) })
                        .on_press( if editing { Message::CancelEdit(security_level.id) } else { Message::RequestDelete(security_level.id) })
                        .style(button::danger),
                    ].width(150),
                ].align_y(iced::Alignment::Center),

            )
        )
        .width(iced::Length::Shrink)
        .on_press(Message::Select(security_level.id))
        .style(
            button::secondary
        ).into();


        
        button_content.into()
}