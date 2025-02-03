pub mod edit;
pub mod view;

use std::collections::HashMap;
use crate::data_types::{
    EntityId,
    Currency,
    ValidationError,
    Validatable,
    IdRange,
    ExportError,
};
use crate::Action;
use iced::Element;
use rust_decimal::Decimal;
use strsim::jaro_winkler; // For fuzzy matching
use crate::{
    tax_groups::TaxGroup,
    security_levels::SecurityLevel,
    revenue_categories::RevenueCategory,
    report_categories::ReportCategory,
    item_groups::ItemGroup,
    product_classes::ProductClass,
    choice_groups::ChoiceGroup,
    printer_logicals::PrinterLogical,
    price_levels::{PriceLevel, PriceLevelType},
};

#[derive(Debug, Clone)]
pub enum Message {
    Edit(edit::Message),
    View(view::Message),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(Item),
    StartEdit(EntityId),
    Cancel,
    Back,
    ExportToCsv,
}

#[derive(Debug, Clone)]
pub enum Mode {
    View,
    Edit,
}

#[derive(Debug, Clone)]
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
    pub weight_amount: Currency,
    pub sku: Option<String>,
    pub bar_gun_code: Option<String>,
    pub cost_amount: Option<Currency>,
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

impl Item {
    pub fn new(id: EntityId, name: String) -> Self {
        Self {
            id,
            name,
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

    pub fn generate_id(item_group: &ItemGroup) -> EntityId {
        // Generate a new ID within the item group's range
        // For now, just return the start of the range
        // In a real implementation, you'd need to track used IDs
        item_group.id_range.start
    }

    pub fn find_similar<'a>(&self, items: &'a [&Item]) -> Vec<&'a Item> {
        const SIMILARITY_THRESHOLD: f64 = 0.9;
        
        items.iter()
            .filter(|item| {
                if item.id == self.id {
                    return false; // Don't match self
                }
                
                use strsim::jaro_winkler;
                
                // Check name similarity
                let name_similarity = jaro_winkler(&self.name, &item.name);
                if name_similarity > SIMILARITY_THRESHOLD {
                    return true;
                }
                
                // Check button text similarity
                let button1_similarity = jaro_winkler(&self.button1, &item.button1);
                if button1_similarity > SIMILARITY_THRESHOLD {
                    return true;
                }
                
                if let (Some(ref b1), Some(ref b2)) = (&self.button2, &item.button2) {
                    let button2_similarity = jaro_winkler(b1, b2);
                    if button2_similarity > SIMILARITY_THRESHOLD {
                        return true;
                    }
                }
                
                false
            })
            .copied()
            .collect()
    }

    pub fn to_csv_row(&self) -> Result<String, ExportError> {
        let mut row = Vec::new();
        
        row.push(self.id.to_string());
        row.push(format!("\"{}\"", self.name.replace("\"", "\"\"")));
        row.push(format!("\"{}\"", self.button1.replace("\"", "\"\"")));
        row.push(self.button2.as_ref().map_or(String::new(), |b| format!("\"{}\"", b.replace("\"", "\"\""))));
        row.push(format!("\"{}\"", self.printer_text.replace("\"", "\"\"")));
        // Add all other fields in the correct order
        
        Ok(row.join(","))
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Validatable for Item {
    fn validate(&self) -> Result<(), ValidationError> {
        // Validate name is not empty
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Item name cannot be empty".to_string()
            ));
        }

        // Validate button text lengths
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

        // Validate price amounts are non-negative
        if let Some(cost) = self.cost_amount {
            if cost < Decimal::ZERO {
                return Err(ValidationError::InvalidValue(
                    "Cost amount cannot be negative".to_string()
                ));
            }
        }

        if self.weight_amount < Decimal::ZERO {
            return Err(ValidationError::InvalidValue(
                "Weight amount cannot be negative".to_string()
            ));
        }

        Ok(())
    }
}

pub struct ViewContext<'a> {
    pub available_item_groups: &'a HashMap<EntityId, ItemGroup>,
    pub available_tax_groups: &'a HashMap<EntityId, TaxGroup>,
    pub available_security_levels: &'a HashMap<EntityId, SecurityLevel>,
    pub available_revenue_categories: &'a HashMap<EntityId, RevenueCategory>,
    pub available_report_categories: &'a HashMap<EntityId, ReportCategory>,
    pub available_product_classes: &'a HashMap<EntityId, ProductClass>,
    pub available_choice_groups: &'a HashMap<EntityId, ChoiceGroup>,
    pub available_printer_logicals: &'a HashMap<EntityId, PrinterLogical>,
    pub available_price_levels: &'a HashMap<EntityId, PriceLevel>,
}

pub fn update<'a>(
    item: &'a mut Item, 
    message: Message, 
    context: &'a ViewContext<'a>,
    state: &'a mut edit::EditState,
) -> Action<Operation, Message> {
    match message {
        Message::Edit(msg) => {
            // Map the edit::Message to Message
            edit::update(item, msg, state)
                .map(Message::Edit)  // Map the edit Message type to our Message enum
        }
        Message::View(msg) => match msg {
            view::Message::Edit => Action::operation(Operation::StartEdit(item.id)),
            view::Message::Back => Action::operation(Operation::Back),
            view::Message::ExportToCsv => Action::operation(Operation::ExportToCsv),
        },
    }
}

pub fn view<'a>(
    item: &'a Item, 
    mode: &'a Mode, 
    context: &'a ViewContext<'a>
) -> Element<'a, Message> {
    match mode {
        Mode::View => view::view(item, context).map(Message::View),
        Mode::Edit => {
            edit::view(
                item,
                edit::EditState::new(item),
                context
            ).map(Message::Edit)
        }
    }
}