pub mod edit;
pub mod view;

use crate::data_types::{
    self, EntityId, Validatable, ValidationError
};
use crate::Action;
use crate::icon;
use serde::{Serialize, Deserialize};
use iced::{Alignment, Element, Length};
use iced::widget::{button, column, container, row, text, scrollable, text_input};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Message {
    Edit(edit::Message),
    View(view::Message),
    CreateNew,
    RequestDelete(EntityId),
    CopyPrinterLogical(EntityId),
    EditPrinterLogical(EntityId),
    UpdateId(String),
    UpdateName(String),
    Select(EntityId),
    SaveMultiTest(EntityId, EditState),
    UpdateMultiName(EntityId, String),
    CreateNewMulti,
    CancelEdit(EntityId),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(PrinterLogical),
    StartEdit(EntityId),
    Cancel,
    Back,
    CreateNew(PrinterLogical),
    RequestDelete(EntityId),
    CopyPrinterLogical(EntityId),
    EditPrinterLogical(EntityId),
    Select(EntityId),
    SaveMultiTest(EntityId, EditState),
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
    pub validation_error: Option<String>,
}

impl EditState {
    pub fn new(printer: &PrinterLogical) -> Self {
        Self {
            name: printer.name.clone(),
            original_name: printer.name.clone(),
            id: printer.id.to_string(),
            validation_error: None,
        }
    }

