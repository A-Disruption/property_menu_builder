use iced::widget::{
    button, checkbox, column, container, pick_list, row, 
    text, text_input, horizontal_space, vertical_space, scrollable
};
use iced::{Element, Length};
use crate::data_types::{EntityId, Currency, Validatable, ValidationError};
use crate::{
    choice_groups::ChoiceGroup,
    item_groups::ItemGroup,
    price_levels::{PriceLevel, PriceLevelType},
    printer_logicals::PrinterLogical,
    product_classes::ProductClass,
    report_categories::ReportCategory,
    revenue_categories::RevenueCategory,
    security_levels::SecurityLevel,
    tax_groups::TaxGroup,
};
use crate::HotKey;
use super::{Item, Action, Operation, ViewContext};

#[derive(Debug, Clone)]
pub enum Message {
    // Basic Info
    UpdateName(String),
    UpdateButton1(String),
    UpdateButton2(String),
    UpdatePrinterText(String),

    // Classifications
    SelectItemGroup(Option<EntityId>),
    SelectProductClass(Option<EntityId>),
    SelectRevenueCategory(Option<EntityId>),
    SelectTaxGroup(Option<EntityId>),
    SelectSecurityLevel(Option<EntityId>),
    SelectReportCategory(Option<EntityId>),

    // Pricing
    UpdateCostAmount(String),
    ToggleAskPrice(bool),
    ToggleAllowPriceOverride(bool),

    // Price Levels
    AddPriceLevel(EntityId),
    RemovePriceLevel(EntityId),
    UpdateStorePriceLevel(Option<EntityId>),

    // Weight
    ToggleUseWeight(bool),
    UpdateWeightAmount(String),

    // Identifiers
    UpdateSku(String),
    UpdateBarGunCode(String),

    // Flags
    ToggleReserved1(bool),
    TogglePrintOnCheck(bool),
    ToggleDiscountable(bool),
    ToggleVoidable(bool),
    ToggleNotActive(bool),
    ToggleTaxIncluded(bool),
    ToggleStockItem(bool),
    ToggleReserved2(bool),

    // Receipt & Kitchen
    UpdateCustomerReceipt(String),
    UpdateKitchenVideo(String),
    UpdateKdsCategory(String),
    UpdateKdsCooktime(String),
    UpdateKdsDept(String),

    // Store Settings
    UpdateStoreId(String),
    UpdateCovers(String),
    UpdateImageId(String),
    UpdateLanguageIsoCode(String),

    // Related Items
    AddChoiceGroup(EntityId),
    RemoveChoiceGroup(EntityId),
    AddPrinterLogical(EntityId),
    RemovePrinterLogical(EntityId),

    Save,
    Cancel,
}

pub struct EditState {
    pub name: String,
    pub button1: String,
    pub button2: Option<String>,
    pub printer_text: String,
    pub price_levels: Vec<EntityId>,
    pub product_class: Option<EntityId>,
    pub revenue_category: Option<EntityId>,
    pub tax_group: Option<EntityId>,
    pub security_level: Option<EntityId>,
    pub report_category: Option<EntityId>,
    pub use_weight: bool,
    pub weight_amount: Currency,
    pub sku: Option<String>,
    pub bar_gun_code: Option<String>,
    pub cost_amount: Option<Currency>,
    pub reserved1: bool,
    pub ask_price: bool,
    pub print_on_check: bool,
    pub discountable: bool,
    pub voidable: bool,
    pub not_active: bool,
    pub tax_included: bool,
    pub item_group: Option<EntityId>,
    pub customer_receipt: String,
    pub allow_price_override: bool,
    pub reserved2: bool,
    pub choice_groups: Option<Vec<EntityId>>,
    pub printer_logicals: Option<Vec<EntityId>>,
    pub covers: i32,
    pub store_id: i32,
    pub kitchen_video: String,
    pub kds_dept: i32,
    pub kds_category: String,
    pub kds_cooktime: i32,
    pub store_price_level: Option<Vec<EntityId>>,
    pub image_id: i32,
    pub stock_item: bool,
    pub language_iso_code: String,
    pub validation_error: Option<String>,
}

