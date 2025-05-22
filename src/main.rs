// #![windows_subsystem = "windows"]
use iced::advanced::graphics::core::window;
use iced::event;
use iced::keyboard::{self, Key, Modifiers};
use iced::widget::{
    focus_next, focus_previous,
    button, column, container, row, text, vertical_space, opaque, stack
};
use iced::{Element, Length, Size, Subscription, Task, Theme};
use persistence::FileManager;
use price_levels::PriceLevelType;
use std::collections::BTreeMap;
use rust_decimal::Decimal;
use std::path::PathBuf;
use std::ops::Range;
use iced_modern_theme::Modern;

mod action;
mod settings;
mod items;
mod item_groups;
mod price_levels;
mod product_classes;
mod tax_groups;
mod security_levels;
mod revenue_categories;
mod report_categories;
mod choice_groups;
mod printer_logicals;
mod data_types;
mod persistence;
mod entity_component;
mod icon;
mod multiwindow;

use crate::{
    items::import_items,
    items::{Item, ViewContext},
    item_groups::ItemGroup,
    price_levels::PriceLevel,
    product_classes::ProductClass,
    tax_groups::TaxGroup,
    security_levels::SecurityLevel,
    revenue_categories::RevenueCategory,
    report_categories::ReportCategory,
    choice_groups::ChoiceGroup,
    printer_logicals::PrinterLogical,
    data_types::ValidationError,
};

use data_types::{EntityId, ItemPrice};
pub use action::Action;

fn main() -> iced::Result {
    iced::application(MenuBuilder::title, MenuBuilder::update, MenuBuilder::view)
        .window(settings::settings())
        .window_size(Size::new(1201.0, 700.0))
        .theme(MenuBuilder::theme)
        .antialiasing(true)
        .centered()
        .font(icon::FONT)
        .subscription(MenuBuilder::subscription)
        .run_with(MenuBuilder::new)
}

#[derive(Debug, Clone)]
pub enum Screen {
    Settings(settings::AppSettings),
    Items(items::Mode),
    ItemGroups,
    PriceLevels,
    ProductClasses,
    TaxGroups,
    SecurityLevels,
    RevenueCategories,
    ReportCategories,
    ChoiceGroups,
    PrinterLogicals,
}

#[derive(Debug, Clone)]
pub enum Message {
    Settings(settings::Message),
    PrinterLogicals(EntityId, printer_logicals::Message),
    Items(EntityId, items::Message),
    ItemGroups(EntityId, item_groups::Message), 
    PriceLevels(EntityId, price_levels::Message),
    ProductClasses(EntityId, product_classes::Message),
    TaxGroups(EntityId, tax_groups::Message),
    SecurityLevels(EntityId, security_levels::Message),
    RevenueCategories(EntityId, revenue_categories::Message),
    ReportCategories(EntityId, report_categories::Message),
    ChoiceGroups(EntityId, choice_groups::Message),
    Navigate(Screen),
    HotKey(HotKey),
    ConfirmDelete(data_types::DeletionInfo),
    CancelDelete,
    ToggleTheme(bool),

    //import handles
    FileDropped(PathBuf),
    ImportItemsOverwriteExisting,
    ImportItemsIntoExisting,
    CancelItemImport,

    //window handles
    WindowClosed,
    WindowResized(iced::Size),
}

#[derive(Debug)]
pub enum Operation {
    Settings(settings::Operation),
    Items(EntityId, items::Operation),
    ItemGroups(EntityId, item_groups::Operation),
    PriceLevels(EntityId, price_levels::Operation),
    ProductClasses(EntityId, product_classes::Operation),
    TaxGroups(EntityId, tax_groups::Operation),
    SecurityLevels(EntityId, security_levels::Operation),
    RevenueCategories(EntityId, revenue_categories::Operation),
    ReportCategories(EntityId, report_categories::Operation),
    ChoiceGroups(EntityId, choice_groups::Operation),
    PrinterLogicals(EntityId, printer_logicals::Operation),
}

pub struct MenuBuilder {
    windows: BTreeMap<window::Id, multiwindow::Window>,
    screen: Screen,
    settings: settings::AppSettings,
    theme: iced::Theme,
    file_manager: persistence::FileManager,
    deletion_info: data_types::DeletionInfo,
    show_modal: bool,
    show_item_import_confirmation: bool,
    error_message: Option<String>,
    toggle_theme: bool,
    import_item_path: PathBuf,

    // Items
    items: BTreeMap<EntityId, Item>,
    draft_item: Item,
    draft_item_id: Option<EntityId>,
    selected_item_id: Option<EntityId>,
    item_edit_state: items::EditState,
    item_search: String,
 
    // Item Groups 
    item_groups: BTreeMap<EntityId, ItemGroup>,
    item_group_edit_state_vec: Vec<item_groups::ItemGroupEditState>,
 
    // Price Levels
    price_levels: BTreeMap<EntityId, PriceLevel>,
    price_level_edit_state_vec: Vec<price_levels::PriceLevelEditState>,

    // Product Classes
    product_classes: BTreeMap<EntityId, ProductClass>,
    product_class_edit_state_vec: Vec<entity_component::EditState>,
 
    // Tax Groups
    tax_groups: BTreeMap<EntityId, TaxGroup>,
    tax_group_edit_state_vec: Vec<tax_groups::TaxGroupEditState>,
 
    // Security Levels
    security_levels: BTreeMap<EntityId, SecurityLevel>,
    security_level_edit_state_vec: Vec<entity_component::EditState>,

    // Revenue Categories
    revenue_categories: BTreeMap<EntityId, RevenueCategory>,
    revenue_category_edit_state_vec: Vec<entity_component::EditState>,
 
    // Report Categories
    report_categories: BTreeMap<EntityId, ReportCategory>,
    report_category_edit_state_vec: Vec<entity_component::EditState>,
 
    // Choice Groups
    choice_groups: BTreeMap<EntityId, ChoiceGroup>,
    choice_group_edit_state_vec: Vec<entity_component::EditState>,
 
    // Printer Logicals
    printer_logicals: BTreeMap<EntityId, PrinterLogical>,
    printer_logical_edit_state_vec: Vec<entity_component::EditState>,
 }
 
 impl Default for MenuBuilder {
    fn default() -> Self {
        // Initialize file manager first
        let file_manager = FileManager::new()
            .expect("Failed to initialize file manager");
        
        // Ensure data directory exists
        file_manager.ensure_data_dir()
            .expect("Failed to create data directory");

        let (main_window_id, open_main_window) = iced::window::open(
            iced::window::Settings {
                size: Size::new(1201.0, 700.0),
                position: window::Position::Centered,
                min_size: Some(Size::new( 1250_f32, 700_f32)),
                exit_on_close_request: true,
                icon: settings::load_icon(),
                ..iced::window::Settings::default()
            }
        );

        let main_window = multiwindow::Window {
            id: main_window_id,
            title: String::from("Menu Builder :D"),
            focused: true,
            size: Size::new(1201.0, 700.0),
        };

        let mut windows = BTreeMap::new();
        windows.insert(main_window_id, main_window);

        Self {
            windows: windows,
            screen: Screen::Items(items::Mode::View),
            settings: settings::AppSettings::default(),
            theme: iced_modern_theme::Modern::dark_theme(),
            file_manager: file_manager,
            show_item_import_confirmation: false,
            show_modal: false,
            deletion_info: data_types::DeletionInfo::new(),
            error_message: None,
            toggle_theme: true,
            import_item_path: PathBuf::new(),

            // Items
            items: BTreeMap::new(),
            draft_item: Item::default(),
            draft_item_id: None,
            selected_item_id: None,
            item_edit_state: items::EditState::default(),
            item_search: String::new(),
 
            // Item Groups
            item_groups: BTreeMap::new(),
            item_group_edit_state_vec: Vec::new(),

            // Price Levels 
            price_levels: BTreeMap::new(),
            price_level_edit_state_vec: Vec::new(),
 
            // Product Classes
            product_classes: BTreeMap::new(),
            product_class_edit_state_vec: Vec::new(),
 
            // Tax Groups
            tax_groups: BTreeMap::new(),
            tax_group_edit_state_vec: Vec::new(),
 
            // Security Levels
            security_levels: BTreeMap::new(),
            security_level_edit_state_vec: Vec::new(),
 
            // Revenue Categories
            revenue_categories: BTreeMap::new(),
            report_category_edit_state_vec: Vec::new(),
 
            // Report Categories
            report_categories: BTreeMap::new(),
            revenue_category_edit_state_vec: Vec::new(),
 
            // Choice Groups
            choice_groups: BTreeMap::new(),
            choice_group_edit_state_vec: Vec::new(),
 
            // Printer Logicals
            printer_logicals: BTreeMap::new(),
            printer_logical_edit_state_vec: Vec::new(),
        }
    }
 }

impl MenuBuilder {

    fn theme(&self) -> iced::Theme {
        self.theme.clone()
    }

    fn title(&self) -> String {
        String::from("Menu Builder :D")
    }

