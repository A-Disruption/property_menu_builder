use std::fs;
use std::path::PathBuf;
use std::collections::{BTreeMap, HashSet};
use csv::ReaderBuilder;
use rust_decimal::Decimal;
use crate::{
    items::{Item, ItemPrice},
    choice_groups::ChoiceGroup,
    item_groups::ItemGroup,
    price_levels::PriceLevel,
    printer_logicals::PrinterLogical,
    product_classes::ProductClass,
    report_categories::ReportCategory,
    revenue_categories::RevenueCategory,
    security_levels::SecurityLevel,
    tax_groups::TaxGroup, 
    data_types::EntityId
};


const EXPECTED_FIELD_COUNT: usize = 42;

pub fn verify_csv_format(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(&path)?;

    let masked_contents = mask_braced_commas(&contents);

    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(masked_contents.as_bytes());

    for result in reader.records() {
        let record = result?;

        if record.len() != EXPECTED_FIELD_COUNT {
            return Err(format!("Unexpected field count: {}", record.len()).into());
        }
    }

    Ok(())
}

pub fn is_csv_or_txt(path: PathBuf) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map_or(false, |ext_str| ext_str == "csv" || ext_str == "txt")
}


fn mask_braced_commas(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut inside_braces = false;

    for c in s.chars() {
        match c {
            '{' => {
                inside_braces = true;
                result.push(c);
            }
            '}' => {
                inside_braces = false;
                result.push(c);
            }
            ',' if inside_braces=> {
                result.push('␟');
            }
            _ => result.push(c),
        }
    }

    result
}

fn unmask_braced_commas(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut inside_braces = false;

    for c in s.chars() {
        match c {
            '{' => {
                inside_braces = true;
            }
            '}' => {
                inside_braces = false;
            }
            '␟' if inside_braces=> {
                result.push(',');
            }
            _ => result.push(c),
        }
    }

    result
}

