pub mod edit;
pub mod view;

use crate::data_types::{
    EntityId,
    ValidationError,
    Validatable,
};
use crate::Action;
use serde::{Serialize, Deserialize};
use iced::Element;
use iced::widget::{button, container, column, row, text};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Message {
    Edit(edit::Message),
    View(view::Message),
    CreateNew,
    Select(EntityId),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(ReportCategory),
    StartEdit(EntityId),
    Cancel,
    Back,
    CreateNew(ReportCategory),
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
    pub validation_error: Option<String>,
}

impl EditState {
    pub fn new(report_category: &ReportCategory) -> Self {
        Self {
            name: report_category.name.clone(),
            id: report_category.id.to_string(),
            validation_error: None,
        }
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
        Message::Edit(msg) => match msg {
            edit::Message::UpdateName(name) => {
                report_category.name = name;
                Action::none()
            }
            edit::Message::UpdateId(id) => {
                if let Ok(id) = id.parse() {
                    report_category.id = id;
                    Action::none()
                } else {
                    state.validation_error = Some("Invalid ID format".to_string());
                    Action::none()
                }
            }
            edit::Message::Save => {
                if report_category.validate(other_categories).is_ok() {
                    Action::operation(Operation::Save(report_category.clone()))
                } else {
                    state.validation_error = Some("Validation failed".to_string());
                    Action::none()
                }
            }
            edit::Message::Cancel => Action::operation(Operation::Cancel),
        },
        Message::View(msg) => match msg {
            view::Message::Edit => Action::operation(Operation::StartEdit(report_category.id)),
            view::Message::Back => Action::operation(Operation::Back),
        }
        Message::CreateNew => {
            let new_report_category = ReportCategory::default();
            Action::operation(Operation::CreateNew(new_report_category))
        },
        Message::Select(id) => {
            Action::operation(Operation::Select(id))
        },
    }
}


pub fn view<'a>(
    report_category: &'a ReportCategory, 
    mode: &'a Mode,
    all_categories: &'a HashMap<EntityId, ReportCategory>
) -> Element<'a, Message> {

    let category_list = column(
        all_categories
            .values()
            .map(|category| {
                button(text(&category.name))
                    .width(iced::Length::Fill)
                    .on_press(Message::Select(category.id))
                    .style(if category.id == report_category.id {
                        button::primary
                    } else {
                        button::secondary
                    })
                    .into()
            })
            .collect::<Vec<_>>()
    )
    .spacing(5)
    .width(iced::Length::Fixed(200.0));

    let content = match mode {
        Mode::View => view::view(report_category).map(Message::View),
        Mode::Edit => {
            edit::view(
                report_category,
                EditState::new(report_category),
                all_categories
            ).map(Message::Edit)
        }
    };

    row![
        container(
            column![
                text("Report Category").size(24),
                button("Create New")
                    .on_press(Message::CreateNew)
                    .style(button::primary),
                category_list,
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