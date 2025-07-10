pub mod edit;
pub mod view;
pub mod import_items;
pub mod export_items;
pub mod preview_changes;

use std::collections::BTreeMap;
use crate::data_types::{
    EntityId, ValidationError, ItemPrice, EntityResolver
};
use crate::Action;
use iced_modern_theme::Modern;
use iced::{Alignment, Element, Length, Task};
use serde::{Serialize, Deserialize};
use iced::widget::{button, combo_box, container, column, row, text, scrollable};
use rust_decimal::Decimal;
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
    icon,
};


#[derive(Debug, Clone)]
pub enum Message {
    Edit(edit::Message),
    View(view::Message),
    CreateNew,
    Select(EntityId),
    SearchItems(String),
    RequestDelete(EntityId),
    CopyItem(EntityId),
    HideModal,
    ShowModal,
    LaunchMassItemEditWindow,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(Item),
    StartEdit,
    Cancel,
    Back,
    CreateNew(Item),
    Select(EntityId),
    UpdateSearchQuery(String),
    RequestDelete(EntityId),
    CopyItem(EntityId),
    HideModal,
    ShowModal,
    UpdatePrice(EntityId, EntityId, String),
    LaunchMassItemEditWindow,
}

#[derive(Debug, Clone)]
pub enum Mode {
    View,
    Edit,
}

#[derive(Default, Debug, Clone)]
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
    pub price_levels_combo: combo_box::State<PriceLevel>,
    pub price_levels_selection: Option<PriceLevel>,
    pub price: String,
    pub prices: Option<Vec<(EntityId, String)>>,
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
    pub choice_groups: Vec<(EntityId, i32)>,
    pub choice_groups_combo: combo_box::State<ChoiceGroup>,
    pub choice_group_selection: Option<ChoiceGroup>,
    pub printer_logicals: Vec<(EntityId, bool)>,
    pub printer_logicals_combo: combo_box::State<PrinterLogical>,
    pub printer_logicals_selection: Option<PrinterLogical>,

    // Validation
    pub validation_error: Option<String>,
}

impl EditState {
    pub fn new(item: &Item, choice_group_list: Vec<ChoiceGroup>, printer_logical_list: Vec<PrinterLogical>, price_level_list: Vec<PriceLevel>) -> Self {
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
            price_levels_combo: combo_box::State::with_selection(price_level_list.clone(), None),
            price_levels_selection: None,
            price: String::new(),
            prices: item.item_prices.as_ref().map(|prices_vec| {
                prices_vec.iter()
                    .map(|item_price| {
                        // Assuming price_level_id is the correct type here.
                        (item_price.price_level_id, item_price.price.to_string())
                    })
                    .collect()
            }),
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
            choice_groups_combo: combo_box::State::with_selection(choice_group_list.clone(), None),
            choice_group_selection: None,
            printer_logicals: item.printer_logicals.clone().unwrap_or_default(),
            printer_logicals_combo: combo_box::State::with_selection(printer_logical_list.clone(), None),
            printer_logicals_selection: None,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub id: EntityId,
    pub name: String,
    pub button1: String,
    pub button2: Option<String>,
    pub printer_text: String,
    pub price_levels: Option<Vec<EntityId>>, //initial setup, before storing price level IDs with Price per item. No longer users
    pub default_price: Option<Decimal>,
    pub item_prices: Option<Vec<ItemPrice>>, //actually the price levels in the code
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
    pub choice_groups: Option<Vec<(EntityId, i32)>>,  // i32, to track sequence number for the choice groups, per item
    pub printer_logicals: Option<Vec<(EntityId, bool)>>, // bool to track if a printer is primary or not
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
            name: "New Item".to_string(),
            button1: String::new(),
            button2: None,
            printer_text: String::new(),
            default_price: None,
            price_levels: None,
            item_prices: None,
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
            print_on_check: true,
            discountable: true,
            voidable: true,
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
            for (group_id, _) in groups {
                if !context.available_choice_groups.contains_key(group_id) {
                    return Err(ValidationError::InvalidReference(
                        "Referenced choice group does not exist".to_string()
                    ));
                }
            }
        }

