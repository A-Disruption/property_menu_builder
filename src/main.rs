use iced::event;
use iced::keyboard::key::Named;
use iced::keyboard::{self, Key, Modifiers};
use iced::widget::{
    focus_next, focus_previous,
    button, checkbox, column, container, pick_list,
    row, scrollable, text, text_input, vertical_space,
};
use iced::{Element, Length, Size, Subscription, Task};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

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

#[derive(Debug)]
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
                        // Get references to all required data for context
                        let context = UpdateContext {
                            available_item_groups: &self.item_groups.values().collect::<Vec<_>>(),
                            available_tax_groups: &self.tax_groups.values().collect::<Vec<_>>(),
                            available_security_levels: &self.security_levels.values().collect::<Vec<_>>(),
                            available_revenue_categories: &self.revenue_categories.values().collect::<Vec<_>>(),
                            available_report_categories: &self.report_categories.values().collect::<Vec<_>>(),
                            available_product_classes: &self.product_classes.values().collect::<Vec<_>>(),
                            available_choice_groups: &self.choice_groups.values().collect::<Vec<_>>(),
                            available_printer_logicals: &self.printer_logicals.values().collect::<Vec<_>>(),
                        };

                        // Handle the message
                        let action = items::update(msg, *mode, context)
                            .map_operation(Operation::Items)
                            .map(Message::Items);

                        if let Some(operation) = action.operation {
                            self.perform(operation)
                        } else {
                            action.task
                        }
                    }
                    _ => Task::none(),
                }
            }

            Message::ItemGroups(msg) => {
                match &mut self.screen {
                    Screen::ItemGroups(mode) => {
                        let other_groups = self.item_groups.values().collect::<Vec<_>>();
                        let action = item_groups::update(msg, *mode, &other_groups)
                            .map_operation(Operation::ItemGroups)
                            .map(Message::ItemGroups);

                        if let Some(operation) = action.operation {
                            self.perform(operation)
                        } else {
                            action.task
                        }
                    }
                    _ => Task::none(),
                }
            }

            Message::TaxGroups(msg) => {
                match &mut self.screen {
                    Screen::TaxGroups(mode) => {
                        let other_groups = self.tax_groups.values().collect::<Vec<_>>();
                        let action = tax_groups::update(msg, *mode, &other_groups)
                            .map_operation(Operation::TaxGroups)
                            .map(Message::TaxGroups);

                        if let Some(operation) = action.operation {
                            self.perform(operation)
                        } else {
                            action.task
                        }
                    }
                    _ => Task::none(),
                }
            }

            Message::SecurityLevels(msg) => {
                match &mut self.screen {
                    Screen::SecurityLevels(mode) => {
                        let other_levels = self.security_levels.values().collect::<Vec<_>>();
                        let action = security_levels::update(msg, *mode, &other_levels)
                            .map_operation(Operation::SecurityLevels)
                            .map(Message::SecurityLevels);

                        if let Some(operation) = action.operation {
                            self.perform(operation)
                        } else {
                            action.task
                        }
                    }
                    _ => Task::none(),
                }
            }

            Message::RevenueCategories(msg) => {
                match &mut self.screen {
                    Screen::RevenueCategories(mode) => {
                        let other_categories = self.revenue_categories.values().collect::<Vec<_>>();
                        let action = revenue_categories::update(msg, *mode, &other_categories)
                            .map_operation(Operation::RevenueCategories)
                            .map(Message::RevenueCategories);

                        if let Some(operation) = action.operation {
                            self.perform(operation)
                        } else {
                            action.task
                        }
                    }
                    _ => Task::none(),
                }
            }

            Message::ReportCategories(msg) => {
                match &mut self.screen {
                    Screen::ReportCategories(mode) => {
                        let other_categories = self.report_categories.values().collect::<Vec<_>>();
                        let action = report_categories::update(msg, *mode, &other_categories)
                            .map_operation(Operation::ReportCategories)
                            .map(Message::ReportCategories);

                        if let Some(operation) = action.operation {
                            self.perform(operation)
                        } else {
                            action.task
                        }
                    }
                    _ => Task::none(),
                }
            }

            Message::ProductClasses(msg) => {
                match &mut self.screen {
                    Screen::ProductClasses(mode) => {
                        let context = product_classes::UpdateContext {
                            other_classes: &self.product_classes.values().collect::<Vec<_>>(),
                            available_item_groups: &self.item_groups.values().collect::<Vec<_>>(),
                            available_revenue_categories: &self.revenue_categories.values().collect::<Vec<_>>(),
                        };

                        let action = product_classes::update(msg, *mode, &context)
                            .map_operation(Operation::ProductClasses)
                            .map(Message::ProductClasses);

                        if let Some(operation) = action.operation {
                            self.perform(operation)
                        } else {
                            action.task
                        }
                    }
                    _ => Task::none(),
                }
            }

            Message::ChoiceGroups(msg) => {
                match &mut self.screen {
                    Screen::ChoiceGroups(mode) => {
                        let other_groups = self.choice_groups.values().collect::<Vec<_>>();
                        let action = choice_groups::update(msg, *mode, &other_groups)
                            .map_operation(Operation::ChoiceGroups)
                            .map(Message::ChoiceGroups);

                        if let Some(operation) = action.operation {
                            self.perform(operation)
                        } else {
                            action.task
                        }
                    }
                    _ => Task::none(),
                }
            }

            Message::PrinterLogicals(msg) => {
                match &mut self.screen {
                    Screen::PrinterLogicals(mode) => {
                        let other_printers = self.printer_logicals.values().collect::<Vec<_>>();
                        let action = printer_logicals::update(msg, *mode, &other_printers)
                            .map_operation(Operation::PrinterLogicals)
                            .map(Message::PrinterLogicals);

                        if let Some(operation) = action.operation {
                            self.perform(operation)
                        } else {
                            action.task
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
                    HotKey::Escape => {
                        // Handle escape key based on current screen
                        match self.screen {
                            Screen::Items(mode) => {
                                // Handle escape in items screen
                                Task::none()
                            }
                            // Handle other screens...
                            _ => Task::none(),
                        }
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match &self.screen {
            Screen::Items(mode) => {
                // Render items view
                text("Items View").into()
            }
            // ... render other screens
        };

        container(
            row![
                self.view_sidebar(),
                container(content)
                    .width(Length::Fill)
                    .padding(20)
            ]
        )
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
        }
    }

    fn view_sidebar(&self) -> Element<Message> {
        let nav_button = |label: &str, screen: Screen| {
            button(text(label))
                .width(Length::Fill)
                .on_press(Message::Navigate(screen))
        };

        container(
            column![
                nav_button("Items", Screen::Items(items::Mode::View)),
                nav_button("Item Groups", Screen::ItemGroups(item_groups::Mode::View)),
                // ... add other navigation buttons
            ]
            .spacing(5)
            .width(Length::Fixed(200.0))
        )
        .style(theme::Container::Box)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen_with(handle_event)
    }
}

#[derive(Debug)]
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