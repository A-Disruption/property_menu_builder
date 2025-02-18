use iced::widget::{button, checkbox, column, container, row, text, text_input};
use iced::{Element, Length, Theme};
pub use iced::window::Settings;
use crate::HotKey;
use serde::{Serialize, Deserialize};
use crate::persistence;
use std::fs::File;

#[derive(Debug, Clone)]
pub enum Message {
    UpdateFilePath(String),
    ToggleAutoSave(bool),
    ToggleBackups(bool),
    ValidateAndSave,
    Back,
    ShowError(String),
    UpdateTheme(String),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(AppSettings),
    Back,
    ShowError(String),
    UpdateTheme(Theme),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub file_path: String,
    pub auto_save: bool,
    pub create_backups: bool,
    pub app_theme: String,
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
            app_theme: theme_to_string(Theme::SolarizedDark),
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
        Message::UpdateTheme(theme) => {
            settings.app_theme = theme.clone();
            crate::Action::operation(Operation::UpdateTheme(string_to_theme(theme.as_str())))
        }
    }
}

pub fn view<'a>(
    settings: &'a AppSettings,
    error_message: Option<&'a str>,
) -> Element<'a, Message> {
    println!("Theme: {:?}", settings.app_theme);
    container(
        column![
            row![
//                button("←")
//                    .width(40)
//                    .on_press(Message::Back),
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

                 iced::widget::pick_list(
                    Theme::ALL,
                    Some(string_to_theme(&settings.app_theme)),
                    |m| Message::UpdateTheme(theme_to_string(m))
                ), 
                
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





#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ThemeConfig(String);

impl ThemeConfig {
    pub fn from_theme(theme: Theme) -> Self {
        Self(theme_to_string(theme))
    }

    pub fn to_theme(&self) -> Theme {
        string_to_theme(&self.0)
    }
}

//helper functions
fn theme_to_string(theme: Theme) -> String {
    match theme {
        Theme::Light => "Light".to_string(),
        Theme::Dark => "Dark".to_string(),
        Theme::Dracula => "Dracula".to_string(),
        Theme::Nord => "Nord".to_string(),
        Theme::SolarizedLight => "SolarizedLight".to_string(),
        Theme::SolarizedDark => "SolarizedDark".to_string(),
        Theme::GruvboxLight => "GruvboxLight".to_string(),
        Theme::GruvboxDark => "GruvboxDark".to_string(),
        Theme::CatppuccinLatte => "CatppuccinLatte".to_string(),
        Theme::CatppuccinFrappe => "CatppuccinFrappe".to_string(),
        Theme::CatppuccinMacchiato => "CatppuccinMacchiato".to_string(),
        Theme::CatppuccinMocha => "CatppuccinMocha".to_string(),
        Theme::TokyoNight => "TokyoNight".to_string(),
        Theme::TokyoNightStorm => "TokyoNightStorm".to_string(),
        Theme::TokyoNightLight => "TokyoNightLight".to_string(),
        Theme::KanagawaWave => "KanagawaWave".to_string(),
        Theme::KanagawaDragon => "KanagawaDragon".to_string(),
        Theme::KanagawaLotus => "KanagawaLotus".to_string(),
        Theme::Moonfly => "Moonfly".to_string(),
        Theme::Nightfly => "Nightfly".to_string(),
        Theme::Oxocarbon => "Oxocarbon".to_string(),
        Theme::Ferra => "Ferra".to_string(),
        _ => {"".to_string()}
    }
}

pub fn string_to_theme(s: &str) -> Theme {
    match s {
        "Light" => Theme::Light,
        "Dark" => Theme::Dark,
        "Dracula"=> Theme::Dracula,
        "Nord" => Theme::Nord,
        "SolarizedLight" => Theme::SolarizedLight,
        "SolarizedDark" => Theme::SolarizedDark,
        "GruvboxLight" => Theme::GruvboxLight,
        "GruvboxDark" => Theme::GruvboxDark,
        "CatppuccinLatte" => Theme::CatppuccinLatte,
        "CatppuccinFrappe" => Theme::CatppuccinFrappe,
        "CatppuccinMacchiato" => Theme::CatppuccinMacchiato,
        "CatppuccinMocha" => Theme::CatppuccinMocha,
        "TokyoNight" => Theme::TokyoNight,
        "TokyoNightStorm" => Theme::TokyoNightStorm,
        "TokyoNightLight" => Theme::TokyoNightLight,
        "KanagawaWave" => Theme::KanagawaWave,
        "KanagawaDragon" => Theme::KanagawaDragon,
        "KanagawaLotus" => Theme::KanagawaLotus,
        "Moonfly" => Theme::Moonfly,
        "Nightfly" => Theme::Nightfly,
        "Oxocarbon" => Theme::Oxocarbon,
        "Ferra" => Theme::Ferra,
        _ => Theme::SolarizedDark,
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
        min_size: Some(iced::Size::new( 1201_f32, 700_f32)),
        ..Default::default()
    }
}