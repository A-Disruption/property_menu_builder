use crate::data_types::{ EntityId, ValidationError };
use crate::Action;
use crate::entity_component::{self, Entity, EditState as BaseEditState};
use crate::icon;
use iced_modern_theme::Modern;
use serde::{Serialize, Deserialize};
use iced::{Element, Length};
use iced::widget::{button, container, column, row, text, text_input, scrollable, tooltip};
use std::collections::BTreeMap;
use rust_decimal::Decimal;


#[derive(Debug, Clone)]
pub enum Message {
    RequestDelete(EntityId),
    CopyTaxGroup(EntityId),
    EditTaxGroup(EntityId),
    SaveAll(EntityId, TaxGroupEditState),
    UpdateName(EntityId, String),
    UpdateTaxRate(EntityId, String),
    CreateNew,
    CancelEdit(EntityId),
}

#[derive(Debug, Clone)]
pub enum Operation {
    RequestDelete(EntityId),
    CopyTaxGroup(EntityId),
    EditTaxGroup(EntityId),
    SaveAll(EntityId, TaxGroupEditState),
    UpdateName(EntityId, String),
    UpdateTaxRate(EntityId, String),
    CreateNew,
    CancelEdit(EntityId),
}

#[derive(Default, Debug, Clone)]
pub struct TaxGroupEditState {
    pub base: BaseEditState,
    pub rate: String,
    pub original_rate: String,
    pub rate_validation_error: Option<String>,
}

impl TaxGroupEditState {
    pub fn new(tax_group: &TaxGroup) -> Self {
        Self {
            base: BaseEditState::new(tax_group),
            rate: tax_group.rate_percentage().to_string(),
            original_rate: tax_group.rate_percentage().to_string(),
            rate_validation_error: None,
        }
    }

    pub fn reset(&mut self) {
        self.base.reset();
        self.rate = self.original_rate.clone();
        self.rate_validation_error = None;
    }
 
    pub fn validate(&self) -> Result<(), ValidationError> {
        // First validate the base fields
        self.base.validate(1..=99999)?;
 
        if let Ok(id) = self.base.id.parse::<EntityId>() {
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

impl std::fmt::Display for TaxGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl Entity for TaxGroup {
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
    message: Message,
) -> Action<Operation, Message> {
    match message {
        Message::RequestDelete(id) => {
            Action::operation(Operation::RequestDelete(id))
        },
        Message::CopyTaxGroup(id) => {
            Action::operation(Operation::CopyTaxGroup(id))
        },
        Message::EditTaxGroup(id) => {
            Action::operation(Operation::EditTaxGroup(id))
        },
        Message::UpdateTaxRate(id, rate) => {
            Action::operation(Operation::UpdateTaxRate(id, rate))
        },
        Message::CreateNew => {
            Action::operation(Operation::CreateNew)
        },
        Message::SaveAll(id, edit_state) => {
            Action::operation(Operation::SaveAll(id, edit_state))
        }
        Message::UpdateName(id, new_name) => {
            Action::operation(Operation::UpdateName(id, new_name))
        }
        Message::CancelEdit(id) => {
            Action::operation(Operation::CancelEdit(id))
        }
    }
}

pub fn view<'a>(
    all_groups: &'a BTreeMap<EntityId, TaxGroup>,
    edit_states: &'a Vec<TaxGroupEditState>,
) -> Element<'a, Message> {
    let title_row = entity_component::render_title_row(
        "Tax groups", 
        Message::CreateNew,
        605.0 // view width
    );

    // Custom header row for columns including range fields
    let header_row = row![
        text("ID").width(Length::Fixed(75.0)),
        text("Name").width(Length::Fixed(250.0)),
        text("Tax Rate").width(Length::Fixed(100.0)),
        text("Actions").width(Length::Fixed(150.0)),
    ]
    .padding(15);

    // List of Tax groups
    let groups_list = scrollable(
        column(
            all_groups
                .values()
                .map(|group| 
                    row![
                        render_tax_group_row(group, edit_states)
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

fn render_tax_group_row<'a>(
    tax_group: &'a TaxGroup,
    edit_states: &'a Vec<TaxGroupEditState>
) -> Element<'a, Message> {
    // Find edit state for this tax_group if it exists
    let edit_state = edit_states.iter()
        .find(|state| state.base.id.parse::<i32>().unwrap_or(-999) == tax_group.id);

    let editing = edit_state.is_some();

    // Get display values
    let display_name = edit_state
        .map(|state| state.base.name.clone())
        .unwrap_or_else(|| tax_group.name.clone());

    let tax_rate = edit_state
        .map(|state| state.rate.clone())
        .unwrap_or_else(|| tax_group.rate.to_string());

    // Check for validation errors
    let id_validation_error = edit_state
        .and_then(|state| state.base.id_validation_error.as_ref());

    let name_validation_error = edit_state
        .and_then(|state| state.base.name_validation_error.as_ref());

    let rate_validation_error = edit_state
        .and_then(|state| state.rate_validation_error.as_ref());

    // ID input with validation
    let id_input: Element<'_, Message> = {
        let input = text_input("ID (1-999)", &tax_group.id.to_string())
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
        let input = text_input("Tax group Name", &display_name)
            .on_input_maybe(
                if editing {
                    Some(|name| Message::UpdateName(tax_group.id, name))
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

    let rate_input: Element<'_, Message> = {
        let input = text_input("Tax Rate", &tax_rate)
            .on_input_maybe(
                if editing {
                    Some( |a_tax_rate| Message::UpdateTaxRate(tax_group.id, a_tax_rate) )
                } else {
                    None 
                }
            )
            .style(Modern::validated_text_input(rate_validation_error.is_some()))
            .width(Length::Fixed(100.0));

        if let Some(error) = rate_validation_error {
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
                    Message::SaveAll(tax_group.id, edit_state.unwrap().clone()) 
                } else { 
                    Message::EditTaxGroup(tax_group.id) 
                }
            )
            .style(Modern::primary_button()),
        iced::widget::horizontal_space().width(2),
        button(icon::copy().size(14))
            .on_press(Message::CopyTaxGroup(tax_group.id))
            .style(Modern::primary_button()),
        iced::widget::horizontal_space().width(2),
        button(if editing { icon::cancel().size(14) } else { icon::trash().size(14) })
            .on_press(
                if editing { 
                    Message::CancelEdit(tax_group.id) 
                } else { 
                    Message::RequestDelete(tax_group.id) 
                }
            )
            .style(Modern::danger_button()),
    ].width(150);


    row![
        iced::widget::horizontal_space().width(3),
        id_input,
        name_input,
        rate_input,
        iced::widget::horizontal_space().width(5),
        action_row,
    ]
    .align_y(iced::Alignment::Center)
    .width(Length::Fixed(595.0))
    .into()
}