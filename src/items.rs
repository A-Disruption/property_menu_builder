pub mod edit;
pub mod view;

use std::collections::HashMap;
use crate::data_types::{
    EntityId,
    ValidationError,
};
use crate::Action;
use iced::Element;
use iced::widget::{button, container, column, row, text};
use rust_decimal::Decimal;
 // For fuzzy matching
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

#[derive(Debug, Clone)]
pub enum Message {
    Edit(edit::Message),
    View(view::Message),
    CreateNew,
    Select(EntityId),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(Item),
    StartEdit(EntityId),
    Cancel,
    Back,
    ExportToCsv,
    CreateNew(Item),
    Select(EntityId),
}

#[derive(Debug, Clone)]
pub enum Mode {
    View,
    Edit,
}

#[derive(Default, Clone)]
pub struct EditState {
    // Basic info
    pub name: String,
    pub id: String,
    pub button1: String,
    pub button2: String,
    pub printer_text: String,

    // Classifications
    pub item_group: Option<EntityId>,
    pub product_class: Option<EntityId>,
    pub revenue_category: Option<EntityId>,
    pub tax_group: Option<EntityId>,
    pub security_level: Option<EntityId>,
    pub report_category: Option<EntityId>,

    // Pricing
    pub cost_amount: String,
    pub ask_price: bool,
    pub allow_price_override: bool,
    pub price_levels: Vec<EntityId>,
    pub store_price_level: Vec<EntityId>,

    // Weight
    pub use_weight: bool,
    pub weight_amount: String,

    // Identifiers
    pub sku: String,
    pub bar_gun_code: String,

    // Flags
    pub reserved1: bool,
    pub print_on_check: bool,
    pub discountable: bool,
    pub voidable: bool,
    pub not_active: bool,
    pub tax_included: bool,
    pub stock_item: bool,
    pub reserved2: bool,

    // Receipt & Kitchen
    pub customer_receipt: String,
    pub kitchen_video: String,
    pub kds_category: String,
    pub kds_cooktime: String,
    pub kds_dept: String,

    // Store Settings
    pub store_id: String,
    pub covers: String,
    pub image_id: String,
    pub language_iso_code: String,

    // Related Items
    pub choice_groups: Vec<EntityId>,
    pub printer_logicals: Vec<EntityId>,

    // Validation
    pub validation_error: Option<String>,
}

impl EditState {
    pub fn new(item: &Item) -> Self {
        Self {
            name: item.name.clone(),
            id: item.id.to_string(),
            button1: item.button1.clone(),
            button2: item.button2.clone().unwrap_or_default(),
            printer_text: item.printer_text.clone(),
            item_group: item.item_group,
            product_class: item.product_class,
            revenue_category: item.revenue_category,
            tax_group: item.tax_group,
            security_level: item.security_level,
            report_category: item.report_category,
            cost_amount: item.cost_amount.map_or_else(String::new, |c| c.to_string()),
            ask_price: item.ask_price,
            allow_price_override: item.allow_price_override,
            price_levels: item.price_levels.clone().unwrap_or_default(),
            store_price_level: item.store_price_level.clone().unwrap_or_default(),
            use_weight: item.use_weight,
            weight_amount: item.weight_amount.to_string(),
            sku: item.sku.clone().unwrap_or_default(),
            bar_gun_code: item.bar_gun_code.clone().unwrap_or_default(),
            reserved1: item.reserved1,
            print_on_check: item.print_on_check,
            discountable: item.discountable,
            voidable: item.voidable,
            not_active: item.not_active,
            tax_included: item.tax_included,
            stock_item: item.stock_item,
            reserved2: item.reserved2,
            customer_receipt: item.customer_receipt.clone(),
            kitchen_video: item.kitchen_video.clone(),
            kds_category: item.kds_category.clone(),
            kds_cooktime: item.kds_cooktime.to_string(),
            kds_dept: item.kds_dept.to_string(),
            store_id: item.store_id.to_string(),
            covers: item.covers.to_string(),
            image_id: item.image_id.to_string(),
            language_iso_code: item.language_iso_code.clone(),
            choice_groups: item.choice_groups.clone().unwrap_or_default(),
            printer_logicals: item.printer_logicals.clone().unwrap_or_default(),
            validation_error: None,
        }
    }

