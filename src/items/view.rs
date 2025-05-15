use iced::widget::{
    button, checkbox, column, container, row, text, scrollable,
    horizontal_space, text_input
};
use iced::{Element, Length};
use iced_modern_theme::Modern;
use std::collections::BTreeMap;
//use crate::HotKey;
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
    data_types,
};


#[derive(Debug, Clone)]
pub enum Message {
    Edit,
    Back,
    ExportToCsv,
}

pub fn view<'a>(
    item: &'a Item,
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
        button(icon::edit().size(14))
            .on_press(Message::Edit)
            .style(Modern::primary_button()),
        horizontal_space().width(4),
    ]
    .spacing(10);

    let basic_info = container(
        column![
                row![
                    long_info_column(
                        "Item Name".to_string(),
                        item.name.clone()),
                    info_column(
                        "Base Price".to_string(),
                        //item.default_price.map_or("Not Set".to_string(), |c| format!("${:.2}", c))),
                        item.default_price.map_or("$0.00".to_string(), |c| format!("${}", c))),
                ].wrap(),
                row![
                    info_column(
                        "Button Text 1".to_string(), 
                        item.button1.clone()),
                    info_column(
                        "Button Text 2".to_string(), 
                        item.button2.clone().unwrap_or_default()),
                    info_column(
                        "Customer Receipt Text".to_string(),
                        item.customer_receipt.clone()),
                ].wrap(),
                row![
                    info_column(
                        "Kitchen Printer Text".to_string(), 
                        item.printer_text.clone()),
                    info_column(
                        "Kitchen Video Text".to_string(), 
                        item.kitchen_video.clone())
                ].wrap(),
        ]
    )
    .style(Modern::sheet_container())
    .width(Length::Fill)
    .padding(10);

    let weight_str = format!("{:.3}", &item.weight_amount);
    let classifications = container(
        column![
            row![
                info_column(
                    "Item Group".to_string(), 
                    item.item_group
                        .and_then(|id| item_groups.get(&id))
                        .map_or("None".to_string(), |g| g.name.clone())
                ),
                info_column(
                    "Product Class".to_string(), 
                    item.product_class
                        .and_then(|id| product_classes.get(&id))
                        .map_or("None".to_string(), |c| c.name.clone())
                ),
                info_column(
                    "Revenue Category".to_string(), 
                    item.revenue_category
                        .and_then(|id| revenue_categories.get(&id))
                        .map_or("None".to_string(), |c| c.name.clone())
                ),
            ].wrap(),
            row![
                info_column(
                    "Tax Group".to_string(), 
                    item.tax_group
                        .and_then(|id| tax_groups.get(&id))
                        .map_or("None".to_string(), |g| g.name.clone())
                ),
                info_column(
                    "Security Level".to_string(), 
                    item.security_level
                        .and_then(|id| security_levels.get(&id))
                        .map_or("None".to_string(), |l| l.name.clone())
                ),
                info_column(
                    "Report Category".to_string(), 
                    item.report_category
                        .and_then(|id| report_categories.get(&id))
                        .map_or("None".to_string(), |c| c.name.clone())
                ),
            ].wrap(),
            row![
                column![
                    checkbox(
                        "Sold by weight".to_string(), 
                        item.use_weight
                    )
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
                            &weight_str.to_string()
                        )
                        .style(Modern::inline_text_input())
                        .padding(5)
                        .width(200)
                    ],
                ].spacing(10).padding(10),
            ]
            .wrap()
        ]
        .width(Length::Fill)
    )
    .width(Length::Fill)
    .style(Modern::sheet_container())
    .padding(10);

    let pricing = container(
        column![
            text("Price Levels").style(Modern::primary_text()),
            iced::widget::horizontal_space().height(5),
            row![
                if let Some(ref prices) = item.item_prices {
                    row(
                        prices.iter()
                            .filter_map(|item_price| {
                                price_levels.get(&item_price.price_level_id).map(|price_level|{
                                    let label2 = row![
                                        text(price_level.name.clone() + ": ").style(Modern::primary_text()),
                                        text("$".to_string() + item_price.price.to_string().as_str()),
                                        ];

                                    button(label2).style(Modern::gray_button()).into()
                                })
                            }).collect::<Vec<_>>()
                    ).spacing(10).wrap()
                } else {
                    row![button(text("No Price Level Assigned")).style(Modern::gray_button())].wrap()
                }
            ],
        ]
    )
    .style(Modern::sheet_container())
    .width(Length::Fill)
    .padding(10);

    let flags = container(
        column![
            column![
                row![
                    checkbox(
                        "Print on Check".to_string(), 
                        item.print_on_check
                    ).spacing(10).width(200).style(Modern::checkbox()),
                    checkbox(
                        "Discountable".to_string(), 
                        item.discountable
                    ).spacing(10).width(200).style(Modern::checkbox()),
                    checkbox(
                        "Voidable".to_string(), 
                        item.voidable
                    ).spacing(10).width(200).style(Modern::checkbox()),
                ].wrap()
            ],
            column![
                row![
                    checkbox(
                        "Active".to_string(), 
                        item.not_active
                    ).spacing(10).width(200).style(Modern::checkbox()),
                    checkbox(
                        "Tax Included".to_string(), 
                        item.tax_included
                    ).spacing(10).width(200).style(Modern::checkbox()),
                    checkbox(
                        "Stock Item".to_string(), 
                        item.stock_item
                    ).spacing(10).width(200).style(Modern::checkbox()),
                ].wrap()
            ],
            column![
                row![
                    checkbox(
                        "Prompt for price".to_string(), 
                        item.ask_price
                    ).spacing(10).width(200).style(Modern::checkbox()),
                    checkbox(
                        "Allow price override".to_string(), 
                        item.allow_price_override
                    ).spacing(10).width(200).style(Modern::checkbox()),
                ].wrap()
            ],
        ],
    )
    .style(Modern::sheet_container())
    .width(Length::Fill)
    .padding(10);