pub fn collect_item_information(path: &PathBuf) -> Result<BTreeMap<EntityId, Item>, Box<dyn std::error::Error>> {
    println!("Running collect_item_information function on path: {:?}", &path);
    let contents = fs::read_to_string(&path)?;
    let masked_contents = mask_braced_commas(&contents);
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(masked_contents.as_bytes());

    let mut items_map = BTreeMap::new();
    
    for result in reader.deserialize() {
        let record: ImportStructure = result?;
        
        // Parse the item ID
        let entity_id = match record.item_id.parse::<i32>() {
            Ok(id) => id,
            Err(_) => continue, // Skip records with invalid IDs
        };
        
        // Helper function to parse a string into an Optional EntityId
        let parse_entity_id = |s: &str| -> Option<EntityId> {
            if s.trim().is_empty() {
                None
            } else {
                s.parse::<i32>().ok()
            }
        };
        
        // Helper function to parse comma-separated EntityIds into a Vec
        let parse_entity_ids = |s: &str| -> Option<Vec<EntityId>> {
            if s.trim().is_empty() {
                None
            } else {
                let ids: Vec<EntityId> = s.split(',')
                    .filter_map(|id| id.trim().parse::<i32>().ok())
                    .collect();
                
                if ids.is_empty() { None } else { Some(ids) }
            }
        };
        
        // Helper function to parse a string into a bool
        let parse_bool = |s: &str| -> bool {
            match s.trim() {
                "1" | "Y" | "YES" | "TRUE" | "true" | "yes" | "y" => true,
                _ => false,
            }
        };
        
        // Helper function to parse a string into a Decimal
        let parse_decimal = |s: &str| -> Decimal {
            s.parse::<Decimal>().unwrap_or(Decimal::ZERO)
        };
        
        // Helper function to parse an optional Decimal
        let parse_optional_decimal = |s: &str| -> Option<Decimal> {
            if s.trim().is_empty() {
                None
            } else {
                s.parse::<Decimal>().ok()
            }
        };
        
        // Helper function to parse a string into an i32
        let parse_i32 = |s: &str| -> i32 {
            s.parse::<i32>().unwrap_or(0)
        };
        
        // Helper to convert empty strings to None
        let string_or_none = |s: &str| -> Option<String> {
            if s.trim().is_empty() {
                None
            } else {
                Some(s.to_string())
            }
        };
        
        // Parse price levels - this is a more complex field
        // Assuming format is something like "1:5.99,2:6.99,3:7.99"
        // where the numbers before colons are price level IDs and after are prices
        let price_level_ids = None;
        let (default_price, item_prices) = if let Some(mut prices) = parse_item_prices(&record.price_levels) {
            if prices.is_empty() {
                // If no prices, use a default
                (
                    ItemPrice { price_level_id: 1, price: Decimal::new(0, 2) }, // $0.00
                    None
                )
            } else {
                // Take the first price as default
                let default = prices.remove(0); // Remove and return the first element
                
                // If there are remaining prices, process them
                let processed_prices = if prices.is_empty() {
                    None
                } else {
                    // Decrement each price level ID by 1
                    Some(prices.into_iter()
                        .map(|mut price| {
                            price.price_level_id -= 1;
                            price
                        })
                        .collect::<Vec<ItemPrice>>())
                };
                
                (default, processed_prices)
            }
        } else {
            // No prices at all, use default
            (
                ItemPrice { price_level_id: 1, price: Decimal::new(0, 2) }, // $0.00
                None
            )
        };
        
        // Create Item from ImportStructure with proper type conversions
        let item = Item {
            id: entity_id,
            name: record.item_name.clone(),
            button1: record.button_1.clone(),
            button2: string_or_none(&record.button_2),
            printer_text: record.kitchen_printer_text.clone(),
            default_price: Some(default_price.price),
            price_levels: price_level_ids,
            item_prices: item_prices,
            product_class: parse_entity_id(&record.product_class_id),
            revenue_category: parse_entity_id(&record.revenue_category_id),
            tax_group: parse_entity_id(&record.tax_group_id),
            security_level: parse_entity_id(&record.security_level_id),
            report_category: parse_entity_id(&record.report_category_id),
            use_weight: parse_bool(&record.use_weight_flag),
            weight_amount: parse_decimal(&record.weight_tare_amount),
            sku: string_or_none(&record.sku),
            bar_gun_code: string_or_none(&record.bar_gun_code),
            cost_amount: parse_optional_decimal(&record.cost_amount),
            reserved1: parse_bool(&record.reserved1),
            ask_price: parse_bool(&record.ask_price),
            print_on_check: parse_bool(&record.print_on_check),
            discountable: parse_bool(&record.discountable),
            voidable: parse_bool(&record.voidable),
            not_active: parse_bool(&record.not_active),
            tax_included: parse_bool(&record.tax_included),
            item_group: parse_entity_id(&record.item_group_id),
            customer_receipt: record.customer_receipt_text.clone(),
            allow_price_override: parse_bool(&record.allow_price_override),
            reserved2: parse_bool(&record.reserved2),
            choice_groups: parse_choice_groups(&record.choice_groups),
            printer_logicals: parse_printer_logicals(&record.kitchen_printers),
            covers: parse_i32(&record.covers),
            store_id: parse_i32(&record.store_id),
            kitchen_video: record.kitchen_video_text.clone(),
            kds_dept: parse_i32(&record.kds_department),
            kds_category: record.kds_category.clone(),
            kds_cooktime: parse_i32(&record.kdc_cook_time),
            store_price_level: parse_entity_ids(&record.store_price_level),
            image_id: parse_i32(&record.image_id),
            stock_item: parse_bool(&record.stock_item_flag),
            language_iso_code: record.language_iso_code.clone(),
        };
        //println!("{:?}", &item);
        
        items_map.insert(entity_id, item);
    }
    
    if items_map.is_empty() {
        return Err(format!("No valid items found in the file.").into());
    }
    
    Ok(items_map)
}

