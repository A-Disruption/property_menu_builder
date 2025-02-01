use iced::widget::{
    button, checkbox, column, container, pick_list, row, 
    text, text_input, horizontal_space, vertical_space,
};
use iced::{Element, Length};
use std::collections::HashMap;
use crate::data_types::{EntityId, Currency};
use crate::{
    tax_groups::TaxGroup,
    security_levels::SecurityLevel,
    revenue_categories::RevenueCategory,
    report_categories::ReportCategory,
    item_groups::ItemGroup,
    product_classes::ProductClass,
    choice_groups::ChoiceGroup,
    printer_logicals::PrinterLogical,
    price_levels::PriceLevel,
};
use crate::HotKey;
use super::{Item, Action, Operation};

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

pub struct EditState<'a> {
    available_item_groups: &'a HashMap<EntityId, ItemGroup>,
    available_tax_groups: &'a HashMap<EntityId, TaxGroup>,
    available_security_levels: &'a HashMap<EntityId, SecurityLevel>,
    available_revenue_categories: &'a HashMap<EntityId, RevenueCategory>,
    available_report_categories: &'a HashMap<EntityId, ReportCategory>,
    available_product_classes: &'a HashMap<EntityId, ProductClass>,
    available_choice_groups: &'a HashMap<EntityId, ChoiceGroup>,
    available_printer_logicals: &'a HashMap<EntityId, PrinterLogical>,
    available_price_levels: &'a HashMap<EntityId, PriceLevel>,
    validation_error: Option<String>,
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

pub fn view(item: &Item, state: &EditState) -> Element<Message> {
    let basic_info = container(
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
    .padding(20);

    let classifications = container(
        column![
            row![
                text("Item Group").width(Length::Fixed(150.0)),
                pick_list(
                    &state.available_item_groups.values().collect::<Vec<_>>(),
                    item.item_group.and_then(|id| state.available_item_groups.get(&id)),
                    Message::SelectItemGroup
                )
            ],
            row![
                text("Product Class").width(Length::Fixed(150.0)),
                pick_list(
                    &state.available_product_classes.values().collect::<Vec<_>>(),
                    item.product_class.and_then(|id| state.available_product_classes.get(&id)),
                    Message::SelectProductClass
                )
            ],
            row![
                text("Tax Group").width(Length::Fixed(150.0)),
                pick_list(
                    &state.available_tax_groups.values().collect::<Vec<_>>(),
                    item.tax_group.and_then(|id| state.available_tax_groups.get(&id)),
                    Message::SelectTaxGroup
                )
            ],
            row![
                text("Security Level").width(Length::Fixed(150.0)),
                pick_list(
                    &state.available_security_levels.values().collect::<Vec<_>>(),
                    item.security_level.and_then(|id| state.available_security_levels.get(&id)),
                    Message::SelectSecurityLevel
                )
            ],
            row![
                text("Revenue Category").width(Length::Fixed(150.0)),
                pick_list(
                    &state.available_revenue_categories.values().collect::<Vec<_>>(),
                    item.revenue_category.and_then(|id| state.available_revenue_categories.get(&id)),
                    Message::SelectRevenueCategory
                )
            ],
            row![
                text("Report Category").width(Length::Fixed(150.0)),
                pick_list(
                    &state.available_report_categories.values().collect::<Vec<_>>(),
                    item.report_category.and_then(|id| state.available_report_categories.get(&id)),
                    Message::SelectReportCategory
                )
            ],
        ]
        .spacing(10)
    )
    .style(container::rounded_box)
    .padding(20);

    let pricing = container(
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
    .padding(20);

    let flags = container(
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
    .padding(20);

    let kitchen_info = container(
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
    .padding(20);

    let related_items = container(
        column![
            text("Choice Groups").size(16),
            pick_list(
                &state.available_choice_groups.values().collect::<Vec<_>>(),
                None,
                Message::AddChoiceGroup
            ),
            if let Some(ref groups) = item.choice_groups {
                column(
                    groups.iter()
                        .filter_map(|id| state.available_choice_groups.get(id))
                        .map(|group| {
                            row![
                                text(&group.name),
                                button("×")
                                    .on_press(Message::RemoveChoiceGroup(group.id))
                                    .style(button::danger)
                            ]
                            .spacing(10)
                            .into()
                        })
                        .collect()
                ).into()
            } else {
                text("No Choice Groups Selected").into()
            },
            vertical_space(),
            text("Printer Logicals").size(16),
            pick_list(
                &state.available_printer_logicals.values().collect::<Vec<_>>(),
                None,
                Message::AddPrinterLogical
            ),
            if let Some(ref printers) = item.printer_logicals {
                column(
                    printers.iter()
                        .filter_map(|id| state.available_printer_logicals.get(id))
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
                ).into()
            } else {
                text("No Printer Logicals Selected").into()
            },
        ]
        .spacing(10)
    )
    .style(container::rounded_box)
    .padding(20);

    let controls = row![
        horizontal_space(),
        button("Cancel")
            .on_press(Message::Cancel)
            .style(button::danger),
        button("Save")
            .on_press(Message::Save)
            .style(button::success),
    ]
    .spacing(10)
    .padding(20);

    if let Some(error) = &state.validation_error {
        container(
            column![
                text(error).style(iced::widget::text::danger),
                controls
            ]
            .spacing(20)
        )
        .padding(20)
        .into()
    } else {
        container(
            column![
                basic_info,
                classifications,
                pricing,
                flags,
                kitchen_info,
                related_items,
                controls,
            ]
            .spacing(20)
        )
        .padding(20)
        .into()
    }
}

pub fn handle_hotkey(hotkey: HotKey) -> Action<Operation, Message> {
    match hotkey {
        HotKey::Escape => Action::operation(Operation::Cancel),
        _ => Action::none(),
    }
}