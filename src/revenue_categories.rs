pub mod edit;
pub mod view;

use crate::data_types::{
    EntityId,
    Validatable,
    ValidationError,
};
use crate::Action;
use crate::icon;
use serde::{Serialize, Deserialize};
use iced::Element;
use iced::widget::{button, container, column, row, text};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Message {
    Edit(edit::Message),
    View(view::Message),
    CreateNew,
    Select(EntityId),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(RevenueCategory),
    StartEdit(EntityId),
    Cancel,
    Back,
    CreateNew(RevenueCategory),
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
    pub fn new(revenue_category: &RevenueCategory) -> Self {
        Self {
            name: revenue_category.name.clone(),
            id: revenue_category.id.to_string(),
            validation_error: None,
        }
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
        Message::Select(id) => {
            Action::operation(Operation::Select(id))
        },
    }
}

pub fn view<'a>(
    revenue_category: &'a RevenueCategory, 
    mode: &'a Mode,
    all_categories: &'a BTreeMap<EntityId, RevenueCategory>
) -> Element<'a, Message> {

    let category_list = column(
        all_categories
            .values()
            .map(|category| {
                button(text(&category.name))
                    .width(iced::Length::Fill)
                    .on_press(Message::Select(category.id))
                    .style(if category.id == revenue_category.id {
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
        Mode::View => view::view(revenue_category).map(Message::View),
        Mode::Edit => {
            edit::view(
                revenue_category,
                EditState::new(revenue_category),
                all_categories
            ).map(Message::Edit)
        }
    };

    row![
        container(
            column![
                row![
                    text("Revenue Categories").size(18),
                    iced::widget::horizontal_space(),
                    button(icon::new().shaping(text::Shaping::Advanced))
                        .on_press(Message::CreateNew)
                        .style(button::primary),
                ].width(200),
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