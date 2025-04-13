use crate::data_types::{EntityId, ValidationError};
use crate::icon;
use iced_modern_theme::Modern;
use iced::{Element, Length};
use iced::widget::{button, column, container, row, text, scrollable, text_input, tooltip, TextInput};
use std::collections::BTreeMap;

/// Trait that defines common behavior for entity types
pub trait Entity: Clone + std::fmt::Display {
    fn id(&self) -> EntityId;
    fn name(&self) -> &str;
    fn with_id(&self, id: EntityId) -> Self;
    fn with_name(&self, name: String) -> Self;
    fn default_new() -> Self;
}

/// Generic edit state for editing entities
#[derive(Default, Debug, Clone)]
pub struct EditState {
    pub name: String,
    pub original_name: String,
    pub id: String,
    pub id_validation_error: Option<String>,
    pub name_validation_error: Option<String>,
}

impl EditState {
    pub fn new<T: Entity>(entity: &T) -> Self {
        Self {
            name: entity.name().to_string(),
            original_name: entity.name().to_string(),
            id: entity.id().to_string(),
            id_validation_error: None,
            name_validation_error: None,
        }
    }

    pub fn reset(&mut self) {
        self.name = self.original_name.clone();
        self.id_validation_error = None;
        self.name_validation_error = None;
    }

    pub fn validate(&self, id_range: std::ops::RangeInclusive<i32>) -> Result<(), ValidationError> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Name cannot be empty".to_string()
            ));
        }

        if self.name.len() > 16 {
            return Err(ValidationError::NameTooLong(
                "Name cannot be more than 16 Characters".to_string()
            ));
        }

        if let Ok(id) = self.id.parse::<EntityId>() {
            if !id_range.contains(&id) {
                return Err(ValidationError::InvalidId(
                    format!("ID must be between {} and {}", id_range.start(), id_range.end())
                ));
            }
        } else {
            return Err(ValidationError::InvalidId(
                "Invalid ID format".to_string()
            ));
        }

        Ok(())
    }
}

/// Generic function to render title row
pub fn render_title_row<'a, Message>(
    title: &'a str, 
    create_message: Message,
    width: f32
) -> Element<'a, Message> 
where
    Message: Clone + 'a,
{
    row![
        text(title).size(18).style(Modern::primary_text()),
        iced::widget::horizontal_space(),
        button(icon::new().size(14))
            .on_press(create_message)
            .style(Modern::primary_button()),
    ]
    .width(Length::Fixed(width))
    .padding(15)
    .into()
}

/// Generic function to render header row
pub fn render_header_row<'a, Message>() -> Element<'a, Message> 
where
    Message: 'a,
{
    row![
        text("ID").width(Length::Fixed(75.0)),
        text("Name").width(Length::Fixed(250.0)),
        text("Actions").width(Length::Fixed(150.0)),
    ]
    .padding(15)
    .into()
}

/// Generic function to render entity list
pub fn render_entity_list<'a, T, Message, F>(
    entities: &'a BTreeMap<EntityId, T>,
    edit_states: &'a Vec<EditState>,
    row_renderer: F
) -> Element<'a, Message> 
where
    T: Entity + 'a,
    Message: 'a,
    F: Fn(&'a T, &'a Vec<EditState>) -> Element<'a, Message> + 'a,
{
    scrollable(
        column(
            entities.values()
                .map(|entity| 
                    row![
                        row_renderer(entity, edit_states)
                    ]
                    .padding(5)
                    .into()
                )
                .collect::<Vec<_>>()
        )
    )
    .height(Length::Fill)
    .into()
}

/// Generic function for entity view layout
pub fn entity_view<'a, T, Message, F>(
    title: &'a str,
    create_message: Message,
    entities: &'a BTreeMap<EntityId, T>,
    edit_states: &'a Vec<EditState>,
    row_renderer: F,
) -> Element<'a, Message> 
where
    T: Entity + 'a,
    Message: Clone + 'a,
    F: Fn(&'a T, &'a Vec<EditState>) -> Element<'a, Message> + 'a,
{
    let title_row = render_title_row(title, create_message, 505.0);
    let header_row = render_header_row();
    let entity_list = render_entity_list(entities, edit_states, row_renderer);

    let all_content = column![title_row, header_row, entity_list];

    column![
        container(all_content)
            .height(Length::Shrink)
            .style(Modern::card_container())
    ]
    .into()
}

/// Generic function for quick edit view of an entity
pub fn entity_quick_edit_view<'a, T, Message>(
    entity: &'a T,
    edit_states: &'a Vec<EditState>,
    edit_message: impl Fn(EntityId) -> Message + 'a,
    save_message: impl Fn(EntityId, EditState) -> Message + 'a,
    copy_message: impl Fn(EntityId) -> Message + 'a,
    delete_message: impl Fn(EntityId) -> Message + 'a,
    cancel_message: impl Fn(EntityId) -> Message + 'a,
    update_name_message: impl Fn(EntityId, String) -> Message + 'a,
    input_placeholder: &'a str
) -> Element<'a, Message> 
where
    T: Entity + 'a,
    Message: Clone + 'a,
{
    // Find edit state for this entity if it exists
    let edit_state = edit_states.iter()
        .find(|state| state.id.parse::<i32>().unwrap_or(-999) == entity.id());

    let editing = edit_state.is_some();

    let display_name = edit_state
        .map(|state| state.name.clone())
        .unwrap_or_else(|| entity.name().to_string());

    // Check for validation errors
    let id_validation_error = edit_state
        .and_then(|state| state.id_validation_error.as_ref());

    let name_validation_error = edit_state
        .and_then(|state| state.name_validation_error.as_ref());

    let id_input: Element<'_, Message> = {
        let input: TextInput<'_, Message> = text_input("ID", &entity.id().to_string())
            .style(Modern::validated_text_input(id_validation_error.is_some()))
            .width(Length::Fixed(75.0));

        if let Some(error) = id_validation_error.as_ref() {
            tooltip(
                input,
                container(error.as_str()).padding(10).style(Modern::danger_tooltip_container()),
                tooltip::Position::Top,
            ).into()
        } else {
            input.into()
        }
    };

    let name_input: Element<'_, Message> = {
        let input: TextInput<'_, Message> = text_input(input_placeholder, &display_name)
            .on_input_maybe(
                if editing {
                    Some(move |name| update_name_message(entity.id(), name))
                } else {
                    None 
                }
            )
            .style(Modern::validated_text_input(name_validation_error.is_some()))
            .width(Length::Fixed(250.0));

        if let Some(error) = name_validation_error.as_ref() {
            tooltip(
                input,
                container(error.as_str()).padding(10).style(Modern::danger_tooltip_container()),
                tooltip::Position::Top,
            ).into()
        } else {
            input.into()
        }
    };

    let action_row = row![
            button(if editing { icon::save().size(14) } else { icon::edit().size(14) })
                .on_press(
                    if editing { 
                        save_message(entity.id(), edit_state.unwrap().clone()) 
                    } else { 
                        edit_message(entity.id()) 
                    }
                )
                .style(Modern::primary_button()),
            iced::widget::horizontal_space().width(2),
            button(icon::copy().size(14))
                .on_press(copy_message(entity.id()))
                .style(Modern::primary_button()),
            iced::widget::horizontal_space().width(2),
            button(if editing { icon::cancel().size(14) } else { icon::trash().size(14) })
                .on_press(
                    if editing { 
                        cancel_message(entity.id()) 
                    } else { 
                        delete_message(entity.id()) 
                    }
                )
                .style(Modern::danger_button()),
        ].width(150);


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