use iced::widget::{
    button, column, container, row, text, scrollable,
    horizontal_space,
};
use iced::{Alignment, Element, Length};
use std::collections::HashMap;
use crate::HotKey;
use crate::{
    items::{Item, EntityId},
    item_groups::ItemGroup,
    price_levels::PriceLevel,
    product_classes::ProductClass,
    tax_groups::TaxGroup,
    security_levels::SecurityLevel,
    revenue_categories::RevenueCategory,
    report_categories::ReportCategory,
    choice_groups::ChoiceGroup,
    printer_logicals::PrinterLogical,
    icon,
};


#[derive(Debug, Clone)]
pub enum Message {
    Edit,
    Back,
    ExportToCsv,
}

pub fn view<'a>(
    item: &'a Item,
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
    let header = row![
        horizontal_space().width(10),
        text(&item.name).size(18).style(text::primary),
        horizontal_space(),
        button(icon::edit().shaping(text::Shaping::Advanced)).on_press(Message::Edit),
        horizontal_space().width(4),
    ]
    .spacing(10)
    .padding(20)
    .align_y(iced::Alignment::Center);

    let basic_info = container(
        column![
            text("Basic Information").size(16).style(iced::widget::text::primary),
            row![
                info_row("Button 1:".to_string(), item.button1.clone()),
                info_row(
                    "Button 2:".to_string(), 
                    item.button2.clone().unwrap_or_default()
                ),
                info_row("Printer Text:".to_string(), item.printer_text.clone()),
            ].wrap(),
        ]
    )
    .style(container::rounded_box)
    .width(Length::Fill)
    .padding(20);

    let classifications = container(
        column![
            text("Classifications").size(16).style(iced::widget::text::primary),
            row![
                info_row(
                    "Item Group:".to_string(), 
                    item.item_group
                        .and_then(|id| item_groups.get(&id))
                        .map_or("None".to_string(), |g| g.name.clone())
                ),
                info_row(
                    "Product Class:".to_string(), 
                    item.product_class
                        .and_then(|id| product_classes.get(&id))
                        .map_or("None".to_string(), |c| c.name.clone())
                ),
                info_row(
                    "Rev Category:".to_string(), 
                    item.revenue_category
                        .and_then(|id| revenue_categories.get(&id))
                        .map_or("None".to_string(), |c| c.name.clone())
                ),
            ].wrap(),
            row![
                info_row(
                    "Tax Group:".to_string(), 
                    item.tax_group
                        .and_then(|id| tax_groups.get(&id))
                        .map_or("None".to_string(), |g| g.name.clone())
                ),
                info_row(
                    "Security Level:".to_string(), 
                    item.security_level
                        .and_then(|id| security_levels.get(&id))
                        .map_or("None".to_string(), |l| l.name.clone())
                ),
                info_row(
                    "Report Category:".to_string(), 
                    item.report_category
                        .and_then(|id| report_categories.get(&id))
                        .map_or("None".to_string(), |c| c.name.clone())
                ),
            ].wrap(),
        ]
        .width(Length::Fill)
        .spacing(10)
    )
    .width(Length::Fill)
    .style(container::rounded_box)
    .padding(20);

    let pricing = container(
        column![
            text("Pricing").size(16).style(iced::widget::text::primary),
            row![
                info_row(
                    "Default Price:".to_string(), 
                    item.cost_amount.map_or("Not Set".to_string(), |c| format!("${:.2}", c))
                ),
                info_row(
                    "Ask Price:".to_string(), 
                    (if item.ask_price { "Yes" } else { "No" }).to_string()
                ),
                info_row(
                    "Allow Price Override:".to_string(), 
                    (if item.allow_price_override { "Yes" } else { "No" }).to_string()
                ),
            ],
            if let Some(ref levels) = item.price_levels {
                column(
                    levels.iter()
                        .filter_map(|id| price_levels.get(id))
                        .map(|level| info_row(
                            "Price Level: ".to_string() + level.name.clone().as_str(), 
                            format!("${:.2}", level.price)
                        ))
                        .collect::<Vec<_>>()
                )
            } else {
                column![info_row("Price Levels:".to_string(), "None".to_string())]
            }
        ]
    )
    .style(container::rounded_box)
    .width(Length::Fill)
    .padding(20);

    let weight_info = container(
        column![
            text("Weight Information").size(16).style(iced::widget::text::primary),
            info_row(
                "Use Weight:".to_string(), 
                (if item.use_weight { "Yes" } else { "No" }).to_string()
            ),
            info_row(
                "Weight Amount:".to_string(), 
                format!("{:.3}", item.weight_amount)
            ),
        ]
    )
    .style(container::rounded_box)
    .width(Length::Fill)
    .padding(20);

    let flags = container(
        column![
            text("Flags").size(16).style(iced::widget::text::primary),
            info_row(
                "Print on Check:".to_string(), 
                (if item.print_on_check { "Yes" } else { "No" }).to_string()
            ),
            info_row(
                "Discountable:".to_string(), 
                (if item.discountable { "Yes" } else { "No" }).to_string()
            ),
            info_row(
                "Voidable:".to_string(), 
                (if item.voidable { "Yes" } else { "No" }).to_string()
            ),
            info_row(
                "Active:".to_string(), 
                (if item.not_active { "No" } else { "Yes" }).to_string()
            ),
            info_row(
                "Tax Included:".to_string(), 
                (if item.tax_included { "Yes" } else { "No" }).to_string()
            ),
            info_row(
                "Stock Item:".to_string(), 
                (if item.stock_item { "Yes" } else { "No" }).to_string()
            ),
        ]
    )
    .style(container::rounded_box)
    .width(Length::Fill)
    .padding(20);

    let kitchen_info = container(
        column![
            text("Kitchen Information").size(16).style(iced::widget::text::primary),
            info_row("Kitchen Video:".to_string(), item.kitchen_video.clone()),
            info_row("KDS Category:".to_string(), item.kds_category.clone()),
            info_row("KDS Cook Time:".to_string(), item.kds_cooktime.to_string()),
            info_row("KDS Department:".to_string(), item.kds_dept.to_string()),
            if let Some(ref printers) = item.printer_logicals {
                column![
                    text("Printer Logicals:").size(14),
                    column(
                        printers.iter()
                            .filter_map(|id| printer_logicals.get(id))
                            .map(|printer| info_row(
                                "".to_string(), 
                                printer.name.clone()
                            ))
                            .collect::<Vec<_>>()
                    )
                ]

            } else {
                column![info_row("Printer Logicals:".to_string(), "None".to_string())]
            }
        ]
    )
    .style(container::rounded_box)
    .width(Length::Fill)
    .padding(20);

    let store_info = container(
        column![
            text("Store Information").size(16).style(iced::widget::text::primary),
            info_row("Store ID:".to_string(), item.store_id.to_string()),
            info_row("Covers:".to_string(), item.covers.to_string()),
            info_row("Language:".to_string(), item.language_iso_code.clone()),
            if let Some(ref levels) = item.store_price_level {
                column![
                    text("Store Price Levels:").size(14),
                    column(
                        levels.iter()
                            .filter_map(|id| price_levels.get(id))
                            .map(|level| info_row(
                                level.name.clone(),
                                format!("${:.2}", level.price)
                            ))
                            .collect::<Vec<_>>()
                    )
                ]
            } else {
                column![info_row("Store Price Levels:".to_string(), "None".to_string())]
            }
        ]
    )
    .style(container::rounded_box)
    .width(Length::Fill)
    .padding(20);

    let choice_groups = container(
        column![
            text("Choice Groups").size(16).style(iced::widget::text::primary),
            if let Some(ref groups) = item.choice_groups {
                column(
                    groups.iter()
                        .filter_map(|id| choice_groups.get(id))
                        .map(|group| info_row(
                            "".to_string(),
                            group.name.clone()
                        ))
                        .collect::<Vec<_>>()
                )
            } else {
                column![info_row("".to_string(), "None".to_string())]
            }
        ]
    )
    .style(container::rounded_box)
    .width(Length::Fill)
    .padding(20);

    container(
        column![
            header,
            scrollable(
                column![
                    basic_info,
                    classifications,
                    pricing,
                    weight_info,
                    flags,
                    kitchen_info,
                    store_info,
                    choice_groups,
                ]
                .spacing(20)
            )
            .spacing(10)
            .width(Length::Fill)
            .height(Length::Fill)
        ]
        .spacing(20)
    )
    .padding(20)
    .into()
}

fn info_row(label: String, value: String) -> Element<'static, Message> {
    container(
        row![
            text(label).width(Length::Shrink).style(text::secondary),
            text(value)
        ]
        .spacing(10)
        .padding(10)
    )
    .into()
}

pub fn handle_hotkey(hotkey: HotKey) -> crate::Action<super::Operation, Message> {
    match hotkey {
        HotKey::Escape => crate::Action::operation(super::Operation::Back),
        _ => crate::Action::none(),
    }
}