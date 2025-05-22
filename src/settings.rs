use iced::widget::{button, checkbox, column, container, row, text, text_input};
use iced::{Element, Task};
pub use iced::window::Settings;
use iced_modern_theme::Modern;
use serde::{Serialize, Deserialize};
use crate::persistence;
use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;
use std::path::{Path, PathBuf};
use std::io;

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
    OpenFile,
    FileOpened(Result<(PathBuf, Option<Arc<String>>), Error>),
    ProcessItems((BTreeMap<i32, crate::items::Item>, PathBuf)),
    ExportMessage(Result<PathBuf, Error>),
    UpdateExportSuccess(bool),
    UpdateExportMessage(String),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(AppSettings),
    Back,
    ShowError(String),
    ThemeChanged(ThemeChoice),
    RequestItemsList(PathBuf),
    UpdateExportSuccess(bool),
    UpdateExportMessage(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub file_path: String,
    pub auto_save: bool,
    pub create_backups: bool,
    pub app_theme: ThemeChoice,
    pub export_success: bool,
    pub export_message: String,
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
            export_success: true,
            export_message: String::new(),
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
            crate::Action::none()
        }
        Message::OpenFile => {
            let task = Task::perform(open_or_create_file(), Message::FileOpened);

            crate::Action::none().with_task(task)
        }
        Message::FileOpened(result) => {
            println!("File Opened Message triggered");
            match result {
                Ok( (path, _err) ) => {
                    let update_message = Task::done(Message::UpdateExportSuccess(true));

                    println!("Exporting item export to: {:?}", path);

                    return crate::Action::operation(Operation::RequestItemsList(path)).with_task(update_message)
                }
                Err(e) => {
                    println!("Error with the path: {:?}", e);
                    let update_success_task = Task::done(Message::UpdateExportSuccess(false));
                    let update_message_task = Task::done(Message::UpdateExportMessage(format!("Error with the path: {:?}", e)));

                    let combined_task = update_success_task.chain(update_message_task);

                    return crate::Action::none().with_task(combined_task)
                }
            }
        }
        Message::ProcessItems( (items, path) ) => {
            println!("Processing Items!");
            println!("Item Count: {}", &items.len());
            println!("Path: {:?}", &path);

            let task = Task::perform(
                write_to_item_export(items, Some(path)),
                Message::ExportMessage
            );
            println!("Task Created");
            println!("export message: {:?}", settings.export_message);
            println!("export success: {:?}", settings.export_success);

            return crate::Action::none().with_task(task)
        }
        Message::ExportMessage(result) => {
            println!("Export Message triggered: {:?}", &result);
            match result {
                Ok(saved_path) => {
                    let update_success_task = Task::done(Message::UpdateExportSuccess(true));
                    let update_message_task = Task::done(Message::UpdateExportMessage(format!("Items successfully exported to {}", saved_path.to_string_lossy().to_string())));

                    let combined_task = update_success_task.chain(update_message_task);

                    return crate::Action::none().with_task(combined_task)
                }
                Err(e) => {
                    let update_success_task = Task::done(Message::UpdateExportSuccess(false));
                    let update_message_task = Task::done(Message::UpdateExportMessage(format!("Items successfully exported to {:?}", e)));

                    let combined_task = update_success_task.chain(update_message_task);

                    return crate::Action::none().with_task(combined_task)
                }
            }
        }
        Message::UpdateExportSuccess(b) => crate::Action::operation(Operation::UpdateExportSuccess(b)),
        Message::UpdateExportMessage(msg) => crate::Action::operation(Operation::UpdateExportMessage(msg)),
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
                    .on_press(Message::OpenFile)
                    .style(Modern::system_button()),
            ]
            .spacing(10),
            text(&settings.export_message).size(12).style(
                Modern::validated_text(!settings.export_success.clone())
            ),
        ]

        .spacing(10)
        .padding(10), 
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

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IoError(std::io::ErrorKind),
}

pub async fn open_or_create_file() -> Result<(PathBuf, Option<Arc<String>>), Error> {
    // Use AsyncFileDialog to let user pick a file or create one
    let file_handle = rfd::AsyncFileDialog::new()
        .set_title("Open existing file or enter new file name...")
        .add_filter("Text Files", &["txt"])
        .add_filter("CSV Files", &["csv"])
        .add_filter("All Files", &["*"])
        .save_file() // Using save_file() allows creating new files
        .await
        .ok_or(Error::DialogClosed)?;
    
    let path = file_handle.path().to_owned();
    
    // Check if the selected file exists
    if path.exists() {
        // If it exists, load its contents
        match load_file(&path).await {
            Ok((path, contents)) => Ok((path, Some(contents))),
            Err(e) => Err(e),
        }
    } else {
        // If it doesn't exist yet, return the path but no contents
        Ok((path, None))
    }
}

pub async fn load_file(
    path: impl AsRef<Path>,
) -> Result<(PathBuf, Arc<String>), Error> {
    let path = path.as_ref().to_owned();

    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok((path, contents))
}

pub async fn write_to_item_export(
    items: BTreeMap<i32, crate::items::Item>, 
    path: Option<PathBuf>
) -> Result<PathBuf, Error> {
    println!("write-to-items-export function triggered");
    // If path is None, prompt for a save location
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
            .add_filter("CSV Files", &["csv"])
            .add_filter("Text Files", &["txt"])
            .set_title("Save Items Export")
            .save_file()
            .await
            .as_ref()
            .map(rfd::FileHandle::path)
            .map(Path::to_owned)
            .ok_or(Error::DialogClosed)?
    };

    // Convert items to export strings
    let mut content = String::new();

    let items_vec: Vec<&crate::items::Item> = items.values().collect();
    
    for (i, item) in items_vec.iter().enumerate() {
        // Convert each item to its export string representation
        let export_string = crate::items::export_items::item_to_export_string(item);
        content.push_str(&export_string);
        
        // Add newline if not the last item
        if i < items.len() - 1 {
            content.push('\n');
        }
    }

    // Write the content to the file
    tokio::fs::write(&path, content)
        .await
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok(path)
}