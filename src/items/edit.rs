use iced::widget::text::LineHeight;
use iced::widget::{
    button, checkbox, column, combo_box, container, pick_list, row, 
    text, text_input, horizontal_space, scrollable
};
use iced_modern_theme::Modern;
use iced::{Element, Length};
use std::collections::BTreeMap;
use crate::data_types::{EntityId, ItemPrice};
use crate::data_types;
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
    icon,
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
    ChoiceGroupSelected(EntityId),
    PriceLevelSelected(EntityId),
    PrinterLogicalSelected(EntityId),

    // Pricing
    UpdateCostAmount(String),
    ToggleAskPrice(bool),
    ToggleAllowPriceOverride(bool),
    AddPriceLevel(EntityId),
    RemovePriceLevel(EntityId),
    UpdateStorePriceLevel(Option<EntityId>),
    UpdatePrice(EntityId, String),

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

pub fn view<'a>(
    item: &'a Item,
    state: &'a EditState,
    item_groups: &'a BTreeMap<EntityId, ItemGroup>,
    tax_groups: &'a BTreeMap<EntityId, TaxGroup>,
    security_levels: &'a BTreeMap<EntityId, SecurityLevel>,
    revenue_categories: &'a BTreeMap<EntityId, RevenueCategory>,
    report_categories: &'a BTreeMap<EntityId, ReportCategory>,
    product_classes: &'a BTreeMap<EntityId, ProductClass>,
    choice_groups: &'a BTreeMap<EntityId, ChoiceGroup>,
    printer_logicals: &'a BTreeMap<EntityId, PrinterLogical>,
    price_levels: &'a BTreeMap<EntityId, PriceLevel>,
) -> Element<'a, Message> {
    let header = row![
        button(icon::save().size(14)).on_press(Message::Save).style(Modern::primary_button()),
        button(icon::cancel().size(14)).on_press(Message::Cancel).style(Modern::danger_button()),
        horizontal_space().width(4),
    ]
    .spacing(10);

    let validation_error = &state.validation_error;

    let basic_info = container(
        column![
            row![
                column![
                    text("Item Name").style(Modern::primary_text()),
                    text_input("Item Name", &item.name)
                        .on_input(Message::UpdateName)
                        .style(Modern::inline_text_input())
                        .width(200)
                        .padding(5)
                ].spacing(10).padding(10),
                column![
                    text("Base Price").style(Modern::primary_text()),
                    text_input(
                        "Base Price",
                        &item.cost_amount.map_or(String::new(), |c| c.to_string())
                    )
                    .on_input(Message::UpdateCostAmount)
                    .style(Modern::inline_text_input())
                    .width(200)
                    .padding(5)
                ].spacing(10).padding(10),
            ].wrap(),
            row![
                column![
                    text("Button Text 1").style(Modern::primary_text()),
                    text_input("Button Text 1", &item.button1)
                        .on_input(Message::UpdateButton1)
                        .style(Modern::inline_text_input())
                        .width(200)
                        .padding(5)
                ].spacing(10).padding(10),
                column![
                    text("Button Text 2").style(Modern::primary_text()),
                    text_input("Button Text 2", &item.button2.clone().unwrap_or_default())
                        .on_input(Message::UpdateButton2)
                        .style(Modern::inline_text_input())
                        .width(200)
                        .padding(5)
                ].spacing(10).padding(10),
                column![
                    text("Customer Receipt Text").style(Modern::primary_text()),
                    text_input(
                        "Customer Receipt Text", 
                        &item.customer_receipt
                    )
                    .on_input(Message::UpdateCustomerReceipt)
                    .style(Modern::inline_text_input())
                    .width(200)
                    .padding(5)
                ].spacing(10).padding(10),
            ],
            row![
                column![
                    text("Kitchen Printer Text").style(Modern::primary_text()),
                    text_input(
                        "Kitchen Printer Text", 
                        &item.printer_text
                    )
                    .on_input(Message::UpdatePrinterText)
                    .style(Modern::inline_text_input())
                    .width(200)
                    .padding(5)
                ].spacing(10).padding(10),
                column![
                    text("Kitchen Video Text").style(Modern::primary_text()),
                    text_input(
                        "Kitchen Video Text", 
                        &item.kitchen_video
                    )
                    .on_input(Message::UpdateKitchenVideo)
                    .style(Modern::inline_text_input())
                    .width(200)
                    .padding(5)
                ].spacing(10).padding(10)
            ].wrap(),
        ]
    )
    .style(Modern::sheet_container())
    .width(Length::Fill)
    .padding(10);

    let classifications = container(
        column![
            row![
                column![
                    text("Item Group").style(Modern::primary_text()),
                    pick_list(
                        item_groups.values().collect::<Vec<_>>(),
                        item.item_group.and_then(|id| item_groups.get(&id)),
                        |group: &ItemGroup| Message::SelectItemGroup(Some(group.id))
                    ).width(200).style(Modern::pick_list())
                ].spacing(10).padding(10),
                column![
                    text("Product Class").style(Modern::primary_text()),
                    pick_list(
                        product_classes.values().collect::<Vec<_>>(),
                        item.product_class.and_then(|id| product_classes.get(&id)),
                        |product_class: &ProductClass| Message::SelectProductClass(Some(product_class.id))
                    ).width(200).style(Modern::pick_list())
                ].spacing(10).padding(10),
                column![
                    text("Revenue Category").style(Modern::primary_text()),
                    pick_list(
                        revenue_categories.values().collect::<Vec<_>>(),
                        item.revenue_category.and_then(|id| revenue_categories.get(&id)),
                        |revenue_category: &RevenueCategory| Message::SelectRevenueCategory(Some(revenue_category.id))
                    ).width(200).style(Modern::pick_list())
                ].spacing(10).padding(10),
            ].wrap(),
            row![
                column![
                    text("Tax Group").style(Modern::primary_text()),
                    pick_list(
                        tax_groups.values().collect::<Vec<_>>(),
                        item.tax_group.and_then(|id| tax_groups.get(&id)),
                        |tax_group: &TaxGroup| Message::SelectTaxGroup(Some(tax_group.id))
                    ).width(200).style(Modern::pick_list())
                ].spacing(10).padding(10),
                column![
                    text("Security Level").style(Modern::primary_text()),
                    pick_list(
                        security_levels.values().collect::<Vec<_>>(),
                        item.security_level.and_then(|id| security_levels.get(&id)),
                        |security_level| Message::SelectSecurityLevel(Some(security_level.id))
                    ).width(200).style(Modern::pick_list())
                ].spacing(10).padding(10),
                column![
                    text("Report Category").style(Modern::primary_text()),
                    pick_list(
                        report_categories.values().collect::<Vec<_>>(),
                        item.report_category.and_then(|id| report_categories.get(&id)),
                        |report_category: &ReportCategory| Message::SelectReportCategory(Some(report_category.id))
                    ).width(200).style(Modern::pick_list())
                ].spacing(10).padding(10),
            ].wrap(),
            row![
                column![
                    checkbox(
                        "Sold by weight".to_string(),
                        item.use_weight
                    )
                    .on_toggle(Message::ToggleUseWeight)
                        .width(200)
                        .spacing(10)
                        .style(Modern::checkbox()),
                ].spacing(10).padding(10),

                column![
                    row![
                        horizontal_space().width(5),
                        text("Tar Weight").style(Modern::primary_text()),
                    ],
                    row![
                        horizontal_space().width(5),
                        text_input(
                            "Weight",
                            &item.weight_amount.to_string()
                        )
                        .on_input(Message::UpdateWeightAmount)
                        .style(Modern::inline_text_input())
                        .padding(5)
                        .width(200)
                    ],
                ].spacing(10).padding(10),
            ]
            .wrap(),
        ]
        .width(Length::Fill)
    )
    .style(Modern::sheet_container())
    .width(Length::Fill)
    .padding(10);

