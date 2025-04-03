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
    CopyReportCategory(EntityId),
    EditReportCategory(EntityId),
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
    Save(ReportCategory),
    StartEdit(EntityId),
    Cancel,
    Back,
    CreateNew(ReportCategory),
    RequestDelete(EntityId),
    CopyReportCategory(EntityId),
    EditReportCategory(EntityId),
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
    pub fn new(report_category: &ReportCategory) -> Self {
        Self {
            name: report_category.name.clone(),
            original_name: report_category.name.clone(),
            id: report_category.id.to_string(),
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
                "Report category name cannot be empty".to_string()
            ));
        }

        if let Ok(id) = self.id.parse::<EntityId>() {
            if !(1..=999).contains(&id) {
                return Err(ValidationError::InvalidId(
                    "Report category ID must be between 1 and 999".to_string()
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
pub struct ReportCategory {
    pub id: EntityId,
    pub name: String,
}

impl std::fmt::Display for ReportCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Default for ReportCategory {
    fn default() -> Self {
        Self {
            id: -1,
            name: String::new(),
        }
    }
}

impl ReportCategory {

    pub fn new_draft() -> Self {
        Self::default()
    }

    fn validate(&self, other_categories: &[&ReportCategory]) -> Result<(), ValidationError> {
        if !(1..=999).contains(&self.id) {
            return Err(ValidationError::InvalidId(
                "Report category ID must be between 1 and 999".to_string()
            ));
        }

        for other in other_categories {
            if other.id == self.id {
                return Err(ValidationError::DuplicateId(
                    format!("Report category with ID {} already exists", self.id)
                ));
            }
        }

        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Report category name cannot be empty".to_string()
            ));
        }

        Ok(())
    }
}


pub fn update(
    report_category: &mut ReportCategory,
    message: Message,
    state: &mut EditState,
    other_categories: &[&ReportCategory]
) -> Action<Operation, Message> {
    match message {
        Message::CreateNew => {
            let new_report_category = ReportCategory::default();
            Action::operation(Operation::CreateNew(new_report_category))
        },
        Message::RequestDelete(id) => {
            Action::operation(Operation::RequestDelete(id))
        },
        Message::CopyReportCategory(id) => {
            Action::operation(Operation::CopyReportCategory(id))
        },
        Message::EditReportCategory(id) => {
            Action::operation(Operation::EditReportCategory(id))
        },
        Message::Select(id) => {
            Action::operation(Operation::Select(id))
        },
        Message::UpdateId(id) => {
            if let Ok(id) = id.parse() {
                report_category.id = id;
                Action::none()
            } else {
                state.validation_error = Some("Invalid ID format".to_string());
                Action::none()
            }
        },
        Message::UpdateName(name) => {
            report_category.name = name;
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
    all_categories: &'a BTreeMap<EntityId, ReportCategory>,
    edit_states: &'a Vec<EditState>,
) -> Element<'a, Message> {

    let title_row = container(
        row![
            text("Report Category").size(18).style(text::primary),
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
    report_category: &'a ReportCategory,
    edit_states: &'a Vec<EditState>
    ) 
    -> Element<'a, Message> {

        // Find edit state for this report_category if it exists
        let edit_state = edit_states.iter()
            .find(|state| state.id.parse::<i32>().unwrap() == report_category.id);

        let editing = edit_state.is_some();

        let display_name = edit_state
            .map(|state| state.name.clone())
            .unwrap_or_else(|| report_category.name.clone());

        // Check for validation error
        let validation_error = edit_state
        .and_then(|state| state.validation_error.as_ref())
        .cloned();

        let button_content: iced::widget::Button<'a, Message> = button(
            container(
                row![
                    text_input("ID (1-25)", &report_category.id.to_string())
                        //.on_input(Message::UpdateId)
                        .width(Length::Fixed(75.0)),
                    text_input("Report Category Name", &display_name)
                        .on_input_maybe(
                            if editing {
                               Some( |a_report_category| Message::UpdateMultiName(report_category.id, a_report_category) )
                             } else {
                                None 
                             }
                        ).style(if validation_error.is_some() { data_types::validated_error } else { text_input::default })
                        .width(Length::Fixed(250.0)),

                    row![

                        button( if editing { icon::save().size(14) } else { icon::edit().size(14) })
                        .on_press( if editing { Message::SaveAll(report_category.id, edit_state.unwrap().clone()) } else { Message::EditReportCategory(report_category.id) })
                        .style(
                            button::primary
                    ),
                        iced::widget::horizontal_space().width(2),
                    button(icon::copy().size(14))
                        .on_press(Message::CopyReportCategory(report_category.id))
                        .style(
                            button::primary
                    ),
                    iced::widget::horizontal_space().width(2),
                    button(if editing { icon::cancel().size(14) } else { icon::trash().size(14) })
                        .on_press( if editing { Message::CancelEdit(report_category.id) } else { Message::RequestDelete(report_category.id) })
                        .style(button::danger),
                    ].width(150),
                ].align_y(iced::Alignment::Center),

            )
        )
        .width(iced::Length::Shrink)
        .on_press(Message::Select(report_category.id))
        .style(
            button::secondary
        ).into();


        
        button_content.into()
}