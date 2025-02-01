use iced::widget::{
    button, column, container, row, text,
    horizontal_space,
};
use iced::{Alignment, Element, Length};

use crate::HotKey;
use crate::item_groups::ItemGroup;
use crate::revenue_categories::RevenueCategory;
use crate::data_types::{EntityId, ValidationError};
use super::ProductClass;

#[derive(Debug, Clone)]
pub enum Message {
    Edit,
    Back,
}

pub fn view(
    class: &ProductClass,
    available_item_groups: &[&ItemGroup],
    available_revenue_categories: &[&RevenueCategory],
) -> Element<Message> {
    let header = row![
        button("←").width(40).on_press(Message::Back),
        text(&class.name).size(16),
        horizontal_space(),
        button("Edit").on_press(Message::Edit)
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    let content = container(
        column![
            row![
                text("ID:").width(Length::Fixed(150.0)),
                text(class.id.to_string())
            ],
            row![
                text("Item Group:").width(Length::Fixed(150.0)),
                text(
                    class.item_group
                        .and_then(|id| available_item_groups.iter().find(|g| g.id == id))
                        .map_or("None".to_string(), |g| g.name.clone())
                )
            ],
            row![
                text("Revenue Category:").width(Length::Fixed(150.0)),
                text(
                    class.revenue_category
                        .and_then(|id| available_revenue_categories.iter().find(|c| c.id == id))
                        .map_or("None".to_string(), |c| c.name.clone())
                )
            ],
        ]
        .spacing(10)
    )
    .style(container::rounded_box)
    .padding(20);

    container(
        column![
            header,
            content,
        ]
        .spacing(20)
    )
    .padding(20)
    .into()
}

pub fn handle_hotkey(hotkey: HotKey) -> crate::Action<super::Operation, Message> {
    match hotkey {
        HotKey::Escape => crate::Action::operation(super::Operation::Back),
        _ => crate::Action::none(),
    }
}