/*     let weight_info = container(
        column![
            text("Weight").size(16).style(iced::widget::Modern::primary_text()),
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
    )
    .style(Modern::card_container())
    .width(Length::Fill)
    .padding(10); */

    let flags = container(
        column![
            column![
                row![
                    checkbox(
                        "Print on Check",
                        item.print_on_check
                    )
                    .on_toggle(Message::TogglePrintOnCheck)
                    .style(Modern::checkbox())
                    .spacing(10)
                    .width(200),
                    checkbox(
                        "Discountable",
                        item.discountable
                    )
                    .on_toggle(Message::ToggleDiscountable)
                    .style(Modern::checkbox())
                    .spacing(10)
                    .width(200),
                    checkbox(
                        "Voidable",
                        item.voidable
                    )
                    .on_toggle(Message::ToggleVoidable)
                    .style(Modern::checkbox())
                    .spacing(10)
                    .width(200),
                ].wrap(),
            ],
            column![
                row![
                    checkbox(
                        "Active",
                        item.not_active
                    )
                    .on_toggle(Message::ToggleNotActive)
                    .style(Modern::checkbox())
                    .spacing(10)
                    .width(200),
                    checkbox(
                        "Tax Included",
                        item.tax_included
                    )
                    .on_toggle(Message::ToggleTaxIncluded)
                    .style(Modern::checkbox())
                    .spacing(10)
                    .width(200),
                    checkbox(
                        "Stock Item",
                        item.stock_item
                    )
                    .on_toggle(Message::ToggleStockItem)
                    .style(Modern::checkbox())
                    .spacing(10)
                    .width(200),
                ].wrap(),
            ],
            column![
                row![
                    checkbox(
                        "Prompt for price",
                        item.ask_price
                    )
                    .on_toggle(Message::ToggleAskPrice)
                    .style(Modern::checkbox())
                    .spacing(10)
                    .width(200),
                    checkbox(
                        "Allow price override",
                        item.allow_price_override
                    )
                    .on_toggle(Message::ToggleAllowPriceOverride)
                    .style(Modern::checkbox())
                    .spacing(10)
                    .width(200),
                ].wrap(),
            ],
        ]
    )
    .style(Modern::sheet_container())
    .width(Length::Fill)
    .padding(10);

