use crate::items::{Item, ItemPrice};
use rust_decimal::Decimal;

pub fn prepare_item_export(items: &[Item], file_path: &str) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;
    
    let mut file = File::create(file_path)?;
    
    for (i, item) in items.iter().enumerate() {
        // Write each item's export string
        let export_string = item_to_export_string(item);
        file.write_all(export_string.as_bytes())?;
        
        // Add newline if not the last item
        if i < items.len() - 1 {
            file.write_all(b"\n")?;
        }
    }
    
    Ok(())
}

pub fn item_to_export_string(item: &Item) -> String {

    // Pre-allocate buffer to avoid reallocations
    let mut result = String::with_capacity(1024);
    
    // Helper function to wrap text in quotes and append to buffer
    fn append_quoted(buffer: &mut String, text: &str) {
        buffer.push('"');
        buffer.push_str(text);
        buffer.push('"');
    }
    
    // add_edit_delete
    result.push_str("\"A\"");
    result.push(',');
    
    // item_id
    result.push_str(&item.id.to_string());
    result.push(',');
    
    // item_name
    append_quoted(&mut result, &item.name);
    result.push(',');
    
    // button_1
    append_quoted(&mut result, &item.button1);
    result.push(',');
    
    // button_2
    append_quoted(&mut result, &item.button2.as_deref().unwrap_or(""));
    result.push(',');
    
    // printer_text
    append_quoted(&mut result, &item.printer_text);
    result.push(',');
    
    // prices
    let prices = prepare_item_prices(item.default_price, item.item_prices.clone());
    result.push_str(&prices);
    result.push(',');
    
    // product_class
    result.push_str(&item.product_class.unwrap_or_default().to_string());
    result.push(',');
    
    // revenue_category
    result.push_str(&item.revenue_category.unwrap_or_default().to_string());
    result.push(',');
    
    // tax_group
    result.push_str(&item.tax_group.unwrap_or_default().to_string());
    result.push(',');
    
    // security_level
    result.push_str(&item.security_level.unwrap_or_default().to_string());
    result.push(',');
    
    // report_category
    result.push_str(&item.report_category.unwrap_or_default().to_string());
    result.push(',');
    
    // weight_flag
    result.push_str(if item.use_weight { "1" } else { "0" });
    result.push(',');
    
    // weight_tar
    result.push_str(&item.weight_amount.to_string());
    result.push(',');
    
    // sku
    result.push_str(&item.sku.as_deref().unwrap_or_default());
    result.push(',');
    
    // bar_gun_code
    result.push_str(&item.bar_gun_code.as_deref().unwrap_or_default());
    result.push(',');
    
    // cost_amount
    let cost_str = prepare_item_cost(item.cost_amount);
    result.push_str(&cost_str);
    result.push(',');
    
    // reserved1
    result.push('0');
    result.push(',');
    
    // ask_price
    result.push_str(if item.ask_price { "1" } else { "0" });
    result.push(',');
    
    // print_on_check
    result.push_str(if item.print_on_check { "1" } else { "0" });
    result.push(',');
    
    // discountable
    result.push_str(if item.discountable { "1" } else { "0" });
    result.push(',');
    
    // voidable
    result.push_str(if item.voidable { "1" } else { "0" });
    result.push(',');
    
    // not_active
    result.push_str(if item.not_active { "1" } else { "0" });
    result.push(',');
    
    // tax_included
    result.push_str(if item.tax_included { "1" } else { "0" });
    result.push(',');
    
    // item_group_id
    result.push_str(&item.item_group.unwrap_or_default().to_string());
    result.push(',');
    
    // receipt_text
    append_quoted(&mut result, &item.customer_receipt);
    result.push(',');
    
    // allow_price_override
    result.push_str(if item.allow_price_override { "1" } else { "0" });
    result.push(',');
    
    // reserved2
    result.push('0');
    result.push(',');
    
    // choice_groups
    let choice_str = prepare_choice_groups(item.choice_groups.clone());
    result.push_str(&choice_str);
    result.push(',');
    
    // printer_logicals
    let printer_str = prepare_kitchen_printers(item.printer_logicals.clone());
    result.push_str(&printer_str);
    result.push(',');
    
    // covers
    result.push_str(&item.covers.to_string());
    result.push(',');
    
    // store_id
    result.push_str(&item.store_id.to_string());
    result.push(',');
    
    // kds_text
    append_quoted(&mut result, &item.kitchen_video);
    result.push(',');
    
    // kds_dept
    result.push_str(&item.kds_dept.to_string());
    result.push(',');
    
    // kds_category
    result.push_str(item.kds_category.as_str());
    result.push(',');
    
    // kds_time
    result.push_str(&item.kds_cooktime.to_string());
    result.push(',');
    
    // store_price
    result.push_str("{}");
    result.push(',');
    
    // image_id
    result.push_str(&item.image_id.to_string());
    result.push(',');
    
    // stock_item_flag
    result.push_str(if item.stock_item { "1" } else { "0" });
    result.push(',');
    
    // lang_iso
    if !item.language_iso_code.as_str().is_empty() {
        result.push_str(item.language_iso_code.as_str());
    } else {
        result.push_str("\"\"");
    }
    result.push(',');
    
    // reserved3
    result.push('0');
    result.push(',');
    
    // reserved4
    result.push_str("\"\"");
    
    // Debug print (optional)
    println!("One Item's Export format:");
    println!("{}", &result);
    
    result
}




 /* // export data headers / example items
"A","B","C","D","E","F","G","G","H","I","J","K","L","M","N","O","O","P","Q","R","S","T","U","V","W","X","Y","Z","AA","AB","AC","AC","AD","AD","AE","AF","AG","AH","AI","AJ","AK","AK","AL","AM","AN","AO","AP"
"Add","Item ID","Item Name","Button 1 (Upper half)","Button 2 (Lower half)","Kitchen Printer Text","Default Price & Price Levels","[Join with above]",
"Product Class ID","Revenue Category ID","Tax Group ID","Security Level ID","Report Category ID","Use Weight Flag","Weight Tare Amount","SKU #",
"[Join with above]","Bar Gun Code","Cost Amount","Reserved","Ask Price","Print on Check","Discountable","Voidable","Not Active (86'd)","Tax Included",
"Item Group ID","Customer Receipt Text","Allow Price Override","Reserved","Choice Groups","[Join with above]","Kitchen Printers (Logical)","[Join with above]",
"Covers","Store ID","Kitchen Video Text","KDS Department","KDS Category","KDS Cook Time (secs.)","Store Price Level","[Join with above]","Image ID",
"Stock Item Flag","Language ISO Code*","Reserved","Reserved"
"A",7000000,"**Start** Food-Breakfast Alera","**Start**","","**Start**",{1,$0.00},101,1,1,0,0,0,0,{},,$0.00,0,0,1,1,1,0,0,152,"**Start**",1,0,{},{2,1,4,0},0,0,"**Start**",0,0,0,{},0,0,"",0,""
"A",7000001,"Chocolate Pancakes","Pancakes","","Pancakes",{1,$1.00},101,1,1,0,0,0,0,{},,$0.00,0,0,1,1,1,0,0,152,"Choc Pancakes",1,0,{},{2,1,4,0},0,0,"Choc Pancakes",0,0,0,{},0,0,"",0,""
"A",7000002,"Apple French Toast","French Toast","","French Toast",{1,$1.00},101,1,1,0,0,0,0,{},,$0.00,0,0,1,1,1,0,0,152,"French Toast",1,0,{},{2,1,4,0},0,0,"French Toast",0,0,0,{},0,0,"",0,""
 */

 fn prepare_item_prices(default_price: Option<Decimal>, prices: Option<Vec<ItemPrice>>) -> String {
    let mut price_string = String::new();
    println!("Preparing item prices.");
    
    match prices {
        Some(prices) => {
            price_string.push_str("{");
            match default_price {
                Some(price) => {
                    let price_str = "1".to_string() + ",$" + price.to_string().as_str() + ",";
                    price_string.push_str(price_str.as_str());
                }
                None => {
                    let price_str = "1".to_string() + ",$" + "0.00" + ",";
                    price_string.push_str(price_str.as_str());
                }
            }

            for price in prices {
                let price_str = (price.price_level_id + 1).to_string() + ",$" + price.price.to_string().as_str() + ",";
                price_string.push_str(price_str.as_str());
            }
            price_string = price_string.trim_end_matches(',').to_string();
            price_string.push_str("}");
        }
        None => {
            // return "" if there are no prices attached to an item.
            price_string = "\"\"".to_string();
        }
    };

    println!("Item prices prepared!");
    println!("{:?}", &price_string);

    price_string
}

 fn prepare_choice_groups(choice_groups: Option<Vec<(i32, i32)>>) -> String {
    let mut choice_group_string = String::new();
    println!("Preparing Choice Groups!");

    match choice_groups {
        Some(groups) => {
            choice_group_string.push_str("{");
            for group in groups {
                let group_str = group.0.to_string() + "," + group.1.to_string().as_str() + ",";
                choice_group_string.push_str(group_str.as_str());
            }
            choice_group_string = choice_group_string.trim_end_matches(',').to_string();
            choice_group_string.push_str("}");

        }
        None => { 
            // return {} if there are no choice groups attached to an item
            choice_group_string = "{}".to_string(); 
        }
    }

    println!("Choice Groups prepared!");
    println!("{:?}", &choice_group_string);

    choice_group_string
}

 fn prepare_kitchen_printers(printer_logicals: Option<Vec<(i32, bool)>>) -> String {
    let mut printer_logicals_string = String::new();
    println!("Preparing Printer Logicals!");

    match printer_logicals {
        Some(printers) => {
            printer_logicals_string.push('{');
            for printer in printers {
                printer_logicals_string.push_str(printer.0.to_string().as_str());
                printer_logicals_string.push(',');
                printer_logicals_string.push_str(if printer.1 {"1"} else {"0"});
                printer_logicals_string.push(',');
            }
            if printer_logicals_string.ends_with(',') {
                printer_logicals_string.pop(); // Remove comma from the last printer logical
            }
            printer_logicals_string.push('}');

        }
        None => { 
            // return {} if there are no printer logicals attached to an item
            printer_logicals_string.push_str("{}");
        }
    }

    println!("Printer Logicals prepared!");
    println!("{:?}", &printer_logicals_string);

    printer_logicals_string
}

 fn prepare_item_cost(x: Option<Decimal>) -> String {
        let mut cost_str = String::new();

        match x {
            Some(decimal) => {
                cost_str = "$".to_string() + decimal.to_string().as_str();
            }
            None => {
                cost_str = "$0.00".to_string();
            }
        }

        cost_str
}