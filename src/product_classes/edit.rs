use iced::widget::{
    button, column, container, row, text, text_input, pick_list,
    horizontal_space,
};
use iced::{Alignment, Element, Length, Color};

use crate::HotKey;
use crate::item_groups::ItemGroup;
use crate::revenue_categories::RevenueCategory;
use super::ProductClass;
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
    name: String,
    id: String,
    item_group_id: Option<EntityId>,
    revenue_category_id: Option<EntityId>,
    validation_error: Option<String>,
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

pub fn view(
    state: &EditState,
    available_item_groups: &[&ItemGroup],
    available_revenue_categories: &[&RevenueCategory],
) -> Element<Message> {
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