// Helper function to parse price levels and item prices from a string
fn parse_item_prices (price_levels_str: &str) -> (Option<Vec<ItemPrice>>) { //updating above function because I'm not using price_levels on items anymore.
    if price_levels_str.trim().is_empty() {
        return None;
    }

//println!("{}", &price_levels_str);

    let mut price_levels = price_levels_str.replace("␟$",":");
    price_levels = price_levels.replace("{","");
    price_levels = price_levels.replace("}","");

//println!("{}", &price_levels);
    
    let mut item_prices = Vec::new();
    
    // Split by comma to get each price level entry
    for entry in price_levels.split('␟') {
        println!("Running as least once!");
        // Split by colon to get level ID and price
        let parts: Vec<&str> = entry.split(':').collect();
//println!("Parts count: {}", &parts.len());
//println!("Part 0: {}", &parts[0]);
//println!("Part 1: {}", &parts[1]);
        
        if parts.len() == 2 {
            if let Ok(level_id) = parts[0].trim().parse::<i32>() { 
                println!("level_id: {}", &level_id);
                if let Ok(price_value) = parts[1].trim().parse::<Decimal>() {
                    println!("Decimal: {}", &price_value);
                    item_prices.push(ItemPrice {
                        price_level_id: level_id,
                        price: price_value,
                    });
                }
            }
        }
    }

// println!("{:?}", &item_prices);
    
    let items_option = if item_prices.is_empty() { None } else { Some(item_prices) };

//println!("{:?}", &items_option);

    items_option
}

fn parse_printer_logicals(input: &str) -> Option<Vec<(i32, bool)>> {
    // Trim the brackets
    let trimmed = input.trim_start_matches('{').trim_end_matches('}');
    println!("Trimmed: {:?}", trimmed);
    
    // Split by 'comma', then, parse the numbers into a vector
    let numbers: Vec<i32> = trimmed
        .split('␟')
        .filter_map(|s| s.trim().parse::<i32>().ok())
        .collect();
    println!("Numbers: {:?}", numbers);
    
    // Process pairs of numbers to create the final result
    let mut result = Vec::new();
    let mut iter = numbers.iter();
    
    while let (Some(&id), Some(&value)) = (iter.next(), iter.next()) {
        result.push((id, value != 0));
    }

    let printers_option = if result.is_empty() { None } else { Some(result) };
    println!("Printer Options: {:?}", printers_option);
    
    printers_option
}

fn parse_choice_groups(input: &str) -> Option<Vec<(i32, i32)>> {
    // Trim the brackets
    let trimmed = input.trim_start_matches('{').trim_end_matches('}');
    println!("Trimmed: {:?}", trimmed);
    
    // Split by 'comma', then, parse the numbers into a vector
    let numbers: Vec<i32> = trimmed
        .split('␟')
        .filter_map(|s| s.trim().parse::<i32>().ok())
        .collect();
    println!("Numbers: {:?}", numbers);
    
    // Process pairs of numbers to create the final result
    let mut result = Vec::new();
    let mut iter = numbers.iter();
    
    while let (Some(&id), Some(&sequence_number)) = (iter.next(), iter.next()) {
        result.push((id, sequence_number));
    }

    let groups_option = if result.is_empty() { None } else { Some(result) };
    println!("Choice Groups: {:?}", groups_option);
    
    groups_option
}


#[derive(serde::Deserialize)]
struct ImportStructure {
    add: String,
    item_id: String,
    item_name: String,
    button_1: String,
    button_2: String,
    kitchen_printer_text: String,
    price_levels: String,
    product_class_id: String,
    revenue_category_id: String,
    tax_group_id: String,
    security_level_id: String,
    report_category_id: String,
    use_weight_flag: String,
    weight_tare_amount: String,
    sku: String,
    bar_gun_code: String,
    cost_amount: String,
    reserved1: String,
    ask_price: String,
    print_on_check: String,
    discountable: String,
    voidable: String,
    not_active: String,
    tax_included: String,
    item_group_id: String,
    customer_receipt_text: String,
    allow_price_override: String,
    reserved2: String,
    choice_groups: String,
    kitchen_printers: String,
    covers: String,
    store_id: String,
    kitchen_video_text: String,
    kds_department: String,
    kds_category: String,
    kdc_cook_time: String,
    store_price_level: String,
    image_id: String,
    stock_item_flag: String,
    language_iso_code: String,
    reserved3: String,
    reserved4: String,
}

