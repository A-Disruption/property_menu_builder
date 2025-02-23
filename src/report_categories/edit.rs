use iced::widget::{
    button, column, container, row, text, text_input,
    horizontal_space,
};
use iced::{Element, Length};
use std::collections::BTreeMap;
use crate::icon;
use crate::HotKey;
use super::ReportCategory;
use crate::data_types::EntityId;

#[derive(Debug, Clone)]
pub enum Message {
    UpdateName(String),
    UpdateId(String),
    Save,
    Cancel,
}

pub fn view<'a>(
    report_category: &'a ReportCategory,
    state: super::EditState,
    all_categories: &'a BTreeMap<EntityId, ReportCategory>
) -> Element<'a, Message> {

    let header = row![
        horizontal_space().width(10),
        text(&report_category.name).size(18).style(text::primary),
        horizontal_space(),
        button(icon::save().size(14)).on_press(Message::Save).style(button::primary),
        button(icon::cancel().size(14)).on_press(Message::Cancel).style(button::danger),
        horizontal_space().width(4),
    ]
    .spacing(10)
    .padding(20)
    .align_y(iced::Alignment::Center);

    let validation_error = &state.validation_error;

    let other_categories: Vec<&ReportCategory> = all_categories.values()
    .filter(|c| c.id != report_category.id)
    .collect();

    let content = container(
        column![
            row![
                text("Name").width(Length::Fixed(150.0)),
                text_input("Report Category Name", &report_category.name)
                    .on_input(Message::UpdateName)
                    .padding(5)
            ],
            row![
                text("ID").width(Length::Fixed(150.0)),
                text_input("ID (1-999)", &report_category.id.to_string())
                    .on_input(Message::UpdateId)
                    .padding(5)
            ],
            // Show validation error if any
            if let Some(error) = validation_error {
                text(error.to_string()).style(text::danger)
            } else {
                text("".to_string())
            },
        ]
        .spacing(10)
    )
    .padding(20);

    container(column![header, content].padding(10)).into()
}

pub fn handle_hotkey(hotkey: HotKey) -> crate::Action<super::Operation, Message> {
    match hotkey {
        HotKey::Escape => crate::Action::operation(super::Operation::Cancel),
        _ => crate::Action::none(),
    }
}