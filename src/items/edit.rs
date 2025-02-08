use iced::widget::{
    button, checkbox, column, container, pick_list, row, 
    text, text_input, horizontal_space, vertical_space, scrollable
};
use iced::{Element, Length};
use std::collections::HashMap;
use crate::data_types::EntityId;
use crate::{
    choice_groups::ChoiceGroup,
    item_groups::ItemGroup,
    price_levels::PriceLevel,
    printer_logicals::PrinterLogical,
    product_classes::ProductClass,
    report_categories::ReportCategory,
    revenue_categories::RevenueCategory,
    security_levels::SecurityLevel,
    tax_groups::TaxGroup,
};
use crate::HotKey;
use super::{Item, Action, Operation, EditState};

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

pub fn update(item: &mut Item, msg: Message, state: &mut super::EditState) -> super::Action<super::Operation, Message> {
    match msg {
        // Basic Info
        Message::UpdateName(name) => {
            item.name = name;
            super::Action::none()
        }
        Message::UpdateButton1(text) => {
            if text.len() <= 15 {
                item.button1 = text;
                state.validation_error = None;
            } else {
                state.validation_error = Some("Button 1 text cannot exceed 15 characters".to_string());
            }
            super::Action::none()
        }
        Message::UpdateButton2(text) => {
            if text.is_empty() {
                item.button2 = None;
            } else if text.len() <= 15 {
                item.button2 = Some(text);
                state.validation_error = None;
            } else {
                state.validation_error = Some("Button 2 text cannot exceed 15 characters".to_string());
            }
            super::Action::none()
        }
        Message::UpdatePrinterText(text) => {
            item.printer_text = text;
            super::Action::none()
        }

        // Classifications
        Message::SelectItemGroup(group_id) => {
            item.item_group = group_id;
            super::Action::none()
        }
        Message::SelectProductClass(class_id) => {
            item.product_class = class_id;
            super::Action::none()
        }
        Message::SelectRevenueCategory(category_id) => {
            item.revenue_category = category_id;
            super::Action::none()
        }
        Message::SelectTaxGroup(group_id) => {
            item.tax_group = group_id;
            super::Action::none()
        }
        Message::SelectSecurityLevel(level_id) => {
            item.security_level = level_id;
            super::Action::none()
        }
        Message::SelectReportCategory(category_id) => {
            item.report_category = category_id;
            super::Action::none()
        }

        // Pricing
        Message::UpdateCostAmount(amount) => {
            item.cost_amount = if amount.is_empty() {
                None
            } else {
                match amount.parse() {
                    Ok(amount) => Some(amount),
                    Err(_) => {
                        state.validation_error = Some("Invalid cost amount format".to_string());
                        return super::Action::none();
                    }
                }
            };
            super::Action::none()
        }
        Message::ToggleAskPrice(value) => {
            item.ask_price = value;
            super::Action::none()
        }
        Message::ToggleAllowPriceOverride(value) => {
            item.allow_price_override = value;
            super::Action::none()
        }
        Message::AddPriceLevel(level_id) => {
            if let Some(ref mut levels) = item.price_levels {
                if !levels.contains(&level_id) {
                    levels.push(level_id);
                }
            } else {
                item.price_levels = Some(vec![level_id]);
            }
            super::Action::none()
        }
        Message::RemovePriceLevel(level_id) => {
            if let Some(ref mut levels) = item.price_levels {
                levels.retain(|&id| id != level_id);
                if levels.is_empty() {
                    item.price_levels = None;
                }
            }
            super::Action::none()
        }
        Message::UpdateStorePriceLevel(level_id) => {
            if let Some(level_id) = level_id {
                if let Some(ref mut levels) = item.store_price_level {
                    levels.push(level_id);
                } else {
                    item.store_price_level = Some(vec![level_id]);
                }
            }
            super::Action::none()
        }

        // Weight
        Message::ToggleUseWeight(value) => {
            item.use_weight = value;
            if !value {
                item.weight_amount = rust_decimal::Decimal::ZERO;
            }
            super::Action::none()
        }
        Message::UpdateWeightAmount(amount) => {
            match amount.parse() {
                Ok(amount) => {
                    item.weight_amount = amount;
                    state.validation_error = None;
                }
                Err(_) => {
                    state.validation_error = Some("Invalid weight amount format".to_string());
                }
            }
            super::Action::none()
        }

        // Identifiers
        Message::UpdateSku(sku) => {
            item.sku = if sku.is_empty() { None } else { Some(sku) };
            super::Action::none()
        }
        Message::UpdateBarGunCode(code) => {
            item.bar_gun_code = if code.is_empty() { None } else { Some(code) };
            super::Action::none()
        }

        // Flags
        Message::ToggleReserved1(value) => {
            item.reserved1 = value;
            super::Action::none()
        }
        Message::TogglePrintOnCheck(value) => {
            item.print_on_check = value;
            super::Action::none()
        }
        Message::ToggleDiscountable(value) => {
            item.discountable = value;
            super::Action::none()
        }
        Message::ToggleVoidable(value) => {
            item.voidable = value;
            super::Action::none()
        }
        Message::ToggleNotActive(value) => {
            item.not_active = value;
            super::Action::none()
        }
        Message::ToggleTaxIncluded(value) => {
            item.tax_included = value;
            super::Action::none()
        }
        Message::ToggleStockItem(value) => {
            item.stock_item = value;
            super::Action::none()
        }
        Message::ToggleReserved2(value) => {
            item.reserved2 = value;
            super::Action::none()
        }

        // Receipt & Kitchen
        Message::UpdateCustomerReceipt(text) => {
            item.customer_receipt = text;
            super::Action::none()
        }
        Message::UpdateKitchenVideo(text) => {
            item.kitchen_video = text;
            super::Action::none()
        }
        Message::UpdateKdsCategory(text) => {
            item.kds_category = text;
            super::Action::none()
        }
        Message::UpdateKdsCooktime(time) => {
            match time.parse() {
                Ok(time) => {
                    item.kds_cooktime = time;
                    state.validation_error = None;
                }
                Err(_) => {
                    state.validation_error = Some("Invalid cook time format".to_string());
                }
            }
            super::Action::none()
        }
        Message::UpdateKdsDept(dept) => {
            match dept.parse() {
                Ok(dept) => {
                    item.kds_dept = dept;
                    state.validation_error = None;
                }
                Err(_) => {
                    state.validation_error = Some("Invalid department number".to_string());
                }
            }
            super::Action::none()
        }

        // Store Settings
        Message::UpdateStoreId(id) => {
            match id.parse() {
                Ok(id) => {
                    item.store_id = id;
                    state.validation_error = None;
                }
                Err(_) => {
                    state.validation_error = Some("Invalid store ID".to_string());
                }
            }
            super::Action::none()
        }
        Message::UpdateCovers(covers) => {
            match covers.parse() {
                Ok(covers) => {
                    item.covers = covers;
                    state.validation_error = None;
                }
                Err(_) => {
                    state.validation_error = Some("Invalid covers number".to_string());
                }
            }
            super::Action::none()
        }
        Message::UpdateImageId(id) => {
            match id.parse() {
                Ok(id) => {
                    item.image_id = id;
                    state.validation_error = None;
                }
                Err(_) => {
                    state.validation_error = Some("Invalid image ID".to_string());
                }
            }
            super::Action::none()
        }
        Message::UpdateLanguageIsoCode(code) => {
            item.language_iso_code = code;
            super::Action::none()
        }

        // Related Items
        Message::AddChoiceGroup(group_id) => {
            if let Some(ref mut groups) = item.choice_groups {
                if !groups.contains(&group_id) {
                    groups.push(group_id);
                }
            } else {
                item.choice_groups = Some(vec![group_id]);
            }
            super::Action::none()
        }
        Message::RemoveChoiceGroup(group_id) => {
            if let Some(ref mut groups) = item.choice_groups {
                groups.retain(|&id| id != group_id);
                if groups.is_empty() {
                    item.choice_groups = None;
                }
            }
            super::Action::none()
        }
        Message::AddPrinterLogical(printer_id) => {
            if let Some(ref mut printers) = item.printer_logicals {
                if !printers.contains(&printer_id) {
                    printers.push(printer_id);
                }
            } else {
                item.printer_logicals = Some(vec![printer_id]);
            }
            super::Action::none()
        }
        Message::RemovePrinterLogical(printer_id) => {
            if let Some(ref mut printers) = item.printer_logicals {
                printers.retain(|&id| id != printer_id);
                if printers.is_empty() {
                    item.printer_logicals = None;
                }
            }
            super::Action::none()
        }

        Message::Save => super::Action::operation(super::Operation::Save(item.clone())),
        Message::Cancel => super::Action::operation(super::Operation::Cancel),
    }
}

