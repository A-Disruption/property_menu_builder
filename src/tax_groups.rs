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
use rust_decimal::Decimal;
use std::fmt;
use std::str::FromStr;


#[derive(Debug, Clone)]
pub enum Message {
    CreateNew,
    RequestDelete(EntityId),
    CopyTaxGroup(EntityId),
    EditTaxGroup(EntityId),
    UpdateId(String),
    UpdateName(String),
    Select(EntityId),
    SaveAll(EntityId, EditState),
    UpdateMultiName(EntityId, String),
    UpdateTaxRate(EntityId, String),
    CreateNewMulti,
    CancelEdit(EntityId),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(TaxGroup),
    StartEdit(EntityId),
    Cancel,
    Back,
    CreateNew(TaxGroup),
    RequestDelete(EntityId),
    CopyTaxGroup(EntityId),
    EditTaxGroup(EntityId),
    Select(EntityId),
    SaveAll(EntityId, EditState),
    UpdateMultiName(EntityId, String),
    UpdateTaxRate(EntityId, String),
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
    pub rate: String,
    pub original_rate: String,
    pub validation_error: Option<String>,
}

impl EditState {
    pub fn new(tax_group: &TaxGroup) -> Self {
        Self {
            name: tax_group.name.clone(),
            original_name: tax_group.name.clone(),
            id: tax_group.id.to_string(),
            rate: tax_group.rate_percentage().to_string(),
            original_rate: tax_group.rate_percentage().to_string(),
            validation_error: None,
        }
    }

    pub fn reset(& mut self) {
        self.name = self.original_name.clone();
        self.rate = self.original_rate.clone();
        self.validation_error = None;
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
        Message::CreateNew => {
            let new_tax_group = TaxGroup::default();
            Action::operation(Operation::CreateNew(new_tax_group))
        },
        Message::RequestDelete(id) => {
            Action::operation(Operation::RequestDelete(id))
        },
        Message::CopyTaxGroup(id) => {
            Action::operation(Operation::CopyTaxGroup(id))
        },
        Message::EditTaxGroup(id) => {
            Action::operation(Operation::EditTaxGroup(id))
        },
        Message::Select(id) => {
            Action::operation(Operation::Select(id))
        },
        Message::UpdateId(id) => {
            if let Ok(id) = id.parse() {
                tax_group.id = id;
                Action::none()
            } else {
                state.validation_error = Some("Invalid ID format".to_string());
                Action::none()
            }
        },
        Message::UpdateName(name) => {
            tax_group.name = name;
            Action::none()
        },
        Message::UpdateTaxRate(id, rate) => {
            Action::operation(Operation::UpdateTaxRate(id, rate))
        },
        Message::CreateNewMulti => {
            Action::operation(Operation::CreateNewMulti)
        },
        Message::SaveAll(id, edit_state) => {
            Action::operation(Operation::SaveAll(id, edit_state))
        },
        Message::UpdateMultiName(id, new_name) => {
            Action::operation(Operation::UpdateMultiName(id, new_name))
        },
        Message::CancelEdit(id) => {
            Action::operation(Operation::CancelEdit(id))
        },
    }
}

pub fn view<'a>(
    all_groups: &'a BTreeMap<EntityId, TaxGroup>,
    edit_states: &'a Vec<EditState>,
) -> Element<'a, Message> {

    let title_row = container(
        row![
            text("Tax Groups").size(18).style(text::primary),
            iced::widget::horizontal_space(),
            button(icon::new().size(14))
                .on_press(Message::CreateNewMulti)
                .style(button::primary),
        ]
        .width(Length::Fixed(605.0))
        .padding(15)
    )
    .style(container::rounded_box);

    // Header row for columns
    let header_row = container(
        row![
            text("ID").width(Length::Fixed(75.0)),
            text("Name").width(Length::Fixed(250.0)),
            text("Tax Rate").width(Length::Fixed(100.0)),
            text("Actions").width(Length::Fixed(150.0)),
        ]
        .padding(15)
    )
    .style(container::rounded_box);

    let groups_list = scrollable(
        column(
            all_groups
                .values()
                .map(|group| 
                    container(
                        logical_quick_edit_view(
                            group,
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
        container(groups_list)
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

pub fn string_to_decimal(input: &str) -> Result<Decimal, String> {
    Decimal::from_str(input)
        .map_err(|e| format!("Failed to convert '{}' to Decimal: {}", input, e))
}

fn logical_quick_edit_view<'a>(
    tax_group: &'a TaxGroup,
    edit_states: &'a Vec<EditState>
    ) 
    -> Element<'a, Message> {

        // Find edit state for this tax_group if it exists
        let edit_state = edit_states.iter()
            .find(|state| state.id.parse::<i32>().unwrap() == tax_group.id);

        let editing = edit_state.is_some();

        let display_name = edit_state
            .map(|state| state.name.clone())
            .unwrap_or_else(|| tax_group.name.clone());

        let tax_rate = edit_state
            .map(|state| state.rate.clone())
            .unwrap_or_else(|| tax_group.rate.to_string());


        // Check for validation error
        let validation_error = edit_state
        .and_then(|state| state.validation_error.as_ref())
        .cloned();

        let button_content: iced::widget::Button<'a, Message> = button(
            container(
                row![
                    text_input("ID (1-25)", &tax_group.id.to_string())
                        //.on_input(Message::UpdateId)
                        .width(Length::Fixed(75.0)),
                    text_input("Tax Group Name", &display_name)
                        .on_input_maybe(
                            if editing {
                               Some( |a_tax_group| Message::UpdateMultiName(tax_group.id, a_tax_group) )
                             } else {
                                None 
                             }
                        ).style(if validation_error.is_some() { data_types::validated_error } else { text_input::default })
                        .width(Length::Fixed(250.0)),

                    text_input("Tax Rate", &tax_rate)
                        .on_input_maybe(
                            if editing {
                            Some( |a_tax_rate| Message::UpdateTaxRate(tax_group.id, a_tax_rate) )
                            } else {
                                None 
                            }
                        ).style(if validation_error.is_some() { data_types::validated_error } else { text_input::default })
                        .width(Length::Fixed(100.0)),

                    row![

                        button( if editing { icon::save().size(14) } else { icon::edit().size(14) })
                        .on_press( if editing { Message::SaveAll(tax_group.id, edit_state.unwrap().clone()) } else { Message::EditTaxGroup(tax_group.id) })
                        .style(
                            button::primary
                    ),
                        iced::widget::horizontal_space().width(2),
                    button(icon::copy().size(14))
                        .on_press(Message::CopyTaxGroup(tax_group.id))
                        .style(
                            button::primary
                    ),
                    iced::widget::horizontal_space().width(2),
                    button(if editing { icon::cancel().size(14) } else { icon::trash().size(14) })
                        .on_press( if editing { Message::CancelEdit(tax_group.id) } else { Message::RequestDelete(tax_group.id) })
                        .style(button::danger),
                    ].width(150),
                ].align_y(iced::Alignment::Center),

            )
        )
        .width(iced::Length::Shrink)
        .on_press(Message::Select(tax_group.id))
        .style(
            button::secondary
        ).into();


        
        button_content.into()
}