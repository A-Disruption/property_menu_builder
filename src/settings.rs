use iced::widget::{button, checkbox, column, container, row, text, text_input};
use iced::{Element, Length, Theme};
pub use iced::window::Settings;
use iced_modern_theme::Modern;
use crate::HotKey;
use serde::{Serialize, Deserialize};
use crate::persistence;
use std::fs::File;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Message {
    UpdateFilePath(String),
    ToggleAutoSave(bool),
    ToggleBackups(bool),
    ValidateAndSave,
    Back,
    ShowError(String),
    ThemeChanged(ThemeChoice),
    ExportItemsToCSV,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(AppSettings),
    Back,
    ShowError(String),
    ThemeChanged(ThemeChoice),
    ExportItemsToCSV,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub file_path: String,
    pub auto_save: bool,
    pub create_backups: bool,
    pub app_theme: ThemeChoice,
}

impl Default for AppSettings {
    fn default() -> Self {
        let file_manager = persistence::FileManager::new()
            .expect("Failed to initialize file manager");
        
        Self {
            file_path: file_manager.get_default_path()
                .to_string_lossy()
                .into_owned(),
            auto_save: true,
            create_backups: true,
            app_theme: ThemeChoice::Dark,
        }
    }
}

pub fn update(
    settings: &mut AppSettings,
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
        Message::ThemeChanged(theme) => {
            settings.app_theme = theme.clone();
            crate::Action::operation(Operation::ThemeChanged(theme))
        }
        Message::ExportItemsToCSV => {
            crate::Action::operation(Operation::ExportItemsToCSV)
        }
    }
}

pub fn view<'a>(
    settings: &'a AppSettings,
    error_message: Option<&'a str>,
) -> Element<'a, Message> {

    let title_row = row![
        text("Settings").style(Modern::primary_text()).size(18)
    ].padding(15);

    let setting_column = column![
        text("Data File Path:"),
        text_input("Path to RON file", &settings.file_path)
            .on_input(Message::UpdateFilePath)
            .style(Modern::inline_text_input())
            .padding(5),
        
        row![
            checkbox("Auto-save on changes", settings.auto_save)
            .on_toggle(Message::ToggleAutoSave)
            .style(Modern::checkbox()),
        
            checkbox("Create backups before saving", settings.create_backups)
                .on_toggle(Message::ToggleBackups)
                .style(Modern::checkbox()),
        ].spacing(15),
        
        if let Some(error) = error_message {
            text(error).style(Modern::error_text())
        } else {
            text("")
        },

        row![
            button("Save Settings")
                .on_press(Message::ValidateAndSave)
                .style(Modern::primary_button()),
        ]
        .spacing(10),

/* 
         iced::widget::pick_list(
            ThemeChoice::ALL,
            Some(settings.app_theme),
            |m| Message::ThemeChanged(m)
        ).style(Modern::pick_list()),
         */

    ].spacing(10);

    let setting_container = container(
        column![
            title_row,
            setting_column,
        ]
        .spacing(10)
        .padding(15)
    )
    .width(805)
    .style(Modern::card_container());

    let import_export = container(
        // Add an export section
        column![
            text("Data Export").size(18),
            row![
                button("Export Menu Items to CSV")
                    .on_press(Message::ExportItemsToCSV)
                    .style(Modern::system_button()),
            ]
            .spacing(10),
        ]

        .spacing(10)
        .padding(15), 
    )
    .style(Modern::card_container())
    .width(805)
    .padding(15);


    column![
        setting_container,
        import_export,
    ]
    .spacing(10)
    .into()
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeChoice {
    Light,
    Dark,
}

impl ThemeChoice {
    pub const ALL: &'static [Self] = &[
        Self::Light,
        Self::Dark,
    ];
}

impl fmt::Display for ThemeChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Light => write!(f, "Light"),
            Self::Dark => write!(f, "Dark"),
        }
    }
}

#[cfg(target_os = "windows")]
pub fn load_icon() -> Option<iced::window::icon::Icon> {
    use iced::window::icon;
    use image::EncodableLayout;

    let img = image::load_from_memory_with_format(
        include_bytes!("../icons/MenuBuilder.ico"),
        image::ImageFormat::Ico,
    );

    match img {
        Ok(img) => match img.as_rgba8() {
            Some(icon) => icon::from_rgba(
                icon.as_bytes().to_vec(),
                icon.width(),
                icon.height(),
            )
            .ok(),
            None => None,
        },
        Err(_) => None,
    }
}

pub fn settings() -> Settings {
    Settings {
        icon: load_icon(),
        min_size: Some(iced::Size::new( 1250_f32, 700_f32)),
        ..Default::default()
    }
}