impl EditState {
    pub fn new(item: &Item) -> Self {
        Self {
            name: item.name.clone(),
            button1: item.button1.clone(),
            button2: item.button2.clone(),
            printer_text: item.printer_text.clone(),
            price_levels: item.price_levels.clone().unwrap_or_default(),
            product_class: item.product_class,
            revenue_category: item.revenue_category,
            tax_group: item.tax_group,
            security_level: item.security_level,
            report_category: item.report_category,
            use_weight: item.use_weight,
            weight_amount: item.weight_amount,
            sku: item.sku.clone(),
            bar_gun_code: item.bar_gun_code.clone(),
            cost_amount: item.cost_amount,
            reserved1: item.reserved1,
            ask_price: item.ask_price,
            print_on_check: item.print_on_check,
            discountable: item.discountable,
            voidable: item.voidable,
            not_active: item.not_active,
            tax_included: item.tax_included,
            item_group: item.item_group,
            customer_receipt: item.customer_receipt.clone(),
            allow_price_override: item.allow_price_override,
            reserved2: item.reserved2,
            choice_groups: item.choice_groups.clone(),
            printer_logicals: item.printer_logicals.clone(),
            covers: item.covers,
            store_id: item.store_id,
            kitchen_video: item.kitchen_video.clone(),
            kds_dept: item.kds_dept,
            kds_category: item.kds_category.clone(),
            kds_cooktime: item.kds_cooktime,
            store_price_level: item.store_price_level.clone(),
            image_id: item.image_id,
            stock_item: item.stock_item,
            language_iso_code: item.language_iso_code.clone(),
            validation_error: None,
        }
    }

    pub fn validate(&self, context: &ViewContext) -> Result<(), ValidationError> {
        // Add validation logic here
        Ok(())
    }
}

