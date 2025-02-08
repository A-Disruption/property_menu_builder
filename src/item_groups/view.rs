use iced::widget::{
    button, column, container, row, text,
    horizontal_space,
};
use iced::{Alignment, Element, Length};
use crate::HotKey;

#[derive(Debug, Clone)]
pub enum Message {
    Edit,
    Back,
}

pub fn view<'a>(item_group: &'a super::ItemGroup) -> Element<'a, Message> {
    let header = row![
        button("←").width(40).on_press(Message::Back),
        text(&item_group.name).size(16),
        horizontal_space(),
        button("Edit").on_press(Message::Edit)
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    let content = container(
        column![
            row![
                text("ID:").width(Length::Fixed(150.0)),
                text(item_group.id.to_string())
            ],
            row![
                text("Name:").width(Length::Fixed(150.0)), 
                text(&item_group.name)
            ],
            row![
                text("ID Range:").width(Length::Fixed(150.0)),
                text(format!("{} - {}", 
                    item_group.id_range.start, 
                    item_group.id_range.end
                ))
            ]
        ]
        .spacing(10)
    )
    .style(container::rounded_box)
    .padding(20);

    container(
        column![header, content]
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