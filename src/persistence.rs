use std::fs;
use std::path::{Path, PathBuf};
use chrono::Local;
use directories::ProjectDirs;
use serde::{Serialize, Deserialize};
use crate::{
    items::Item, 
    item_groups::ItemGroup,
    price_levels::PriceLevel,
    product_classes::ProductClass,
    tax_groups::TaxGroup,
    security_levels::SecurityLevel,
    revenue_categories::RevenueCategory,
    report_categories::ReportCategory,
    choice_groups::ChoiceGroup,
    printer_logicals::PrinterLogical,
    settings::Settings,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppState {
    pub items: Vec<Item>,
    pub item_groups: Vec<ItemGroup>,
    pub price_levels: Vec<PriceLevel>,
    pub product_classes: Vec<ProductClass>,
    pub tax_groups: Vec<TaxGroup>,
    pub security_levels: Vec<SecurityLevel>,
    pub revenue_categories: Vec<RevenueCategory>,
    pub report_categories: Vec<ReportCategory>,
    pub choice_groups: Vec<ChoiceGroup>,
    pub printer_logicals: Vec<PrinterLogical>,
    pub settings: Settings,
}

pub fn save_to_file(state: &AppState, path: &str) -> Result<(), String> {
    let serialized = ron::ser::to_string_pretty(
        state,
        ron::ser::PrettyConfig::default(),
    ).map_err(|e| format!("Failed to serialize state: {}", e))?;

    fs::write(path, serialized)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}

pub fn load_from_file(path: &str) -> Result<AppState, String> {
    if !Path::new(path).exists() {
        return Ok(AppState::default());
    }

    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    ron::from_str(&content)
        .map_err(|e| format!("Failed to parse file: {}", e))
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            item_groups: Vec::new(),
            price_levels: Vec::new(),
            product_classes: Vec::new(),
            tax_groups: Vec::new(),
            security_levels: Vec::new(),
            revenue_categories: Vec::new(),
            report_categories: Vec::new(),
            choice_groups: Vec::new(),
            printer_logicals: Vec::new(),
            settings: Settings::default(),
        }
    }
}

pub struct FileManager {
    project_dirs: ProjectDirs,
}

impl FileManager {
    pub fn new() -> Option<Self> {
        ProjectDirs::from("com", "MenBuilder", "menu_builder").map(|dirs| Self { project_dirs: dirs })
    }

    pub fn get_default_path(&self) -> PathBuf {
        self.project_dirs.data_dir().join("menu_data.ron")
    }

    pub fn ensure_data_dir(&self) -> std::io::Result<()> {
        fs::create_dir_all(self.project_dirs.data_dir())
    }

    pub fn create_backup(&self, path: &Path) -> Result<(), String> {
        if !path.exists() {
            return Ok(());
        }

        let backup_name = format!(
            "{}_backup_{}.ron",
            path.file_stem().unwrap().to_string_lossy(),
            Local::now().format("%Y%m%d_%H%M%S")
        );
        
        let backup_path = path.with_file_name(backup_name);
        fs::copy(path, backup_path)
            .map_err(|e| format!("Failed to create backup: {}", e))?;
        
        Ok(())
    }

    pub fn validate_path(&self, path: &str) -> Result<(), String> {
        let path = Path::new(path);
        
        // Check if directory exists or can be created
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Cannot create directory: {}", e))?;
            }
        }

        // Check if file is writable by attempting to create it
        if !path.exists() {
            fs::write(path, "")
                .map_err(|e| format!("Cannot write to file: {}", e))?;
            fs::remove_file(path)
                .map_err(|e| format!("Cannot remove test file: {}", e))?;
        }

        Ok(())
    }
}