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
use iced::widget::{button, container, column, row, text};
use std::collections::HashMap;
use rust_decimal::Decimal;
use std::fmt;


#[derive(Debug, Clone)]
pub enum Message {
    Edit(edit::Message),
    View(view::Message),
    CreateNew,
    Select(EntityId),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(TaxGroup),
    StartEdit(EntityId),
    Cancel,
    Back,
    CreateNew(TaxGroup),
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
    pub rate: String,
    pub validation_error: Option<String>,
}

impl EditState {
    pub fn new(tax_group: &TaxGroup) -> Self {
        Self {
            name: tax_group.name.clone(),
            id: tax_group.id.to_string(),
            rate: tax_group.rate_percentage().to_string(),
            validation_error: None,
        }
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Tax group name cannot be empty".to_string()
            ));
        }

        if let Ok(id) = self.id.parse::<EntityId>() {
            if !(1..=99).contains(&id) {
                return Err(ValidationError::InvalidId(
                    "Tax group ID must be between 1 and 99".to_string()
                ));
            }
        } else {
            return Err(ValidationError::InvalidId(
                "Invalid ID format".to_string()
            ));
        }

        match self.rate.parse::<Decimal>() {
            Ok(rate) => {
                if !(Decimal::ZERO..=Decimal::from(100)).contains(&rate) {
                    return Err(ValidationError::InvalidValue(
                        "Tax rate must be between 0 and 100%".to_string()
                    ));
                }
            }
            Err(_) => {
                return Err(ValidationError::InvalidValue(
                    "Invalid tax rate format".to_string()
                ));
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaxGroup {
    pub id: EntityId,
    pub name: String,
    pub rate: Decimal, // Stored as decimal (e.g., 0.08 for 8%)
}

impl fmt::Display for TaxGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Default for TaxGroup {
    fn default() -> Self {
        Self {
            id: -1,
            name: String::new(),
            rate: Decimal::ZERO,
        }
    }
}

impl TaxGroup {

    pub fn new_draft() -> Self {
        Self::default()
    }

    fn validate(&self, other_groups: &[&TaxGroup]) -> Result<(), ValidationError> {
        if !(1..=99).contains(&self.id) {
            return Err(ValidationError::InvalidId(
                "Tax group ID must be between 1 and 99".to_string()
            ));
        }

        for other in other_groups {
            if other.id == self.id {
                return Err(ValidationError::DuplicateId(
                    format!("Tax group with ID {} already exists", self.id)
                ));
            }
        }

        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Tax group name cannot be empty".to_string()
            ));
        }

        if !(Decimal::ZERO..=Decimal::ONE).contains(&self.rate) {
            return Err(ValidationError::InvalidValue(
                "Tax rate must be between 0 and 100%".to_string()
            ));
        }

        Ok(())
    }

    // Helper method to get rate as percentage
    pub fn rate_percentage(&self) -> Decimal {
        self.rate * Decimal::from(100)
    }
}

pub fn update(
    tax_group: &mut TaxGroup,
    message: Message,
    state: &mut EditState,
    other_groups: &[&TaxGroup]
) -> Action<Operation, Message> {
    match message {
        Message::Edit(msg) => match msg {
            edit::Message::UpdateName(name) => {
                tax_group.name = name;
                Action::none()
            }
            edit::Message::UpdateId(id) => {
                if let Ok(id) = id.parse() {
                    if tax_group.id < 0 {
                        tax_group.id = id;
                    }
                    Action::none()
                } else {
                    state.validation_error = Some("Invalid ID format".to_string());
                    Action::none()
                }
            }
            edit::Message::UpdateRate(rate_str) => {
                match rate_str.parse::<Decimal>() {
                    Ok(rate_percentage) => {
                        tax_group.rate = rate_percentage / Decimal::from(100);
                        Action::none()
                    }
                    Err(_) => {
                        state.validation_error = Some("Invalid tax rate format".to_string());
                        Action::none()
                    }
                }
            }
            edit::Message::Save => {
                if tax_group.validate(other_groups).is_ok() {
                    Action::operation(Operation::Save(tax_group.clone()))
                } else {
                    state.validation_error = Some("Validation failed".to_string());
                    Action::none()
                }
            }
            edit::Message::Cancel => Action::operation(Operation::Cancel),
        },
        Message::View(msg) => match msg {
            view::Message::Edit => Action::operation(Operation::StartEdit(tax_group.id)),
            view::Message::Back => Action::operation(Operation::Back),
        }
        Message::CreateNew => {
            let new_tax_group = TaxGroup::default();
            Action::operation(Operation::CreateNew(new_tax_group))
        },
        Message::Select(id) => {
            Action::operation(Operation::Select(id))
        },
    }
}

pub fn view<'a>(
    tax_group: &'a TaxGroup, 
    mode: &'a Mode,
    all_groups: &'a HashMap<EntityId, TaxGroup>
) -> Element<'a, Message> {

    let groups_list = column(
        all_groups
            .values()
            .map(|group| {
                button(text(&group.name))
                    .width(iced::Length::Fill)
                    .on_press(Message::Select(group.id))
                    .style(if group.id == tax_group.id {
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
        Mode::View => view::view(tax_group).map(Message::View),
        Mode::Edit => {
            edit::view(
                tax_group,
                EditState::new(tax_group),
                all_groups
            ).map(Message::Edit)
        }
    };

    row![
        container(
            column![
                row![
                    text("Tax Groups").size(18),
                    iced::widget::horizontal_space(),
                    button(icon::new().shaping(text::Shaping::Advanced))
                        .on_press(Message::CreateNew)
                        .style(button::primary),
                ].width(200),
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