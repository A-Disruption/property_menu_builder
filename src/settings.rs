use iced::widget::{button, checkbox, column, container, row, text, text_input};
use iced::{Element, Length};
use crate::HotKey;
use serde::{Serialize, Deserialize};
use crate::persistence;

#[derive(Debug, Clone)]
pub enum Message {
    UpdateFilePath(String),
    ToggleAutoSave(bool),
    ToggleBackups(bool),
    ValidateAndSave,
    Back,
    ShowError(String),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(Settings),
    Back,
    ShowError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub file_path: String,
    pub auto_save: bool,
    pub create_backups: bool,
}

impl Default for Settings {
    fn default() -> Self {
        let file_manager = persistence::FileManager::new()
            .expect("Failed to initialize file manager");
        
        Self {
            file_path: file_manager.get_default_path()
                .to_string_lossy()
                .into_owned(),
            auto_save: true,
            create_backups: true,
        }
    }
}

pub fn update(
    settings: &mut Settings,
    message: Message,
    file_manager: &persistence::FileManager,
) -> crate::Action<Operation, Message> {
    match message {
        Message::UpdateFilePath(path) => {
            settings.file_path = path;
            crate::Action::none()
        }
        Message::ToggleAutoSave(enabled) => {
            settings.auto_save = enabled;
            crate::Action::none()
        }
        Message::ToggleBackups(enabled) => {
            settings.create_backups = enabled;
            crate::Action::none()
        }
        Message::ValidateAndSave => {
            match file_manager.validate_path(&settings.file_path) {
                Ok(()) => crate::Action::operation(Operation::Save(settings.clone())),
                Err(e) => crate::Action::operation(Operation::ShowError(e)),
            }
        }
        Message::Back => {
            crate::Action::operation(Operation::Back)
        }
        Message::ShowError(_) => {
            crate::Action::none()
        }
    }
}

pub fn view<'a>(
    settings: &'a Settings,
    error_message: Option<&'a str>,
) -> Element<'a, Message> {
    container(
        column![
            row![
                button("←")
                    .width(40)
                    .on_press(Message::Back),
                text("Settings").size(24),
            ].spacing(10),
            
            column![
                text("Data File Path:"),
                text_input("Path to RON file", &settings.file_path)
                    .on_input(Message::UpdateFilePath)
                    .padding(5),
                
                checkbox("Auto-save on changes", settings.auto_save)
                    .on_toggle(Message::ToggleAutoSave),
                
                checkbox("Create backups before saving", settings.create_backups)
                    .on_toggle(Message::ToggleBackups),
                
                if let Some(error) = error_message {
                    text(error).style(text::danger)
                } else {
                    text("")
                },
                
                row![
                    button("Save Settings")
                        .on_press(Message::ValidateAndSave)
                        .style(button::primary),
                ]
                .spacing(10)
            ]
            .spacing(10)
            .padding(20)
        ]
        .spacing(20)
        .padding(20)
    )
    .style(container::rounded_box)
    .into()
}