    fn new() -> (Self, Task<Message>) {
        
        let mut menu_builder = MenuBuilder::default();

        let available_choice_groups: Vec<ChoiceGroup> = menu_builder.choice_groups.values().cloned().collect();
        let available_printer_logicals: Vec<PrinterLogical> = menu_builder.printer_logicals.values().cloned().collect();
        let available_price_levels: Vec<PriceLevel> = menu_builder.price_levels.values().cloned().collect();
        // Try to load state from file
        match menu_builder.load_state() {
            Ok(()) => {
                println!("Successfully loaded saved data");
                menu_builder.item_edit_state = items::EditState::new(
                    &menu_builder.draft_item,
                    available_choice_groups,
                    available_printer_logicals,
                    available_price_levels,
                );

                // If no items were loaded, create a default one
                if menu_builder.items.is_empty() {
                    let mut default_item = Item::default();
                    default_item.name = "Default".to_string();
                    menu_builder.items.insert(1, default_item);
                    menu_builder.selected_item_id = Some(1);
                }

                menu_builder.settings.export_message = "".to_string();
                menu_builder.settings.export_success = true;
                menu_builder.error_message = None;
            }
            Err(e) => {
                eprintln!("Failed to load state: {}", e);
                menu_builder.error_message = Some(format!("Failed to load saved data: {}", e));

                // Create a default item for new users
                let mut default_item = Item::default();
                default_item.name = "Default".to_string();
                menu_builder.items.insert(1, default_item);
                menu_builder.selected_item_id = Some(1);
                menu_builder.error_message = Some(format!("Failed to load saved data: {}", e));
            }
        }

        (menu_builder, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        println!("Update Message received: {:?}", &message);
        match message {
            Message::Settings(msg) => {
                println!("Settings message received: {:?}", &msg);

                let action = settings::update(
                    &mut self.settings,
                    msg,
                    &self.file_manager
                )
                .map_operation(move |o| Operation::Settings(o))
                .map(move |m| Message::Settings(m));

                let operation_task = if let Some(operation) = action.operation {
                    self.perform(operation)
                } else {
                    Task::none()
                };

                operation_task.chain(action.task)
            }
            Message::Items(id, msg) => {
                let cloned_items = self.items.clone();

                if id < 0 {  // New item case

                    let context = ViewContext {
                        available_items: cloned_items,
                        available_choice_groups: self.choice_groups.clone(),
                        available_item_groups: self.item_groups.clone(),
                        available_price_levels: self.price_levels.clone(),
                        available_printer_logicals: self.printer_logicals.clone(),
                        available_product_classes: self.product_classes.clone(),
                        available_report_categories: self.report_categories.clone(),
                        available_revenue_categories: self.revenue_categories.clone(),
                        available_security_levels: self.security_levels.clone(),
                        available_tax_groups: self.tax_groups.clone(),
                    };

                    let action = items::update(
                        &mut self.draft_item,
                        msg,
                        &mut self.item_edit_state,
                        &context
                    )
                    .map_operation(move |o| Operation::Items(id, o))
                    .map(move |m| Message::Items(id, m));

                    let operation_task = if let Some(operation) = action.operation {
                        self.perform(operation)
                    } else {
                        Task::none()
                    };
    
                    operation_task.chain(action.task)
                } else {
                    let item = if let Some(draft_id) = self.draft_item_id {
                        if draft_id == id {
                            &mut self.draft_item
                        } else {
                            self.items.get_mut(&id).expect("Item should exist")
                        }
                    } else {
                        self.items.get_mut(&id).expect("Item should exist")
                    };

                    let other_items: BTreeMap<EntityId, &Item> = cloned_items
                        .iter()
                        .filter(|(&item_id, _)| item_id != id)
                        .map(|(&k, v)| (k, v))
                        .collect();

                let context = ViewContext {
                    available_items: cloned_items,
                    available_choice_groups: self.choice_groups.clone(),
                    available_item_groups: self.item_groups.clone(),
                    available_price_levels: self.price_levels.clone(),
                    available_printer_logicals: self.printer_logicals.clone(),
                    available_product_classes: self.product_classes.clone(),
                    available_report_categories: self.report_categories.clone(),
                    available_revenue_categories: self.revenue_categories.clone(),
                    available_security_levels: self.security_levels.clone(),
                    available_tax_groups: self.tax_groups.clone(),
                };

                let action = items::update(
                    item, 
                    msg, 
                    &mut self.item_edit_state,
                    &context
                )
                    .map_operation(move |o| Operation::Items(id, o))
                    .map(move |m| Message::Items(id, m));

                let operation_task = if let Some(operation) = action.operation {
                    self.perform(operation)
                } else {
                    Task::none()
                };

                operation_task.chain(action.task)
                }


            },
            Message::ItemGroups(id, msg) => {
                let cloned_item_groups = self.item_groups.clone();

                // Get a list of all other item groups for validation
                let other_item_groups: Vec<&ItemGroup> = cloned_item_groups
                    .values()
                    .filter(|ig| ig.id != id)
                    .collect();

                let action = item_groups::update(msg)
                    .map_operation(move |o| Operation::ItemGroups(id, o))
                    .map(move |m| Message::ItemGroups(id, m));
                
                let operation_task = if let Some(operation) = action.operation {
                    self.perform(operation)
                } else {
                    Task::none()
                };
                
                operation_task.chain(action.task)
            },
            Message::PriceLevels(id, msg) => {
                let cloned_price_levels = self.price_levels.clone();

                let other_price_levels: Vec<&PriceLevel> = cloned_price_levels
                    .values()
                    .filter(|pl| pl.id != id)
                    .collect();

                let action = price_levels::update(msg)
                    .map_operation(move |o| Operation::PriceLevels(id, o))
                    .map(move |m| Message::PriceLevels(id, m));

                let operation_task = if let Some(operation) = action.operation {
                    self.perform(operation)
                } else {
                    Task::none()
                };
                
                operation_task.chain(action.task)
            },
            Message::ProductClasses(id, msg) => {
                let cloned_product_classes = self.product_classes.clone();

                let other_product_classes: Vec<&ProductClass> = cloned_product_classes
                    .values()
                    .filter(|pc| pc.id != id)
                    .collect();

                let action = product_classes::update(msg)
                    .map_operation(move |o| Operation::ProductClasses(id, o))
                    .map(move |m| Message::ProductClasses(id, m));

                let operation_task = if let Some(operation) = action.operation {
                    self.perform(operation)
                } else {
                    Task::none()
                };

                operation_task.chain(action.task)  
            },
            Message::TaxGroups(id, msg) => {
                let cloned_tax_groups = self.tax_groups.clone();

                let other_tax_groups: Vec<&TaxGroup> = cloned_tax_groups
                    .values()
                    .filter(|tg| tg.id != id)
                    .collect();

                let action = tax_groups::update(msg, )
                    .map_operation(move |o| Operation::TaxGroups(id, o))
                    .map(move |m| Message::TaxGroups(id, m));

                let operation_task = if let Some(operation) = action.operation {
                    self.perform(operation)
                } else {
                    Task::none()
                };

                operation_task.chain(action.task)
            },
            Message::SecurityLevels(id, msg) => {
                let cloned_security_levels = self.security_levels.clone();

                let other_security_levels: Vec<&SecurityLevel> = cloned_security_levels
                    .values()
                    .filter(|sl| sl.id != id)
                    .collect();

                let action = security_levels::update(msg)
                    .map_operation(move |o| Operation::SecurityLevels(id, o))
                    .map(move |m| Message::SecurityLevels(id, m));

                let operation_task = if let Some(operation) = action.operation {
                    self.perform(operation)
                } else {
                    Task::none()
                };

                operation_task.chain(action.task)
            },
            Message::RevenueCategories(id, msg) => {
                let cloned_revenue_categories = self.revenue_categories.clone();

                let other_revenue_categories: Vec<&RevenueCategory> = cloned_revenue_categories
                    .values()
                    .filter(|rc| rc.id != id)
                    .collect();

                let action = revenue_categories::update(msg)
                    .map_operation(move |o| Operation::RevenueCategories(id, o))
                    .map(move |m| Message::RevenueCategories(id, m));

                let operation_task = if let Some(operation) = action.operation {
                    self.perform(operation)
                } else {
                    Task::none()
                };

                operation_task.chain(action.task)
            },
            Message::ReportCategories(id, msg) => {
                let cloned_report_categories = self.report_categories.clone();

                let other_report_categories : Vec<&ReportCategory> = cloned_report_categories
                    .values()
                    .filter(|rc| rc.id != id)
                    .collect();


                let action = report_categories::update(msg)
                    .map_operation(move |o| Operation::ReportCategories(id, o))
                    .map(move |m| Message::ReportCategories(id, m));

                let operation_task = if let Some(operation) = action.operation {
                    self.perform(operation)
                } else {
                    Task::none()
                };

                operation_task.chain(action.task)
            },
            Message::ChoiceGroups(id, msg) => {
                let cloned_choice_groups = self.choice_groups.clone();

                let other_choice_groups: Vec<&ChoiceGroup> = cloned_choice_groups
                    .values()
                    .filter(|c| c.id != id)
                    .collect();

                    let action = choice_groups::update(msg)
                        .map_operation(move |o| Operation::ChoiceGroups(id, o))
                        .map(move |m| Message::ChoiceGroups(id, m));

                    let operation_task = if let Some(operation) = action.operation {
                        self.perform(operation)
                    } else {
                        Task::none()
                    };

                operation_task.chain(action.task)
            },
            Message::PrinterLogicals(id, msg) => {
                let cloned_printers = self.printer_logicals.clone();

                let other_printers: Vec<&PrinterLogical> = cloned_printers
                    .values()
                    .filter(|p| p.id != id)
                    .collect();
            
                let action = printer_logicals::update(msg)
                    .map_operation(move |o| Operation::PrinterLogicals(id, o))
                    .map(move |m| Message::PrinterLogicals(id, m));
            
                let operation_task = if let Some(operation) = action.operation {
                    self.perform(operation)
                } else {
                    Task::none()
                };
            
                operation_task.chain(action.task)
            }
            Message::Navigate(screen) => {
                self.screen = screen;
                Task::none()
            },
            Message::HotKey(hotkey) => {
                match hotkey {
                    HotKey::Tab(modifiers) => {
                        if modifiers.shift() {
                            focus_previous()
                        } else {
                            focus_next()
                        }
                    }
                    HotKey::Escape => Task::none(),
                }
            }
            Message::ConfirmDelete(deletion_info) => {
                println!("Deleting Type: {}, id: {}", deletion_info.entity_type, deletion_info.entity_id);

                match deletion_info.entity_type.as_str() {
                    "ChoiceGroup" => {
                        // Clean up references in all items
                        for (_, item) in self.items.iter_mut() {
                            if let Some(groups) = &mut item.choice_groups {
                                // Remove this specific choice group ID from the Item.choice_groups vec
                                groups.retain(|&group_id| group_id.0 != deletion_info.entity_id);
                                
                                // If vec is empty after removal, set to None
                                if groups.is_empty() {
                                    item.choice_groups = None;
                                }
                            }
                        }

                        // Delete the choice group
                        self.choice_groups.remove(&deletion_info.entity_id);
                        self.screen = Screen::ChoiceGroups;
                    }
                    "ItemGroup" => {
                        // Find all items using this item group
                        for (_, item) in self.items.iter_mut() {
                            if let Some(group_id) = item.item_group {
                                if group_id == deletion_info.entity_id {
                                    // This item has this item group, set it to None
                                    item.item_group = None;
                                }
                            }
                        }

                        // Delete the item group
                        self.item_groups.remove(&deletion_info.entity_id);
                        self.screen = Screen::ItemGroups;
                    }
                    "Item" => {
                        //Delete the item
                        if self.items.contains_key(&deletion_info.entity_id) { self.items.remove(&deletion_info.entity_id); }
                    }
                    "PriceLevel" => {
                        // Clean up references in all items
                        for (_, item) in self.items.iter_mut() {
                            if let Some(price_levels) = &mut item.price_levels {
                                // Remove this specific price level ID from the Item.price_levels vec
                                price_levels.retain(|&price_id| price_id != deletion_info.entity_id);
                                
                                // If vec is empty after removal, set to None
                                if price_levels.is_empty() {
                                    item.price_levels = None;
                                }
                            }
                        }

                        // Delete the price level
                        self.price_levels.remove(&deletion_info.entity_id);
                        self.screen = Screen::PriceLevels;
                    }
                    "PrinterLogical" => {
                        // Clean up references in all items
                        for (_, item) in self.items.iter_mut() {
                            if let Some(printers) = &mut item.printer_logicals {
                                // Remove this specific printer logical ID from the Item.printer_logicals vec
                                printers.retain(|&(printer_id, _)| printer_id != deletion_info.entity_id);
                                
                                // If vec is empty after removal, set to None
                                if printers.is_empty() {
                                    item.printer_logicals = None;
                                }
                            }
                        }

                        // Delete the printer logical
                        self.printer_logicals.remove(&deletion_info.entity_id);
                        self.screen = Screen::PrinterLogicals;
                    }
                    "ProductClass" => {
                        // Find all items using this product class
                        for (_, item) in self.items.iter_mut() {
                            if let Some(pc_id) = item.product_class {
                                if pc_id == deletion_info.entity_id {
                                    // This item has this product class, set it to None
                                    item.product_class = None;
                                }
                            }
                        }

                        // Delete the product class
                        self.product_classes.remove(&deletion_info.entity_id);
                        self.screen = Screen::ProductClasses;
                    }
                    "ReportCategory" => {
                        // Find all items using this report category
                        for (_, item) in self.items.iter_mut() {
                            if let Some(rc_id) = item.report_category {
                                if rc_id == deletion_info.entity_id {
                                    // This item has this report category, set it to None
                                    item.report_category = None;
                                }
                            }
                        }

                        // Delete the report category
                        self.report_categories.remove(&deletion_info.entity_id);
                        self.screen = Screen::ReportCategories;
                    }
                    "RevenueCategory" => {
                        // Find all items using this revenue category
                        for (_, item) in self.items.iter_mut() {
                            if let Some(rc_id) = item.revenue_category {
                                if rc_id == deletion_info.entity_id {
                                    // This item has this revenue category, set it to None
                                    item.revenue_category = None;
                                }
                            }
                        }

                        // Delete the revenue category
                        self.revenue_categories.remove(&deletion_info.entity_id);
                        self.screen = Screen::RevenueCategories;
                    }
                    "SecurityLevel" => {
                        // Find all items using this security level
                        for (_, item) in self.items.iter_mut() {
                            if let Some(sl_id) = item.security_level {
                                if sl_id == deletion_info.entity_id {
                                    // This item has this security level, set it to None
                                    item.security_level = None;
                                }
                            }
                        }

                        // Delete the security level
                        self.security_levels.remove(&deletion_info.entity_id);
                        self.screen = Screen::SecurityLevels;
                    }
                    "TaxGroup" => {
                        // Find all items using this tax group
                        for (_, item) in self.items.iter_mut() {
                            if let Some(tg_id) = item.tax_group {
                                if tg_id == deletion_info.entity_id {
                                    // This item has this tax group, set it to None
                                    item.tax_group = None;
                                }
                            }
                        }

                        // Delete the tax group
                        self.tax_groups.remove(&deletion_info.entity_id);
                        self.screen = Screen::TaxGroups;
                    }
                    _ => {println!("Oh No! You've tried to delete an unknown type: {}", deletion_info.entity_type);}
                }

                self.deletion_info = data_types::DeletionInfo::new();
                self.show_modal = false;
                self.save_state().expect("Failed to save to file.");
                Task::none()
            }
            Message::CancelDelete => {
                println!("Canceling Delete Request");
                self.deletion_info = data_types::DeletionInfo::new();
                self.show_modal = false;
                Task::none()
            }
            Message::ToggleTheme(bool) => {
                if bool {
                    self.theme = iced_modern_theme::Modern::dark_theme()
                } else {
                    self.theme = iced_modern_theme::Modern::light_theme()
                }
                self.toggle_theme = !self.toggle_theme;
                Task::none()
            }
            Message::FileDropped(path) => {

                println!("File Dropped: {:?}", &path);
                self.import_item_path = path.clone();
                if import_items::is_csv_or_txt(path.clone()) {
                    match import_items::verify_csv_format(path) {
                        Ok(_) => {
                            self.show_item_import_confirmation = true;
                            println!("File format confirmed.")
                        }
                        Err(e) => {println!("{:?}", e);}
                    }
                }

                Task::none()
            }
            Message::ImportItemsOverwriteExisting => {
                println!("File Path to import items: {:?}", &self.import_item_path.clone());

                //clear existing data, keep settings
                let default = MenuBuilder::default();
                let settings = self.settings.clone();
                let import_path = &self.import_item_path.clone();
                *self = default;
                self.settings = settings;

                //import items from the import file.
                match import_items::collect_item_information(import_path) {
                    Ok(imported_items) => { 
                        println!("It worked!");
                        self.items = imported_items;
                    },
                    Err(e) => { println!("Error collecting item information!: {}", e); }
                };
                println!("{:?}", self.items.last_entry());

                import_items::ensure_all_referenced_entities_exist(
                    &self.items,
                    &mut self.price_levels,
                    &mut self.product_classes,
                    &mut self.revenue_categories,
                    &mut self.tax_groups,
                    &mut self.security_levels,
                    &mut self.report_categories,
                    &mut self.item_groups,
                    &mut self.choice_groups,
                    &mut self.printer_logicals,
                );

                self.show_item_import_confirmation = false;
                Task::none()
            },
            Message::ImportItemsIntoExisting => {
                self.show_item_import_confirmation = false;
                Task::none()
            },
            Message::CancelItemImport => {
                self.show_item_import_confirmation = false;
                Task::none()
            },
            Message::WindowClosed => {
/*            Message::WindowClosed(id) => {
                 state.windows.remove(&id);

                if state.windows.is_empty() {
                    iced::exit()
                } else {
                    Task::none()
                } */
               Task::none()
            },
            Message::WindowResized(size) => {
                Task::none()
            },
            _ => {
                println!("Received unhandled messaged! ");
                Task::none()
            }
        }   
    }

    fn view(&self) -> Element<Message> {
        let sidebar = container(
            column![
                button("Items")
                    .on_press(Message::Navigate(Screen::Items(items::Mode::View)))
                    .width(Length::Fill)
                    .style(
                        Modern::conditional_button_style(
                            matches!(self.screen, Screen::Items(_)),
                            Modern::selected_button_style(Modern::system_button()),
                            Modern::system_button()
                        )
                    ),
                button("Item Groups")
                    .on_press(Message::Navigate(Screen::ItemGroups))
                    .width(Length::Fill)
                    .style(
                        Modern::conditional_button_style(
                            matches!(self.screen, Screen::ItemGroups),
                            Modern::selected_button_style(Modern::system_button()),
                            Modern::system_button()
                        )
                    ),
                button("Price Levels")
                    .on_press(Message::Navigate(Screen::PriceLevels))
                    .width(Length::Fill)
                    .style(
                        Modern::conditional_button_style(
                            matches!(self.screen, Screen::PriceLevels),
                            Modern::selected_button_style(Modern::system_button()),
                            Modern::system_button()
                        )
                    ),
                button("Product Classes")
                    .on_press(Message::Navigate(Screen::ProductClasses))
                    .width(Length::Fill)
                    .style(
                        Modern::conditional_button_style(
                            matches!(self.screen, Screen::ProductClasses),
                            Modern::selected_button_style(Modern::system_button()),
                            Modern::system_button()
                        )
                    ),
                button("Tax Groups")
                    .on_press(Message::Navigate(Screen::TaxGroups))
                    .width(Length::Fill)
                    .style(
                        Modern::conditional_button_style(
                            matches!(self.screen, Screen::TaxGroups),
                            Modern::selected_button_style(Modern::system_button()),
                            Modern::system_button()
                        )
                    ),
                button("Security Levels")
                    .on_press(Message::Navigate(Screen::SecurityLevels))
                    .width(Length::Fill)
                    .style(
                        Modern::conditional_button_style(
                            matches!(self.screen, Screen::SecurityLevels),
                            Modern::selected_button_style(Modern::system_button()),
                            Modern::system_button()
                        )
                    ),
                button("Revenue Categories")
                    .on_press(Message::Navigate(Screen::RevenueCategories))
                    .width(Length::Fill)
                    .style(
                        Modern::conditional_button_style(
                            matches!(self.screen, Screen::RevenueCategories),
                            Modern::selected_button_style(Modern::system_button()),
                            Modern::system_button()
                        )
                    ),
                button("Report Categories")
                    .on_press(Message::Navigate(Screen::ReportCategories))
                    .width(Length::Fill)
                    .style(
                        Modern::conditional_button_style(
                            matches!(self.screen, Screen::ReportCategories),
                            Modern::selected_button_style(Modern::system_button()),
                            Modern::system_button()
                        )
                    ),
                button("Choice Groups")
                    .on_press(Message::Navigate(Screen::ChoiceGroups))
                    .width(Length::Fill)
                    .style(
                        Modern::conditional_button_style(
                            matches!(self.screen, Screen::ChoiceGroups),
                            Modern::selected_button_style(Modern::system_button()),
                            Modern::system_button()
                        )
                    ),
                button("Printer Logicals")
                    .on_press(Message::Navigate(Screen::PrinterLogicals))
                    .width(Length::Fill)
                    .style(
                        Modern::conditional_button_style(
                            matches!(self.screen, Screen::PrinterLogicals),
                            Modern::selected_button_style(Modern::system_button()),
                            Modern::system_button()
                        )
                    ),

                vertical_space(),
                row![
                    column![
                        text("Toggle Theme").size(10),
                        iced::widget::vertical_space().height(2),
                        iced::widget::toggler(self.toggle_theme).on_toggle(Message::ToggleTheme),
                    ],
                    iced::widget::horizontal_space(),
                    button(icon::settings().size(14)) 
                        .on_press(Message::Navigate(Screen::Settings(self.settings.clone())))
                        //.width(Length::Fixed(40.0))
                        .style(
                            Modern::conditional_button_style(
                                matches!(self.screen, Screen::Settings(_)),
                                Modern::selected_button_style(Modern::system_button()),
                                Modern::system_button()
                            )
                        ),
                ]
            ]
            .spacing(5)
            .padding(10)
        )
        .width(Length::Fixed(200.0))
        .height(Length::Fill)
        .style(Modern::sidebar_container());

        let content = match &self.screen {
            Screen::Settings(settings) => {
                settings::view(settings, self.error_message.as_deref()).map(Message::Settings)
            },
            Screen::Items(mode) => {
                if let Some(id) = self.selected_item_id {
                    // When an item is selected, determine whether it represents a new item
                    // (negative ID) or an existing one, and if there’s a draft override.
                    let item = if id < 0 {
                        // Negative ID indicates a new item; use the draft.
                        &self.draft_item
                    } else if let Some(draft_id) = self.draft_item_id {
                        if draft_id == id {
                            &self.draft_item
                        } else {
                            &self.items[&id]
                        }
                    } else {
                        &self.items[&id]
                    };
                
                    items::view(
                        item,
                        mode,
                        &self.items,
                        &self.item_search,
                        &self.item_edit_state,
                        &self.item_groups,
                        &self.tax_groups,
                        &self.security_levels,
                        &self.revenue_categories,
                        &self.report_categories,
                        &self.product_classes,
                        &self.choice_groups,
                        &self.printer_logicals,
                        &self.price_levels,
                    )
                    .map(move |msg| Message::Items(id, msg))
                } else if let Some((&first_id, first_item)) = self.items.iter().next() {
                    // No selected item, but there is at least one in the collection.
                    items::view(
                        first_item,
                        mode,
                        &self.items,
                        &self.item_search,
                        &self.item_edit_state,
                        &self.item_groups,
                        &self.tax_groups,
                        &self.security_levels,
                        &self.revenue_categories,
                        &self.report_categories,
                        &self.product_classes,
                        &self.choice_groups,
                        &self.printer_logicals,
                        &self.price_levels,
                    )
                    .map(move |msg| Message::Items(first_id, msg))
                } else {
                    // No selected item and no items available: show the welcome screen.
                    container(
                        column![
                            text("Item Management")
                                .size(24)
                                .width(Length::Fill),
                            vertical_space(),
                            text("No items have been created yet.")
                                .width(Length::Fill),
                            vertical_space(),
                            button("Create New Item")
                                .on_press(Message::Items(-1, items::Message::CreateNew))
                                .style(button::primary)
                        ]
                        .spacing(10)
                        .max_width(500)
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .padding(30)
                    .into()
                }
            }
            Screen::ItemGroups => {
                item_groups::view(
                    &self.item_groups,
                    &self.item_group_edit_state_vec)
                .map(move |msg| Message::ItemGroups(-1, msg)) // Default ID for new messages
            }
            Screen::PriceLevels => {
                price_levels::view(
                    &self.price_levels,
                    &self.price_level_edit_state_vec)
                .map(move |msg| Message::PriceLevels(-1, msg))
            }
            Screen::ProductClasses => {

                product_classes::view(
                    &self.product_classes,
                    &self.product_class_edit_state_vec)
                .map(move |msg| Message::ProductClasses(-1, msg))
            }
            Screen::TaxGroups => {
                tax_groups::view(
                    &self.tax_groups,
                    &self.tax_group_edit_state_vec)
                .map(move |msg| Message::TaxGroups(-1, msg))
            }
            Screen::SecurityLevels => {
                security_levels::view(
                    &self.security_levels,
                    &self.security_level_edit_state_vec)
                .map(move |msg| Message::SecurityLevels(-1, msg))
            }
            Screen::RevenueCategories => {
                revenue_categories::view(
                    &self.revenue_categories,
                    &self.revenue_category_edit_state_vec)
                .map(move |msg| Message::RevenueCategories(-1, msg))
            }
            Screen::ReportCategories => {

                report_categories::view(
                    &self.report_categories,
                    &self.report_category_edit_state_vec)
                .map(move |msg| Message::ReportCategories(-1, msg))
            }
            Screen::ChoiceGroups => {
                choice_groups::view(
                    &self.choice_groups,
                    &self.choice_group_edit_state_vec)
                .map(move |msg| Message::ChoiceGroups(-1, msg))
            }
            Screen::PrinterLogicals => {
                printer_logicals::view(
                    &self.printer_logicals, 
                    &self.printer_logical_edit_state_vec)
                .map(move |msg| Message::PrinterLogicals(-1, msg))
            }
        };

        let delete_confirmation_popup = container(
            container(
                column![
                    vertical_space().height(10),
                    row![
                        iced::widget::horizontal_space().width(6),
                        text("Are you sure you want to delete this ".to_string() + &self.deletion_info.entity_type).style(Modern::primary_text()).size(16),
                        iced::widget::horizontal_space().width(6),
                    ],
                    
                    iced::widget::vertical_space().height(15),
                    row![
                        iced::widget::horizontal_space().width(6),
                        button("Delete").on_press(Message::ConfirmDelete(self.deletion_info.clone())).style(Modern::danger_button()),
                        iced::widget::horizontal_space(),
                        button("Cancel").on_press(Message::CancelDelete).style(Modern::system_button()),
                        iced::widget::horizontal_space().width(6),
                    ]
                ].width(275).height(100)
            ).style(Modern::separated_container())
        ).padding(250);

        let import_items_confirmation = container(
            container(
                column![
                    vertical_space().height(10),
                    row![
                        iced::widget::horizontal_space().width(6),
                        text("Import Items").style(Modern::primary_text()).size(18),
                        iced::widget::horizontal_space().width(6),
                    ],
                    iced::widget::vertical_space().height(10),
                    row![
                        iced::widget::horizontal_space().width(6),
                        text("Do you want to overwrite existing data, or import into the existing database?").style(Modern::secondary_text()).size(14),
                        iced::widget::horizontal_space().width(6),
                    ],
                    
                    iced::widget::vertical_space().height(15),
                    row![
                        iced::widget::horizontal_space().width(6),
                        button("New Database").on_press(Message::ImportItemsOverwriteExisting).style(Modern::warning_button()),
                        iced::widget::horizontal_space(),
                        button("Add to existing").on_press(Message::ImportItemsIntoExisting).style(Modern::primary_button()),
                        iced::widget::horizontal_space(),
                        button("Cancel").on_press(Message::CancelItemImport).style(Modern::system_button()),
                        iced::widget::horizontal_space().width(6),
                    ]
                ].width(335).height(135)
            ).style(Modern::accent_container())
        ).padding(250);

        //iced::widget::stack
        let app_view = row![
            sidebar,
            container(content)
                .width(Length::Fill)
                .padding(20),
        ];
        
        
        if self.show_modal { //Show Deletion confirmation popup
            stack![
                app_view,
                opaque(delete_confirmation_popup)
            ].into()
        } else if self.show_item_import_confirmation { // Show Item Import Confirmation popup
            stack![
                app_view,
                opaque(import_items_confirmation)
            ].into()
        }
        else {
            app_view.into()
        }
     }


    fn perform(&mut self, operation: Operation) -> Task<Message> {
        match operation {
            Operation::Settings(op) => {
                match op {
                    settings::Operation::Save(new_settings) => {
                        self.settings = new_settings;

                        if let Err(e) = self.save_state() {
                            self.error_message = Some(e);
                        } else {
                            self.error_message = None;
                        }

                        self.screen = Screen::Settings(self.settings.clone());
                        Task::none()
                    }
                    settings::Operation::Back => {
                        self.screen = Screen::Items(items::Mode::View);
                        self.error_message = None;
                        Task::none()
                    }
                    settings::Operation::ShowError(error) => {
                        self.error_message = Some(error);
                        self.screen = Screen::Settings(self.settings.clone());
                        Task::none()
                    }
                    settings::Operation::ThemeChanged(theme) => {

                        self.theme = match theme {
                            settings::ThemeChoice::Light => Modern::light_theme(),
                            settings::ThemeChoice::Dark => Modern::dark_theme(),
                        };

                        self.screen = Screen::Settings(self.settings.clone());
                        Task::none()
                    }
                    settings::Operation::RequestItemsList(path) => {
                        println!("Direct handling - bypassing task system");
    
                        self.update(
                            Message::Settings(
                                settings::Message::ProcessItems(
                                    ( self.items.clone(), path )))
                            )
                    }
                    settings::Operation::UpdateExportMessage(msg) => {
                        println!("Updating Export Message to: {}", &msg);
                        self.settings.export_message = msg;
                        self.screen = Screen::Settings(self.settings.clone());
                        Task::none()
                    }
                    settings::Operation::UpdateExportSuccess(success) => {
                        println!("Updating Export Success value: {:?}", &success);
                        self.settings.export_success = success;
                        self.screen = Screen::Settings(self.settings.clone());
                        Task::none()
                    }
                }
            }
            Operation::Items(id, op) => {
                match op {
                    items::Operation::Save(mut item) => {
                        println!("Saving Item ID: {}, with prices: {:?}", item.id, item.item_prices);
                        println!("EditState information: {:?}", self.item_edit_state.prices);

                        let edit_state_prices = self.item_edit_state.prices.clone();
                        //Copy prices from edit_state,to item
                        let item_prices = edit_state_prices.unwrap_or(Vec::new()).iter().map(
                            |price| ItemPrice {
                                price_level_id: price.0,
                                price: price.1.parse::<Decimal>().unwrap_or(Decimal::new(0, 2))
                            }
                        ).collect::<Vec<_>>();

                        item.item_prices = Some(item_prices);

                        if item.id < 0 {
                            let next_id = self.items
                                .keys()
                                .max()
                                .map_or(1, |max_id| max_id + 1);
                            item.id = next_id;

                            self.items.insert(next_id, item.clone());
                            self.draft_item_id = None;
                            self.draft_item = Item::default();
                            self.selected_item_id = Some(next_id);
                        } else {
                            self.items.insert(item.id, item.clone());
                            self.selected_item_id = Some(item.id);
                        }
                        self.screen = Screen::Items(items::Mode::View);

                        if self.settings.auto_save {
                            if let Err(e) = self.save_state() {
                                self.handle_save_error(e);
                            }
                        }

                        if let Err(e) = self.save_state() {
                            self.error_message = Some(e);
                        } else {
                            self.error_message = None;
                        }

                        Task::none()
                    }
                    items::Operation::StartEdit(id) => {
                        // Start editing an existing Item
                        self.draft_item_id = Some(id);
                        self.draft_item = self.items[&id].clone();

                        let item = self.items.get_mut(&id).expect("Item should exist");
                        let available_choice_groups: Vec<ChoiceGroup> = self.choice_groups.values().cloned().collect();
                        let available_printer_logicals: Vec<PrinterLogical> = self.printer_logicals.values().cloned().collect();
                        let available_price_levels: Vec<PriceLevel> = self.price_levels.values().cloned().collect();

                        // We only do this once, when the user explicitly begins editing:
                        self.item_edit_state = items::EditState::new(
                            item,
                            available_choice_groups,
                            available_printer_logicals,
                            available_price_levels,
                        );

                        println!("Prices: {:?}", item.item_prices);
                        self.screen = Screen::Items(items::Mode::Edit);
                        Task::none()
                    }
                    items::Operation::Cancel => {
                        if self.draft_item_id.is_some() {
                            self.draft_item_id = None;
                            self.draft_item = Item::default();
                        }
                        self.screen = Screen::Items(items::Mode::View);
                        Task::none()
                    }
                    items::Operation::Back => {
                        self.screen = Screen::Items(items::Mode::View);
                        Task::none()
                    }
                    items::Operation::CreateNew(mut item) => {
                        let next_id = self.items
                            .keys()
                            .max()
                            .map_or(1, |max_id| max_id + 1);
                        item.id = next_id;

                        self.items.insert(next_id, item.clone());
                        self.draft_item = item;
                        self.draft_item_id = Some(next_id);
                        self.selected_item_id = Some(next_id);
                        self.screen = Screen::Items(items::Mode::Edit);
                        Task::none()
                    },
                    items::Operation::Select(id) => {
                        let test = self.items.get(&id).unwrap();
                        self.selected_item_id = Some(id);
                        self.screen = Screen::Items(items::Mode::View);
                        items::export_items::item_to_export_string(test);
                        Task::none()
                    },
                    items::Operation::UpdateSearchQuery(query) => {
                        self.item_search = query;
                        Task::none()
                    }
                     items::Operation::RequestDelete(id) => {
                        println!("Deleting Item id: {}", id);
                        self.deletion_info = data_types::DeletionInfo { 
                            entity_type: "Item".to_string(),
                            entity_id: id,
                            affected_items: Vec::new()
                        };
                        self.show_modal = true;
                        Task::none()
                    }
                    items::Operation::CopyItem(id) => {
                        println!("Copying Item: {}", id);
                        let copy_item = self.items.get(&id).unwrap();
                        let next_id = self.items
                            .keys()
                            .max()
                            .map_or(1, |max_id| max_id + 1);
                        
                        let new_item = Item {
                            id: next_id,
                            name: copy_item.name.clone() + "(" + next_id.to_string().as_str() + ")",
                            ..copy_item.clone()
                        };

                        self.items.insert(next_id, new_item.clone());
                        self.draft_item_id = Some(next_id);
                        self.draft_item = new_item;
                        self.selected_item_id = Some(next_id);
                        self.screen = Screen::Items(items::Mode::Edit);

                        Task::none()
                    }
                    items::Operation::HideModal => {
                        self.show_modal = false;
                        Task::none()
                    }
                    items::Operation::ShowModal => {
                        self.show_modal = true;
                        Task::none()
                    }
                    items::Operation::UpdatePrice(item_id, price_level_id, item_price) => {
                        // Use a mutable reference instead of cloning:
                        let state = &mut self.item_edit_state;
                
                        // Update the persistent edit state directly:
                        if let Some(prices) = &mut state.prices {
                            if let Some((_, price_str)) = prices.iter_mut().find(|(id, _)| *id == price_level_id) {
                                *price_str = item_price;
                            } else {
                                prices.push((price_level_id, item_price));
                            }
                        } else {
                            state.prices = Some(vec![(price_level_id, item_price)]);
                        }

                        Task::none()
                    }
                }
            } 
            Operation::ItemGroups(id, op) => {
                match op {
                    item_groups::Operation::RequestDelete(id) => {
                        self.deletion_info = data_types::DeletionInfo { 
                            entity_type: "ItemGroup".to_string(),
                            entity_id: id,
                            affected_items: Vec::new()
                        };
                        self.show_modal = true;
                        Task::none()
                    }
                    item_groups::Operation::CopyItemGroup(id) => {
                        let copy_item = self.item_groups.get(&id).unwrap();
                        let next_id = self.item_groups
                            .keys()
                            .max()
                            .map_or(1, |max_id| max_id + 1);
                        
                        let new_item = ItemGroup {
                            id: next_id,
                            name: copy_item.name.clone() + "(" + next_id.to_string().as_str() + ")",
                            ..copy_item.clone()
                        };

                        self.item_groups.insert(next_id, new_item.clone());
                        self.screen = Screen::ItemGroups;

                        Task::none()
                    }
                    item_groups::Operation::EditItemGroup(id) => {
                        // First check if we already have an edit state for this item_group
                        let already_editing = self.item_group_edit_state_vec
                            .iter()
                            .any(|state| state.base.id.parse::<i32>().unwrap() == id);

                        // Only create new edit state if we're not already editing this item_group
                        if !already_editing {
                            if let Some(item_group) = self.item_groups.get(&id) {
                                let edit_state = item_groups::ItemGroupEditState::new(&item_group);
                                
                                self.item_group_edit_state_vec.push(edit_state);
                            }
                        }

                        self.screen = Screen::ItemGroups;
                        Task::none()
                    },
                    item_groups::Operation::Save(id, edit_state) => {
                        // First, find the edit state for this item_group
                        if let Some(edit_state) = self.item_group_edit_state_vec
                            .iter()
                            .find(|state| state.base.id.parse::<i32>().unwrap() == id)
                        {
                            // Parse range values
                            if let (Ok(start), Ok(end)) = (
                                edit_state.id_range_start.parse::<i32>(), 
                                edit_state.id_range_end.parse::<i32>()
                            ) {
                                // Create a temporary ItemGroup with the new values for validation
                                let updated_group = item_groups::ItemGroup {
                                    id: id,
                                    name: edit_state.base.name.clone(),
                                    id_range: Range {
                                        start: start,
                                        end: end
                                    }
                                };
                                
                                // Get a list of other groups for validation
                                let other_groups: Vec<&item_groups::ItemGroup> = self.item_groups.values()
                                    .filter(|g| g.id != id)  // Exclude the current group
                                    .collect();
                                
                                // Validate the updated group
                                match updated_group.validate(&other_groups) {
                                    Ok(()) => {
                                        // Validation passed, update the item_group
                                        if let Some(item_group) = self.item_groups.get_mut(&id) {
                                            item_group.name = edit_state.base.name.clone();
                                            item_group.id_range = Range {
                                                start: start,
                                                end: end
                                            };
                                        }
                                        
                                        // Remove the edit state
                                        self.item_group_edit_state_vec.retain(|edit| {
                                            edit.base.id.parse::<i32>().unwrap() != id
                                        });
                                    },
                                    Err(error) => {
                                        // Validation failed, update the edit state with the error
                                        if let Some(edit_state) = self.item_group_edit_state_vec
                                            .iter_mut()
                                            .find(|state| state.base.id.parse::<i32>().unwrap() == id)
                                        {
                                            match error {
                                                ValidationError::InvalidId(msg) | 
                                                ValidationError::DuplicateId(msg) => {
                                                    edit_state.base.id_validation_error = Some(msg);
                                                },
                                                ValidationError::EmptyName(msg) | 
                                                ValidationError::NameTooLong(msg) => {
                                                    edit_state.base.name_validation_error = Some(msg);
                                                },
                                                ValidationError::RangeOverlap(msg) |
                                                ValidationError::InvalidValue(msg) => {
                                                    edit_state.range_validation_error = Some(msg);
                                                },
                                                _ => {
                                                    // Fall back for other validation errors
                                                    edit_state.range_validation_error = Some(format!("{:?}", error));
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                // Invalid range format
                                if let Some(edit_state) = self.item_group_edit_state_vec
                                    .iter_mut()
                                    .find(|state| state.base.id.parse::<i32>().unwrap() == id)
                                {
                                    edit_state.range_validation_error = Some("Invalid range format".to_string());
                                }
                            }
                        }

                        self.save_state().expect("Failed to save to file.");
                        self.screen = Screen::ItemGroups;
                        Task::none()
                    },
                    item_groups::Operation::UpdateName(id, new_name) => {
                        if let Some(edit_state) = self.item_group_edit_state_vec
                        .iter_mut()
                        .find(|state| state.base.id.parse::<i32>().unwrap() == id) 
                        { 
                            //check if name var is less than 17 characters
                            if new_name.len() < 17 {
                                // Update the name
                                edit_state.base.name = new_name;
                            } else {
                                //set validation error message if it's longer than 16 characters
                                edit_state.base.name_validation_error = Some("Must be less than 16 characters".to_string());
                            }
                        }
    
                        self.screen = Screen::ItemGroups;
                        Task::none()
                    },
                    item_groups::Operation::UpdateIdRangeStart(id, new_range) => {
                        if let Some(edit_state) = self.item_group_edit_state_vec
                        .iter_mut()
                        .find(|state| state.base.id.parse::<i32>().unwrap() == id) 
                        { // Update the range start
                            edit_state.id_range_start = new_range;
                        }
    
                        self.screen = Screen::ItemGroups;
                        Task::none()
                    },
                    item_groups::Operation::UpdateIdRangeEnd(id, new_range) => {
                        if let Some(edit_state) = self.item_group_edit_state_vec
                        .iter_mut()
                        .find(|state| state.base.id.parse::<i32>().unwrap() == id) 
                        { // Update the range end
                            edit_state.id_range_end = new_range;
                        }
    
                        self.screen = Screen::ItemGroups;
                        Task::none()
                    },
                    item_groups::Operation::CreateNew => {
                        let next_id = self.item_groups
                            .keys()
                            .max()
                            .map_or(1, |max_id| max_id + 1);

                        //Create a new ItemGroup
                        let item_group = ItemGroup {
                            id: next_id,
                            id_range: Range { 
                                start: 0, 
                                end: 0 
                                },
                            name: String::new()
                        };

                        //Add new ItemGroup to the app state
                        self.item_groups.insert(next_id, item_group.clone());

                        //Create a new edit_state for the new item_group
                        let edit_state = item_groups::ItemGroupEditState::new(&item_group);
                        
                        //Add new item_group edit_state to app state
                        self.item_group_edit_state_vec.push(edit_state);

                        Task::none()
                    },
                    item_groups::Operation::CancelEdit(id) => {
                        let item_group = self.item_groups.get(&id).expect("I created an editstate without an item_group?");

                        // Find the edit state and reset it before removing
                        if let Some(edit_state) = self.item_group_edit_state_vec
                        .iter_mut()
                        .find(|state| state.base.id.parse::<i32>().unwrap() == id) 
                        { // If all editable fields are blank, and the ItemGroup never had a name saved, delete ItemGroup without confirmation.
                            if (    !(edit_state.base.name.len()        > 0) 
                                ||  !(edit_state.id_range_end.len()     > 0) 
                                ||  !(edit_state.id_range_start.len()   > 0)) 
                                &&  item_group.name.len() < 1
                            {
                                self.item_groups.remove(&id);
                            }

                            // Reset the data to original values if needed
                            edit_state.reset();
                        }

                        // Remove the edit state from the vec
                        self.item_group_edit_state_vec.retain(|state| {
                        state.base.id.parse::<i32>().unwrap() != id
                        });

                        self.screen = Screen::ItemGroups;
                        Task::none()
                    },
                    
                }
            }
            Operation::TaxGroups(id, op) => {
                match op {
                    tax_groups::Operation::RequestDelete(id) => {
                        self.deletion_info = data_types::DeletionInfo { 
                           entity_type: "TaxGroup".to_string(),
                           entity_id: id,
                           affected_items: Vec::new()
                       };
                        self.show_modal = true;
                       Task::none()
                   }
                    tax_groups::Operation::CopyTaxGroup(id) => {
                        let copy_item = self.tax_groups.get(&id).unwrap();
                        let next_id = self.tax_groups
                            .keys()
                            .max()
                            .map_or(1, |max_id| max_id + 1);
                       
                        let new_item = TaxGroup {
                            id: next_id,
                            name: copy_item.name.clone() + "(" + next_id.to_string().as_str() + ")",
                            ..copy_item.clone()
                        };

                       self.tax_groups.insert(next_id, new_item.clone());
                       self.screen = Screen::TaxGroups;

                       Task::none()
                   }
                    tax_groups::Operation::EditTaxGroup(id) => {
                    // First check if we already have an edit state for this tax_group
                    let already_editing = self.tax_group_edit_state_vec
                        .iter()
                        .any(|state| state.base.id.parse::<i32>().unwrap() == id);

                    // Only create new edit state if we're not already editing this tax_group
                    if !already_editing {
                        if let Some(tax_group) = self.tax_groups.get(&id) {
                            let edit_state = tax_groups::TaxGroupEditState::new(&tax_group);
                            
                            self.tax_group_edit_state_vec.push(edit_state);
                        }
                    }

                    self.screen = Screen::TaxGroups;
                    Task::none()
                    },
                    tax_groups::Operation::SaveAll(id, edit_state) => {

                        // First, find the edit state for this tax_group
                        if let Some(edit_state) = self.tax_group_edit_state_vec
                            .iter()
                            .find(|state| state.base.id.parse::<i32>().unwrap() == id)
                        {
                            // Clone the edit state name since we'll need it after removing the edit state
                            let new_name = edit_state.base.name.clone();
                            let new_rate = edit_state.rate.clone();
                            
                            // Get a mutable reference to the tax_group and update it
                            if let Some(tax_group) = self.tax_groups.get_mut(&id) {
                                tax_group.name = new_name;
                                tax_group.rate = data_types::string_to_decimal(&new_rate)
                                    .expect("Rate should be validated before message is triggered");
                            }
                        }

                        self.tax_group_edit_state_vec.retain(|edit| {
                            edit.base.id.parse::<i32>().unwrap() != id
                        });

                        self.save_state().expect("Failed to save to file.");
                        self.screen = Screen::TaxGroups;
                        Task::none()
                    },
                    tax_groups::Operation::UpdateName(id, new_name) => {
                        if let Some(edit_state) = self.tax_group_edit_state_vec
                        .iter_mut()
                        .find(|state| state.base.id.parse::<i32>().unwrap() == id) 
                        { 
                            //check if name var is less than 17 characters
                            if new_name.len() < 17 {
                                // Update the name
                                edit_state.base.name = new_name;
                            } else {
                                //set validation error message if it's longer than 16 characters
                                edit_state.base.name_validation_error = Some("Must be less than 16 characters".to_string());
                            }
                        }
    
                        self.screen = Screen::TaxGroups;
                        Task::none()
                    },
                    tax_groups::Operation::UpdateTaxRate(id, new_rate) => {
                        if let Some(edit_state) = self.tax_group_edit_state_vec
                        .iter_mut()
                        .find(|state| state.base.id.parse::<i32>().unwrap() == id) 
                        { 
                            // Update the tax_rate
                            edit_state.rate = new_rate;
                        }
    
                        self.screen = Screen::TaxGroups;
                        Task::none()
                    },
                    tax_groups::Operation::CreateNew => {
                        let next_id = self.tax_groups
                            .keys()
                            .max()
                            .map_or(1, |max_id| max_id + 1);

                        //Create a new TaxGroup
                        let tax_group = TaxGroup {
                            id: next_id,
                            name: String::new(),
                            rate: Decimal::new( 000, 2),
                        };

                        //Add new TaxGroup to the app state
                        self.tax_groups.insert(next_id, tax_group.clone());

                        //Create a new edit_state for the new choice_group
                        let edit_state = tax_groups::TaxGroupEditState::new(&tax_group);
                        
                        //Add new choice_group edit_state to app state
                        self.tax_group_edit_state_vec.push(edit_state);

                        Task::none()
                    },
                    tax_groups::Operation::CancelEdit(id) => {
                        let tax_group = self.tax_groups.get(&id).expect("I created and editstate without a TaxGroup?");


                        // Find the edit state and reset it before removing
                        if let Some(edit_state) = self.tax_group_edit_state_vec
                        .iter_mut()
                        .find(|state| state.base.id.parse::<i32>().unwrap() == id) 
                        {
                            if (    !(edit_state.base.name.len()    > 0)
                                ||  !(edit_state.rate.len()         > 0 ))
                                && tax_group.name.len() < 1 
                            {
                                self.tax_groups.remove(&id);
                            }

                            // Reset the data to original values if needed
                            edit_state.reset();
                        }

                        // Remove the edit state from the vec
                        self.tax_group_edit_state_vec.retain(|state| {
                        state.base.id.parse::<i32>().unwrap() != id
                        });

                        self.screen = Screen::TaxGroups;
                        Task::none()
                    },
                }
            }    
            Operation::SecurityLevels(id, op) => {
                match op {
                    security_levels::Operation::RequestDelete(id) => {
                        self.deletion_info = data_types::DeletionInfo { 
                           entity_type: "SecurityLevel".to_string(),
                           entity_id: id,
                           affected_items: Vec::new()
                       };
                        self.show_modal = true;
                       Task::none()
                   }
                    security_levels::Operation::CopySecurityLevel(id) => {
                        let copy_item = self.security_levels.get(&id).unwrap();
                       let next_id = self.security_levels
                           .keys()
                           .max()
                           .map_or(1, |max_id| max_id + 1);
                       
                       let new_item = SecurityLevel {
                           id: next_id,
                           name: copy_item.name.clone() + "(" + next_id.to_string().as_str() + ")",
                           ..copy_item.clone()
                       };

                       self.security_levels.insert(next_id, new_item.clone());
                       self.screen = Screen::SecurityLevels;

                       Task::none()
                   }
                    security_levels::Operation::EditSecurityLevel(id) => {
                        // First check if we already have an edit state for this security_level
                        let already_editing = self.security_level_edit_state_vec
                            .iter()
                            .any(|state| state.id.parse::<i32>().unwrap() == id);
    
                        // Only create new edit state if we're not already editing this security_level
                        if !already_editing {
                            if let Some(security_level) = self.security_levels.get(&id) {
                                let edit_state = entity_component::EditState {
                                    name: security_level.name.clone(),
                                    original_name: security_level.name.clone(),
                                    id: security_level.id.to_string(),
                                    id_validation_error: None,
                                    name_validation_error: None,
                                };
                                
                                self.security_level_edit_state_vec.push(edit_state);
                            }
                        }
    
                        self.screen = Screen::SecurityLevels;
                        Task::none()
                    },
                    security_levels::Operation::SaveAll(id, edit_state) => {
                        // First, find the edit state for this security_level
                        if let Some(edit_state) = self.security_level_edit_state_vec
                            .iter()
                            .find(|state| state.id.parse::<i32>().unwrap() == id)
                        {
                            // Clone the edit state name since we'll need it after removing the edit state
                            let new_name = edit_state.name.clone();
                            
                            // Get a mutable reference to the security_level and update it
                            if let Some(security_group) = self.security_levels.get_mut(&id) {
                                security_group.name = new_name;
                            }
                        }

                        self.security_level_edit_state_vec.retain(|edit| {
                            edit.id.parse::<i32>().unwrap() != id
                        });

                        self.save_state().expect("Failed to save to file.");
                        self.screen = Screen::SecurityLevels;
                        Task::none()
                    },
                    security_levels::Operation::UpdateName(id, new_name) => {
                        if let Some(edit_state) = self.security_level_edit_state_vec
                        .iter_mut()
                        .find(|state| state.id.parse::<i32>().unwrap() == id) 
                        { 
                            //check if name var is less than 17 characters
                            if new_name.len() < 17 {
                                // Update the name
                                edit_state.name = new_name;
                            } else {
                                //set validation error message if it's longer than 16 characters
                                edit_state.name_validation_error = Some("Must be less than 16 characters".to_string());
                            }
                        }
    
                        self.screen = Screen::SecurityLevels;
                        Task::none()
                    },
                    security_levels::Operation::CreateNew => {
                        let next_id = self.security_levels
                            .keys()
                            .max()
                            .map_or(1, |max_id| max_id + 1);

                        //Create a new SecurityLevel
                        let security_level = SecurityLevel {
                            id: next_id,
                            name: String::new()
                        };

                        //Add new SecurityLevel to the app state
                        self.security_levels.insert(next_id, security_level.clone());

                        //Create a new edit_state for the new security_level
                        let edit_state = entity_component::EditState {
                            name: security_level.name.clone(),
                            original_name: security_level.name.clone(),
                            id: security_level.id.to_string(),
                            id_validation_error: None,
                            name_validation_error: None,
                        };
                        
                        //Add new security_level edit_state to app state
                        self.security_level_edit_state_vec.push(edit_state);

                        Task::none()
                    },
                    security_levels::Operation::CancelEdit(id) => {
                        let security_level = self.security_levels.get(&id).expect("I created an editstate without a SecurityLevel?");

                        // Find the edit state and reset it before removing
                        if let Some(edit_state) = self.security_level_edit_state_vec
                        .iter_mut()
                        .find(|state| state.id.parse::<i32>().unwrap() == id) 
                        {
                            if security_level.name.len() < 1 
                            {
                                self.security_levels.remove(&id);
                            }
                            // Reset the data to original values if needed
                            edit_state.reset();
                        }

                        // Remove the edit state from the vec
                        self.security_level_edit_state_vec.retain(|state| {
                        state.id.parse::<i32>().unwrap() != id
                        });

                        self.screen = Screen::SecurityLevels;
                        Task::none()
                    },
                }
            }    
            Operation::RevenueCategories(id, op) => {
                match op {
                    revenue_categories::Operation::RequestDelete(id) => {
                        self.deletion_info = data_types::DeletionInfo { 
                           entity_type: "RevenueCategory".to_string(),
                           entity_id: id,
                           affected_items: Vec::new()
                       };
                        self.show_modal = true;
                       Task::none()
                   }
                    revenue_categories::Operation::CopyRevenueCategory(id) => {
                        let copy_item = self.revenue_categories.get(&id).unwrap();
                       let next_id = self.revenue_categories
                           .keys()
                           .max()
                           .map_or(1, |max_id| max_id + 1);
                       
                       let new_item = RevenueCategory {
                           id: next_id,
                           name: copy_item.name.clone() + "(" + next_id.to_string().as_str() + ")",
                           ..copy_item.clone()
                       };

                       self.revenue_categories.insert(next_id, new_item.clone());
                       self.screen = Screen::RevenueCategories;

                       Task::none()
                   }
                   revenue_categories::Operation::EditRevenueCategory(id) => {
                    // First check if we already have an edit state for this revenue_category
                    let already_editing = self.revenue_category_edit_state_vec
                        .iter()
                        .any(|state| state.id.parse::<i32>().unwrap() == id);

                    // Only create new edit state if we're not already editing this revenue_category
                    if !already_editing {
                        if let Some(revenue_category) = self.revenue_categories.get(&id) {
                            let edit_state = entity_component::EditState {
                                name: revenue_category.name.clone(),
                                original_name: revenue_category.name.clone(),
                                id: revenue_category.id.to_string(),
                                id_validation_error: None,
                                name_validation_error: None,
                            };
                            
                            self.revenue_category_edit_state_vec.push(edit_state);
                        }
                    }

                    self.screen = Screen::RevenueCategories;
                    Task::none()
                   },
                    revenue_categories::Operation::SaveAll(id, edit_state) => {
                        // First, find the edit state for this revenue_category
                        if let Some(edit_state) = self.revenue_category_edit_state_vec
                        .iter()
                        .find(|state| state.id.parse::<i32>().unwrap() == id)
                        {
                        // Clone the edit state name since we'll need it after removing the edit state
                        let new_name = edit_state.name.clone();

                        // Get a mutable reference to the revenue_category and update it
                        if let Some(revenue_category) = self.revenue_categories.get_mut(&id) {
                            revenue_category.name = new_name;
                        }
                        }

                        self.revenue_category_edit_state_vec.retain(|edit| {
                        edit.id.parse::<i32>().unwrap() != id
                        });

                        self.save_state().expect("Failed to save to file.");
                        self.screen = Screen::RevenueCategories;
                        Task::none()
                    },
                    revenue_categories::Operation::UpdateName(id, new_name) => {
                        if let Some(edit_state) = self.revenue_category_edit_state_vec
                        .iter_mut()
                        .find(|state| state.id.parse::<i32>().unwrap() == id) 
                        { 
                            //check if name var is less than 17 characters
                            if new_name.len() < 17 {
                                // Update the name
                                edit_state.name = new_name;
                            } else {
                                //set validation error message if it's longer than 16 characters
                                edit_state.name_validation_error = Some("Must be less than 16 characters".to_string());
                            }
                        }
    
                        self.screen = Screen::RevenueCategories;
                        Task::none()
                    },
                    revenue_categories::Operation::CreateNew => {
                        let next_id = self.revenue_categories
                            .keys()
                            .max()
                            .map_or(1, |max_id| max_id + 1);

                        //Create a new RevenueCategory
                        let revenue_category = RevenueCategory {
                            id: next_id,
                            name: String::new()
                        };

                        //Add new RevenueCategory to the app state
                        self.revenue_categories.insert(next_id, revenue_category.clone());

                        //Create a new edit_state for the new revenue_category
                        let edit_state = entity_component::EditState {
                            name: revenue_category.name.clone(),
                            original_name: revenue_category.name.clone(),
                            id: revenue_category.id.to_string(),
                            id_validation_error: None,
                            name_validation_error: None,
                        };
                        
                        //Add new revenue_category edit_state to app state
                        self.revenue_category_edit_state_vec.push(edit_state);

                        Task::none()
                    },
                    revenue_categories::Operation::CancelEdit(id) => {
                        let revenue_category = self.revenue_categories.get(&id).expect("I created an editstate without a RevenueCategory?");

                        // Find the edit state and reset it before removing
                        if let Some(edit_state) = self.revenue_category_edit_state_vec
                        .iter_mut()
                        .find(|state| state.id.parse::<i32>().unwrap() == id) 
                        {
                            if revenue_category.name.len() < 1 
                            {
                                self.revenue_categories.remove(&id);
                            }

                            // Reset the data to original values if needed
                            edit_state.reset();
                        }

                        // Remove the edit state from the vec
                        self.revenue_category_edit_state_vec.retain(|state| {
                        state.id.parse::<i32>().unwrap() != id
                        });

                        self.screen = Screen::RevenueCategories;
                        Task::none()
                    },
                }
            }    
            Operation::ReportCategories(id, op) => {
                match op {
                    report_categories::Operation::RequestDelete(id) => {
                        self.deletion_info = data_types::DeletionInfo { 
                           entity_type: "ReportCategory".to_string(),
                           entity_id: id,
                           affected_items: Vec::new()
                        };
                        self.show_modal = true;
                        Task::none()
                   }
                    report_categories::Operation::CopyReportCategory(id) => {
                        let copy_item = self.report_categories.get(&id).unwrap();
                        let next_id = self.report_categories
                            .keys()
                            .max()
                            .map_or(1, |max_id| max_id + 1);
                       
                        let new_item = ReportCategory {
                            id: next_id,
                            name: copy_item.name.clone() + "(" + next_id.to_string().as_str() + ")",
                            ..copy_item.clone()
                        };

                        self.report_categories.insert(next_id, new_item.clone());
                        self.screen = Screen::ReportCategories;

                        Task::none()
                   }
                    report_categories::Operation::EditReportCategory(id) => {
                        // First check if we already have an edit state for this report_category
                        let already_editing = self.report_category_edit_state_vec
                            .iter()
                            .any(|state| state.id.parse::<i32>().unwrap() == id);

                        // Only create new edit state if we're not already editing this report_category
                        if !already_editing {
                            if let Some(report_category) = self.report_categories.get(&id) {
                                let edit_state = entity_component::EditState {
                                    name: report_category.name.clone(),
                                    original_name: report_category.name.clone(),
                                    id: report_category.id.to_string(),
                                    id_validation_error: None,
                                    name_validation_error: None,
                                };
                                
                                self.report_category_edit_state_vec.push(edit_state);
                            }
                        }

                        self.screen = Screen::ReportCategories;
                        Task::none()
                    },
                    report_categories::Operation::SaveAll(id, edit_state) => {
                        // First, find the edit state for this report_category
                        if let Some(edit_state) = self.report_category_edit_state_vec
                            .iter()
                            .find(|state| state.id.parse::<i32>().unwrap() == id)
                        {
                            // Clone the edit state name since we'll need it after removing the edit state
                            let new_name = edit_state.name.clone();
                        
                            // Get a mutable reference to the report_category and update it
                            if let Some(report_category) = self.report_categories.get_mut(&id) {
                                report_category.name = new_name;
                            }
                        }

                        self.report_category_edit_state_vec.retain(|edit| {
                            edit.id.parse::<i32>().unwrap() != id
                        });

                        self.save_state().expect("Failed to save to file.");
                        self.screen = Screen::ReportCategories;
                        Task::none()
                    },
                    report_categories::Operation::UpdateName(id, new_name) => {
                        if let Some(edit_state) = self.report_category_edit_state_vec
                        .iter_mut()
                        .find(|state| state.id.parse::<i32>().unwrap() == id) 
                        { 
                            //check if name var is less than 17 characters
                            if new_name.len() < 17 {
                                // Update the name
                                edit_state.name = new_name;
                            } else {
                                //set validation error message if it's longer than 16 characters
                                edit_state.name_validation_error = Some("Must be less than 16 characters".to_string());
                            }
                        }
    
                        self.screen = Screen::ReportCategories;
                        Task::none()
                    },
                    report_categories::Operation::CreateNew => {
                        let next_id = self.report_categories
                            .keys()
                            .max()
                            .map_or(1, |max_id| max_id + 1);

                        //Create a new ReportCategory
                        let report_category = ReportCategory {
                            id: next_id,
                            name: String::new()
                        };

                        //Add new ReportCategory to the app state
                        self.report_categories.insert(next_id, report_category.clone());

                        //Create a new edit_state for the new report_category
                        let edit_state = entity_component::EditState {
                            name: report_category.name.clone(),
                            original_name: report_category.name.clone(),
                            id: report_category.id.to_string(),
                            id_validation_error: None,
                            name_validation_error: None,
                        };
                        
                        //Add new report_category edit_state to app state
                        self.report_category_edit_state_vec.push(edit_state);

                        Task::none()
                    },
                    report_categories::Operation::CancelEdit(id) => {
                        let report_category = self.report_categories.get(&id).expect("I created an editstate without a ReportCategory?");

                        // Find the edit state and reset it before removing
                        if let Some(edit_state) = self.report_category_edit_state_vec
                        .iter_mut()
                        .find(|state| state.id.parse::<i32>().unwrap() == id) 
                        {
                            if report_category.name.len() < 1 
                            {
                                self.report_categories.remove(&id);
                            }

                            // Reset the data to original values if needed
                            edit_state.reset();
                        }

                        // Remove the edit state from the vec
                        self.report_category_edit_state_vec.retain(|state| {
                        state.id.parse::<i32>().unwrap() != id
                        });

                        self.screen = Screen::ReportCategories;
                        Task::none()
                    },
                }
            }    
            Operation::ProductClasses(id, op) => {
                match op {
                    product_classes::Operation::RequestDelete(id) => {
                        self.deletion_info = data_types::DeletionInfo { 
                           entity_type: "ProductClass".to_string(),
                           entity_id: id,
                           affected_items: Vec::new()
                        };
                        self.show_modal = true;
                        Task::none()
                   }
                    product_classes::Operation::CopyProductClass(id) => {
                        let copy_item = self.product_classes.get(&id).unwrap();
                        let next_id = self.product_classes
                            .keys()
                            .max()
                            .map_or(1, |max_id| max_id + 1);
                       
                        let new_item = ProductClass {
                            id: next_id,
                            name: copy_item.name.clone() + "(" + next_id.to_string().as_str() + ")",
                            ..copy_item.clone()
                        };

                       self.product_classes.insert(next_id, new_item.clone());
                       self.screen = Screen::ProductClasses;

                       Task::none()
                   }
                    product_classes::Operation::EditProductClass(id) => {
                        // First check if we already have an edit state for this product_class
                        let already_editing = self.product_class_edit_state_vec
                            .iter()
                            .any(|state| state.id.parse::<i32>().unwrap() == id);
    
                        // Only create new edit state if we're not already editing this product_class
                        if !already_editing {
                            if let Some(product_class) = self.product_classes.get(&id) {
                                let edit_state = entity_component::EditState {
                                    name: product_class.name.clone(),
                                    original_name: product_class.name.clone(),
                                    id: product_class.id.to_string(),
                                    id_validation_error: None,
                                    name_validation_error: None,
                                };
                                
                                self.product_class_edit_state_vec.push(edit_state);
                            }
                        }
    
                        self.screen = Screen::ProductClasses;
                        Task::none()
                    },
                    product_classes::Operation::SaveAll(id, edit_state) => {
                        // First, find the edit state for this product_class
                        if let Some(edit_state) = self.product_class_edit_state_vec
                            .iter()
                            .find(|state| state.id.parse::<i32>().unwrap() == id)
                        {
                            // Clone the edit state name since we'll need it after removing the edit state
                            let new_name = edit_state.name.clone();
                            
                            // Get a mutable reference to the product_class and update it
                            if let Some(product_class) = self.product_classes.get_mut(&id) {
                                product_class.name = new_name;
                            }
                        }

                        self.product_class_edit_state_vec.retain(|edit| {
                            edit.id.parse::<i32>().unwrap() != id
                        });

                        self.save_state().expect("Failed to save to file.");
                        self.screen = Screen::ProductClasses;
                        Task::none()
                    },
                    product_classes::Operation::UpdateName(id, new_name) => {
                        if let Some(edit_state) = self.product_class_edit_state_vec
                        .iter_mut()
                        .find(|state| state.id.parse::<i32>().unwrap() == id) 
                        { 
                            //check if name var is less than 17 characters
                            if new_name.len() < 17 {
                                // Update the name
                                edit_state.name = new_name;
                            } else {
                                //set validation error message if it's longer than 16 characters
                                edit_state.name_validation_error = Some("Must be less than 16 characters".to_string());
                            }
                        }
    
                        self.screen = Screen::ProductClasses;
                        Task::none()
                    },
                    product_classes::Operation::CreateNew => {
                        let next_id = self.product_classes
                            .keys()
                            .max()
                            .map_or(1, |max_id| max_id + 1);

                        //Create a new ProductClass
                        let product_class = ProductClass {
                            id: next_id,
                            name: String::new()
                        };

                        //Add new ProductClass to the app state
                        self.product_classes.insert(next_id, product_class.clone());

                        //Create a new edit_state for the new product_class
                        let edit_state = entity_component::EditState {
                            name: product_class.name.clone(),
                            original_name: product_class.name.clone(),
                            id: product_class.id.to_string(),
                            id_validation_error: None,
                            name_validation_error: None,
                        };
                        
                        //Add new product_class edit_state to app state
                        self.product_class_edit_state_vec.push(edit_state);

                        Task::none()
                    },
                    product_classes::Operation::CancelEdit(id) => {
                        let product_class = self.product_classes.get(&id).expect("I created an editstate without a ProductClass?");

                        // Find the edit state and reset it before removing
                        if let Some(edit_state) = self.product_class_edit_state_vec
                        .iter_mut()
                        .find(|state| state.id.parse::<i32>().unwrap() == id) 
                        {
                            if product_class.name.len() < 1 
                            {
                                self.product_classes.remove(&id);
                            }

                            // Reset the data to original values if needed
                            edit_state.reset();
                        }

                        // Remove the edit state from the vec
                        self.product_class_edit_state_vec.retain(|state| {
                        state.id.parse::<i32>().unwrap() != id
                        });

                        self.screen = Screen::ProductClasses;
                        Task::none()
                    },
                }
            }    
            Operation::ChoiceGroups(id, op) => match op {
                choice_groups::Operation::RequestDelete(id) => {

                    self.deletion_info = data_types::DeletionInfo { 
                        entity_type: "ChoiceGroup".to_string(),
                        entity_id: id,
                        affected_items: Vec::new()
                    };
                     self.show_modal = true;
                    Task::none()
                },
                choice_groups::Operation::CopyChoiceGroup(id) => {
                    let copy_item = self.choice_groups.get(&id).unwrap();
                    let next_id = self.choice_groups
                        .keys()
                        .max()
                        .map_or(1, |max_id| max_id + 1);
                    
                    let new_item = ChoiceGroup {
                        id: next_id,
                        name: copy_item.name.clone() + "(" + next_id.to_string().as_str() + ")",
                        ..copy_item.clone()
                    };

                    self.choice_groups.insert(next_id, new_item.clone());
                    self.screen = Screen::ChoiceGroups;

                    Task::none()
                }
                choice_groups::Operation::EditChoiceGroup(id) => {
                    // First check if we already have an edit state for this choice_group
                    let already_editing = self.choice_group_edit_state_vec
                        .iter()
                        .any(|state| state.id.parse::<i32>().unwrap() == id);

                    // Only create new edit state if we're not already editing this choice_group
                    if !already_editing {
                        if let Some(choice_group) = self.choice_groups.get(&id) {
                            let edit_state = entity_component::EditState {
                                name: choice_group.name.clone(),
                                original_name: choice_group.name.clone(),
                                id: choice_group.id.to_string(),
                                id_validation_error: None,
                                name_validation_error: None,
                            };
                            
                            self.choice_group_edit_state_vec.push(edit_state);
                        }
                    }

                    self.screen = Screen::ChoiceGroups;
                    Task::none()

                },
                choice_groups::Operation::SaveAll(id, edit_state) => {
                    // First, find the edit state for this choice_group
                    if let Some(edit_state) = self.choice_group_edit_state_vec
                        .iter()
                        .find(|state| state.id.parse::<i32>().unwrap() == id)
                    {
                        // Clone the edit state name since we'll need it after removing the edit state
                        let new_name = edit_state.name.clone();
                        
                        // Get a mutable reference to the choice_group and update it
                        if let Some(choice_group) = self.choice_groups.get_mut(&id) {
                            choice_group.name = new_name;
                        }
                    }

                    self.choice_group_edit_state_vec.retain(|edit| {
                        edit.id.parse::<i32>().unwrap() != id
                    });

                    self.save_state().expect("Failed to save to file.");
                    self.screen = Screen::ChoiceGroups;
                    Task::none()
                },
                choice_groups::Operation::UpdateName(id, new_name) => {
                    if let Some(edit_state) = self.choice_group_edit_state_vec
                    .iter_mut()
                    .find(|state| state.id.parse::<i32>().unwrap() == id) 
                    { 
                        //check if name var is less than 17 characters
                        if new_name.len() < 17 {
                            // Update the name
                            edit_state.name = new_name;
                        } else {
                            //set validation error message if it's longer than 16 characters
                            edit_state.name_validation_error = Some("Must be less than 16 characters".to_string());
                        }
                    }

                    self.screen = Screen::ChoiceGroups;
                    Task::none()
                },
                choice_groups::Operation::CreateNew => {
                    let next_id = self.choice_groups
                        .keys()
                        .max()
                        .map_or(1, |max_id| max_id + 1);

                    //Create a new ChoiceGroup
                    let choice_group = ChoiceGroup {
                        id: next_id,
                        name: String::new()
                    };

                    //Add new ChoiceGroup to the app state
                    self.choice_groups.insert(next_id, choice_group.clone());

                    //Create a new edit_state for the new choice_group
                    let edit_state = entity_component::EditState {
                        name: choice_group.name.clone(),
                        original_name: choice_group.name.clone(),
                        id: choice_group.id.to_string(),
                        id_validation_error: None,
                        name_validation_error: None,
                    };
                    
                    //Add new choice_group edit_state to app state
                    self.choice_group_edit_state_vec.push(edit_state);

                    Task::none()
                },
                choice_groups::Operation::CancelEdit(id) => {
                    let choice_group = self.choice_groups.get(&id).expect("I created an editstate without a ChoiceGroup?");

                    // Find the edit state and reset it before removing
                    if let Some(edit_state) = self.choice_group_edit_state_vec
                    .iter_mut()
                    .find(|state| state.id.parse::<i32>().unwrap() == id) 
                    {
                        if choice_group.name.len() < 1 
                        {
                            self.choice_groups.remove(&id);
                        }

                        // Reset the data to original values if needed
                        edit_state.reset();
                    }

                    // Remove the edit state from the vec
                    self.choice_group_edit_state_vec.retain(|state| {
                    state.id.parse::<i32>().unwrap() != id
                    });

                    self.screen = Screen::ChoiceGroups;
                    Task::none()
                },
            },    
            Operation::PrinterLogicals(id, op) => match op {
                printer_logicals::Operation::RequestDelete(id) => {
                    self.deletion_info = data_types::DeletionInfo { 
                       entity_type: "PrinterLogical".to_string(),
                       entity_id: id,
                       affected_items: Vec::new()
                    };
                    
                    self.show_modal = true;
                    Task::none()
                }
                printer_logicals::Operation::CopyPrinterLogical(id) => {
                    let copy_item = self.printer_logicals.get(&id).unwrap();
                    let next_id = self.printer_logicals
                        .keys()
                        .max()
                        .map_or(1, |max_id| max_id + 1);
                   
                    let new_item = PrinterLogical {
                        id: next_id,
                        name: copy_item.name.clone() + "(" + next_id.to_string().as_str() + ")",
                        ..copy_item.clone()
                    };

                    self.printer_logicals.insert(next_id, new_item.clone());
                    self.screen = Screen::PrinterLogicals;

                    Task::none()
                }
                printer_logicals::Operation::EditPrinterLogical(id) => {
                    // First check if we already have an edit state for this printer
                    let already_editing = self.printer_logical_edit_state_vec
                        .iter()
                        .any(|state| state.id.parse::<i32>().unwrap() == id);

                    // Only create new edit state if we're not already editing this printer
                    if !already_editing {
                        if let Some(printer) = self.printer_logicals.get(&id) {
                            let edit_state = entity_component::EditState {
                                name: printer.name.clone(),
                                original_name: printer.name.clone(),
                                id: printer.id.to_string(),
                                id_validation_error: None,
                                name_validation_error: None,
                            };
                            
                            self.printer_logical_edit_state_vec.push(edit_state);
                        }
                    }

                    self.screen = Screen::PrinterLogicals;
                    Task::none()
                }
                printer_logicals::Operation::CreateNew => {
                    let next_id = self.printer_logicals
                        .keys()
                        .max()
                        .map_or(1, |max_id| max_id + 1);

                    //Create a new PrinterLogical
                    let printer = PrinterLogical {
                        id: next_id,
                        name: String::new(),
                    };

                    //Add new PrinterLogical to the app state
                    self.printer_logicals.insert(next_id, printer.clone());

                    //Create a new edit_state for the new printer
                    let edit_state = entity_component::EditState {
                        name: printer.name.clone(),
                        original_name: printer.name.clone(),
                        id: printer.id.to_string(),
                        id_validation_error: None,
                        name_validation_error: None,
                    };
                    
                    //Add new printer edit_state to app state
                    self.printer_logical_edit_state_vec.push(edit_state);

                    Task::none()
                }
                printer_logicals::Operation::Save(id, edit_state) => {

                    // First, find the edit state for this printer
                    if let Some(edit_state) = self.printer_logical_edit_state_vec
                        .iter()
                        .find(|state| state.id.parse::<i32>().unwrap() == id)
                    {
                        // Clone the edit state name since we'll need it after removing the edit state
                        let new_name = edit_state.name.clone();
                        
                        // Get a mutable reference to the printer and update it
                        if let Some(printer) = self.printer_logicals.get_mut(&id) {
                            printer.name = new_name;
                        }
                    }

                    self.printer_logical_edit_state_vec.retain(|edit| {
                        edit.id.parse::<i32>().unwrap() != id
                    });

                    self.save_state().expect("Failed to save to file.");
                    self.screen = Screen::PrinterLogicals;
                    Task::none()
                }
                printer_logicals::Operation::CancelEdit(id) => {
                    let printer_logical = self.printer_logicals.get(&id).expect("I created an editstate without a PrinterLogical?");

                    // Find the edit state and reset it before removing
                    if let Some(edit_state) = self.printer_logical_edit_state_vec
                        .iter_mut()
                        .find(|state| state.id.parse::<i32>().unwrap() == id) 
                        {
                            if printer_logical.name.len() < 1
                            {
                                self.printer_logicals.remove(&id);
                            }

                            // Reset the data to original values if needed
                            edit_state.reset();
                        }

                        // Remove the edit state from the vec
                        self.printer_logical_edit_state_vec.retain(|state| {
                        state.id.parse::<i32>().unwrap() != id
                    });

                    self.screen = Screen::PrinterLogicals;
                    Task::none()
                }
                printer_logicals::Operation::UpdateName(id, new_name) => {
                    if let Some(edit_state) = self.printer_logical_edit_state_vec
                    .iter_mut()
                    .find(|state| state.id.parse::<i32>().unwrap() == id) 
                    {
                        //check if name var is less than 17 characters
                        if new_name.len() < 17 {
                            // Update the name
                            edit_state.name = new_name;
                        } else {
                            //set validation error message if it's longer than 16 characters
                            edit_state.name_validation_error = Some("Must be less than 16 characters".to_string());
                        }

                    }

                    self.screen = Screen::PrinterLogicals;
                    Task::none()
                }
            },
            Operation::PriceLevels(id, op) => match op {
                price_levels::Operation::RequestDelete(id) => {
                    self.deletion_info = data_types::DeletionInfo { 
                       entity_type: "PriceLevel".to_string(),
                       entity_id: id,
                       affected_items: Vec::new()
                    };
                    self.show_modal = true;
                    Task::none()
               }
                price_levels::Operation::CopyPriceLevel(id) => {
                    let copy_item = self.price_levels.get(&id).unwrap();
                    let next_id = self.price_levels
                        .keys()
                        .max()
                        .map_or(1, |max_id| max_id + 1);
                   
                    let new_item = PriceLevel {
                        id: next_id,
                        name: copy_item.name.clone() + "(" + next_id.to_string().as_str() + ")",
                        ..copy_item.clone()
                    };

                   self.price_levels.insert(next_id, new_item.clone());
                   self.screen = Screen::PriceLevels;

                   Task::none()
               }
                price_levels::Operation::EditPriceLevel(id) => {
                    // First check if we already have an edit state for this price_level
                    let already_editing = self.price_level_edit_state_vec
                        .iter()
                        .any(|state| state.base.id.parse::<i32>().unwrap() == id);

                    // Only create new edit state if we're not already editing this price_level
                    if !already_editing {
                        if let Some(price_level) = self.price_levels.get(&id) {
                            let edit_state = price_levels::PriceLevelEditState::new(&price_level);
                            
                            self.price_level_edit_state_vec.push(edit_state);
                        }
                    }

                    self.screen = Screen::PriceLevels;
                    Task::none()
                },
                price_levels::Operation::SaveAll(id, edit_state) => {
                    // First, find the edit state for this price_level
                    if let Some(edit_state) = self.price_level_edit_state_vec
                        .iter()
                        .find(|state| state.base.id.parse::<i32>().unwrap() == id)
                    {
                        // Clone the edit state name since we'll need it after removing the edit state
                        let new_name = edit_state.base.name.clone();
                        
                        // Get a mutable reference to the price_level and update it
                        if let Some(price_level) = self.price_levels.get_mut(&id) {
                            price_level.name = new_name;
                        }
                    }

                    self.price_level_edit_state_vec.retain(|edit| {
                        edit.base.id.parse::<i32>().unwrap() != id
                    });

                    self.save_state().expect("Failed to save to file.");
                    self.screen = Screen::PriceLevels;
                    Task::none()
                },
                price_levels::Operation::UpdateName(id, new_name) => {

                    if let Some(edit_state) = self.price_level_edit_state_vec
                        .iter_mut()
                        .find(|state| state.base.id.parse::<i32>().unwrap() == id) 
                        { 
                            //check if name var is less than 17 characters
                            if new_name.len() < 17 {
                                // Update the name
                                edit_state.base.name = new_name;
                            } else {
                                //set validation error message if it's longer than 16 characters
                                edit_state.base.name_validation_error = Some("Must be less than 16 characters".to_string());
                            }
                        }

                    self.screen = Screen::PriceLevels;
                    Task::none()
                },
                price_levels::Operation::CreateNew => {
                    let next_id = self.price_levels
                        .keys()
                        .max()
                        .map_or(1, |max_id| max_id + 1);

                    let price_level = PriceLevel {
                        id: next_id,
                        name: String::new(),
                        level_type: PriceLevelType::Enterprise,
                        price: Decimal::new(000, 2),
                    };

                    self.price_levels.insert(next_id, price_level.clone());

                    let edit_state = price_levels::PriceLevelEditState::new(&price_level);
                    
                    self.price_level_edit_state_vec.push(edit_state);

                    Task::none()
                },
                price_levels::Operation::CancelEdit(id) => {
                    let price_level = self.price_levels.get(&id).expect("I created an editstate without a PriceLevel?");

                    // Find the edit state and reset it before removing
                    if let Some(edit_state) = self.price_level_edit_state_vec
                        .iter_mut()
                        .find(|state| state.base.id.parse::<i32>().unwrap() == id) 
                        {
                            if price_level.name.len() < 1 
                            {
                                self.price_levels.remove(&id);
                            }   
                            
                            // Reset the data to original values if needed
                            edit_state.reset();
                        }

                    // Remove the edit state from the vec
                    self.price_level_edit_state_vec.retain(|state| {
                    state.base.id.parse::<i32>().unwrap() != id
                    });

                    self.screen = Screen::PriceLevels;
                    Task::none()
                },
            },
        }
    }

    pub fn save_state(&self) -> Result<(), String> {
        //println!("Save State Triggered!");
        let state = persistence::AppState {
            items: self.items.values().cloned().collect(),
            item_groups: self.item_groups.values().cloned().collect(),
            price_levels: self.price_levels.values().cloned().collect(),
            product_classes: self.product_classes.values().cloned().collect(),
            tax_groups: self.tax_groups.values().cloned().collect(),
            security_levels: self.security_levels.values().cloned().collect(),
            revenue_categories: self.revenue_categories.values().cloned().collect(),
            report_categories: self.report_categories.values().cloned().collect(),
            choice_groups: self.choice_groups.values().cloned().collect(),
            printer_logicals: self.printer_logicals.values().cloned().collect(),
            settings: self.settings.clone(),
        };

        if self.settings.create_backups {
            self.file_manager.create_backup(std::path::Path::new(&self.settings.file_path))?;
        }

        persistence::save_to_file(&state, &self.settings.file_path)
    }

    fn handle_save_error(&mut self, error: String) {
        self.error_message = Some(error);
        // Switch to settings screen to show error
        self.screen = Screen::Settings(self.settings.clone());
    }

    pub fn load_state(&mut self) -> Result<(), String> {
        // Check if file exists
        let path = std::path::Path::new(&self.settings.file_path);
        if !path.exists() {
            println!("No saved data file found at: {}", self.settings.file_path);
            return Ok(());  // Not an error if file doesn't exist yet
        }

        let state = persistence::load_from_file(&self.settings.file_path)?;

        // Convert Vec to BTreeMap using id as key
        self.items = state.items.into_iter().map(|i| (i.id, i)).collect();
        self.item_groups = state.item_groups.into_iter().map(|i| (i.id, i)).collect();
        self.price_levels = state.price_levels.into_iter().map(|i| (i.id, i)).collect();
        self.product_classes = state.product_classes.into_iter().map(|i| (i.id, i)).collect();
        self.tax_groups = state.tax_groups.into_iter().map(|i| (i.id, i)).collect();
        self.security_levels = state.security_levels.into_iter().map(|i| (i.id, i)).collect();
        self.revenue_categories = state.revenue_categories.into_iter().map(|i| (i.id, i)).collect();
        self.report_categories = state.report_categories.into_iter().map(|i| (i.id, i)).collect();
        self.choice_groups = state.choice_groups.into_iter().map(|i| (i.id, i)).collect();
        self.printer_logicals = state.printer_logicals.into_iter().map(|i| (i.id, i)).collect();
        self.settings = state.settings.clone();

        // Only update settings if they exist in the loaded state
        if state.settings.file_path.is_empty() {
            // Keep current settings if none in file
            println!("No settings found in save file, keeping current settings");
        } else {
            self.theme =  match &state.settings.app_theme {
                settings::ThemeChoice::Light => Modern::light_theme(),
                settings::ThemeChoice::Dark => Modern::dark_theme(),
            };

            self.settings = state.settings;
        }

        Ok(())
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen_with(handle_event)
    }
}

#[derive(Debug, Clone)]
pub enum HotKey {
    Escape,
    Tab(Modifiers),
}

fn handle_event(event: event::Event, _: event::Status, _: iced::window::Id) -> Option<Message> {
    match event {
        event::Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
            match key {
                Key::Named(keyboard::key::Named::Escape) => Some(Message::HotKey(HotKey::Escape)),
                Key::Named(keyboard::key::Named::Tab) => Some(Message::HotKey(HotKey::Tab(modifiers))),
                _ => None,
            }
        }
        event::Event::Window(window::Event::FileDropped(path)) => {
            Some(Message::FileDropped(path))
        }
        event::Event::Window(window::Event::CloseRequested) => Some(Message::WindowClosed),
        event::Event::Window(window::Event::Resized(size)) => Some(Message::WindowResized(size)),
/*         event::Event::Window(window::Event::Resized(size)) => {
            Some(Message::AppResized(size))
        }, */
        _ => None,
    }
}