/*     let kitchen_info = container(
        column![
/*       row![
            text("Kitchen Video").style(text::secondary),
            text_input("Kitchen Video", &item.kitchen_video)
                .on_input(Message::UpdateKitchenVideo)
                .padding(5)
        ], */
        row![
            text("KDS Category").style(text::secondary),
            text_input("KDS Category", &item.kds_category)
                .on_input(Message::UpdateKdsCategory)
                .padding(5)
        ],
        row![
            text("KDS Cook Time").style(text::secondary),
            text_input("Cook Time", &item.kds_cooktime.to_string())
                .on_input(Message::UpdateKdsCooktime)
                .padding(5)
        ],
        row![
            text("KDS Department").style(text::secondary),
            text_input("Department", &item.kds_dept.to_string())
                .on_input(Message::UpdateKdsDept)
                .padding(5)
        ],
    ]
    )
    .style(Modern::card_container())
    .width(Length::Fill)
    .padding(10); */

/*     let store_info = container(
        column![
            row![
                text("Store ID").style(text::secondary),
                text_input("Store ID", &item.store_id.to_string())
                    .on_input(Message::UpdateStoreId)
                    .padding(5)
            ],
            row![
                text("Covers").style(text::secondary),
                text_input("Covers", &item.covers.to_string())
                    .on_input(Message::UpdateCovers)
                    .padding(5)
            ],
            row![
                text("Image ID").style(text::secondary),
                text_input("Image ID", &item.image_id.to_string())
                    .on_input(Message::UpdateImageId)
                    .padding(5)
            ],
            row![
                text("Language ISO Code").style(text::secondary),
                text_input("Language Code", &item.language_iso_code)
                    .on_input(Message::UpdateLanguageIsoCode)
                    .padding(5)
            ],
        ]
    )
    .style(Modern::card_container())
    .width(Length::Fill)
    .padding(10); */

    let choice_groups = container(
        column![
            column![
                text("Choice Groups").style(Modern::primary_text()),
                iced::widget::horizontal_space().height(5),
                combo_box(
                    &state.choice_groups_combo,
                    "Add Choice Group",
                    state.choice_group_selection.as_ref(),
                    |choice_group: ChoiceGroup| Message::ChoiceGroupSelected(choice_group.id)
                )
                .input_style(Modern::combo_box())
                .menu_style(Modern::combo_box_menu())
                .width(200),
            ].spacing(5),
            iced::widget::horizontal_space().height(5),
            row![ // Display Selected Choice Groups
                if let Some(selected_groups) = &item.choice_groups {
                    row(
                        selected_groups
                            .iter()
                            .filter_map(|id| choice_groups.get(id))
                            .map(|group| {
                                container(
                                    button(
                                        row![
                                            text(&group.name),
                                        ].spacing(10)
                                    )
                                    .on_press(Message::RemoveChoiceGroup(group.id))
                                    .style(Modern::gray_button())
                                    .width(Length::Shrink)
                                ).padding(5).into()
                            })
                            .collect::<Vec<_>>()
                    ).wrap()
                } else {
                    row![].wrap()
                }
            ],
        ],
    )
    .style(Modern::sheet_container())
    .width(Length::Fill)
    .padding(10);

    let printer_info = container(
        column![
            column![
                text("Printer Logicals").style(Modern::primary_text()),
                iced::widget::horizontal_space().height(5),
                combo_box(
                    &state.printer_logicals_combo,
                    "Add Printer Logical",
                    state.printer_logicals_selection.as_ref(),
                    |printer_logical: PrinterLogical| Message::PrinterLogicalSelected(printer_logical.id)
                )
                .input_style(Modern::combo_box())
                .menu_style(Modern::combo_box_menu())
                .width(200),
            ].spacing(5),
            iced::widget::horizontal_space().height(5),
            row![ // Display Selected Printer Logicals
            if let Some(selected_logicals) = &item.printer_logicals {
                row(
                    selected_logicals
                        .iter()
                        .filter_map(|id| printer_logicals.get(id))
                        .map(|logical| {
                            container(
                                button(
                                    row![
                                        text(&logical.name),
                                    ].spacing(10)
                                )
                                .on_press(Message::RemovePrinterLogical(logical.id))
                                .style(Modern::gray_button())
                                .width(Length::Shrink)
                            ).padding(5).into()
                        })
                        .collect::<Vec<_>>()
                ).wrap()
            } else {
                row![].wrap()
            }
            ],
        ],
    )
    .style(Modern::sheet_container())
    .width(Length::Fill)
    .padding(10);

    // temp variables for pricing,
    let assigned_price_level_ids = item.price_levels.clone().unwrap_or_default();
    let available_price_levels: Vec<PriceLevel> = price_levels.iter().filter(|(id, _)| !assigned_price_level_ids.contains(id) ).map(|(_, price_level)| price_level.clone()).collect();

    let pricing = container(
        column![
            text("Price Levels").style(Modern::primary_text()),
            iced::widget::horizontal_space().height(10),

            row![ // Display Selected Price Levels
            if let Some(selected_prices) = &item.price_levels {
                row(
                    selected_prices
                        .iter()
                        .filter_map(|id| price_levels.get(id))
                        .map( |price_level| {
                            // Look up the current price from state.prices using the price_level id.
                            // If no matching price is found, default to an empty string.
                            let current_price = state.prices.as_ref()
                            .and_then(|all_prices| {
                                all_prices.iter()
                                    .find(|(id, _)| *id == price_level.id)
                                    .map(|(_, price_str)| price_str.as_str())
                            })
                            .unwrap_or("");

                            row![
                                text(&price_level.name).width(75),
                                text_input("Price", current_price)
                                    .on_input( |price|
                                        Message::UpdatePrice(price_level.id, price)
                                    )
                                    .style(Modern::inline_text_input())
                                    .width(125),
                                    horizontal_space().width(10),
                                button(icon::trash().size(14)).on_press(Message::RemovePriceLevel(price_level.id)).style(Modern::danger_button()),
                                horizontal_space().width(10),
                            ].align_y(iced::Alignment::Center).into()
                        })
                        .collect::<Vec<_>>()
                ).width(900).wrap()
            } else {
                row![].wrap()
            }
            ],
            iced::widget::horizontal_space().height(5),
            row![
                pick_list(
                    available_price_levels,
                    None::<PriceLevel>,
                    |price_level: PriceLevel| Message::PriceLevelSelected(price_level.id)
                )
                .width(75)
                .placeholder("Add Price Levels")
                .style(Modern::pick_list())
            ].spacing(5),
        ],
    )
    .style(Modern::sheet_container())
    .width(Length::Fill)
    .padding(10);

/*     // Validation Error
    if let Some(error) = validation_error {
        container(
            text(error.to_string()).style(text::danger)
        )
        .padding(10)
    } else {
        container(text("".to_string()))
    } */

    container(
        column![
            header,
            scrollable(
                column![
                    basic_info,
                    classifications,
                    //weight_info,
                    flags,
                    //kitchen_info,
                    //store_info,
                    choice_groups,
                    printer_info,
                    pricing,
                ]
                .spacing(20)
            )
            .spacing(10)
            .width(Length::Fill)
            .height(Length::Fill)
        ]
        .spacing(20)
    )
    .padding(10)
    .into()
}

pub fn handle_hotkey(hotkey: HotKey) -> Action<Operation, Message> {
    match hotkey {
        HotKey::Escape => Action::operation(Operation::Cancel),
        _ => Action::none(),
    }
}