    pub fn validate(&self, item_group: Option<&ItemGroup>) -> Result<(), ValidationError> {
        // Name validation
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Item name cannot be empty".to_string()
            ));
        }

        // Button text length validation
        if self.button1.len() > 15 {
            return Err(ValidationError::InvalidValue(
                "Button 1 text exceeds 15 characters".to_string()
            ));
        }

        if !self.button2.is_empty() && self.button2.len() > 15 {
            return Err(ValidationError::InvalidValue(
                "Button 2 text exceeds 15 characters".to_string()
            ));
        }

        // ID validation within item group range
        if let Some(group) = item_group {
            let id = self.id.parse::<EntityId>().map_err(|_| {
                ValidationError::InvalidId("Invalid ID format".to_string())
            })?;

            if id < group.id_range.start || id > group.id_range.end {
                return Err(ValidationError::InvalidId(
                    format!("ID must be within group range ({}-{})",
                        group.id_range.start, group.id_range.end)
                ));
            }
        }

        // Numeric field validations
        if !self.cost_amount.is_empty() {
            if let Err(_) = self.cost_amount.parse::<Decimal>() {
                return Err(ValidationError::InvalidValue(
                    "Invalid cost amount format".to_string()
                ));
            }
        }

        if let Err(_) = self.weight_amount.parse::<Decimal>() {
            return Err(ValidationError::InvalidValue(
                "Invalid weight amount format".to_string()
            ));
        }

        // Integer field validations
        for (field_name, value) in [
            ("KDS Cook Time", &self.kds_cooktime),
            ("KDS Department", &self.kds_dept),
            ("Store ID", &self.store_id),
            ("Covers", &self.covers),
            ("Image ID", &self.image_id),
        ] {
            if !value.is_empty() {
                if let Err(_) = value.parse::<i32>() {
                    return Err(ValidationError::InvalidValue(
                        format!("Invalid {} format", field_name)
                    ));
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub id: EntityId,
    pub name: String,
    pub button1: String,
    pub button2: Option<String>,
    pub printer_text: String,
    pub price_levels: Option<Vec<EntityId>>,
    pub product_class: Option<EntityId>,
    pub revenue_category: Option<EntityId>,
    pub tax_group: Option<EntityId>,
    pub security_level: Option<EntityId>,
    pub report_category: Option<EntityId>,
    pub use_weight: bool,
    pub weight_amount: Decimal,
    pub sku: Option<String>,
    pub bar_gun_code: Option<String>,
    pub cost_amount: Option<Decimal>,
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
}

impl Default for Item {
    fn default() -> Self {
        Self {
            id: -1,
            name: String::new(),
            button1: String::new(),
            button2: None,
            printer_text: String::new(),
            price_levels: None,
            product_class: None,
            revenue_category: None,
            tax_group: None,
            security_level: None,
            report_category: None,
            use_weight: false,
            weight_amount: Decimal::ZERO,
            sku: None,
            bar_gun_code: None,
            cost_amount: None,
            reserved1: false,
            ask_price: false,
            print_on_check: false,
            discountable: true,  // Most items are discountable by default
            voidable: true,      // Most items are voidable by default
            not_active: false,
            tax_included: false,
            item_group: None,
            customer_receipt: String::new(),
            allow_price_override: false,
            reserved2: false,
            choice_groups: None,
            printer_logicals: None,
            covers: 0,
            store_id: 0,
            kitchen_video: String::new(),
            kds_dept: 0,
            kds_category: String::new(),
            kds_cooktime: 0,
            store_price_level: None,
            image_id: 0,
            stock_item: false,
            language_iso_code: String::new(),
        }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Item {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn assign_id_from_group(&mut self, item_group: &ItemGroup) -> Result<(), ValidationError> {
        // Find first available ID in the group's range
        let range = item_group.id_range.start..=item_group.id_range.end;
        
        // This logic would need to consider existing items in the group
        // to find the next available ID, or auto-increment on export
        self.id = item_group.id_range.start;
        Ok(())
    }

    pub fn validate(&self, context: &ViewContext) -> Result<(), ValidationError> {
        // Name validation
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Item name cannot be empty".to_string()
            ));
        }

        // Button text length validation
        if self.button1.len() > 15 {
            return Err(ValidationError::InvalidValue(
                "Button 1 text exceeds 15 characters".to_string()
            ));
        }

        if let Some(ref b2) = self.button2 {
            if b2.len() > 15 {
                return Err(ValidationError::InvalidValue(
                    "Button 2 text exceeds 15 characters".to_string()
                ));
            }
        }

        // ID validation within item group range
        if let Some(group_id) = self.item_group {
            if let Some(group) = context.available_item_groups.get(&group_id) {
                if self.id < group.id_range.start || self.id > group.id_range.end {
                    return Err(ValidationError::InvalidId(
                        format!("ID must be within group range ({}-{})",
                            group.id_range.start, group.id_range.end)
                    ));
                }
            } else {
                return Err(ValidationError::InvalidReference(
                    "Referenced item group does not exist".to_string()
                ));
            }
        }

        // Validate references exist
        if let Some(tax_group_id) = self.tax_group {
            if !context.available_tax_groups.contains_key(&tax_group_id) {
                return Err(ValidationError::InvalidReference(
                    "Referenced tax group does not exist".to_string(),
                ));
            }
        }

        if let Some(security_level_id) = self.security_level {
            if !context.available_security_levels.contains_key(&security_level_id) {
                return Err(ValidationError::InvalidReference(
                    "Referenced security level does not exist".to_string(),
                ));
            }
        }

        if let Some(revenue_category_id) = self.revenue_category {
            if !context.available_revenue_categories.contains_key(&revenue_category_id) {
                return Err(ValidationError::InvalidReference(
                    "Referenced revenue category does not exist".to_string(),
                ));
            }
        }

        if let Some(report_category_id) = self.report_category {
            if !context.available_report_categories.contains_key(&report_category_id) {
                return Err(ValidationError::InvalidReference(
                    "Referenced report category does not exist".to_string(),
                ));
            }
        }

        if let Some(product_class_id) = self.product_class {
            if !context.available_product_classes.contains_key(&product_class_id) {
                return Err(ValidationError::InvalidReference(
                    "Referenced product class does not exist".to_string(),
                ));
            }
        }

        // Validate collections
        if let Some(ref levels) = self.price_levels {
            for level_id in levels {
                if !context.available_price_levels.contains_key(level_id) {
                    return Err(ValidationError::InvalidReference(
                        "Referenced price level does not exist".to_string()
                    ));
                }
            }
        }

        if let Some(ref groups) = self.choice_groups {
            for group_id in groups {
                if !context.available_choice_groups.contains_key(group_id) {
                    return Err(ValidationError::InvalidReference(
                        "Referenced choice group does not exist".to_string()
                    ));
                }
            }
        }

        if let Some(ref printers) = self.printer_logicals {
            for printer_id in printers {
                if !context.available_printer_logicals.contains_key(printer_id) {
                    return Err(ValidationError::InvalidReference(
                        "Referenced printer logical does not exist".to_string()
                    ));
                }
            }
        }

        Ok(())
    }
}

pub fn update(
    item: &mut Item,
    message: Message,
    state: &mut EditState,
    context: &ViewContext,
) -> Action<Operation, Message> {
    match message {
        Message::Edit(msg) => match msg  {
            // Basic Info
            edit::Message::UpdateName(name) => {
                item.name = name;
                Action::none()
            }
            edit::Message::UpdateButton1(text) => {
                if text.len() <= 15 {
                    item.button1 = text;
                    state.validation_error = None;
                } else {
                    state.validation_error = Some("Button 1 text cannot exceed 15 characters".to_string());
                }
                Action::none()
            }
            edit::Message::UpdateButton2(text) => {
                if text.is_empty() {
                    item.button2 = None;
                } else if text.len() <= 15 {
                    item.button2 = Some(text);
                    state.validation_error = None;
                } else {
                    state.validation_error = Some("Button 2 text cannot exceed 15 characters".to_string());
                }
                Action::none()
            }
            edit::Message::UpdatePrinterText(text) => {
                item.printer_text = text;
                Action::none()
            }

            // Classifications
            edit::Message::SelectItemGroup(group_id) => {
                item.item_group = group_id;
                Action::none()
            }
            edit::Message::SelectProductClass(class_id) => {
                item.product_class = class_id;
                Action::none()
            }
            edit::Message::SelectRevenueCategory(category_id) => {
                item.revenue_category = category_id;
                Action::none()
            }
            edit::Message::SelectTaxGroup(group_id) => {
                item.tax_group = group_id;
                Action::none()
            }
            edit::Message::SelectSecurityLevel(level_id) => {
                item.security_level = level_id;
                Action::none()
            }
            edit::Message::SelectReportCategory(category_id) => {
                item.report_category = category_id;
                Action::none()
            }

            // Pricing
            edit::Message::UpdateCostAmount(amount) => {
                item.cost_amount = if amount.is_empty() {
                    None
                } else {
                    match amount.parse() {
                        Ok(amount) => Some(amount),
                        Err(_) => {
                            state.validation_error = Some("Invalid cost amount format".to_string());
                            return Action::none();
                        }
                    }
                };
                Action::none()
            }
            edit::Message::ToggleAskPrice(value) => {
                item.ask_price = value;
                Action::none()
            }
            edit::Message::ToggleAllowPriceOverride(value) => {
                item.allow_price_override = value;
                Action::none()
            }
            edit::Message::AddPriceLevel(level_id) => {
                if let Some(ref mut levels) = item.price_levels {
                    if !levels.contains(&level_id) {
                        levels.push(level_id);
                    }
                } else {
                    item.price_levels = Some(vec![level_id]);
                }
                Action::none()
            }
            edit::Message::RemovePriceLevel(level_id) => {
                if let Some(ref mut levels) = item.price_levels {
                    levels.retain(|&id| id != level_id);
                    if levels.is_empty() {
                        item.price_levels = None;
                    }
                }
                Action::none()
            }
            edit::Message::UpdateStorePriceLevel(level_id) => {
                if let Some(level_id) = level_id {
                    if let Some(ref mut levels) = item.store_price_level {
                        levels.push(level_id);
                    } else {
                        item.store_price_level = Some(vec![level_id]);
                    }
                }
                Action::none()
            }

            // Weight
            edit::Message::ToggleUseWeight(value) => {
                item.use_weight = value;
                if !value {
                    item.weight_amount = rust_decimal::Decimal::ZERO;
                }
                Action::none()
            }
            edit::Message::UpdateWeightAmount(amount) => {
                match amount.parse() {
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
            edit::Message::UpdateSku(sku) => {
                item.sku = if sku.is_empty() { None } else { Some(sku) };
                Action::none()
            }
            edit::Message::UpdateBarGunCode(code) => {
                item.bar_gun_code = if code.is_empty() { None } else { Some(code) };
                Action::none()
            }

            // Flags
            edit::Message::ToggleReserved1(value) => {
                item.reserved1 = value;
                Action::none()
            }
            edit::Message::TogglePrintOnCheck(value) => {
                item.print_on_check = value;
                Action::none()
            }
            edit::Message::ToggleDiscountable(value) => {
                item.discountable = value;
                Action::none()
            }
            edit::Message::ToggleVoidable(value) => {
                item.voidable = value;
                Action::none()
            }
            edit::Message::ToggleNotActive(value) => {
                item.not_active = value;
                Action::none()
            }
            edit::Message::ToggleTaxIncluded(value) => {
                item.tax_included = value;
                Action::none()
            }
            edit::Message::ToggleStockItem(value) => {
                item.stock_item = value;
                Action::none()
            }
            edit::Message::ToggleReserved2(value) => {
                item.reserved2 = value;
                Action::none()
            }

            // Receipt & Kitchen
            edit::Message::UpdateCustomerReceipt(text) => {
                item.customer_receipt = text;
                Action::none()
            }
            edit::Message::UpdateKitchenVideo(text) => {
                item.kitchen_video = text;
                Action::none()
            }
            edit::Message::UpdateKdsCategory(text) => {
                item.kds_category = text;
                Action::none()
            }
            edit::Message::UpdateKdsCooktime(time) => {
                match time.parse() {
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
            edit::Message::UpdateKdsDept(dept) => {
                match dept.parse() {
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
            edit::Message::UpdateStoreId(id) => {
                match id.parse() {
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
            edit::Message::UpdateCovers(covers) => {
                match covers.parse() {
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
            edit::Message::UpdateImageId(id) => {
                match id.parse() {
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
            edit::Message::UpdateLanguageIsoCode(code) => {
                item.language_iso_code = code;
                Action::none()
            }

            // Related Items
            edit::Message::AddChoiceGroup(group_id) => {
                if let Some(ref mut groups) = item.choice_groups {
                    if !groups.contains(&group_id) {
                        groups.push(group_id);
                    }
                } else {
                    item.choice_groups = Some(vec![group_id]);
                }
                Action::none()
            }
            edit::Message::RemoveChoiceGroup(group_id) => {
                if let Some(ref mut groups) = item.choice_groups {
                    groups.retain(|&id| id != group_id);
                    if groups.is_empty() {
                        item.choice_groups = None;
                    }
                }
                Action::none()
            }
            edit::Message::AddPrinterLogical(printer_id) => {
                if let Some(ref mut printers) = item.printer_logicals {
                    if !printers.contains(&printer_id) {
                        printers.push(printer_id);
                    }
                } else {
                    item.printer_logicals = Some(vec![printer_id]);
                }
                Action::none()
            }
            edit::Message::RemovePrinterLogical(printer_id) => {
                if let Some(ref mut printers) = item.printer_logicals {
                    printers.retain(|&id| id != printer_id);
                    if printers.is_empty() {
                        item.printer_logicals = None;
                    }
                }
                Action::none()
            }

            edit::Message::Save => Action::operation(Operation::Save(item.clone())),
            edit::Message::Cancel => Action::operation(Operation::Cancel),
        }
        Message::View(msg) => match msg {
            view::Message::Edit => Action::operation(Operation::StartEdit(item.id)),
            view::Message::Back => Action::operation(Operation::Back),
            view::Message::ExportToCsv => Action::none() //Need to implement export and imports
        }
        Message::CreateNew => {
            let new_item = Item::default();
            Action::operation(Operation::CreateNew(new_item))
        },
        Message::Select(id) => {
            Action::operation(Operation::Select(id))
        },
    }
}

pub fn view<'a>(
    item: &'a Item, 
    mode: &'a Mode,
    items: &'a HashMap<EntityId, Item>,
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

    let items_list = column(
        items
            .values()
            .map(|an_item| {
                button(text(&an_item.name))
                    .width(iced::Length::Fill)
                    .on_press(Message::Select(an_item.id))
                    .style(if an_item.id == item.id {
                        button::primary
                    } else {
                        button::secondary
                    })
                    .into()
            })
            .collect::<Vec<_>>()
    )
    .spacing(5)
    .width(iced::Length::Fixed(200.0));

    let content = match mode {
        Mode::View => view::view(
            item,
            item_groups,
            tax_groups,
            security_levels,
            revenue_categories,
            report_categories,
            product_classes,
            choice_groups,
            printer_logicals,
            price_levels,
        ).map(Message::View),
        Mode::Edit => {
            edit::view(
                item,
                EditState::new(item),
                item_groups,
                tax_groups,
                security_levels,
                revenue_categories,
                report_categories,
                product_classes,
                choice_groups,
                printer_logicals,
                price_levels,
            ).map(Message::Edit)
        }
    };

    row![
        container(
            column![
                text("Items").size(24),
                button("Create New")
                    .on_press(Message::CreateNew)
                    .style(button::primary),
                items_list,
            ]
            .spacing(10)
            .padding(10)
        )
        .style(container::rounded_box),
        container(content)
            .width(iced::Length::Fill)
            .style(container::rounded_box)
    ]
    .spacing(20)
    .into()
}

pub struct ViewContext {
    pub available_items: HashMap<EntityId, Item>,
    pub available_item_groups: HashMap<EntityId, ItemGroup>,
    pub available_tax_groups: HashMap<EntityId, TaxGroup>,
    pub available_security_levels: HashMap<EntityId, SecurityLevel>,
    pub available_revenue_categories: HashMap<EntityId, RevenueCategory>,
    pub available_report_categories: HashMap<EntityId, ReportCategory>,
    pub available_product_classes: HashMap<EntityId, ProductClass>,
    pub available_choice_groups: HashMap<EntityId, ChoiceGroup>,
    pub available_printer_logicals: HashMap<EntityId, PrinterLogical>,
    pub available_price_levels: HashMap<EntityId, PriceLevel>,
}