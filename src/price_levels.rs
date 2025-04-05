use crate::data_types::{
    self,
    EntityId,
    Currency,
    Validatable,
    ValidationError
};
use crate::Action;
use crate::icon;
use serde::{Serialize, Deserialize};
use iced::{Element, Length};
use iced::widget::{button, text, container, row, column, text_input, scrollable};
use std::collections::BTreeMap;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub enum Message {
    CreateNew,
    RequestDelete(EntityId),
    CopyPriceLevel(EntityId),
    EditPriceLevel(EntityId),
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
    Save(PriceLevel),
    StartEdit(EntityId),
    Cancel,
    Back,
    CreateNew(PriceLevel),
    RequestDelete(EntityId),
    CopyPriceLevel(EntityId),
    EditPriceLevel(EntityId),
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
    pub price: String,
    pub original_price: String,
    pub level_type: PriceLevelType,
    pub validation_error: Option<String>,
}

impl EditState {
    pub fn new(price_level: &PriceLevel) -> Self {
        Self {
            name: price_level.name.clone(),
            original_name: price_level.name.clone(),
            id: price_level.id.to_string(),
            price: price_level.price.to_string(),
            original_price: price_level.price.to_string(),
            level_type: price_level.level_type.clone(),
            validation_error: None,
        }
    }

    pub fn reset(&mut self){
        self.name = self.original_name.clone();
        self.price = self.original_price.clone();
        self.validation_error = None;
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Price level name cannot be empty".to_string()
            ));
        }

        if let Ok(id) = self.id.parse::<EntityId>() {
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

impl PriceLevel {

    pub fn new_draft() -> Self {
        Self::default()
    }

    fn validate(&self, other_levels: &[&PriceLevel]) -> Result<(), ValidationError> {
        if !(1..=999).contains(&self.id) {
            return Err(ValidationError::InvalidId(
                "Price level ID must be between 1 and 999".to_string()
            ));
        }

        for other in other_levels {
            if other.id == self.id {
                return Err(ValidationError::DuplicateId(
                    format!("Price level with ID {} already exists", self.id)
                ));
            }
        }

        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Price level name cannot be empty".to_string()
            ));
        }

        if self.price < Decimal::ZERO {
            return Err(ValidationError::InvalidValue(
                "Price cannot be negative".to_string()
            ));
        }

        Ok(())
    }
}

pub fn update(
    price_level: &mut PriceLevel,
    message: Message,
    state: &mut EditState,
    other_levels: &[&PriceLevel]
) -> Action<Operation, Message> {
    match message {
        Message::CreateNew => {
            let new_price_level = PriceLevel::default();
            Action::operation(Operation::CreateNew(new_price_level))
        },
        Message::RequestDelete(id) => {
            Action::operation(Operation::RequestDelete(id))
        },
        Message::CopyPriceLevel(id) => {
            Action::operation(Operation::CopyPriceLevel(id))
        },
        Message::EditPriceLevel(id) => {
            Action::operation(Operation::EditPriceLevel(id))
        },
        Message::Select(id) => {
            Action::operation(Operation::Select(id))
        },
        Message::UpdateId(id) => {
            if let Ok(id) = id.parse() {
                price_level.id = id;
                Action::none()
            } else {
                state.validation_error = Some("Invalid ID format".to_string());
                Action::none()
            }
        },
        Message::UpdateName(name) => {
            price_level.name = name;
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
    all_levels: &'a BTreeMap<EntityId, PriceLevel>,
    edit_states: &'a Vec<EditState>,
) -> Element<'a, Message> {

    let title_row = container(
        row![
            text("Price Levels").size(18).style(text::primary),
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
    price_level: &'a PriceLevel,
    edit_states: &'a Vec<EditState>
    ) 
    -> Element<'a, Message> {

        // Find edit state for this price_level if it exists
        let edit_state = edit_states.iter()
            .find(|state| state.id.parse::<i32>().unwrap() == price_level.id);

        let editing = edit_state.is_some();

        let display_name = edit_state
            .map(|state| state.name.clone())
            .unwrap_or_else(|| price_level.name.clone());

        // Check for validation error
        let validation_error = edit_state
        .and_then(|state| state.validation_error.as_ref())
        .cloned();

        let button_content: iced::widget::Button<'a, Message> = button(
            container(
                row![
                    text_input("ID (1-25)", &price_level.id.to_string())
                        //.on_input(Message::UpdateId)
                        .width(Length::Fixed(75.0)),
                    text_input("Choice Group Name", &display_name)
                        .on_input_maybe(
                            if editing {
                               Some( |a_price_level| Message::UpdateMultiName(price_level.id, a_price_level) )
                             } else {
                                None 
                             }
                        ).style(if validation_error.is_some() { data_types::validated_error } else { text_input::default })
                        .width(Length::Fixed(250.0)),

                    row![

                        button( if editing { icon::save().size(14) } else { icon::edit().size(14) })
                        .on_press( if editing { Message::SaveAll(price_level.id, edit_state.unwrap().clone()) } else { Message::EditPriceLevel(price_level.id) })
                        .style(
                            button::primary
                    ),
                        iced::widget::horizontal_space().width(2),
                    button(icon::copy().size(14))
                        .on_press(Message::CopyPriceLevel(price_level.id))
                        .style(
                            button::primary
                    ),
                    iced::widget::horizontal_space().width(2),
                    button(if editing { icon::cancel().size(14) } else { icon::trash().size(14) })
                        .on_press( if editing { Message::CancelEdit(price_level.id) } else { Message::RequestDelete(price_level.id) })
                        .style(button::danger),
                    ].width(150),
                ].align_y(iced::Alignment::Center),

            )
        )
        .width(iced::Length::Shrink)
        .on_press(Message::Select(price_level.id))
        .style(
            button::secondary
        ).into();


        
        button_content.into()
}