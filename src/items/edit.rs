use iced::widget::{
    button, checkbox, column, combo_box, container, pick_list, row, 
    text, text_input, horizontal_space, scrollable
};
use iced::{Element, Length};
use std::collections::BTreeMap;
use crate::data_types::EntityId;
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
        button(icon::save().shaping(text::Shaping::Advanced)).on_press(Message::Save).width(40).style(button::primary),
        button(icon::cancel().shaping(text::Shaping::Advanced)).on_press(Message::Cancel).style(button::danger),
        horizontal_space().width(4),
    ]
    .spacing(10)
    .padding(10);

    let validation_error = &state.validation_error;

    let basic_info = container(
        column![
            row![
                column![
                    text("Item Name").style(text::primary),
                    text_input("Item Name", &item.name)
                        .on_input(Message::UpdateName)
                        .width(200)
                        .padding(5)
                ].spacing(10).padding(10),
                column![
                    text("Base Price").style(text::primary),
                    text_input(
                        "Base Price",
                        &item.cost_amount.map_or(String::new(), |c| c.to_string())
                    )
                    .on_input(Message::UpdateCostAmount)
                    .width(200)
                    .padding(5)
                ].spacing(10).padding(10),
            ].wrap(),
            row![
                column![
                    text("Button Text 1").style(text::primary),
                    text_input("Button Text 1", &item.button1)
                        .on_input(Message::UpdateButton1)
                        .width(200)
                        .padding(5)
                ].spacing(10).padding(10),
                column![
                    text("Button Text 2").style(text::primary),
                    text_input("Button Text 2", &item.button2.clone().unwrap_or_default())
                        .on_input(Message::UpdateButton2)
                        .width(200)
                        .padding(5)
                ].spacing(10).padding(10),
                column![
                    text("Customer Receipt Text").style(text::primary),
                    text_input(
                        "Customer Receipt Text", 
                        &item.customer_receipt
                    )
                    .on_input(Message::UpdateCustomerReceipt)
                    .width(200)
                    .padding(5)
                ].spacing(10).padding(10),
            ],
            row![
                column![
                    text("Kitchen Printer Text").style(text::primary),
                    text_input(
                        "Kitchen Printer Text", 
                        &item.printer_text
                    )
                    .on_input(Message::UpdatePrinterText)
                    .width(200)
                    .padding(5)
                ].spacing(10).padding(10),
                column![
                    text("Kitchen Video Text").style(text::primary),
                    text_input(
                        "Kitchen Video Text", 
                        &item.kitchen_video
                    )
                    .on_input(Message::UpdateKitchenVideo)
                    .width(200)
                    .padding(5)
                ].spacing(10).padding(10)
            ].wrap(),
            iced::widget::horizontal_rule(5),
        ]
    )
    .style(container::rounded_box)
    .width(Length::Fill)
    .padding(10);

    let classifications = container(
        column![
            row![
                column![
                    text("Item Group").style(text::primary),
                    pick_list(
                        item_groups.values().collect::<Vec<_>>(),
                        item.item_group.and_then(|id| item_groups.get(&id)),
                        |group: &ItemGroup| Message::SelectItemGroup(Some(group.id))
                    ).width(200)
                ].spacing(10).padding(10),
                column![
                    text("Product Class").style(text::primary),
                    pick_list(
                        product_classes.values().collect::<Vec<_>>(),
                        item.product_class.and_then(|id| product_classes.get(&id)),
                        |product_class: &ProductClass| Message::SelectProductClass(Some(product_class.id))
                    ).width(200)
                ].spacing(10).padding(10),
                column![
                    text("Revenue Category").style(text::primary),
                    pick_list(
                        revenue_categories.values().collect::<Vec<_>>(),
                        item.revenue_category.and_then(|id| revenue_categories.get(&id)),
                        |revenue_category: &RevenueCategory| Message::SelectRevenueCategory(Some(revenue_category.id))
                    ).width(200)
                ].spacing(10).padding(10),
            ].wrap(),
            row![
                column![
                    text("Tax Group").style(text::primary),
                    pick_list(
                        tax_groups.values().collect::<Vec<_>>(),
                        item.tax_group.and_then(|id| tax_groups.get(&id)),
                        |tax_group: &TaxGroup| Message::SelectTaxGroup(Some(tax_group.id))
                    ).width(200)
                ].spacing(10).padding(10),
                column![
                    text("Security Level").style(text::primary),
                    pick_list(
                        security_levels.values().collect::<Vec<_>>(),
                        item.security_level.and_then(|id| security_levels.get(&id)),
                        |security_level| Message::SelectSecurityLevel(Some(security_level.id))
                    ).width(200)
                ].spacing(10).padding(10),
                column![
                    text("Report Category").style(text::primary),
                    pick_list(
                        report_categories.values().collect::<Vec<_>>(),
                        item.report_category.and_then(|id| report_categories.get(&id)),
                        |report_category: &ReportCategory| Message::SelectReportCategory(Some(report_category.id))
                    ).width(200)
                ].spacing(10).padding(10),
            ].wrap(),
            row![
                checkbox(
                    "Sold by weight".to_string(),
                    item.use_weight
                )
                .on_toggle(Message::ToggleUseWeight)
                    .width(200)
                    .spacing(10)
                    .style(checkbox::primary),
                column![
                    text("Tar Weight").style(text::primary),
                    text_input(
                        "Weight",
                        &item.weight_amount.to_string()
                    )
                    .on_input(Message::UpdateWeightAmount)
                    .padding(5)
                    .width(200)
                ].spacing(10).padding(10),
            ]
            .spacing(10)
            .padding(10)
            .wrap(),
        ]
        .width(Length::Fill)
        .spacing(10)
    )
    .style(container::rounded_box)
    .width(Length::Fill)
    .padding(10);

