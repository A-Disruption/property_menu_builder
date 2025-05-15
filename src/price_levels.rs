use crate::data_types::{ EntityId, ValidationError, Currency };
use crate::Action;
use crate::entity_component::{self, Entity, EditState as BaseEditState};
use crate::icon;
use iced_modern_theme::Modern;
use serde::{Serialize, Deserialize};
use iced::{Element, Length};
use iced::widget::{button, row, column, container, text, text_input, scrollable, tooltip};
use std::collections::BTreeMap;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub enum Message {
    RequestDelete(EntityId),
    CopyPriceLevel(EntityId),
    EditPriceLevel(EntityId),
    SaveAll(EntityId, PriceLevelEditState),
    UpdateName(EntityId, String),
    CreateNew,
    CancelEdit(EntityId),
}

#[derive(Debug, Clone)]
pub enum Operation {
    RequestDelete(EntityId),
    CopyPriceLevel(EntityId),
    EditPriceLevel(EntityId),
    SaveAll(EntityId, PriceLevelEditState),
    UpdateName(EntityId, String),
    CreateNew,
    CancelEdit(EntityId),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PriceLevelType {
    Enterprise,
    Store
}

impl Default for PriceLevelType {
    fn default() -> Self {
        Self::Enterprise
    }
}

impl std::fmt::Display for PriceLevelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PriceLevelType::Enterprise => write!(f, "Enterprise Price Level"),
            PriceLevelType::Store => write!(f, "Store Price Level"),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct PriceLevelEditState {
    pub base: BaseEditState,
    pub price: String,
    pub original_price: String,
    pub level_type: PriceLevelType,
    pub range_validation_error: Option<String>,
}

impl PriceLevelEditState {
    pub fn new(price_level: &PriceLevel) -> Self {
        Self {
            base: BaseEditState::new(price_level),
            price: price_level.price.to_string(),
            original_price: price_level.price.to_string(),
            level_type: price_level.level_type.clone(),
            range_validation_error: None,
        }
    }

    pub fn reset(&mut self) {
        self.base.reset();
        self.price = self.original_price.clone();
        self.level_type = PriceLevelType::default();
        self.range_validation_error = None;
    }
 
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.base.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Price level name cannot be empty".to_string()
            ));
        }

        if let Ok(id) = self.base.id.parse::<EntityId>() {
            if !(1..=999).contains(&id) {
                return Err(ValidationError::InvalidId(
                    "Price level ID must be between 1 and 999".to_string()
                ));
            }
        } else {
            return Err(ValidationError::InvalidId(
                "Invalid ID format".to_string()
            ));
        }

        if let Err(_) = self.price.parse::<Decimal>() {
            return Err(ValidationError::InvalidValue(
                "Invalid price format".to_string()
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceLevel {
    pub id: EntityId,
    pub name: String,
    pub price: Currency,
    pub level_type: PriceLevelType,
}

impl std::fmt::Display for PriceLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Default for PriceLevel {
    fn default() -> Self {
        Self {
            id: -1,
            name: String::new(),
            price: Decimal::ZERO,
            level_type: PriceLevelType::default(),
        }
    }
}

impl Entity for PriceLevel {
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

impl PriceLevel {
    pub fn new_draft() -> Self {
        Self::default()
    }

    fn validate(&self, other_groups: &[&PriceLevel]) -> Result<(), ValidationError> {
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
 
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Item group name cannot be empty".to_string()
            ));
        }
 
        Ok(())
    }
}

pub fn update(
    message: Message,
) -> Action<Operation, Message> {
    match message {
        Message::RequestDelete(id) => {
            Action::operation(Operation::RequestDelete(id))
        },
        Message::CopyPriceLevel(id) => {
            Action::operation(Operation::CopyPriceLevel(id))
        },
        Message::EditPriceLevel(id) => {
            Action::operation(Operation::EditPriceLevel(id))
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
    all_prices: &'a BTreeMap<EntityId, PriceLevel>,
    edit_states: &'a Vec<PriceLevelEditState>,
) -> Element<'a, Message> {
    let title_row = entity_component::render_title_row(
        "Price Levels", 
        Message::CreateNew,
        505.0 // view width
    );

    // Custom header row for columns including range fields
    let header_row = row![
        text("ID").width(Length::Fixed(75.0)),
        text("Name").width(Length::Fixed(250.0)),
        text("Actions").width(Length::Fixed(150.0)),
    ]
    .padding(15);

    // List of price levels
    let price_list = scrollable(
        column(
            all_prices
                .values()
                .map(|group| 
                    row![
                        render_price_level_row(group, edit_states)
                    ]
                    .padding(5)
                    .into()
                )
                .collect::<Vec<_>>()
        )
    ).height(Length::Fill);

    // Combine all elements
    let all_content = column![title_row, header_row, price_list];

    column![
        container(all_content)
            .height(Length::Shrink)
            .style(Modern::card_container())
    ]
    .into()
}

fn render_price_level_row<'a>(
    price_level: &'a PriceLevel,
    edit_states: &'a Vec<PriceLevelEditState>
) -> Element<'a, Message> {
    // Find edit state for this price_level if it exists
    let edit_state = edit_states.iter()
        .find(|state| state.base.id.parse::<i32>().unwrap_or(-999) == price_level.id);

    let editing = edit_state.is_some();

    // Get display values
    let display_name = edit_state
        .map(|state| state.base.name.clone())
        .unwrap_or_else(|| price_level.name.clone());

    // Check for validation errors
    let id_validation_error = edit_state
        .and_then(|state| state.base.id_validation_error.as_ref());

    let name_validation_error = edit_state
    .and_then(|state| state.base.name_validation_error.as_ref());

    // ID input with validation
    let id_input: Element<'_, Message> = {
        let input = text_input("ID (1-999)", &price_level.id.to_string())
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
        let input = text_input("Price Level Name", &display_name)
            .on_input_maybe(
                if editing {
                    Some(|name| Message::UpdateName(price_level.id, name))
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

    // Action buttons
    let action_row = row![
        button(if editing { icon::save().size(14) } else { icon::edit().size(14) })
            .on_press(
                if editing { 
                    Message::SaveAll(price_level.id, edit_state.unwrap().clone()) 
                } else { 
                    Message::EditPriceLevel(price_level.id) 
                }
            )
            .style(Modern::primary_button()),
        iced::widget::horizontal_space().width(2),
        button(icon::copy().size(14))
            .on_press(Message::CopyPriceLevel(price_level.id))
            .style(Modern::primary_button()),
        iced::widget::horizontal_space().width(2),
        button(if editing { icon::cancel().size(14) } else { icon::trash().size(14) })
            .on_press(
                if editing { 
                    Message::CancelEdit(price_level.id) 
                } else { 
                    Message::RequestDelete(price_level.id) 
                }
            )
            .style(Modern::danger_button()),
    ].width(150);

    // Combine all elements
    row![
        iced::widget::horizontal_space().width(3),
        id_input,
        name_input,
        iced::widget::horizontal_space().width(5),
        action_row,
    ]
    .align_y(iced::Alignment::Center)
    .width(Length::Fixed(495.0))
    .into()
}