pub fn view<'a>(
    item: &'a Item,
    state: EditState,
    item_groups: &'a HashMap<EntityId, ItemGroup>,
    tax_groups: &'a HashMap<EntityId, TaxGroup>,
    security_levels: &'a HashMap<EntityId, SecurityLevel>,
    revenue_categories: &'a HashMap<EntityId, RevenueCategory>,
    report_categories: &'a HashMap<EntityId, ReportCategory>,
    product_classes: &'a HashMap<EntityId, ProductClass>,
    choice_groups: &'a HashMap<EntityId, ChoiceGroup>,
    printer_logicals: &'a HashMap<EntityId, PrinterLogical>,
    price_levels: &'a HashMap<EntityId, PriceLevel>,
) -> Element<'a, Message> {

    let validation_error = &state.validation_error;

    let content = container(
        column![
            // Basic Info Section
            container(
                column![
                    text("Basic Information").size(16),
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

            // Classifications Section
            container(
                column![
                    text("Classifications").size(16),
                    row![
                        text("Item Group").width(Length::Fixed(150.0)),
                        pick_list(
                            item_groups.values().collect::<Vec<_>>(),
                            item.item_group.and_then(|id| item_groups.get(&id)),
                            |group: &ItemGroup| Message::SelectItemGroup(Some(group.id))
                        )
                    ],
                    row![
                        text("Product Class").width(Length::Fixed(150.0)),
                        pick_list(
                            product_classes.values().collect::<Vec<_>>(),
                            item.product_class.and_then(|id| product_classes.get(&id)),
                            |product_class: &ProductClass| Message::SelectProductClass(Some(product_class.id))
                        )
                    ],
                    row![
                        text("Revenue Category").width(Length::Fixed(150.0)),
                        pick_list(
                            revenue_categories.values().collect::<Vec<_>>(),
                            item.revenue_category.and_then(|id| revenue_categories.get(&id)),
                            |revenue_category: &RevenueCategory| Message::SelectRevenueCategory(Some(revenue_category.id))
                        )
                    ],
                    row![
                        text("Tax Group").width(Length::Fixed(150.0)),
                        pick_list(
                            tax_groups.values().collect::<Vec<_>>(),
                            item.tax_group.and_then(|id| tax_groups.get(&id)),
                            |tax_group: &TaxGroup| Message::SelectTaxGroup(Some(tax_group.id))
                        )
                    ],
                    row![
                        text("Security Level").width(Length::Fixed(150.0)),
                        pick_list(
                            security_levels.values().collect::<Vec<_>>(),
                            item.security_level.and_then(|id| security_levels.get(&id)),
                            |secureity_level| Message::SelectSecurityLevel(Some(secureity_level.id))
                        )
                    ],
                    row![
                        text("Report Category").width(Length::Fixed(150.0)),
                        pick_list(
                            report_categories.values().collect::<Vec<_>>(),
                            item.report_category.and_then(|id| report_categories.get(&id)),
                            |report_category: &ReportCategory| Message::SelectReportCategory(Some(report_category.id))
                        )
                    ],
                ]
                .spacing(10)
            )
            .style(container::rounded_box)
            .padding(20),

            // Pricing Section
            container(
                column![
                    text("Pricing").size(16),
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
                        item.allow_price_override
                    ).on_toggle(Message::ToggleAllowPriceOverride),
                ]
                .spacing(10)
            )
            .style(container::rounded_box)
            .padding(20),

            // Weight Section
            container(
                column![
                    text("Weight").size(16),
                    checkbox(
                        "Use Weight",
                        item.use_weight
                    ).on_toggle(Message::ToggleUseWeight),
                    row![
                        text("Weight Amount").width(Length::Fixed(150.0)),
                        text_input(
                            "Weight",
                            &item.weight_amount.to_string()
                        )
                        .on_input(Message::UpdateWeightAmount)
                        .padding(5)
                    ],
                ]
                .spacing(10)
            )
            .style(container::rounded_box)
            .padding(20),

            // Flags Section
            container(
                column![
                    text("Flags").size(16),
                    checkbox(
                        "Print on Check",
                        item.print_on_check
                    ).on_toggle(Message::TogglePrintOnCheck),
                    checkbox(
                        "Discountable",
                        item.discountable
                    ).on_toggle(Message::ToggleDiscountable),
                    checkbox(
                        "Voidable",
                        item.voidable
                    ).on_toggle(Message::ToggleVoidable),
                    checkbox(
                        "Not Active",
                        item.not_active
                    ).on_toggle(Message::ToggleNotActive),
                    checkbox(
                        "Tax Included",
                        item.tax_included
                    ).on_toggle(Message::ToggleTaxIncluded),
                    checkbox(
                        "Stock Item",
                        item.stock_item
                    ).on_toggle(Message::ToggleStockItem),
                ]
                .spacing(10)
            )
            .style(container::rounded_box)
            .padding(20),

            // Kitchen Info Section
            container(
                column![
                    text("Kitchen Information").size(16),
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

            // Store Settings Section
            container(
                column![
                    text("Store Settings").size(16),
                    row![
                        text("Store ID").width(Length::Fixed(150.0)),
                        text_input("Store ID", &item.store_id.to_string())
                            .on_input(Message::UpdateStoreId)
                            .padding(5)
                    ],
                    row![
                        text("Covers").width(Length::Fixed(150.0)),
                        text_input("Covers", &item.covers.to_string())
                            .on_input(Message::UpdateCovers)
                            .padding(5)
                    ],
                    row![
                        text("Image ID").width(Length::Fixed(150.0)),
                        text_input("Image ID", &item.image_id.to_string())
                            .on_input(Message::UpdateImageId)
                            .padding(5)
                    ],
                    row![
                        text("Language ISO Code").width(Length::Fixed(150.0)),
                        text_input("Language Code", &item.language_iso_code)
                            .on_input(Message::UpdateLanguageIsoCode)
                            .padding(5)
                    ],
                ]
                .spacing(10)
            )
            .style(container::rounded_box)
            .padding(20),

            // Validation Error
            if let Some(error) = validation_error {
                container(
                    text(error.to_string()).style(text::danger)
                )
                .padding(10)
            } else {
                container(vertical_space())
            },

            // Action Buttons
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
        ]
        .spacing(20)
    );

    container(content).into()
}

pub fn handle_hotkey(hotkey: HotKey) -> Action<Operation, Message> {
    match hotkey {
        HotKey::Escape => Action::operation(Operation::Cancel),
        _ => Action::none(),
    }
}