/*     let kitchen_info = container(
        column![
            info_row("KDS Category:".to_string(), item.kds_category.clone()),
            info_row("KDS Cook Time:".to_string(), item.kds_cooktime.to_string()),
            info_row("KDS Department:".to_string(), item.kds_dept.to_string()),
        ]
    )
    .style(Modern::card_container())
    .width(Length::Fill)
    .padding(10); */

    let printer_info = container(
        column![
            text("Printer Logicals:").style(Modern::primary_text()),
            iced::widget::horizontal_space().height(5),
            if let Some(ref printers) = item.printer_logicals {
                row(
                    printers.iter()
                        .filter_map(|id| printer_logicals.get(&id.0))
                        .map(|printer| button( &*printer.name).style(Modern::gray_button()).into())
                        .collect::<Vec<_>>()
                ).spacing(10).wrap()
            } else {
                row![button(text("No Printer Logicals Assigned")).style(Modern::gray_button())].wrap()
            }
        ]
    )
    .style(Modern::sheet_container())
    .width(Length::Fill)
    .padding(10);

/*     let store_info = container(
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
    .style(Modern::card_container())
    .width(Length::Fill)
    .padding(10); */

    let choice_groups = container(
        column![
            text("Choice Groups").style(Modern::primary_text()),
            iced::widget::horizontal_space().height(5),
            if let Some(ref groups) = item.choice_groups {
                row(
                    groups.iter()
                        .filter_map(|id| choice_groups.get(&id.0))
                        .map(|group| button(&*group.name).style(Modern::gray_button()).into() )
                        .collect::<Vec<_>>()
                ).spacing(10).wrap()
            } else {
                row![button(text("No Choice Group Assigned")).style(Modern::gray_button())].wrap()
            }
        ]
    )
    .style(Modern::sheet_container())
    .width(Length::Fill)
    .padding(10);

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

fn info_column(label: String, value: String) -> Element<'static, Message> {
    container(
        column![
            text(label).width(Length::Shrink).style(Modern::primary_text()),
            text_input(&value, &value).width(200).style(Modern::inline_text_input())
        ]
        .spacing(10)
        .padding(10)
    )
    .into()
}

fn long_info_column(label: String, value: String) -> Element<'static, Message> {
    container(
        column![
            text(label).width(Length::Shrink).style(Modern::primary_text()),
            text_input(&value, &value).width(420).style(Modern::inline_text_input())
        ]
        .spacing(10)
        .padding(10)
    )
    .into()
}

/* pub fn handle_hotkey(hotkey: HotKey) -> crate::Action<super::Operation, Message> {
    match hotkey {
        HotKey::Escape => crate::Action::operation(super::Operation::Back),
        _ => crate::Action::none(),
    }
} */