/*     let weight_info = container(
        column![
            text("Weight").size(16).style(iced::widget::text::primary),
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
    .style(container::rounded_box)
    .width(Length::Fill)
    .padding(10); */

    let flags = container(
        column![
            iced::widget::horizontal_rule(5),
            column![
                row![
                    checkbox(
                        "Print on Check",
                        item.print_on_check
                    )
                    .on_toggle(Message::TogglePrintOnCheck)
                    .spacing(10)
                    .width(200),
                    checkbox(
                        "Discountable",
                        item.discountable
                    )
                    .on_toggle(Message::ToggleDiscountable)
                    .spacing(10)
                    .width(200),
                    checkbox(
                        "Voidable",
                        item.voidable
                    )
                    .on_toggle(Message::ToggleVoidable)
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
                    .spacing(10)
                    .width(200),
                    checkbox(
                        "Tax Included",
                        item.tax_included
                    )
                    .on_toggle(Message::ToggleTaxIncluded)
                    .spacing(10)
                    .width(200),
                    checkbox(
                        "Stock Item",
                        item.stock_item
                    )
                    .on_toggle(Message::ToggleStockItem)
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
                    .spacing(10)
                    .width(200),
                    checkbox(
                        "Allow price override",
                        item.allow_price_override
                    )
                    .on_toggle(Message::ToggleAllowPriceOverride)
                    .spacing(10)
                    .width(200),
                ].wrap(),
            ],
            iced::widget::horizontal_rule(5),
        ]
    )
    .style(container::rounded_box)
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
    .style(container::rounded_box)
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
    .style(container::rounded_box)
    .width(Length::Fill)
    .padding(10); */

    let choice_groups = container(
        column![
            column![
                text("Choice Groups").size(14).style(text::primary),
                combo_box(
                    &state.choice_groups_combo,
                    "Add Choice Group",
                    state.choice_group_selection.as_ref(),
                    |choice_group: ChoiceGroup| Message::ChoiceGroupSelected(choice_group.id)
                )
                .width(200),
            ].spacing(5),
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
                                    .style(data_types::badge)
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
    .style(container::rounded_box)
    .width(Length::Fill)
    .padding(10);

    let printer_info = container(
        column![
            column![
                text("Printer Logicals").size(14).style(text::primary),
                combo_box(
                    &state.printer_logicals_combo,
                    "Add Printer Logical",
                    state.printer_logicals_selection.as_ref(),
                    |printer_logical: PrinterLogical| Message::PrinterLogicalSelected(printer_logical.id)
                )
                .width(200),
            ].spacing(5),
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
                                .style(data_types::badge)
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
    .style(container::rounded_box)
    .width(Length::Fill)
    .padding(10);

    let pricing = container(
        column![
            column![
                text("Price Levels").size(14).style(text::primary),
                combo_box(
                    &state.price_levels_combo,
                    "Add Price Level",
                    state.price_levels_selection.as_ref(),
                    |price_level: PriceLevel| Message::PriceLevelSelected(price_level.id)
                )
                .width(200),
            ].spacing(5),
            row![ // Display Selected Price Levels
            if let Some(selected_prices) = &item.price_levels {
                row(
                    selected_prices
                        .iter()
                        .filter_map(|id| price_levels.get(id))
                        .map(|price| {
                            container(
                                button(
                                    row![
                                        text(&price.name),
                                    ].spacing(10)
                                )
                                .on_press(Message::RemovePriceLevel(price.id))
                                .style(data_types::badge)
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
    .style(container::rounded_box)
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
