use std::collections::BTreeMap;
use std::io::Write;
use crate::data_types::{EntityId, ItemPrice};
use crate::items::Item;

/// Export all items to CSV file in the format expected by the POS system
pub fn export_to_csv(items: &BTreeMap<EntityId, Item>, file_path: &str) -> Result<(), String> {
    use std::fs::File;
    use std::io::{Write, BufWriter};
    
    let file = File::create(file_path).map_err(|e| format!("Failed to create file: {}", e))?;
    let mut writer = BufWriter::new(file);
    
    // Write header row
    writeln!(writer, "Add\tItem ID\tItem Name\tButton 1 (Upper half)\tButton 2 (Lower half)\tKitchen Printer Text\tDefault Price & Price Levels\tProduct Class ID\tRevenue Category ID\tTax Group ID\tSecurity Level ID\tReport Category ID\tUse Weight Flag\tWeight Tare Amount\tSKU #\tBar Gun Code\tCost Amount\tReserved\tAsk Price\tPrint on Check\tDiscountable\tVoidable\tNot Active (86'd)\tTax Included\tItem Group ID\tCustomer Receipt Text\tAllow Price Override\tReserved\tChoice Groups\tKitchen Printers (Logical)\tCovers\tStore ID\tKitchen Video Text\tKDS Department\tKDS Category\tKDS Cook Time (secs.)\tStore Price Level\tImage ID\tStock Item Flag\tLanguage ISO Code*\tReserved\tReserved")
        .map_err(|e| format!("Failed to write header: {}", e))?;
    
    // Sort items by ID for consistent output
    let mut sorted_items: Vec<&Item> = items.values().collect();
    sorted_items.sort_by_key(|item| item.id);
    
    // Write item rows
    for item in sorted_items {
        write_item_row(&mut writer, item).map_err(|e| format!("Failed to write item: {}", e))?;
    }
    
    writer.flush().map_err(|e| format!("Failed to flush data: {}", e))?;
    Ok(())
}

/// Write a single item as a row in the CSV
fn write_item_row<W: Write>(writer: &mut W, item: &Item) -> std::io::Result<()> {
    // Format price levels as {id,$price,id,$price,...}
    let price_levels_str = format_price_levels(item);
    
    // Format choice groups as {id,id,id,...}
    let choice_groups_str = format_choice_groups(item);
    
    // Format printer logicals as {id,order,id,order,...}
    let printer_logicals_str = format_printer_logicals(item);
    
    // Format store price levels
    let store_price_levels_str = format_store_price_levels(item);
    
    // Convert boolean flags to 0/1
    let use_weight_flag = if item.use_weight { 1 } else { 0 };
    let reserved1 = if item.reserved1 { 1 } else { 0 };
    let ask_price = if item.ask_price { 1 } else { 0 };
    let print_on_check = if item.print_on_check { 1 } else { 0 };
    let discountable = if item.discountable { 1 } else { 0 };
    let voidable = if item.voidable { 1 } else { 0 };
    let not_active = if item.not_active { 1 } else { 0 };
    let tax_included = if item.tax_included { 1 } else { 0 };
    let allow_price_override = if item.allow_price_override { 1 } else { 0 };
    let reserved2 = if item.reserved2 { 1 } else { 0 };
    let stock_item = if item.stock_item { 1 } else { 0 };
    
    // Format optional values
    let button2 = item.button2.as_deref().unwrap_or("");
    let sku = item.sku.as_deref().unwrap_or("");
    let bar_gun_code = item.bar_gun_code.as_deref().unwrap_or("");
    let cost_amount = item.cost_amount.map_or_else(|| "$0.00 ".to_string(), |c| format!("${:.2} ", c));
    
    // Get IDs or 0 for nulls
    let product_class_id = item.product_class.unwrap_or(0);
    let revenue_category_id = item.revenue_category.unwrap_or(0);
    let tax_group_id = item.tax_group.unwrap_or(0);
    let security_level_id = item.security_level.unwrap_or(0);
    let report_category_id = item.report_category.unwrap_or(0);
    let item_group_id = item.item_group.unwrap_or(0);
    
    // Format language code with double quotes if empty
    let language_iso_code = if item.language_iso_code.is_empty() {
        "\"\"".to_string()
    } else {
        format!("\"{}\"", item.language_iso_code)
    };
    
    // Write tab-separated values
    writeln!(
        writer,
        "\"A\"\t{}\t\"{}\"\t\"{}\"\t\"{}\"\t\"{}\"\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{{}}\t\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t\"{}\"\t{}\t{}\t{}\t{}\t{}\t{}\t\"{}\"\t{}\t\"{}\"\t{}\t{{}}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}t\"\"",
        item.id,
        item.name,
        item.button1,
        button2,
        item.printer_text,
        price_levels_str,
        product_class_id,
        revenue_category_id,
        tax_group_id,
        security_level_id,
        report_category_id,
        use_weight_flag,
        item.weight_amount,
        sku,
        bar_gun_code,
        cost_amount,
        reserved1,
        ask_price,
        print_on_check,
        discountable,
        voidable,
        not_active,
        tax_included,
        item_group_id,
        item.customer_receipt,
        allow_price_override,
        reserved2,
        choice_groups_str,
        printer_logicals_str,
        item.covers,
        item.store_id,
        item.kitchen_video,
        item.kds_dept,
        item.kds_category,
        item.kds_cooktime,
        store_price_levels_str,
        item.image_id,
        stock_item,
        language_iso_code,
        0,  // Reserved
        ""   // Reserved
    )
}

