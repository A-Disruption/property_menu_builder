use iced::widget::{
    button, column, container, row, text,
    horizontal_space,
};
use iced::{Alignment, Element, Length};

use crate::HotKey;
use crate::icon;
use super::PrinterLogical;


#[derive(Debug, Clone)]
pub enum Message {
    Edit,
    Back,
}

pub fn view<'a>(printer: &'a PrinterLogical) -> Element<'a, Message> {
    let header = row![
        horizontal_space().width(40),
        text(&printer.name).size(16),
        horizontal_space(),
        button(icon::edit().shaping(text::Shaping::Advanced)).on_press(Message::Edit)
    ]
    .spacing(10)
    .align_y(Alignment::Center);

   let content = container(
       column![
           row![
               text("ID:").width(Length::Fixed(150.0)),
               text(printer.id.to_string())
           ],
           row![
               text("Name:").width(Length::Fixed(150.0)), 
               text(&printer.name)
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