pub fn update(
    item: &mut Item,
    message: Message,
    state: &mut EditState,
) -> Action<Operation, Message> {
    match message {
        // Basic Info
        Message::UpdateName(name) => {
            item.name = name;
            state.validation_error = None;
            Action::none()
        }
        Message::UpdateButton1(text) => {
            if text.len() <= 15 {
                item.button1 = text;
                state.validation_error = None;
            } else {
                state.validation_error = Some("Button 1 text cannot exceed 15 characters".to_string());
            }
            Action::none()
        }
        Message::UpdateButton2(text) => {
            if text.len() <= 15 {
                item.button2 = Some(text);
                state.validation_error = None;
            } else {
                state.validation_error = Some("Button 2 text cannot exceed 15 characters".to_string());
            }
            Action::none()
        }
        Message::UpdatePrinterText(text) => {
            item.printer_text = text;
            state.validation_error = None;
            Action::none()
        }

        // Classifications
        Message::SelectItemGroup(maybe_id) => {
            item.item_group = maybe_id;
            state.validation_error = None;
            Action::none()
        }
        Message::SelectProductClass(maybe_id) => {
            item.product_class = maybe_id;
            state.validation_error = None;
            Action::none()
        }
        Message::SelectRevenueCategory(maybe_id) => {
            item.revenue_category = maybe_id;
            state.validation_error = None;
            Action::none()
        }
        Message::SelectTaxGroup(maybe_id) => {
            item.tax_group = maybe_id;
            state.validation_error = None;
            Action::none()
        }
        Message::SelectSecurityLevel(maybe_id) => {
            item.security_level = maybe_id;
            state.validation_error = None;
            Action::none()
        }
        Message::SelectReportCategory(maybe_id) => {
            item.report_category = maybe_id;
            state.validation_error = None;
            Action::none()
        }

        // Pricing
        Message::UpdateCostAmount(amount_str) => {
            match amount_str.parse::<Currency>() {
                Ok(amount) => {
                    item.cost_amount = Some(amount);
                    state.validation_error = None;
                }
                Err(_) => {
                    if !amount_str.is_empty() {
                        state.validation_error = Some("Invalid cost amount format".to_string());
                    } else {
                        item.cost_amount = None;
                        state.validation_error = None;
                    }
                }
            }
            Action::none()
        }
        Message::ToggleAskPrice(value) => {
            item.ask_price = value;
            Action::none()
        }
        Message::ToggleAllowPriceOverride(value) => {
            item.allow_price_override = value;
            Action::none()
        }

        // Price Levels
        Message::AddPriceLevel(price_level_id) => {
            if let Some(price_levels) = &mut item.price_levels {
                if !price_levels.contains(&price_level_id) {
                    price_levels.push(price_level_id);
                }
            } else {
                item.price_levels = Some(vec![price_level_id]);
            }
            Action::none()
        }
        Message::RemovePriceLevel(price_level_id) => {
            if let Some(price_levels) = &mut item.price_levels {
                price_levels.retain(|&id| id != price_level_id);
                if price_levels.is_empty() {
                    item.price_levels = None;
                }
            }
            Action::none()
        }
        Message::UpdateStorePriceLevel(maybe_id) => {
            if let Some(store_levels) = &mut item.store_price_level {
                if let Some(id) = maybe_id {
                    store_levels.push(id);
                }
            } else if maybe_id.is_some() {
                item.store_price_level = Some(vec![maybe_id.unwrap()]);
            }
            Action::none()
        }

        // Weight
        Message::ToggleUseWeight(value) => {
            item.use_weight = value;
            if !value {
                item.weight_amount = Currency::ZERO;
            }
            Action::none()
        }
        Message::UpdateWeightAmount(amount_str) => {
            match amount_str.parse::<Currency>() {
                Ok(amount) => {
                    item.weight_amount = amount;
                    state.validation_error = None;
                }
                Err(_) => {
                    state.validation_error = Some("Invalid weight amount format".to_string());
                }
            }
            Action::none()
        }

        // Identifiers
        Message::UpdateSku(sku) => {
            item.sku = Some(sku);
            Action::none()
        }
        Message::UpdateBarGunCode(code) => {
            item.bar_gun_code = Some(code);
            Action::none()
        }

        // Flags
        Message::ToggleReserved1(value) => {
            item.reserved1 = value;
            Action::none()
        }
        Message::TogglePrintOnCheck(value) => {
            item.print_on_check = value;
            Action::none()
        }
        Message::ToggleDiscountable(value) => {
            item.discountable = value;
            Action::none()
        }
        Message::ToggleVoidable(value) => {
            item.voidable = value;
            Action::none()
        }
        Message::ToggleNotActive(value) => {
            item.not_active = value;
            Action::none()
        }
        Message::ToggleTaxIncluded(value) => {
            item.tax_included = value;
            Action::none()
        }
        Message::ToggleStockItem(value) => {
            item.stock_item = value;
            Action::none()
        }
        Message::ToggleReserved2(value) => {
            item.reserved2 = value;
            Action::none()
        }

        // Receipt & Kitchen
        Message::UpdateCustomerReceipt(text) => {
            item.customer_receipt = text;
            Action::none()
        }
        Message::UpdateKitchenVideo(text) => {
            item.kitchen_video = text;
            Action::none()
        }
        Message::UpdateKdsCategory(category) => {
            item.kds_category = category;
            Action::none()
        }
        Message::UpdateKdsCooktime(time_str) => {
            match time_str.parse::<i32>() {
                Ok(time) => {
                    item.kds_cooktime = time;
                    state.validation_error = None;
                }
                Err(_) => {
                    state.validation_error = Some("Invalid cook time format".to_string());
                }
            }
            Action::none()
        }
        Message::UpdateKdsDept(dept_str) => {
            match dept_str.parse::<i32>() {
                Ok(dept) => {
                    item.kds_dept = dept;
                    state.validation_error = None;
                }
                Err(_) => {
                    state.validation_error = Some("Invalid department number".to_string());
                }
            }
            Action::none()
        }

        // Store Settings
        Message::UpdateStoreId(id_str) => {
            match id_str.parse::<i32>() {
                Ok(id) => {
                    item.store_id = id;
                    state.validation_error = None;
                }
                Err(_) => {
                    state.validation_error = Some("Invalid store ID".to_string());
                }
            }
            Action::none()
        }
        Message::UpdateCovers(covers_str) => {
            match covers_str.parse::<i32>() {
                Ok(covers) => {
                    item.covers = covers;
                    state.validation_error = None;
                }
                Err(_) => {
                    state.validation_error = Some("Invalid covers number".to_string());
                }
            }
            Action::none()
        }
        Message::UpdateImageId(id_str) => {
            match id_str.parse::<i32>() {
                Ok(id) => {
                    item.image_id = id;
                    state.validation_error = None;
                }
                Err(_) => {
                    state.validation_error = Some("Invalid image ID".to_string());
                }
            }
            Action::none()
        }
        Message::UpdateLanguageIsoCode(code) => {
            item.language_iso_code = code;
            Action::none()
        }

        // Related Items
        Message::AddChoiceGroup(group_id) => {
            if let Some(groups) = &mut item.choice_groups {
                if !groups.contains(&group_id) {
                    groups.push(group_id);
                }
            } else {
                item.choice_groups = Some(vec![group_id]);
            }
            Action::none()
        }
        Message::RemoveChoiceGroup(group_id) => {
            if let Some(groups) = &mut item.choice_groups {
                groups.retain(|&id| id != group_id);
                if groups.is_empty() {
                    item.choice_groups = None;
                }
            }
            Action::none()
        }
        Message::AddPrinterLogical(printer_id) => {
            if let Some(printers) = &mut item.printer_logicals {
                if !printers.contains(&printer_id) {
                    printers.push(printer_id);
                }
            } else {
                item.printer_logicals = Some(vec![printer_id]);
            }
            Action::none()
        }
        Message::RemovePrinterLogical(printer_id) => {
            if let Some(printers) = &mut item.printer_logicals {
                printers.retain(|&id| id != printer_id);
                if printers.is_empty() {
                    item.printer_logicals = None;
                }
            }
            Action::none()
        }

        // Actions
        Message::Save => {
            if item.validate().is_ok() {
                Action::operation(Operation::Save(item.clone()))
            } else {
                state.validation_error = Some("Please fix validation errors before saving".to_string());
                Action::none()
            }
        }
        Message::Cancel => Action::operation(Operation::Cancel),
    }
}

