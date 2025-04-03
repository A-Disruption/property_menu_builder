pub mod edit;
pub mod view;

use crate::data_types::{
    self,
    EntityId,
    Validatable,
    ValidationError,
};
use crate::Action;
use crate::icon;
use serde::{Serialize, Deserialize};
use iced::{Element, Length};
use iced::widget::{button, container, column, row, text, text_input, scrollable};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Message {
    Edit(edit::Message),
    View(view::Message),
    CreateNew,
    RequestDelete(EntityId),
    CopyRevenueCategory(EntityId),
    EditRevenueCategory(EntityId),
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
    Save(RevenueCategory),
    StartEdit(EntityId),
    Cancel,
    Back,
    CreateNew(RevenueCategory),
    RequestDelete(EntityId),
    CopyRevenueCategory(EntityId),
    EditRevenueCategory(EntityId),
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
    pub fn new(revenue_category: &RevenueCategory) -> Self {
        Self {
            name: revenue_category.name.clone(),
            original_name: revenue_category.name.clone(),
            id: revenue_category.id.to_string(),
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
                "Revenue category name cannot be empty".to_string()
            ));
        }

        if let Ok(id) = self.id.parse::<EntityId>() {
            if !(1..=999).contains(&id) {
                return Err(ValidationError::InvalidId(
                    "Revenue category ID must be between 1 and 999".to_string()
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
pub struct RevenueCategory {
    pub id: EntityId,
    pub name: String,
}

impl std::fmt::Display for RevenueCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Default for RevenueCategory {
    fn default() -> Self {
        Self {
            id: -1,
            name: String::new(),
        }
    }
}

impl RevenueCategory {

    pub fn new_draft() -> Self {
        Self::default()
    }

    fn validate(&self, other_categories: &[&RevenueCategory]) -> Result<(), ValidationError> {
        if !(1..=999).contains(&self.id) {
            return Err(ValidationError::InvalidId(
                "Revenue category ID must be between 1 and 999".to_string()
            ));
        }

        for other in other_categories {
            if other.id == self.id {
                return Err(ValidationError::DuplicateId(
                    format!("Revenue category with ID {} already exists", self.id)
                ));
            }
        }

        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Revenue category name cannot be empty".to_string()
            ));
        }

        Ok(())
    }
}

pub fn update(
    revenue_category: &mut RevenueCategory,
    message: Message,
    state: &mut EditState,
    other_categories: &[&RevenueCategory]
) -> Action<Operation, Message> {
    match message {
        Message::Edit(msg) => match msg {
            edit::Message::UpdateName(name) => {
                revenue_category.name = name;
                Action::none()
            }
            edit::Message::UpdateId(id) => {
                if let Ok(id) = id.parse() {
                    if revenue_category.id < 0 {
                        revenue_category.id = id;
                    }
                    Action::none()
                } else {
                    state.validation_error = Some("Invalid ID format".to_string());
                    Action::none()
                }
            }
            edit::Message::Save => {
                if revenue_category.validate(other_categories).is_ok() {
                    Action::operation(Operation::Save(revenue_category.clone()))
                } else {
                    state.validation_error = Some("Validation failed".to_string());
                    Action::none()
                }
            }
            edit::Message::Cancel => Action::operation(Operation::Cancel),
        },
        Message::View(msg) => match msg {
            view::Message::Edit => Action::operation(Operation::StartEdit(revenue_category.id)),
            view::Message::Back => Action::operation(Operation::Back),
        }
        Message::CreateNew => {
            let new_revenue_category = RevenueCategory::default();
            Action::operation(Operation::CreateNew(new_revenue_category))
        },
        Message::RequestDelete(id) => {
            Action::operation(Operation::RequestDelete(id))
        },
        Message::CopyRevenueCategory(id) => {
            Action::operation(Operation::CopyRevenueCategory(id))
        },
        Message::EditRevenueCategory(id) => {
            Action::operation(Operation::EditRevenueCategory(id))
        },
        Message::Select(id) => {
            Action::operation(Operation::Select(id))
        },
        Message::UpdateId(id) => {
            if let Ok(id) = id.parse() {
                revenue_category.id = id;
                Action::none()
            } else {
                state.validation_error = Some("Invalid ID format".to_string());
                Action::none()
            }
        },
        Message::UpdateName(name) => {
            revenue_category.name = name;
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
    all_categories: &'a BTreeMap<EntityId, RevenueCategory>,
    edit_states: &'a Vec<EditState>,
) -> Element<'a, Message> {

    let title_row = container(
        row![
            text("Revenue Category").size(18).style(text::primary),
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

    let category_list = scrollable(
        column(
            all_categories
                .values()
                .map(|category| 
                    container(
                        logical_quick_edit_view(
                            category,
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
        container(category_list)
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
    revenue_category: &'a RevenueCategory,
    edit_states: &'a Vec<EditState>
    ) 
    -> Element<'a, Message> {

        // Find edit state for this revenue_category if it exists
        let edit_state = edit_states.iter()
            .find(|state| state.id.parse::<i32>().unwrap() == revenue_category.id);

        let editing = edit_state.is_some();

        let display_name = edit_state
            .map(|state| state.name.clone())
            .unwrap_or_else(|| revenue_category.name.clone());

        // Check for validation error
        let validation_error = edit_state
        .and_then(|state| state.validation_error.as_ref())
        .cloned();

        let button_content: iced::widget::Button<'a, Message> = button(
            container(
                row![
                    text_input("ID (1-25)", &revenue_category.id.to_string())
                        //.on_input(Message::UpdateId)
                        .width(Length::Fixed(75.0)),
                    text_input("Revenue Category Name", &display_name)
                        .on_input_maybe(
                            if editing {
                               Some( |a_revenue_category| Message::UpdateMultiName(revenue_category.id, a_revenue_category) )
                             } else {
                                None 
                             }
                        ).style(if validation_error.is_some() { data_types::validated_error } else { text_input::default })
                        .width(Length::Fixed(250.0)),

                    row![

                        button( if editing { icon::save().size(14) } else { icon::edit().size(14) })
                        .on_press( if editing { Message::SaveAll(revenue_category.id, edit_state.unwrap().clone()) } else { Message::EditRevenueCategory(revenue_category.id) })
                        .style(
                            button::primary
                    ),
                        iced::widget::horizontal_space().width(2),
                    button(icon::copy().size(14))
                        .on_press(Message::CopyRevenueCategory(revenue_category.id))
                        .style(
                            button::primary
                    ),
                    iced::widget::horizontal_space().width(2),
                    button(if editing { icon::cancel().size(14) } else { icon::trash().size(14) })
                        .on_press( if editing { Message::CancelEdit(revenue_category.id) } else { Message::RequestDelete(revenue_category.id) })
                        .style(button::danger),
                    ].width(150),
                ].align_y(iced::Alignment::Center),

            )
        )
        .width(iced::Length::Shrink)
        .on_press(Message::Select(revenue_category.id))
        .style(
            button::secondary
        ).into();


        
        button_content.into()
}