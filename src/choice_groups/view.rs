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

pub fn view<'a>(choice_group: &'a super::ChoiceGroup) -> Element<'a, Message> {
    let header = row![
        button("←").width(40).on_press(Message::Back),
        text("Choice Group: ".to_string() + &choice_group.name).size(16),
        horizontal_space(),
        button("Edit").on_press(Message::Edit)
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    let content = container(
        column![
            row![
                row![
                    text("ID:").style(text::primary),
                    container(text(choice_group.id.to_string()).align_x(iced::alignment::Horizontal::Left)),
                ].width(Length::Fixed(100.0)).spacing(10),
                row![
                    text("Name:").style(text::primary), 
                    text(&choice_group.name).align_x(iced::alignment::Horizontal::Left)
                ].width(Length::Fixed(200.0)).spacing(10),
            ],
        ].height(Length::Shrink)
        .spacing(10)
    )
    .style(container::rounded_box)
    .padding(10);

    container(
        column![header, content]
            .spacing(20)
    )
    .padding(5)
    .into()
}

pub fn handle_hotkey(hotkey: HotKey) -> crate::Action<super::Operation, Message> {
    match hotkey {
        HotKey::Escape => crate::Action::operation(super::Operation::Back),
        _ => crate::Action::none(),
    }
}