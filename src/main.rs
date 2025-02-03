use iced::event;
use iced::keyboard::key::Named;
use iced::keyboard::{self, Key, Modifiers};
use iced::widget::{
    focus_next, focus_previous,
    button, checkbox, column, container, pick_list,
    row, scrollable, text, text_input, vertical_space,
    Container
};
use iced::{Element, Length, Size, Subscription, Task};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::any::Any;

mod action;
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

use crate::product_classes::UpdateContext;
use data_types::{Currency, EntityId, ValidationError, ExportError};
pub use action::Action;

fn main() -> iced::Result {
    iced::application(MenuBuilder::title, MenuBuilder::update, MenuBuilder::view)
        .window_size(Size::new(900.0, 700.0))
        .theme(MenuBuilder::theme)
        .antialiasing(true)
        .centered()
        .subscription(MenuBuilder::subscription)
        .run_with(MenuBuilder::new)
}

#[derive(Debug, Clone)]
pub enum Screen {
    Items(items::Mode),
    ItemGroups(item_groups::Mode),
    PriceLevels(price_levels::Mode),
    ProductClasses(product_classes::Mode),
    TaxGroups(tax_groups::Mode),
    SecurityLevels(security_levels::Mode),
    RevenueCategories(revenue_categories::Mode),
    ReportCategories(report_categories::Mode),
    ChoiceGroups(choice_groups::Mode),
    PrinterLogicals(printer_logicals::Mode),
}

#[derive(Debug, Clone)]
pub enum Message {
    Items(items::Message),
    ItemGroups(item_groups::Message),
    PriceLevels(price_levels::Message),
    ProductClasses(product_classes::Message),
    TaxGroups(tax_groups::Message),
    SecurityLevels(security_levels::Message),
    RevenueCategories(revenue_categories::Message),
    ReportCategories(report_categories::Message),
    ChoiceGroups(choice_groups::Message),
    PrinterLogicals(printer_logicals::Message),
    Navigate(Screen),
    HotKey(HotKey),
}

#[derive(Debug)]
pub enum Operation {
    Items(items::Operation),
    ItemGroups(item_groups::Operation),
    PriceLevels(price_levels::Operation),
    ProductClasses(product_classes::Operation),
    TaxGroups(tax_groups::Operation),
    SecurityLevels(security_levels::Operation),
    RevenueCategories(revenue_categories::Operation),
    ReportCategories(report_categories::Operation),
    ChoiceGroups(choice_groups::Operation),
    PrinterLogicals(printer_logicals::Operation),
}

pub struct MenuBuilder {
    screen: Screen,
    // HashMaps for storing all entities
    items: HashMap<EntityId, items::Item>,
    item_groups: HashMap<EntityId, item_groups::ItemGroup>,
    price_levels: HashMap<EntityId, price_levels::PriceLevel>,
    product_classes: HashMap<EntityId, product_classes::ProductClass>,
    tax_groups: HashMap<EntityId, tax_groups::TaxGroup>,
    security_levels: HashMap<EntityId, security_levels::SecurityLevel>,
    revenue_categories: HashMap<EntityId, revenue_categories::RevenueCategory>,
    report_categories: HashMap<EntityId, report_categories::ReportCategory>,
    choice_groups: HashMap<EntityId, choice_groups::ChoiceGroup>,
    printer_logicals: HashMap<EntityId, printer_logicals::PrinterLogical>,

    // Draft items for editing (Option<EntityId> for new vs existing items)
    draft_item: Option<(Option<EntityId>, items::Item)>,
    draft_item_group: Option<(Option<EntityId>, item_groups::ItemGroup)>,
    draft_price_level: Option<(Option<EntityId>, price_levels::PriceLevel)>,
    draft_product_class: Option<(Option<EntityId>, product_classes::ProductClass)>,
    draft_tax_group: Option<(Option<EntityId>, tax_groups::TaxGroup)>,
    draft_security_level: Option<(Option<EntityId>, security_levels::SecurityLevel)>,
    draft_revenue_category: Option<(Option<EntityId>, revenue_categories::RevenueCategory)>,
    draft_report_category: Option<(Option<EntityId>, report_categories::ReportCategory)>,
    draft_choice_group: Option<(Option<EntityId>, choice_groups::ChoiceGroup)>,
    draft_printer_logical: Option<(Option<EntityId>, printer_logicals::PrinterLogical)>,