        if let Some(ref printers) = self.printer_logicals {
            for printer_id in printers {
                if !context.available_printer_logicals.contains_key(&printer_id.0) {
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
    context: &mut ViewContext,
) -> Action<Operation, Message> {
    match message {
        Message::Edit(msg) => match msg  {
            // Basic Info
            edit::Message::UpdateName(name) => {
                item.name = name;
                Action::none()
            }
            edit::Message::UpdateBasePrice(base_price) => {
                let decimal_price = base_price.parse::<Decimal>();

                item.default_price = if base_price.is_empty() {
                    Some(Decimal::new(0, 2))
                } else {
                    match decimal_price {
                        Ok(price) => Some(price),
                        Err(_) => {
                            state.validation_error = Some("Invalid cost amount format".to_string());
                            return Action::none();
                        }
                    }
                };

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
            edit::Message::ChoiceGroupSelected(group_id) => {
                match &mut item.choice_groups {
                    Some(choice_groups) => {
                        if choice_groups.iter().any(|(id, _)| *id == group_id ) {
                            // do nothing, it's already selected
                        } else {
                            //add group to the selected groups list
                            let next_sequence: i32 = choice_groups.len().try_into().unwrap_or_else(|_| { i32::MAX });
                            choice_groups.push((group_id, next_sequence ))
                        }
                    }
                    None => { item.choice_groups = Some(vec![(group_id, 0)]) } // add the choice group as the first item in the sequence
                }
                Action::none()
            }
            edit::Message::PriceLevelSelected(level_id) => {
                println!("Price level id selected: {}", level_id);
                
                // Update both price_levels and item_prices
                match &mut item.price_levels {
                    Some(levels) => levels.push(level_id),
                    None => item.price_levels = Some(vec![level_id]),
                }
                
                // Also create a default ItemPrice
                let default_price = ItemPrice {
                    price_level_id: level_id,
                    price: Decimal::ZERO,  // Or some default price
                };
                
                match &mut item.item_prices {
                    Some(prices) => prices.push(default_price),
                    None => item.item_prices = Some(vec![default_price]),
                }
                
                Action::none()
            }
            edit::Message::PrinterLogicalSelected(printer_id) => {
                match &mut item.printer_logicals {
                    Some(printers) => {
                        if printers.iter().any(|(id, _)| *id == printer_id) {
                            // Printer already exists, do nothing
                        } else {
                            // Add new printer with false as it's not the first one
                            printers.push((printer_id, false));
                        }
                    }
                    None => {
                        // This is the first printer being added, so set boolean to true
                        item.printer_logicals = Some(vec![(printer_id, true)]);
                    }
                }
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
                // Update price_levels
                if let Some(ref mut levels) = item.price_levels {
                    levels.retain(|&id| id != level_id);
                    if levels.is_empty() {
                        item.price_levels = None;
                    }
                }
                
                // Also remove from item_prices
                if let Some(ref mut prices) = item.item_prices {
                    prices.retain(|price| price.price_level_id != level_id);
                    if prices.is_empty() {
                        item.item_prices = None;
                    }
                }
                
                Action::none()
            }
            edit::Message::UpdatePrice(price_level_id, item_price) => {
                // Updating item.item_prices directly for immediate effect
                if let Some(ref mut prices) = item.item_prices {
                    if let Some(price) = prices.iter_mut().find(|p| p.price_level_id == price_level_id) {
                        if let Ok(decimal_price) = item_price.parse::<Decimal>() {
                            price.price = decimal_price;
                        }
                    }
                }
                // Update edit_state.prices for UI
                Action::operation(Operation::UpdatePrice(item.id, price_level_id, item_price))
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
                match &mut item.choice_groups {
                    Some(choice_groups) => {
                        if choice_groups.iter().any(|(id, _)| *id == group_id ) {
                            // Don't add the choice group, it already exists
                        }
                        else {
                            //get the sequence number
                            let next_sequence: i32 = choice_groups.len().try_into().unwrap_or_else(|_| { i32::MAX });
                            //Add the choice group
                            choice_groups.push((group_id, next_sequence))
                        }
                    }
                    None => { item.choice_groups = Some(vec![(group_id, 0)]) } // add the choice group as the first item in the sequence
                }
                Action::none()
            }
            edit::Message::RemoveChoiceGroup(group_id) => {
                match &mut item.choice_groups {
                    Some(choice_groups) => {
                        if choice_groups.iter().any(|(id, _)| *id == group_id ) {
                            choice_groups.retain(|&(id, _)| id != group_id)
                        } 
                        else {} //choice group doesn't exist, do nothing
                    }
                    None => {} //no choice group, do nothing
                }
                Action::none()
            }
            edit::Message::AddPrinterLogical(printer_id) => {
                match &mut item.printer_logicals {
                    Some(printers) => {
                        if printers.iter().any(|(id, _)| *id == printer_id ) {
                            // Printer already exists, do nothing
                        } else {
                            // Add new printer with false as it's not the first one
                            printers.push((printer_id, false));
                        }
                    }
                    None => {
                        // This is the first printer being added, so set boolean to true
                        item.printer_logicals = Some(vec![(printer_id, true)]);
                    }
                }
                Action::none()
            }
            edit::Message::RemovePrinterLogical(printer_id) => {
                match &mut item.printer_logicals {
                    Some(printers) => {
                        if printers.iter().any(|(id, _)| *id == printer_id ) {
                            // Keep all Ids, not matching the id we want to remove.
                            printers.retain(|&(id, _)| id != printer_id);
                        } else {} // Printer doesn't exist, do nothing
                    }
                    None => {} // Printer doesn't exist, do nothing
                }
                Action::none()
            }

            edit::Message::Save => Action::operation(Operation::Save(item.clone())),
            edit::Message::Cancel => Action::operation(Operation::Cancel),
        }
        Message::View(msg) => match msg {
            view::Message::Edit => {
                Action::operation(Operation::StartEdit)
            },
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
        Message::SearchItems(query) => {
            Action::operation(Operation::UpdateSearchQuery(query)) //need to implement search
        }
        Message::RequestDelete(id) => {
            Action::operation(Operation::RequestDelete(id))
        }
        Message::CopyItem(id) => {
            Action::operation(Operation::CopyItem(id))
        }
        Message::HideModal => {
            Action::operation(Operation::HideModal)
        }
        Message::ShowModal => {
            Action::operation(Operation::ShowModal)
        }
        Message::LaunchMassItemEditWindow => {
            Action::operation(Operation::LaunchMassItemEditWindow)
        }
    }
}

pub fn view<'a>(
    item: &'a Item, 
    mode: &'a Mode,
    items: &'a BTreeMap<EntityId, Item>,
    item_search: &'a String,
    item_edit_state: &'a EditState,
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

    let search_bar = row![
        iced::widget::text_input(
            "Search Items...",
            &item_search
        )
        .width(iced::Length::Fixed(206.0))
        .style(Modern::search_input())
        .on_input(Message::SearchItems),

        iced::widget::Space::with_width(10),

        button(icon::superpowers().size(13).center())
            .on_press(Message::LaunchMassItemEditWindow)
            .style(Modern::primary_button()),
    ];

    let filtered_items = items.values()
        .filter(|item| matches_search(
            item, 
            &item_search,
            item_groups,
            tax_groups,
            security_levels,
            revenue_categories,
            report_categories,
            product_classes,
            choice_groups,
            printer_logicals,
            price_levels,
        ))
        .collect::<Vec<_>>();

    let header_row = row![
        text("Name").width(Length::Fixed(175.0)),
        text("Actions").width(Length::Fixed(150.0)),
    ]
    .padding(5);

    let items_list = scrollable(
        column(
            filtered_items
                .iter()
                .map(|an_item| {
                    button(
                        list_item(
                            an_item.name.as_str(),
                            button(icon::copy().size(14))
                                .on_press(Message::CopyItem(an_item.id)),
                            button(icon::trash().size(14))
                                .on_press(Message::RequestDelete(an_item.id)),
                        )
                    )
                    .on_press(Message::Select(an_item.id))
                    .style(
                        Modern::conditional_button_style(
                            an_item.id == item.id,
                            Modern::selected_button_style(Modern::system_button()),
                            Modern::system_button()
                        )
                    ).into()
                })
                .collect::<Vec<_>>()
        )
        .spacing(5)
        .width(iced::Length::Fixed(250.0))
    ).height(Length::Fill);

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
                item_edit_state,
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

    let full_view = row![
        container(
            column![
                row![
                    container(text("Items").size(18).style(Modern::primary_text())).padding(5),
                    iced::widget::horizontal_space(),
                    button(icon::new().size(14).center())
                        .on_press(Message::CreateNew)
                        .style(Modern::primary_button()),
                ].width(250),
                search_bar,
                header_row,   
                items_list,
            ]
            .width(270)
            .spacing(10)
            .padding(10)
        )
        .style(Modern::card_container()),

        container(content)
            .width(iced::Length::Fill)
            .style(Modern::card_container())

    ]
    .spacing(20)
    .into();
    
    full_view


}

pub struct ViewContext<'a> {
    pub available_items: &'a mut BTreeMap<EntityId, Item>,
    pub available_item_groups: &'a mut BTreeMap<EntityId, ItemGroup>,
    pub available_tax_groups: &'a mut BTreeMap<EntityId, TaxGroup>,
    pub available_security_levels: &'a mut BTreeMap<EntityId, SecurityLevel>,
    pub available_revenue_categories: &'a mut BTreeMap<EntityId, RevenueCategory>,
    pub available_report_categories: &'a mut BTreeMap<EntityId, ReportCategory>,
    pub available_product_classes: &'a mut BTreeMap<EntityId, ProductClass>,
    pub available_choice_groups: &'a mut BTreeMap<EntityId, ChoiceGroup>,
    pub available_printer_logicals: &'a mut BTreeMap<EntityId, PrinterLogical>,
    pub available_price_levels: &'a mut BTreeMap<EntityId, PriceLevel>,
}

fn matches_search(
    item: &Item, 
    query: &str,
    item_groups: &BTreeMap<EntityId, ItemGroup>,
    tax_groups: &BTreeMap<EntityId, TaxGroup>,
    security_levels: &BTreeMap<EntityId, SecurityLevel>,
    revenue_categories: &BTreeMap<EntityId, RevenueCategory>,
    report_categories: &BTreeMap<EntityId, ReportCategory>,
    product_classes: &BTreeMap<EntityId, ProductClass>,
    choice_groups: &BTreeMap<EntityId, ChoiceGroup>,
    printer_logicals: &BTreeMap<EntityId, PrinterLogical>,
    price_levels: &BTreeMap<EntityId, PriceLevel>,
) -> bool {

    // If the search bar is empty, show all items
    if query.trim().is_empty() {
        return true;
    }

    // Convert everything to lowercase for case-insensitive matching
    let query_lower = query.to_lowercase();

    if item.name.to_lowercase().contains(&query_lower) {
        return true;
    }

    //Match on the item's item_group name
    if let Some(ig_id) = &item.item_group {
        if let Some(ig) = item_groups.get(ig_id) {
            if ig.name.to_lowercase().contains(&query_lower) {
                return true;
            }
        }
    } false;

    //Match on the item's tax_group name
    if let Some(tg_id) = &item.tax_group {
        if let Some(tg) = tax_groups.get(tg_id) {
            if tg.name.to_lowercase().contains(&query_lower) {
                return true;
            }
        }
    } false;

    //Match on the item's security_level name
    if let Some(sl_id) = &item.security_level {
        if let Some(sl) = security_levels.get(sl_id) {
            if sl.name.to_lowercase().contains(&query_lower) {
                return true;
            }
        }
    } false;

    //Match on the item's report_category name
    if let Some(rc_id) = &item.report_category {
        if let Some(rc) = report_categories.get(rc_id) {
            if rc.name.to_lowercase().contains(&query_lower) {
                return true;
            }
        }
    } false;

    //Match on the item's security_level name
    if let Some(sl_id) = &item.security_level {
        if let Some(sl) = security_levels.get(sl_id) {
            if sl.name.to_lowercase().contains(&query_lower) {
                return true;
            }
        }
    } false;

    //Match on each choice_group's name
    if let Some(cg_ids) = &item.choice_groups {
        for cg_id in cg_ids {
            if let Some(cg) = choice_groups.get(&cg_id.0) {
                if cg.name.to_lowercase().contains(&query_lower) {
                    return true;
                }
            }
        }
    } false;

    //Match on each printer_logical's name
    if let Some(pl_ids) = &item.printer_logicals {
        for (pl_id, _) in pl_ids {
            if let Some(pl) = printer_logicals.get(pl_id) {
                if pl.name.to_lowercase().contains(&query_lower) {
                    return true;
                }
            }
        }
    } false;

    
    //Match on each price_level's name
    if let Some(pl_ids) = &item.price_levels {
        for pl_id in pl_ids {
            if let Some(pl) = price_levels.get(pl_id) {
                if pl.name.to_lowercase().contains(&query_lower) {
                    return true;
                }
            }
        }
    } false

}


pub fn list_item<'a>(list_text: &'a str, copy_button: iced::widget::Button<'a, Message>,delete_button: iced::widget::Button<'a, Message>) -> Element<'a, Message> {
    let button_content = row![
        text(list_text).size(12).align_x(iced::Alignment::Start).width(150),
        iced::widget::horizontal_space(),
        copy_button.style(Modern::primary_button()),
        delete_button.style(Modern::danger_button())
    ].align_y(Alignment::Center);
    
    button_content.into()
}