    pub fn reset(& mut self) {
        self.name = self.original_name.clone();
        self.validation_error = None;
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Printer name cannot be empty".to_string()
            ));
        }

        if let Ok(id) = self.id.parse::<EntityId>() {
            if !(1..=999).contains(&id) {
                return Err(ValidationError::InvalidId(
                    "Printer ID must be between 1 and 999".to_string()
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrinterLogical {
    pub id: EntityId,
    pub name: String,
}

impl std::fmt::Display for PrinterLogical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Default for PrinterLogical {
    fn default() -> Self {
        Self {
            id: -1,
            name: String::new(),
        }
    }
}

impl PrinterLogical {

    pub fn new_draft() -> Self {
        Self::default()
    }

    fn validate(&self, other_printers: &[&PrinterLogical]) -> Result<(), ValidationError> {
        // Validate ID range (0-25 based on your screenshot)
        if !(0..=25).contains(&self.id) {
            return Err(ValidationError::InvalidId(
                "Printer Logical ID must be between 0 and 25".to_string()
            ));
        }

        // Check for duplicate IDs
        for other in other_printers {
            if other.id == self.id {
                return Err(ValidationError::DuplicateId(
                    format!("Printer Logical with ID {} already exists", self.id)
                ));
            }
        }

        // Validate name is not empty
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Printer Logical name cannot be empty".to_string()
            ));
        }

        // Validate name is not more than 16 Characters
        if self.name.len() < 17 {
            return Err(ValidationError::EmptyName(
                "Printer Logical name cannot be empty".to_string()
            ));
        }

        Ok(())
    }
}

pub fn update(
    printer: &mut PrinterLogical,
    message: Message,
    state: &mut EditState,
    other_printers: &[&PrinterLogical]
) -> Action<Operation, Message> {
    match message {
        Message::Edit(msg) => match msg {
            edit::Message::UpdateName(name) => {
                printer.name = name;
                Action::none()
            }
            edit::Message::UpdateId(id) => {
                if let Ok(id) = id.parse() {
                    printer.id = id;
                    Action::none()
                } else {
                    state.validation_error = Some("Invalid ID format".to_string());
                    Action::none()
                }
            }
            edit::Message::Save => {
                if printer.validate(other_printers).is_ok() {
                    Action::operation(Operation::Save(printer.clone()))
                } else {
                    state.validation_error = Some("Validation failed".to_string());
                    Action::none()
                }
            }
            edit::Message::Cancel => Action::operation(Operation::Cancel),
        },
        Message::View(msg) => match msg {
            view::Message::Edit => Action::operation(Operation::StartEdit(printer.id)),
            view::Message::Back => Action::operation(Operation::Back),
        }
        Message::CreateNew => {
            let new_printer_logical = PrinterLogical::default();
            Action::operation(Operation::CreateNew(new_printer_logical))
        },
        Message::RequestDelete(id) => {
            Action::operation(Operation::RequestDelete(id))
        },
        Message::CopyPrinterLogical(id) => {
            Action::operation(Operation::CopyPrinterLogical(id))
        },
        Message::EditPrinterLogical(id) => {
            println!("Editing ID: {}", id);
            Action::operation(Operation::EditPrinterLogical(id))
        }
        Message::Select(id) => {
            Action::operation(Operation::Select(id))
            //Action::none()
        },
        Message::UpdateId(id) => {
            if let Ok(id) = id.parse() {
                printer.id = id;
                Action::none()
            } else {
                state.validation_error = Some("Invalid ID format".to_string());
                Action::none()
            }
        }
        Message::UpdateName(name) => {
            printer.name = name;
            Action::none()
        }
        Message::CreateNewMulti => {
            Action::operation(Operation::CreateNewMulti)
        }
        Message::SaveMultiTest(id, edit_state) => {
            Action::operation(Operation::SaveMultiTest(id, edit_state))
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
    printer: &'a PrinterLogical, 
    mode: &'a Mode,
    all_printers: &'a BTreeMap<EntityId, PrinterLogical>,
    quick_view: &'a bool,
    edit_states: &'a Vec<EditState>,
) -> Element<'a, Message> {

    if !quick_view {
        let printer_list = column(
            all_printers
                .values()
                .map(|printer_logical| {
                    button(
                        list_item(
                            &printer_logical.name.as_str(), 
                            button(icon::copy())
                                .on_press(Message::CopyPrinterLogical(printer_logical.id))
                                .style(
                                    if printer_logical.id == printer.id {
                                        button::secondary
                                    } else {
                                        button::primary
                                    }
                                ), 
                            button(icon::trash()).on_press(Message::RequestDelete(printer_logical.id)),
                        )
                    )
                    .width(iced::Length::Fill)
                    .on_press(Message::Select(printer_logical.id))
                    .style(if printer_logical.id == printer.id {
                        button::primary
                    } else {
                        button::secondary
                    })
                    .into()
                })
                .collect::<Vec<_>>()
        )
        .spacing(5)
        .width(iced::Length::Fixed(250.0));

        let content = match mode {
            Mode::View => view::view(printer).map(Message::View),
            Mode::Edit => {
                edit::view(
                    printer,
                    EditState::new(printer),
                    all_printers
                ).map(Message::Edit)
            }
        };

        row![
            container(
                column![
                    row![
                        text("Printer Logicals").size(18),
                        iced::widget::horizontal_space(),
                        button(icon::new().shaping(text::Shaping::Advanced))
                            .on_press(Message::CreateNew)
                            .style(button::primary),
                    ].width(250),
                    printer_list,
                ]
                .spacing(10)
                .padding(10)
            )
            .style(container::rounded_box),
            container(content)
                .width(iced::Length::Fill)
                .style(container::rounded_box)
        ]
        .spacing(20)
        .into()
        } else { // Display quickview

            let title_row = container(
                row![
                    text("Printer Logicals").size(18).style(text::primary),
                    iced::widget::horizontal_space(),
                    button(icon::new())
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
        
            // List of printer logicals in a scrollable container
            let printer_list = scrollable(
                column(
                    all_printers.values()
                        .map(|a_printer| 
                            container(
                                logical_quick_edit_view(
                                    //printer.clone(), 
                                    a_printer,
                                    edit_states
                                )
                            )
                            .style(container::bordered_box)
                            .padding(5)
                            .into()
                        )
                        .collect::<Vec<_>>()
                )
            )
            .height(Length::Fill);

            column![
                title_row,
                header_row,
                container(printer_list)
                    .height(Length::Fill)
                    .style(container::rounded_box)
            ]
            //.spacing(10)
            //.padding(10)
            .into()

        }
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
    //selected_printer: PrinterLogical,
    printer: &'a PrinterLogical,
    edit_states: &'a Vec<EditState>
    ) 
    -> Element<'a, Message> {

        // Find edit state for this printer if it exists
        let edit_state = edit_states.iter()
            .find(|state| state.id.parse::<i32>().unwrap() == printer.id);

        let editing = edit_state.is_some();

        let display_name = edit_state
            .map(|state| state.name.clone())
            .unwrap_or_else(|| printer.name.clone());

        // Check for validation error
        let validation_error = edit_state
        .and_then(|state| state.validation_error.as_ref())
        .cloned();

        let button_content: iced::widget::Button<'a, Message> = button(
            container(
                row![
                    text_input("ID (1-25)", &printer.id.to_string())
                        //.on_input(Message::UpdateId)
                        .width(Length::Fixed(75.0)),
                    text_input("Printer Name", &display_name)
                        .on_input_maybe(
                            if editing {
                               Some( |a_printer| Message::UpdateMultiName(printer.id, a_printer) )
                             } else {
                                None 
                             }
                        ).style(if validation_error.is_some() { data_types::validated_error } else { text_input::default })
                        .width(Length::Fixed(250.0)),

                    row![

                        button( if editing { icon::save() } else { icon::edit() })
                        .on_press( if editing { Message::SaveMultiTest(printer.id, edit_state.unwrap().clone()) } else { Message::EditPrinterLogical(printer.id) })
                        .style(
                            button::primary // delete if going back to switching style based on selected printer
/*                             if selected_printer.id == printer.id {
                                button::secondary
                            } else {
                                button::primary
                            } */
                    ),
                        iced::widget::horizontal_space().width(2),
                    button(icon::copy())
                        .on_press(Message::CopyPrinterLogical(printer.id))
                        .style(
                            button::primary // delete if going back to switching style based on selected printer
/*                             if selected_printer.id == printer.id {
                                button::secondary
                            } else {
                                button::primary
                            } */
                    ),
                    iced::widget::horizontal_space().width(2),
                    button(if editing { icon::cancel() } else { icon::trash() })
                        .on_press( if editing { Message::CancelEdit(printer.id) } else { Message::RequestDelete(printer.id) })
                        .style(button::danger),
                    ].width(150),
                ].align_y(iced::Alignment::Center),

            )
        )
        .width(iced::Length::Shrink)
        .on_press(Message::Select(printer.id))
        .style(
            button::secondary // delete if going back to switching style based on selected printer
/*             if printer.id == selected_printer.id {
                button::primary
            } else {
             button::secondary
            } */
        ).into();


        
        button_content.into()
}
