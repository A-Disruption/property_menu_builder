use iced::widget::{
    button, column, container, row, text, scrollable,
    horizontal_space, vertical_space,
};
use iced::{Alignment, Element, Length};
use crate::HotKey;
use super::{Item, ViewContext};

#[derive(Debug, Clone)]
pub enum Message {
    Edit,
    Back,
    ExportToCsv,
}

pub fn view<'a>(
    item: &'a Item,
    context: &'a ViewContext<'a>,
) -> Element<'a, Message> {
    let header = row![
        button("←").width(40).on_press(Message::Back),
        text(&item.name).size(16),
        horizontal_space(),
        button("Edit").on_press(Message::Edit)
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    let basic_info = container(
        column![
            section_header("Basic Information"),
            info_row("Button 1:".to_string(), item.button1.clone()),
            info_row("Button 2:".to_string(), item.button2.clone().unwrap_or("".to_string())),
            info_row("Printer Text:".to_string(), item.printer_text.clone()),
        ]
    )
    .style(container::rounded_box)
    .padding(20);

    let classifications = container(
        column![
            section_header("Classifications"),
            info_row(
                "Item Group:".to_string(), 
                item.item_group
                    .and_then(|id| context.available_item_groups.get(&id))
                    .map_or("None".to_string(), |g| g.name.clone())
            ),
            info_row(
                "Product Class:".to_string(), 
                item.product_class
                    .and_then(|id| context.available_product_classes.get(&id))
                    .map_or("None".to_string(), |c| c.name.clone())
            ),
            info_row(
                "Revenue Category:".to_string(), 
                item.revenue_category
                    .and_then(|id| context.available_revenue_categories.get(&id))
                    .map_or("None".to_string(), |c| c.name.clone())
            ),
            info_row(
                "Tax Group:".to_string(), 
                item.tax_group
                    .and_then(|id| context.available_tax_groups.get(&id))
                    .map_or("None".to_string(), |g| g.name.clone())
            ),
            info_row(
                "Security Level:".to_string(), 
                item.security_level
                    .and_then(|id| context.available_security_levels.get(&id))
                    .map_or("None".to_string(), |l| l.name.clone())
            ),
            info_row(
                "Report Category:".to_string(), 
                item.report_category
                    .and_then(|id| context.available_report_categories.get(&id))
                    .map_or("None".to_string(), |c| c.name.clone())
            ),
        ]
    )
    .style(container::rounded_box)
    .padding(20);

    let pricing = container(
        column![
            section_header("Pricing"),
            text("Price Levels:").size(14),
            if let Some(ref price_levels) = item.price_levels {
                column(
                    price_levels.iter()
                        .filter_map(|id| context.available_price_levels.get(id))
                        .map(|level| {
                            info_row(level.name.clone(), format!("${:.2}", level.price))
                        })
                        .collect::<Vec<_>>()
                )
            } else {
                column![
                    text("No Price Levels Set")
                ]
            },
            vertical_space(),
            info_row("Cost Amount:".to_string(), item.cost_amount.map_or("Not Set".to_string(), |c| format!("${:.2}", c))),
            info_row("Ask Price:".to_string(), if item.ask_price { "Yes".to_string() } else { "No".to_string() }),
            info_row("Allow Price Override:".to_string(), if item.allow_price_override { "Yes".to_string() } else { "No".to_string() }),
        ]
    )
    .style(container::rounded_box)
    .padding(20);

    let weight_info = container(
        column![
            section_header("Weight Information"),
            info_row("Use Weight:".to_string(), if item.use_weight { "Yes".to_string() } else { "No".to_string() }),
            info_row("Weight Amount:".to_string(), format!("{:.3}", item.weight_amount)),
        ]
    )
    .style(container::rounded_box)
    .padding(20);

    let identifiers = container(
        column![
            section_header("Identifiers"),
            info_row("SKU:".to_string(), item.sku.clone().unwrap_or("Not Set".to_string())),
            info_row("Bar Gun Code:".to_string(), item.bar_gun_code.clone().unwrap_or("Not Set".to_string())),
        ]
    )
    .style(container::rounded_box)
    .padding(20);

    let flags = container(
        column![
            section_header("Flags"),
            info_row("Print on Check:".to_string(), if item.print_on_check { "Yes".to_string() } else { "No".to_string() }),
            info_row("Discountable:".to_string(), if item.discountable { "Yes".to_string() } else { "No".to_string() }),
            info_row("Voidable:".to_string(), if item.voidable { "Yes".to_string() } else { "No".to_string() }),
            info_row("Active:".to_string(), if item.not_active { "No".to_string() } else { "Yes".to_string() }),
            info_row("Tax Included:".to_string(), if item.tax_included { "Yes".to_string() } else { "No".to_string() }),
            info_row("Stock Item:".to_string(), if item.stock_item { "Yes".to_string() } else { "No".to_string() }),
        ]
    )
    .style(container::rounded_box)
    .padding(20);

    let kitchen_info = container(
        column![
            section_header("Kitchen Information"),
            info_row("Kitchen Video:".to_string(), item.kitchen_video.clone()),
            info_row("KDS Category:".to_string(), item.kds_category.clone()),
            info_row("KDS Cook Time:".to_string(), item.kds_cooktime.to_string()),
            info_row("KDS Department:".to_string(), item.kds_dept.to_string()),
            if let Some(ref printers) = item.printer_logicals {
                column![
                    text("Printer Logicals:").size(14),
                    column(
                        printers.iter()
                            .filter_map(|id| context.available_printer_logicals.get(id))
                            .map(|level| {
                                info_row("".to_string(), level.name.clone())
                            })
                            .collect::<Vec<_>>()
                    )
                ]
            } else {
                column![ text("No Printer Logicals Assigned") ] 
            }
        ]
    )
    .style(container::rounded_box)
    .padding(20);

    let store_info = container(
        column![
            section_header("Store Information"),
            info_row("Store ID:".to_string(), item.store_id.to_string()),
            info_row("Covers:".to_string(), item.covers.to_string()),
            info_row("Customer Receipt:".to_string(), item.customer_receipt.clone()),
            info_row("Language:".to_string(), item.language_iso_code.clone()),
            if let Some(ref levels) = item.store_price_level {
                column![
                    text("Store Price Levels:").size(14),
                    column(
                        levels.iter()
                            .filter_map(|id| context.available_price_levels.get(id))
                            .map(|level| info_row(level.name.clone(), format!("${:.2}", level.price)))
                            .collect::<Vec<_>>()
                    )
                ]
            } else {
                column![ text("No Store Price Levels") ]
            }
        ]
    )
    .style(container::rounded_box)
    .padding(20);

    let choice_groups = container(
        column![
            section_header("Choice Groups"),
            if let Some(ref groups) = item.choice_groups {
                column(
                    groups.iter()
                        .filter_map(|id| context.available_choice_groups.get(id))
                        .map(|level| info_row("".to_string(), level.name.clone()))
                        .collect::<Vec<_>>()
                )
            } else {
                column![ text("No Choice Groups Assigned") ] 
            }
        ]
    )
    .style(container::rounded_box)
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
                    identifiers,
                    flags,
                    kitchen_info,
                    store_info,
                    choice_groups,
                ]
                .spacing(20)
            )
            .height(Length::Fill)
        ]
        .spacing(20)
    )
    .padding(20)
    .into()
}

fn section_header(title: &str) -> Element<Message> {
    text(title)
        .size(16)
        .style(iced::widget::text::default)
        .into()
}

/* fn info_row<'a>(
    label: &'a str, 
    value: &'a str
) -> Element<'a, Message> {
    row![
        text(label).width(Length::Fixed(150.0)),
        text(value)
    ]
    .spacing(10)
    .into()
} */

fn info_row<'a>(
    label: String, 
    value: String
) -> Element<'a, Message> {
    row![
        text(label).width(Length::Fixed(150.0)),
        text(value)
    ]
    .spacing(10)
    .into()
}

pub fn handle_hotkey(hotkey: HotKey) -> crate::Action<super::Operation, Message> {
    match hotkey {
        HotKey::Escape => crate::Action::operation(super::Operation::Back),
        _ => crate::Action::none(),
    }
}