pub fn view<'a>(
    item: &'a Item,
    state: &'a EditState,
    context: &'a ViewContext<'a>,
) -> Element<'a, Message> {
    let content = container(
        column![
            //basic info
            container(
                column![
                    row![
                        text("Name").width(Length::Fixed(150.0)),
                        text_input("Item Name", &item.name)
                            .on_input(Message::UpdateName)
                            .padding(5)
                    ],
                    row![
                        text("Button 1").width(Length::Fixed(150.0)),
                        text_input("Button Text 1", &item.button1)
                            .on_input(Message::UpdateButton1)
                            .padding(5)
                    ],
                    row![
                        text("Button 2").width(Length::Fixed(150.0)),
                        text_input("Button Text 2", &item.button2.clone().unwrap_or_default())
                            .on_input(Message::UpdateButton2)
                            .padding(5)
                    ],
                    row![
                        text("Printer Text").width(Length::Fixed(150.0)),
                        text_input("Printer Text", &item.printer_text)
                            .on_input(Message::UpdatePrinterText)
                            .padding(5)
                    ],
                ]
                .spacing(10)
            )
            .style(container::rounded_box)
            .padding(20),

            //classifications
            container(
                column![
                    row![
                        text("Item Group").width(Length::Fixed(150.0)),
                        pick_list(
                            context.available_item_groups.values().collect::<Vec<_>>(),
                            state.item_group.and_then(|id| context.available_item_groups.get(&id)),
                            |group: &ItemGroup| Message::SelectItemGroup(Some(group.id))
                        )
                    ],
                    row![
                        text("Product Class").width(Length::Fixed(150.0)),
                        pick_list(
                            context.available_product_classes.values().collect::<Vec<_>>(),
                            state.product_class.and_then(|id| context.available_product_classes.get(&id)),
                            |class: &ProductClass| Message::SelectProductClass(Some(class.id))
                        )
                    ],
                    row![
                        text("Tax Group").width(Length::Fixed(150.0)),
                        pick_list(
                            context.available_tax_groups.values().collect::<Vec<_>>(),
                            state.tax_group.and_then(|id| context.available_tax_groups.get(&id)),
                            |group: &TaxGroup| Message::SelectTaxGroup(Some(group.id))
                        )
                    ],
                    row![
                        text("Security Level").width(Length::Fixed(150.0)),
                        pick_list(
                            context.available_security_levels.values().collect::<Vec<_>>(),
                            state.security_level.and_then(|id| context.available_security_levels.get(&id)),
                            |level: &SecurityLevel| Message::SelectSecurityLevel(Some(level.id))
                        )
                    ],
                    row![
                        text("Revenue Category").width(Length::Fixed(150.0)),
                        pick_list(
                            context.available_revenue_categories.values().collect::<Vec<_>>(),
                            state.revenue_category.and_then(|id| context.available_revenue_categories.get(&id)),
                            |category: &RevenueCategory| Message::SelectRevenueCategory(Some(category.id))
                        )
                    ],
                    row![
                        text("Report Category").width(Length::Fixed(150.0)),
                        pick_list(
                            context.available_report_categories.values().collect::<Vec<_>>(),
                            state.report_category.and_then(|id| context.available_report_categories.get(&id)),
                            |category: &ReportCategory| Message::SelectReportCategory(Some(category.id))
                        )
                    ],
                ]
                .spacing(10)
            )
            .style(container::rounded_box)
            .padding(20),

            //pricing
            container(
                column![
                    row![
                        text("Cost Amount").width(Length::Fixed(150.0)),
                        text_input(
                            "Cost",
                            &item.cost_amount.map_or(String::new(), |c| c.to_string())
                        )
                        .on_input(Message::UpdateCostAmount)
                        .padding(5)
                    ],
                    checkbox(
                        "Ask Price",
                        item.ask_price
                    ).on_toggle(Message::ToggleAskPrice),
                    checkbox(
                        "Allow Price Override",
                        item.allow_price_override,
                    ).on_toggle(Message::ToggleAllowPriceOverride),
                ]
                .spacing(10)
            )
            .style(container::rounded_box)
            .padding(20),

            //flags
            container(
                column![
                    checkbox(
                        "Use Weight",
                        item.use_weight,
                    ).on_toggle(Message::ToggleUseWeight),
                    checkbox(
                        "Print on Check",
                        item.print_on_check,
                    ).on_toggle(Message::TogglePrintOnCheck),
                    checkbox(
                        "Discountable",
                        item.discountable,
                    ).on_toggle(Message::ToggleDiscountable),
                    checkbox(
                        "Voidable",
                        item.voidable,
                        
                    ).on_toggle(Message::ToggleVoidable),
                    checkbox(
                        "Not Active",
                        item.not_active,
                    ).on_toggle(Message::ToggleNotActive),
                    checkbox(
                        "Tax Included",
                        item.tax_included,
                    ).on_toggle(Message::ToggleTaxIncluded),
                    checkbox(
                        "Stock Item",
                        item.stock_item,
                    ).on_toggle(Message::ToggleStockItem),
                ]
                .spacing(10)
            )
            .style(container::rounded_box)
            .padding(20),

            //kitchen info
            container(
                column![
                    row![
                        text("Kitchen Video").width(Length::Fixed(150.0)),
                        text_input("Kitchen Video", &item.kitchen_video)
                            .on_input(Message::UpdateKitchenVideo)
                            .padding(5)
                    ],
                    row![
                        text("KDS Category").width(Length::Fixed(150.0)),
                        text_input("KDS Category", &item.kds_category)
                            .on_input(Message::UpdateKdsCategory)
                            .padding(5)
                    ],
                    row![
                        text("KDS Cook Time").width(Length::Fixed(150.0)),
                        text_input("Cook Time", &item.kds_cooktime.to_string())
                            .on_input(Message::UpdateKdsCooktime)
                            .padding(5)
                    ],
                    row![
                        text("KDS Department").width(Length::Fixed(150.0)),
                        text_input("Department", &item.kds_dept.to_string())
                            .on_input(Message::UpdateKdsDept)
                            .padding(5)
                    ],
                ]
                .spacing(10)
            )
            .style(container::rounded_box)
            .padding(20),

            //related items
            container(
                column![
                    text("Choice Groups").size(16),
                    pick_list(
                        context.available_choice_groups.values().collect::<Vec<_>>(),
                        None::<&ChoiceGroup>, 
                        |group: &ChoiceGroup| Message::AddChoiceGroup(group.id)
                    ),
/*                     if let Some(ref groups) = state.choice_groups {
                        column(
                            groups.iter()
                                .filter_map(|id| context.available_choice_groups.get(id))
                                .map(|group| {
                                    row![
                                        text(&group.name),
                                        button("×")
                                            .on_press(Message::RemoveChoiceGroup(group.id))
                                            .style(button::danger)
                                    ]
                                    .spacing(10)
                                })
                                .collect()
                        )
                    } else {
                        column![ text("No Choice Groups Selected") ]
                        
                    }, */
                    vertical_space(),
                    text("Printer Logicals").size(16),
                    pick_list(
                        context.available_printer_logicals.values().collect::<Vec<_>>(),
                        None::<&PrinterLogical>,
                        |printer: &PrinterLogical| Message::AddPrinterLogical(printer.id)
                    ),
/*                     if let Some(ref printers) = state.printer_logicals {
                        column(
                            printers.iter()
                                .filter_map(|id| context.available_printer_logicals.get(id))
                                .map(|printer| {
                                    row![
                                        text(&printer.name),
                                        button("×")
                                            .on_press(Message::RemovePrinterLogical(printer.id))
                                            .style(button::danger)
                                    ]
                                    .spacing(10)
                                    .into()
                                })
                                .collect()
                        )
                    } else {
                        column![ text("No Printer Logicals Selected") ] 
                    }, */
                ]
                .spacing(10)
            )
            .style(container::rounded_box)
            .padding(20),

            // Show validation error if any
            if let Some(error) = &state.validation_error {
                container(
                    text(error)
                        .style(iced::widget::text::danger)
                )
                .padding(10)
            } else {
                container(
                    text("")
                        .style(iced::widget::text::danger)
                )
                .padding(10)
            },

            //controls
            row![
                horizontal_space(),
                button("Cancel")
                    .on_press(Message::Cancel)
                    .style(button::danger),
                button("Save")
                    .on_press(Message::Save)
                    .style(button::success),
            ]
            .spacing(10)
            .padding(20),

        ].spacing(20)
    ).padding(20);

    scrollable(content).into()
}

pub fn handle_hotkey(hotkey: HotKey) -> Action<Operation, Message> {
    match hotkey {
        HotKey::Escape => Action::operation(Operation::Cancel),
        _ => Action::none(),
    }
}