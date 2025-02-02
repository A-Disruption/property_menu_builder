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
            info_row("Button 1:", &item.button1),
            info_row("Button 2:", item.button2.as_deref().unwrap_or("")),
            info_row("Printer Text:", &item.printer_text),
        ]
    )
    .style(container::rounded_box)
    .padding(20);

    let classifications = container(
        column![
            section_header("Classifications"),
            info_row("Item Group:", item.item_group.as_ref().map_or("None", |g| &g.name)),
            info_row("Product Class:", item.product_class.as_ref().map_or("None", |c| &c.name)),
            info_row("Revenue Category:", item.revenue_category.as_ref().map_or("None", |c| &c.name)),
            info_row("Tax Group:", item.tax_group.as_ref().map_or("None", |g| &g.name)),
            info_row("Security Level:", item.security_level.as_ref().map_or("None", |l| &l.name)),
            info_row("Report Category:", item.report_category.as_ref().map_or("None", |c| &c.name)),
        ]
    )
    .style(container::rounded_box)
    .padding(20);

    let pricing = container(
        column![
            section_header("Pricing"),
            text("Price Levels:").size(14),
            column(
                item.price_levels
                    .iter()
                    .map(|level| {
                        info_row(&level.name, &format!("${:.2}", level.price))
                    })
                    .collect()
            ),
            vertical_space(),
            info_row("Cost Amount:", &item.cost_amount.map_or("Not Set".to_string(), |c| format!("${:.2}", c))),
            info_row("Ask Price:", if item.ask_price { "Yes" } else { "No" }),
            info_row("Allow Price Override:", if item.allow_price_override { "Yes" } else { "No" }),
        ]
    )
    .style(container::rounded_box)
    .padding(20);

    let weight_info = container(
        column![
            section_header("Weight Information"),
            info_row("Use Weight:", if item.use_weight { "Yes" } else { "No" }),
            info_row("Weight Amount:", &format!("{:.3}", item.weight_amount)),
        ]
    )
    .style(container::rounded_box)
    .padding(20);

    let identifiers = container(
        column![
            section_header("Identifiers"),
            info_row("SKU:", item.sku.as_deref().unwrap_or("Not Set")),
            info_row("Bar Gun Code:", item.bar_gun_code.as_deref().unwrap_or("Not Set")),
        ]
    )
    .style(container::rounded_box)
    .padding(20);

    let flags = container(
        column![
            section_header("Flags"),
            info_row("Print on Check:", if item.print_on_check { "Yes" } else { "No" }),
            info_row("Discountable:", if item.discountable { "Yes" } else { "No" }),
            info_row("Voidable:", if item.voidable { "Yes" } else { "No" }),
            info_row("Active:", if item.not_active { "No" } else { "Yes" }),
            info_row("Tax Included:", if item.tax_included { "Yes" } else { "No" }),
            info_row("Stock Item:", if item.stock_item { "Yes" } else { "No" }),
        ]
    )
    .style(container::rounded_box)
    .padding(20);

    let kitchen_info = container(
        column![
            section_header("Kitchen Information"),
            info_row("Kitchen Video:", &item.kitchen_video),
            info_row("KDS Category:", &item.kds_category),
            info_row("KDS Cook Time:", &item.kds_cooktime.to_string()),
            info_row("KDS Department:", &item.kds_dept.to_string()),
            if let Some(ref printers) = item.printer_logicals {
                column![
                    text("Printer Logicals:").size(14),
                    column(
                        printers.iter()
                            .map(|p| info_row("", &p.name))
                            .collect()
                    )
                ].into()
            } else {
                text("No Printer Logicals Assigned").into()
            }
        ]
    )
    .style(container::rounded_box)
    .padding(20);

    let store_info = container(
        column![
            section_header("Store Information"),
            info_row("Store ID:", &item.store_id.to_string()),
            info_row("Covers:", &item.covers.to_string()),
            info_row("Customer Receipt:", &item.customer_receipt),
            info_row("Language:", &item.language_iso_code),
            if let Some(ref levels) = item.store_price_level {
                column![
                    text("Store Price Levels:").size(14),
                    column(
                        levels.iter()
                            .map(|level| info_row(&level.name, &format!("${:.2}", level.price)))
                            .collect()
                    )
                ].into()
            } else {
                text("No Store Price Levels").into()
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
                        .map(|g| info_row("", &g.name))
                        .collect()
                ).into()
            } else {
                text("No Choice Groups Assigned").into()
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
        .style(iced::theme::Text::Default)
        .into()
}

fn info_row(label: &str, value: &str) -> Element<Message> {
    row![
        text(label).width(Length::Fixed(150.0)),
        text(value)
    ]
    .spacing(10)
    .align_items(Alignment::Center)
    .into()
}

pub fn handle_hotkey(hotkey: HotKey) -> crate::Action<super::Operation, Message> {
    match hotkey {
        HotKey::Escape => crate::Action::operation(super::Operation::Back),
        _ => crate::Action::none(),
    }
}