/// Ensures all referenced entities exist by creating defaults for missing references
pub fn ensure_all_referenced_entities_exist(
    items: &BTreeMap<EntityId, Item>,
    price_levels: &mut BTreeMap<EntityId, PriceLevel>,
    product_classes: &mut BTreeMap<EntityId, ProductClass>,
    revenue_categories: &mut BTreeMap<EntityId, RevenueCategory>,
    tax_groups: &mut BTreeMap<EntityId, TaxGroup>,
    security_levels: &mut BTreeMap<EntityId, SecurityLevel>,
    report_categories: &mut BTreeMap<EntityId, ReportCategory>,
    item_groups: &mut BTreeMap<EntityId, ItemGroup>,
    choice_groups: &mut BTreeMap<EntityId, ChoiceGroup>,
    printer_logicals: &mut BTreeMap<EntityId, PrinterLogical>,
) {
    // Ensure price levels exist
    ensure_price_levels_exist(items, price_levels);
    
    // Ensure product classes exist
    ensure_product_classes_exist(items, product_classes);
    
    // Ensure revenue categories exist
    ensure_revenue_categories_exist(items, revenue_categories);
    
    // Ensure tax groups exist
    ensure_tax_groups_exist(items, tax_groups);
    
    // Ensure security levels exist
    ensure_security_levels_exist(items, security_levels);
    
    // Ensure report categories exist
    ensure_report_categories_exist(items, report_categories);
    
    // Ensure item groups exist
    ensure_item_groups_exist(items, item_groups);
    
    // Ensure choice groups exist
    ensure_choice_groups_exist(items, choice_groups);
    
    // Ensure printer logicals exist
    ensure_printer_logicals_exist(items, printer_logicals);
    
    println!("All referenced entities have been checked and created if missing");
}

/// Ensures all price levels referenced by items exist
fn ensure_price_levels_exist(
    items: &BTreeMap<EntityId, Item>,
    price_levels: &mut BTreeMap<EntityId, PriceLevel>,
) {
    // Collect all referenced price level IDs
    let mut referenced_ids = HashSet::new();
    
    for item in items.values() {
        // Check all fields that reference price levels
        if let Some(item_price_levels) = &item.price_levels {
            for &id in item_price_levels {
                referenced_ids.insert(id);
            }
        }
        
        if let Some(item_prices) = &item.item_prices {
            for price in item_prices {
                referenced_ids.insert(price.price_level_id);
            }
        }
        
        if let Some(store_price_levels) = &item.store_price_level {
            for &id in store_price_levels {
                referenced_ids.insert(id);
            }
        }
    }
    
    // Create missing price levels
    for id in referenced_ids {
        if !price_levels.contains_key(&id) {
            let price_level = PriceLevel {
                id,
                name: format!("Price Level {}", id),
                // Set other fields to defaults
                ..PriceLevel::default()
            };
            
            price_levels.insert(id, price_level);
            println!("Created missing price level with ID: {}", id);
        }
    }
}

// Similarly implement functions for other entity types
fn ensure_product_classes_exist(
    items: &BTreeMap<EntityId, Item>,
    product_classes: &mut BTreeMap<EntityId, ProductClass>,
) {
    let mut referenced_ids = HashSet::new();
    
    for item in items.values() {
        if let Some(id) = item.product_class {
            referenced_ids.insert(id);
        }
    }
    
    for id in referenced_ids {
        if !product_classes.contains_key(&id) {
            let product_class = ProductClass {
                id,
                name: format!("Product Class {}", id),
                // Other default fields
                ..ProductClass::default()
            };
            
            product_classes.insert(id, product_class);
            println!("Created missing product class with ID: {}", id);
        }
    }
}

fn ensure_revenue_categories_exist(    
    items: &BTreeMap<EntityId, Item>,
    revenue_categories: &mut BTreeMap<EntityId, RevenueCategory>,
) {
    let mut referenced_ids = HashSet::new();
    
    for item in items.values() {
        if let Some(id) = item.revenue_category {
            referenced_ids.insert(id);
        }
    }
    
    for id in referenced_ids {
        if !revenue_categories.contains_key(&id) {
            let revenue_category = RevenueCategory {
                id,
                name: format!("Revenue Category {}", id),
                // Other default fields
                ..RevenueCategory::default()
            };
            
            revenue_categories.insert(id, revenue_category);
            println!("Created missing revenue category with ID: {}", id);
        }
    }  
}

fn ensure_tax_groups_exist(    
    items: &BTreeMap<EntityId, Item>,
    tax_groups: &mut BTreeMap<EntityId, TaxGroup>,
) {
    let mut referenced_ids = HashSet::new();
    
    for item in items.values() {
        if let Some(id) = item.tax_group {
            referenced_ids.insert(id);
        }
    }
    
    for id in referenced_ids {
        if !tax_groups.contains_key(&id) {
            let tax_group = TaxGroup {
                id,
                name: format!("Tax Group {}", id),
                // Other default fields
                ..TaxGroup::default()
            };
            
            tax_groups.insert(id, tax_group);
            println!("Created missing tax group with ID: {}", id);
        }
    }  
}

