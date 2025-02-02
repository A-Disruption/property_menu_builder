use iced::widget::{
    button, column, container, row, text, text_input, pick_list,
    horizontal_space,
};
use iced::{Alignment, Element, Length, Color};

use crate::HotKey;
use crate::item_groups::ItemGroup;
use crate::revenue_categories::RevenueCategory;
use super::{ProductClass, UpdateContext};
use crate::data_types::{EntityId, ValidationError};
#[derive(Debug, Clone)]
pub enum Message {
    UpdateName(String),
    UpdateId(String),
    SelectItemGroup(Option<EntityId>),
    SelectRevenueCategory(Option<EntityId>),
    Save,
    Cancel,
}

pub struct EditState {
    pub name: String,
    pub id: String,
    pub item_group_id: Option<EntityId>,
    pub revenue_category_id: Option<EntityId>,
    pub validation_error: Option<String>,
}

impl EditState {
    pub fn new(class: &ProductClass) -> Self {
        Self {
            name: class.name.clone(),
            id: class.id.to_string(),
            item_group_id: class.item_group,
            revenue_category_id: class.revenue_category,
            validation_error: None,
        }
    }
}

impl EditState {
    pub fn validate(&self, context: &UpdateContext) -> Result<(), ValidationError> {
        // Validate name is not empty
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Product class name cannot be empty".to_string()
            ));
        }

        // Validate ID range (1-999 based on screenshot)
        let id: EntityId = self.id.parse().map_err(|_| {
            ValidationError::InvalidId("Invalid ID format".to_string())
        })?;

        if !(1..=999).contains(&id) {
            return Err(ValidationError::InvalidId(
                "Product Class ID must be between 1 and 999".to_string()
            ));
        }

        // Check for duplicate IDs
        for other in context.other_classes {
            if id == other.id {
                return Err(ValidationError::DuplicateId(
                    format!("Product Class with ID {} already exists", id)
                ));
            }
        }

        // Validate item group reference
        if let Some(group_id) = self.item_group_id {
            if !context.available_item_groups.iter().any(|g| g.id == group_id) {
                return Err(ValidationError::InvalidReference(
                    format!("Referenced Item Group {} does not exist", group_id)
                ));
            }
        } else {
            return Err(ValidationError::InvalidReference(
                "Item Group is required".to_string()
            ));
        }

        // Validate revenue category reference
        if let Some(category_id) = self.revenue_category_id {
            if !context.available_revenue_categories.iter().any(|c| c.id == category_id) {
                return Err(ValidationError::InvalidReference(
                    format!("Referenced Revenue Category {} does not exist", category_id)
                ));
            }
        } else {
            return Err(ValidationError::InvalidReference(
                "Revenue Category is required".to_string()
            ));
        }

        Ok(())
    }
}

pub fn view<'a>(
    state: &'a EditState,
    available_item_groups: &'a [&'a ItemGroup],
    available_revenue_categories: &'a [&'a RevenueCategory],
) -> Element<'a, Message> {
    let content = container(
        column![
            row![
                text("Name").width(Length::Fixed(150.0)),
                text_input("Product Class Name", &state.name)
                    .on_input(Message::UpdateName)
                    .padding(5)
            ],
            row![
                text("ID").width(Length::Fixed(150.0)),
                text_input("ID (1-999)", &state.id)
                    .on_input(Message::UpdateId)
                    .padding(5)
            ],
            row![
                text("Item Group").width(Length::Fixed(150.0)),
                pick_list(
                    &available_item_groups[..],
                    state.item_group_id.and_then(|id| available_item_groups.iter().find(|g| g.id == id).copied()),
                    |group| Message::SelectItemGroup(Some(group.id))
                )
            ],
            row![
                text("Revenue Category").width(Length::Fixed(150.0)),
                pick_list(
                    &available_revenue_categories[..],
                    state.revenue_category_id.and_then(|id| available_revenue_categories.iter().find(|c| c.id == id).copied()),
                    |category| Message::SelectRevenueCategory(Some(category.id))
                )
            ],
            if let Some(error) = &state.validation_error {
                container(
                    text(error)
                        .style(iced::widget::text::danger)
                )
                .padding(10)
            } else {
                container(
                    text("")
                        .style(iced::widget::text::danger)
                )
                .padding(10)
            }
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

    container(
        column![
            content,
            controls,
        ]
        .spacing(20)
    )
    .padding(20)
    .into()
}

pub fn handle_hotkey(hotkey: HotKey) -> crate::Action<super::Operation, Message> {
    match hotkey {
        HotKey::Escape => crate::Action::operation(super::Operation::Cancel),
        _ => crate::Action::none(),
    }
}