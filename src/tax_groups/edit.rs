use iced::widget::{
    button, column, container, row, text, text_input,
    horizontal_space,
};
use iced::{Alignment, Element, Length, Color};
use rust_decimal::Decimal;
use crate::data_types::{EntityId, ValidationError};
use crate::HotKey;
use super::TaxGroup;

#[derive(Debug, Clone)]
pub enum Message {
    UpdateName(String),
    UpdateId(String),
    UpdateRate(String),
    Save,
    Cancel,
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
}

impl EditState {
    pub fn validate(&self, other_groups: &[&TaxGroup]) -> Result<(), ValidationError> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Tax group name cannot be empty".to_string()
            ));
        }

        let id: EntityId = self.id.parse().map_err(|_| {
            ValidationError::InvalidId("Invalid ID format".to_string())
        })?;

        if !(1..=99).contains(&id) {
            return Err(ValidationError::InvalidId(
                "Tax Group ID must be between 1 and 99".to_string()
            ));
        }

        let rate: f64 = self.rate.parse().map_err(|_| {
            ValidationError::InvalidValue("Invalid tax rate format".to_string())
        })?;

        if !(0.0..=100.0).contains(&rate) {
            return Err(ValidationError::InvalidValue(
                "Tax rate must be between 0 and 100%".to_string()
            ));
        }

        for other in other_groups {
            if id == other.id {
                return Err(ValidationError::DuplicateId(
                    format!("Tax Group with ID {} already exists", id)
                ));
            }
        }

        Ok(())
    }
}

pub fn view<'a>(
    group: &'a TaxGroup,
    state: EditState,
    other_groups: &'a [&'a TaxGroup],
) -> Element<'a, Message> {

    // Clone state data upfront
    let name = state.name.clone();
    let id = state.id.clone();
    let rate = state.rate.clone();
    let error_message = state.validation_error.clone();

    let content = container(
        column![
            row![
                text("Name").width(Length::Fixed(150.0)),
                text_input("Tax Group Name", &name)
                    .on_input(Message::UpdateName)
                    .padding(5)
            ],
            row![
                text("ID").width(Length::Fixed(150.0)),
                text_input("ID (1-99)", &id)
                    .on_input(Message::UpdateId)
                    .padding(5)
            ],
            row![
                text("Tax Rate (%)").width(Length::Fixed(150.0)),
                text_input("Rate", &rate)
                    .on_input(Message::UpdateRate)
                    .padding(5),
                text("%").width(Length::Fixed(30.0))
            ],
        ]
        .spacing(10)
    )
    .style(container::rounded_box)
    .padding(20);

    let controls = row![
        horizontal_space(),
        button("Cancel")
            .on_press(Message::Cancel)
            .style(button::danger),
        button("Save")
            .on_press(Message::Save)
            .style(button::success),
    ]
    .spacing(10)
    .padding(20);

    let mut col = column![content, controls].spacing(20);

    if let Some(error) = error_message {
        col = col.push(
            container(
                text(error)
                    .style(text::danger)
            )
            .padding(10)
        );
    }

    container(col)
        .padding(20)
        .into()
}

pub fn handle_hotkey(hotkey: HotKey) -> crate::Action<super::Operation, Message> {
    match hotkey {
        HotKey::Escape => crate::Action::operation(super::Operation::Cancel),
        _ => crate::Action::none(),
    }
}