fn ensure_security_levels_exist(    
    items: &BTreeMap<EntityId, Item>,
    security_levels: &mut BTreeMap<EntityId, SecurityLevel>,
) {
    let mut referenced_ids = HashSet::new();
    
    for item in items.values() {
        if let Some(id) = item.tax_group {
            referenced_ids.insert(id);
        }
    }
    
    for id in referenced_ids {
        if !security_levels.contains_key(&id) {
            let security_level = SecurityLevel {
                id,
                name: format!("Security Level {}", id),
                // Other default fields
                ..SecurityLevel::default()
            };
            
            security_levels.insert(id, security_level);
            println!("Created missing security level with ID: {}", id);
        }
    }  
}

fn ensure_report_categories_exist(    
    items: &BTreeMap<EntityId, Item>,
    report_categories: &mut BTreeMap<EntityId, ReportCategory>,
) {
    let mut referenced_ids = HashSet::new();
    
    for item in items.values() {
        if let Some(id) = item.report_category {
            referenced_ids.insert(id);
        }
    }
    
    for id in referenced_ids {
        if !report_categories.contains_key(&id) {
            let report_category = ReportCategory {
                id,
                name: format!("Report Category {}", id),
                // Other default fields
                ..ReportCategory::default()
            };
            
            report_categories.insert(id, report_category);
            println!("Created missing report category with ID: {}", id);
        }
    }  
}

fn ensure_item_groups_exist(    
    items: &BTreeMap<EntityId, Item>,
    item_groups: &mut BTreeMap<EntityId, ItemGroup>,
) {
    let mut referenced_ids = HashSet::new();
    
    for item in items.values() {
        if let Some(id) = item.item_group {
            referenced_ids.insert(id);
        }
    }
    
    for id in referenced_ids {
        if !item_groups.contains_key(&id) {
            let item_group = ItemGroup {
                id,
                name: format!("Item Group {}", id),
                // Other default fields
                ..ItemGroup::default()
            };
            
            item_groups.insert(id, item_group);
            println!("Created missing item group with ID: {}", id);
        }
    }  
}

fn ensure_choice_groups_exist(
    items: &BTreeMap<EntityId, Item>,
    choice_groups: &mut BTreeMap<EntityId, ChoiceGroup>,
) {
    let mut referenced_ids = HashSet::new();
    
    // Collect all referenced choice group IDs from items
    for item in items.values() {
        if let Some(group_ids) = &item.choice_groups {
            // Handle the Vec<EntityId> case - add each ID in the vector 
            for &id in group_ids {
                referenced_ids.insert(id);
            }
        }
    }
    
    // Create missing choice groups
    for (id, _) in referenced_ids {
        if !choice_groups.contains_key(&id) {
            let choice_group = ChoiceGroup {
                id,
                name: format!("Item Group {}", id),
                // Set other fields to defaults
                ..ChoiceGroup::default()
            };
            
            choice_groups.insert(id, choice_group);
            println!("Created missing choice group with ID: {}", id);
        }
    }
}

fn ensure_printer_logicals_exist(
    items: &BTreeMap<EntityId, Item>,
    printer_logicals: &mut BTreeMap<EntityId, PrinterLogical>,
) {
    let mut referenced_ids = HashSet::new();
    
    // Collect all referenced printer logical IDs from items
    for item in items.values() {
        if let Some(printer_ids) = &item.printer_logicals {
            // Handle the Vec<EntityId> case - add each ID in the vector 
            for &id in printer_ids {
                referenced_ids.insert(id);
            }
        }
    }
    
    // Create missing printer logicals
    for (id, _) in referenced_ids {
        if !printer_logicals.contains_key(&id) {
            let printer_logical = PrinterLogical {
                id,
                name: format!("Printer Logical {}", id),
                // Set other fields to defaults
                ..PrinterLogical::default()
            };
            
            printer_logicals.insert(id, printer_logical);
            println!("Created missing printer logical with ID: {}", id);
        }
    }
}