    // View state
    current_edit_state: Option<Box<dyn Any>>,
    current_context: Option<items::ViewContext<'static>>,
    current_other_groups: Option<Vec<item_groups::ItemGroup>>,
    current_available_groups: Option<Vec<item_groups::ItemGroup>>,
    current_available_categories: Option<Vec<revenue_categories::RevenueCategory>>,
}

impl MenuBuilder {

    fn theme(&self) -> iced::Theme {
        iced::Theme::Light
    }

    fn title(&self) -> String {
        String::from("Menu Builder :D")
    }

    fn new() -> (Self, Task<Message>) {
        (
            Self {
                screen: Screen::Items(items::Mode::View),
                // Initialize empty HashMaps
                items: HashMap::new(),
                item_groups: HashMap::new(),
                price_levels: HashMap::new(),
                product_classes: HashMap::new(),
                tax_groups: HashMap::new(),
                security_levels: HashMap::new(),
                revenue_categories: HashMap::new(),
                report_categories: HashMap::new(),
                choice_groups: HashMap::new(),
                printer_logicals: HashMap::new(),

                // Initialize all drafts as None
                draft_item: None,
                draft_item_group: None,
                draft_price_level: None,
                draft_product_class: None,
                draft_tax_group: None,
                draft_security_level: None,
                draft_revenue_category: None,
                draft_report_category: None,
                draft_choice_group: None,
                draft_printer_logical: None,

                //initialize state
                current_edit_state: None,
                current_context: None,
                current_other_groups: None,
                current_available_groups: None,
                current_available_categories: None,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(screen) => {
                self.screen = screen;
                Task::none()
            }
    
            Message::Items(msg) => {
                match &mut self.screen {
                    Screen::Items(mode) => {
                        let context = items::ViewContext {
                            available_item_groups: &self.item_groups,
                            available_tax_groups: &self.tax_groups,
                            available_security_levels: &self.security_levels,
                            available_revenue_categories: &self.revenue_categories,
                            available_report_categories: &self.report_categories,
                            available_product_classes: &self.product_classes,
                            available_choice_groups: &self.choice_groups,
                            available_printer_logicals: &self.printer_logicals,
                            available_price_levels: &self.price_levels,
                        };
    
                        if let Some((id, item)) = &mut self.draft_item {
                            let mut edit_state = items::edit::EditState::new(item);
                            let action = items::update(item, msg, &context, &mut edit_state)
                                .map_operation(|o| Operation::Items(o))
                                .map(Message::Items);
    
                            if let Some(operation) = action.operation {
                                self.perform(operation)
                            } else {
                                action.task
                            }
                        } else if let Screen::Items(items::Mode::View) = self.screen {
                            // Create a new draft item when in view mode and no draft exists
                            let new_item = items::Item::new(0, String::new()); // You'll need to implement this
                            self.draft_item = Some((None, new_item));
                            Task::none()
                        } else {
                            Task::none()
                        }
                    }
                    _ => Task::none(),
                }
            }
    
            Message::ItemGroups(msg) => {
                match &mut self.screen {
                    Screen::ItemGroups(mode) => {
                        if let Some((id, group)) = &mut self.draft_item_group {
                            let mut edit_state = item_groups::edit::EditState::new(group);
                            let other_groups = self.item_groups.values().collect::<Vec<_>>();
                            let action = item_groups::update(group, msg, &mut edit_state, &other_groups)
                                .map_operation(|o| Operation::ItemGroups(o))
                                .map(Message::ItemGroups);
    
                            if let Some(operation) = action.operation {
                                self.perform(operation)
                            } else {
                                action.task
                            }
                        } else {
                            Task::none()
                        }
                    }
                    _ => Task::none(),
                }
            }
    
            Message::TaxGroups(msg) => {
                match &mut self.screen {
                    Screen::TaxGroups(mode) => {
                        if let Some((id, group)) = &mut self.draft_tax_group {
                            let mut edit_state = tax_groups::edit::EditState::new(group);
                            let other_groups = self.tax_groups.values().collect::<Vec<_>>();
                            let action = tax_groups::update(group, msg, &mut edit_state, &other_groups)
                                .map_operation(|o| Operation::TaxGroups(o))
                                .map(Message::TaxGroups);
    
                            if let Some(operation) = action.operation {
                                self.perform(operation)
                            } else {
                                action.task
                            }
                        } else {
                            Task::none()
                        }
                    }
                    _ => Task::none(),
                }
            }
    
            Message::SecurityLevels(msg) => {
                match &mut self.screen {
                    Screen::SecurityLevels(mode) => {
                        if let Some((id, level)) = &mut self.draft_security_level {
                            let mut edit_state = security_levels::edit::EditState::new(level);
                            let other_levels = self.security_levels.values().collect::<Vec<_>>();
                            let action = security_levels::update(level, msg, &mut edit_state, &other_levels)
                                .map_operation(|o| Operation::SecurityLevels(o))
                                .map(Message::SecurityLevels);
    
                            if let Some(operation) = action.operation {
                                self.perform(operation)
                            } else {
                                action.task
                            }
                        } else {
                            Task::none()
                        }
                    }
                    _ => Task::none(),
                }
            }
    
            Message::RevenueCategories(msg) => {
                match &mut self.screen {
                    Screen::RevenueCategories(mode) => {
                        if let Some((id, category)) = &mut self.draft_revenue_category {
                            let mut edit_state = revenue_categories::edit::EditState::new(category);
                            let other_categories = self.revenue_categories.values().collect::<Vec<_>>();
                            let action = revenue_categories::update(category, msg, &mut edit_state, &other_categories)
                                .map_operation(|o| Operation::RevenueCategories(o))
                                .map(Message::RevenueCategories);
    
                            if let Some(operation) = action.operation {
                                self.perform(operation)
                            } else {
                                action.task
                            }
                        } else {
                            Task::none()
                        }
                    }
                    _ => Task::none(),
                }
            }
    
            Message::ReportCategories(msg) => {
                match &mut self.screen {
                    Screen::ReportCategories(mode) => {
                        if let Some((id, category)) = &mut self.draft_report_category {
                            let mut edit_state = report_categories::edit::EditState::new(category);
                            let other_categories = self.report_categories.values().collect::<Vec<_>>();
                            let action = report_categories::update(category, msg, &mut edit_state, &other_categories)
                                .map_operation(|o| Operation::ReportCategories(o))
                                .map(Message::ReportCategories);
    
                            if let Some(operation) = action.operation {
                                self.perform(operation)
                            } else {
                                action.task
                            }
                        } else {
                            Task::none()
                        }
                    }
                    _ => Task::none(),
                }
            }
    
            Message::ProductClasses(msg) => {
                match &mut self.screen {
                    Screen::ProductClasses(mode) => {
                        if let Some((id, class)) = &mut self.draft_product_class {
                            let mut edit_state = product_classes::edit::EditState::new(class);
                            let context = UpdateContext {
                                other_classes: &self.product_classes.values().collect::<Vec<_>>(),
                                available_item_groups: &self.item_groups.values().collect::<Vec<_>>(),
                                available_revenue_categories: &self.revenue_categories.values().collect::<Vec<_>>(),
                            };
                            let action = product_classes::update(class, msg, &mut edit_state, &context)
                                .map_operation(|o| Operation::ProductClasses(o))
                                .map(Message::ProductClasses);
    
                            if let Some(operation) = action.operation {
                                self.perform(operation)
                            } else {
                                action.task
                            }
                        } else {
                            Task::none()
                        }
                    }
                    _ => Task::none(),
                }
            }
    
            Message::ChoiceGroups(msg) => {
                match &mut self.screen {
                    Screen::ChoiceGroups(mode) => {
                        if let Some((id, group)) = &mut self.draft_choice_group {
                            let mut edit_state = choice_groups::edit::EditState::new(group);
                            let other_groups = self.choice_groups.values().collect::<Vec<_>>();
                            let action = choice_groups::update(group, msg, &mut edit_state, &other_groups)
                                .map_operation(|o| Operation::ChoiceGroups(o))
                                .map(Message::ChoiceGroups);
    
                            if let Some(operation) = action.operation {
                                self.perform(operation)
                            } else {
                                action.task
                            }
                        } else {
                            Task::none()
                        }
                    }
                    _ => Task::none(),
                }
            }
    
            Message::PrinterLogicals(msg) => {
                match &mut self.screen {
                    Screen::PrinterLogicals(mode) => {
                        if let Some((id, printer)) = &mut self.draft_printer_logical {
                            let mut edit_state = printer_logicals::edit::EditState::new(printer);
                            let other_printers = self.printer_logicals.values().collect::<Vec<_>>();
                            let action = printer_logicals::update(printer, msg, &mut edit_state, &other_printers)
                                .map_operation(|o| Operation::PrinterLogicals(o))
                                .map(Message::PrinterLogicals);
    
                            if let Some(operation) = action.operation {
                                self.perform(operation)
                            } else {
                                action.task
                            }
                        } else {
                            Task::none()
                        }
                    }
                    _ => Task::none(),
                }
            }
    
            Message::PriceLevels(msg) => {
                match &mut self.screen {
                    Screen::PriceLevels(mode) => {
                        if let Some((id, level)) = &mut self.draft_price_level {
                            let mut edit_state = price_levels::edit::EditState::new(level);
                            let other_levels = self.price_levels.values().collect::<Vec<_>>();
                            let action = price_levels::update(level, msg, &mut edit_state, &other_levels)
                                .map_operation(|o| Operation::PriceLevels(o))
                                .map(Message::PriceLevels);
    
                            if let Some(operation) = action.operation {
                                self.perform(operation)
                            } else {
                                action.task
                            }
                        } else {
                            Task::none()
                        }
                    }
                    _ => Task::none(),
                }
            }
    
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
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match &self.screen {
            Screen::Items(mode) => {
                if let Some((_, item)) = &self.draft_item {
                    let context = items::ViewContext {
                        available_item_groups: &self.item_groups,
                        available_tax_groups: &self.tax_groups,
                        available_security_levels: &self.security_levels,
                        available_revenue_categories: &self.revenue_categories,
                        available_report_categories: &self.report_categories,
                        available_product_classes: &self.product_classes,
                        available_choice_groups: &self.choice_groups,
                        available_printer_logicals: &self.printer_logicals,
                        available_price_levels: &self.price_levels,
                    };
                    items::view(item, mode, &context).map(Message::Items)
                } else {
                    container(text("No item selected")).into()
                }
            }
            Screen::ItemGroups(mode) => {
                if let Some((_, group)) = &self.draft_item_group {
                    let other_groups: Vec<&item_groups::ItemGroup> = self.item_groups.values().collect();
                    item_groups::view(group, mode, &other_groups).map(Message::ItemGroups)
                } else {
                    container(text("No item group selected")).into()
                }
            }
            Screen::TaxGroups(mode) => {
                if let Some((_, group)) = &self.draft_tax_group {
                    let other_groups: Vec<&tax_groups::TaxGroup> = self.tax_groups.values().collect();
                    tax_groups::view(group, mode, &other_groups).map(Message::TaxGroups)
                } else {
                    container(text("No tax group selected")).into()
                }
            }
            Screen::SecurityLevels(mode) => {
                if let Some((_, level)) = &self.draft_security_level {
                    let other_levels: Vec<&security_levels::SecurityLevel> = self.security_levels.values().collect();
                    security_levels::view(level, mode, &other_levels).map(Message::SecurityLevels)
                } else {
                    container(text("No security level selected")).into()
                }
            }
            Screen::RevenueCategories(mode) => {
                if let Some((_, category)) = &self.draft_revenue_category {
                    let other_categories: Vec<&revenue_categories::RevenueCategory> = self.revenue_categories.values().collect();
                    revenue_categories::view(category, mode, &other_categories).map(Message::RevenueCategories)
                } else {
                    container(text("No revenue category selected")).into()
                }
            }
            Screen::ReportCategories(mode) => {
                if let Some((_, category)) = &self.draft_report_category {
                    let other_categories: Vec<&report_categories::ReportCategory> = self.report_categories.values().collect();
                    report_categories::view(category, mode, &other_categories).map(Message::ReportCategories)
                } else {
                    container(text("No report category selected")).into()
                }
            }
            Screen::ProductClasses(mode) => {
                if let Some((_, class)) = &self.draft_product_class {
                    let available_item_groups: Vec<&item_groups::ItemGroup> = self.item_groups.values().collect();
                    let available_revenue_categories: Vec<&revenue_categories::RevenueCategory> = self.revenue_categories.values().collect();
                    product_classes::view(
                        class, 
                        mode,
                        &available_item_groups,
                        &available_revenue_categories
                    ).map(Message::ProductClasses)
                } else {
                    container(text("No product class selected")).into()
                }
            }
            Screen::ChoiceGroups(mode) => {
                if let Some((_, group)) = &self.draft_choice_group {
                    let other_groups: Vec<&choice_groups::ChoiceGroup> = self.choice_groups.values().collect();
                    choice_groups::view(group, mode, &other_groups).map(Message::ChoiceGroups)
                } else {
                    container(text("No choice group selected")).into()
                }
            }
            Screen::PrinterLogicals(mode) => {
                if let Some((_, printer)) = &self.draft_printer_logical {
                    let other_printers: Vec<&printer_logicals::PrinterLogical> = self.printer_logicals.values().collect();
                    printer_logicals::view(printer, mode, &other_printers).map(Message::PrinterLogicals)
                } else {
                    container(text("No printer logical selected")).into()
                }
            }
            Screen::PriceLevels(mode) => {
                if let Some((_, level)) = &self.draft_price_level {
                    let other_levels: Vec<&price_levels::PriceLevel> = self.price_levels.values().collect();
                    price_levels::view(level, mode, &other_levels).map(Message::PriceLevels)
                } else {
                    container(text("No price level selected")).into()
                }
            }
        };

        row![
            self.view_sidebar(),
            container(content)
                .width(Length::Fill)
                .padding(20)
        ]
        .into()
    }


    fn perform(&mut self, operation: Operation) -> Task<Message> {
        match operation {
            Operation::Items(op) => {
                match op {
                    items::Operation::Save(item) => {
                        self.items.insert(item.id, item);
                        self.screen = Screen::Items(items::Mode::View);
                        Task::none()
                    }
                    items::Operation::StartEdit(id) => {
                        self.screen = Screen::Items(items::Mode::Edit);
                        Task::none()
                    }
                    items::Operation::Cancel => {
                        self.screen = Screen::Items(items::Mode::View);
                        Task::none()
                    }
                    items::Operation::Back => {
                        self.screen = Screen::Items(items::Mode::View);
                        Task::none()
                    }
                    items::Operation::ExportToCsv => {
                        todo!();
                        Task::none()
                    }
                }
            }
    
            Operation::ItemGroups(op) => {
                match op {
                    item_groups::Operation::Save(group) => {
                        self.item_groups.insert(group.id, group);
                        self.screen = Screen::ItemGroups(item_groups::Mode::View);
                        Task::none()
                    }
                    item_groups::Operation::StartEdit(id) => {
                        self.screen = Screen::ItemGroups(item_groups::Mode::Edit);
                        Task::none()
                    }
                    item_groups::Operation::Cancel => {
                        self.screen = Screen::ItemGroups(item_groups::Mode::View);
                        Task::none()
                    }
                    item_groups::Operation::Back => {
                        self.screen = Screen::ItemGroups(item_groups::Mode::View);
                        Task::none()
                    }
                }
            }
    
            Operation::TaxGroups(op) => {
                match op {
                    tax_groups::Operation::Save(group) => {
                        self.tax_groups.insert(group.id, group);
                        self.screen = Screen::TaxGroups(tax_groups::Mode::View);
                        Task::none()
                    }
                    tax_groups::Operation::StartEdit(id) => {
                        self.screen = Screen::TaxGroups(tax_groups::Mode::Edit);
                        Task::none()
                    }
                    tax_groups::Operation::Cancel => {
                        self.screen = Screen::TaxGroups(tax_groups::Mode::View);
                        Task::none()
                    }
                    tax_groups::Operation::Back => {
                        self.screen = Screen::TaxGroups(tax_groups::Mode::View);
                        Task::none()
                    }
                }
            }
    
            Operation::SecurityLevels(op) => {
                match op {
                    security_levels::Operation::Save(level) => {
                        self.security_levels.insert(level.id, level);
                        self.screen = Screen::SecurityLevels(security_levels::Mode::View);
                        Task::none()
                    }
                    security_levels::Operation::StartEdit(id) => {
                        self.screen = Screen::SecurityLevels(security_levels::Mode::Edit);
                        Task::none()
                    }
                    security_levels::Operation::Cancel => {
                        self.screen = Screen::SecurityLevels(security_levels::Mode::View);
                        Task::none()
                    }
                    security_levels::Operation::Back => {
                        self.screen = Screen::SecurityLevels(security_levels::Mode::View);
                        Task::none()
                    }
                }
            }
    
            Operation::RevenueCategories(op) => {
                match op {
                    revenue_categories::Operation::Save(category) => {
                        self.revenue_categories.insert(category.id, category);
                        self.screen = Screen::RevenueCategories(revenue_categories::Mode::View);
                        Task::none()
                    }
                    revenue_categories::Operation::StartEdit(id) => {
                        self.screen = Screen::RevenueCategories(revenue_categories::Mode::Edit);
                        Task::none()
                    }
                    revenue_categories::Operation::Cancel => {
                        self.screen = Screen::RevenueCategories(revenue_categories::Mode::View);
                        Task::none()
                    }
                    revenue_categories::Operation::Back => {
                        self.screen = Screen::RevenueCategories(revenue_categories::Mode::View);
                        Task::none()
                    }
                }
            }
    
            Operation::ReportCategories(op) => {
                match op {
                    report_categories::Operation::Save(category) => {
                        self.report_categories.insert(category.id, category);
                        self.screen = Screen::ReportCategories(report_categories::Mode::View);
                        Task::none()
                    }
                    report_categories::Operation::StartEdit(id) => {
                        self.screen = Screen::ReportCategories(report_categories::Mode::Edit);
                        Task::none()
                    }
                    report_categories::Operation::Cancel => {
                        self.screen = Screen::ReportCategories(report_categories::Mode::View);
                        Task::none()
                    }
                    report_categories::Operation::Back => {
                        self.screen = Screen::ReportCategories(report_categories::Mode::View);
                        Task::none()
                    }
                }
            }
    
            Operation::ProductClasses(op) => {
                match op {
                    product_classes::Operation::Save(class) => {
                        self.product_classes.insert(class.id, class);
                        self.screen = Screen::ProductClasses(product_classes::Mode::View);
                        Task::none()
                    }
                    product_classes::Operation::StartEdit(id) => {
                        self.screen = Screen::ProductClasses(product_classes::Mode::Edit);
                        Task::none()
                    }
                    product_classes::Operation::Cancel => {
                        self.screen = Screen::ProductClasses(product_classes::Mode::View);
                        Task::none()
                    }
                    product_classes::Operation::Back => {
                        self.screen = Screen::ProductClasses(product_classes::Mode::View);
                        Task::none()
                    }
                }
            }
    
            Operation::ChoiceGroups(op) => {
                match op {
                    choice_groups::Operation::Save(group) => {
                        self.choice_groups.insert(group.id, group);
                        self.screen = Screen::ChoiceGroups(choice_groups::Mode::View);
                        Task::none()
                    }
                    choice_groups::Operation::StartEdit(id) => {
                        self.screen = Screen::ChoiceGroups(choice_groups::Mode::Edit);
                        Task::none()
                    }
                    choice_groups::Operation::Cancel => {
                        self.screen = Screen::ChoiceGroups(choice_groups::Mode::View);
                        Task::none()
                    }
                    choice_groups::Operation::Back => {
                        self.screen = Screen::ChoiceGroups(choice_groups::Mode::View);
                        Task::none()
                    }
                }
            }
    
            Operation::PrinterLogicals(op) => {
                match op {
                    printer_logicals::Operation::Save(printer) => {
                        self.printer_logicals.insert(printer.id, printer);
                        self.screen = Screen::PrinterLogicals(printer_logicals::Mode::View);
                        Task::none()
                    }
                    printer_logicals::Operation::StartEdit(id) => {
                        self.screen = Screen::PrinterLogicals(printer_logicals::Mode::Edit);
                        Task::none()
                    }
                    printer_logicals::Operation::Cancel => {
                        self.screen = Screen::PrinterLogicals(printer_logicals::Mode::View);
                        Task::none()
                    }
                    printer_logicals::Operation::Back => {
                        self.screen = Screen::PrinterLogicals(printer_logicals::Mode::View);
                        Task::none()
                    }
                }
            }

            Operation::PriceLevels(op) => match op {
                price_levels::Operation::Save(level) => {
                    self.price_levels.insert(level.id, level);
                    self.screen = Screen::PriceLevels(price_levels::Mode::View);
                    Task::none()
                }
                price_levels::Operation::StartEdit(id) => {
                    self.screen = Screen::PriceLevels(price_levels::Mode::Edit);
                    Task::none()
                }
                price_levels::Operation::Cancel => {
                    self.screen = Screen::PriceLevels(price_levels::Mode::View);
                    Task::none()
                }
                price_levels::Operation::Back => {
                    self.screen = Screen::PriceLevels(price_levels::Mode::View);
                    Task::none()
                }
            },

        }
    }

    fn view_sidebar<'a>(&'a self) -> Element<'a, Message> {
        let nav_button = |label: &'a str| {
            move |screen: Screen| {
                button(
                    text(label.to_string())
                        .width(Length::Fill)
                )
                .width(Length::Fill)
                .on_press(Message::Navigate(screen))
            }
        };

        container(
            column![
                nav_button("Items")(Screen::Items(items::Mode::View)),
                nav_button("Item Groups")(Screen::ItemGroups(item_groups::Mode::View)),
                nav_button("Price Levels")(Screen::PriceLevels(price_levels::Mode::View)),
                nav_button("Product Classes")(Screen::ProductClasses(product_classes::Mode::View)),
                nav_button("Tax Groups")(Screen::TaxGroups(tax_groups::Mode::View)),
                nav_button("Security Levels")(Screen::SecurityLevels(security_levels::Mode::View)),
                nav_button("Revenue Categories")(Screen::RevenueCategories(revenue_categories::Mode::View)),
                nav_button("Report Categories")(Screen::ReportCategories(report_categories::Mode::View)),
                nav_button("Choice Groups")(Screen::ChoiceGroups(choice_groups::Mode::View)),
                nav_button("Printer Logicals")(Screen::PrinterLogicals(printer_logicals::Mode::View))
            ]
            .spacing(5)
            .width(Length::Fixed(200.0))
        )
        .style(iced::widget::container::bordered_box)
        .into()
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
        _ => None,
    }
}