/// Format price levels as {id,$price,id,$price,...}
fn format_price_levels(item: &Item) -> String {
    // Format item prices as {level_id,$price}
    // Default is {1,$0.00}
    if let Some(item_prices) = &item.item_prices {
        if !item_prices.is_empty() {
            let price_str = item_prices.iter()
                .map(|price| format!("{},${:.2}", price.price_level_id, price.price))
                .collect::<Vec<_>>()
                .join(",");
            
            return format!("{{{}}}", price_str);
        }
    }
    
    // Default if no prices
    "{1,$0.00}".to_string()
}

/// Format choice groups as {id,id,id,...}
fn format_choice_groups(item: &Item) -> String {
    if let Some(groups) = &item.choice_groups {
        if !groups.is_empty() {
            let groups_str = groups.iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");
            
            return format!("{{{}}}", groups_str);
        }
    }
    
    "{}".to_string() // Empty brackets if no choice groups
}

/// Format printer logicals as {id,order,id,order,...}
fn format_printer_logicals(item: &Item) -> String {
    if let Some(printers) = &item.printer_logicals {
        if !printers.is_empty() {
            // In this format, each printer ID is followed by its order number
            // For simplicity, we'll use the array position as the order
            let mut printer_str = String::new();
            
            for (i, &printer_id) in printers.iter().enumerate() {
                if i > 0 {
                    printer_str.push_str(",");
                }
                // Format as id,order
                printer_str.push_str(&format!("{},{}", printer_id, i+1));
            }
            
            return format!("{{{}}}", printer_str);
        }
    }
    
    "{}".to_string() // Empty brackets if no printers
}

/// Format store price levels as {id,id,id,...}
fn format_store_price_levels(item: &Item) -> String {
    if let Some(levels) = &item.store_price_level {
        if !levels.is_empty() {
            let levels_str = levels.iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");
            
            return format!("{{{}}}", levels_str);
        }
    }
    
    "{}".to_string() // Empty brackets if no price levels
}

/// Verify that a CSV export has the correct format
pub fn verify_export_format(file_path: &str) -> Result<bool, String> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    
    let file = File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;
    let reader = BufReader::new(file);
    
    // Skip header line
    let mut lines = reader.lines();
    let _ = lines.next();
    
    // Check the first item row
    if let Some(Ok(line)) = lines.next() {
        // Just do a basic format check
        if line.starts_with("\"A\"") && line.contains("\t") {
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        Err("No data rows in file".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use std::io::Cursor;

    #[test]
    fn test_csv_format() {
        // Create a test item matching the format from paste.txt
        let item = Item {
            id: 7400002,
            name: "Spiced Nuts".to_string(),
            button1: "Spiced".to_string(),
            button2: Some("Nuts".to_string()),
            printer_text: "Spiced Nuts".to_string(),
            price_levels: Some(vec![1]),
            item_prices: Some(vec![ItemPrice {
                price_level_id: 1,
                price: Decimal::new(800, 2),  // $8.00
            }]),
            product_class: Some(103),
            revenue_category: Some(1),
            tax_group: Some(1),
            security_level: Some(0),
            report_category: Some(0),
            use_weight: false,
            weight_amount: Decimal::ZERO,
            sku: None,
            bar_gun_code: None,
            cost_amount: Some(Decimal::ZERO),
            reserved1: false,
            ask_price: false,
            print_on_check: true,
            discountable: true,
            voidable: true,
            not_active: false,
            tax_included: false,
            item_group: Some(125),
            customer_receipt: "Spiced Nuts".to_string(),
            allow_price_override: true,
            reserved2: false,
            choice_groups: None,
            printer_logicals: Some(vec![1, 2, 3, 20]),
            covers: 0,
            store_id: 0,
            kitchen_video: "Spiced Nuts".to_string(),
            kds_dept: 0,
            kds_category: "".to_string(),
            kds_cooktime: 0,
            store_price_level: None,
            image_id: 0,
            stock_item: false,
            language_iso_code: "".to_string(),
        };

        // Write to a memory buffer
        let mut buffer = Cursor::new(Vec::new());
        write_item_row(&mut buffer, &item).unwrap();
        
        // Convert to string
        let result = String::from_utf8(buffer.into_inner()).unwrap();
        
        // Expected format matches paste.txt example for Spiced Nuts
        let expected = "\"A\"\t7400002\t\"Spiced Nuts\"\t\"Spiced\"\t\"Nuts\"\t\"Spiced Nuts\"\t{1,$8.00}\t103\t1\t1\t0\t0\t0\t0\t{}\t\t$0.00 \t0\t0\t1\t1\t1\t0\t0\t125\t\"Spiced Nuts\"\t1\t0\t{}\t{1,1,2,2,3,3,20,4}\t0\t0\t\"Spiced Nuts\"\t0\t\"\"\t0\t{}\t0\t0\t\"\"\t0\t\"\"\n";
        
        assert_eq!(result, expected);
    }
}