use crate::{
    items::{Item, ItemPrice},
    data_types::EntityId,
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
use std::collections::{HashMap, BTreeMap};
use rust_decimal::Decimal;
use iced_modern_theme::Modern;
use iced::{Element, Length, Theme, Renderer, Color};
use iced::widget::{column, row, scrollable, container, responsive, text, horizontal_space};
use iced_table::{table, ColumnVisibilityMessage};
use crate::superedit::HasName;

#[derive(Debug, Clone)]
pub enum Message {
    SyncHeader(scrollable::AbsoluteOffset),
    Resizing(usize, f32),
    Resized,
    ColumnVisibilityEnabled(bool),
    ColumnVisibility(ColumnVisibilityMessage),
}

// Enum to track cell changes
#[derive(Debug, Clone, PartialEq)]
pub enum CellChange {
    None,
    Modified,
    Added,
    Removed,
}

// Structure to hold both original and modified values
#[derive(Debug, Clone)]
pub struct CellValue {
    pub original: String,
    pub modified: Option<String>,
    pub change_type: CellChange,
}

impl CellValue {
    fn unchanged(value: String) -> Self {
        Self {
            original: value,
            modified: None,
            change_type: CellChange::None,
        }
    }

    fn modified(original: String, new_value: String) -> Self {
        Self {
            original,
            modified: Some(new_value),
            change_type: CellChange::Modified,
        }
    }

    fn display(&self) -> String {
        match &self.modified {
            Some(new_val) => new_val.clone(),
            None => self.original.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ItemsTableView {
    pub columns: Vec<Column>,
    pub rows: Vec<Row>,
    pub header_id: scrollable::Id,
    pub body_id: scrollable::Id,
    pub column_visibility_enabled: bool,
    pub column_visibility: HashMap<String, bool>,
    pub show_diff: bool, // Flag to enable diff visualization
}

impl ItemsTableView {
    pub fn new(
        items: &BTreeMap<EntityId, Item>,
        item_groups: &BTreeMap<EntityId, ItemGroup>,
        tax_groups: &BTreeMap<EntityId, TaxGroup>,
        security_levels: &BTreeMap<EntityId, SecurityLevel>,
        revenue_categories: &BTreeMap<EntityId, RevenueCategory>,
        report_categories: &BTreeMap<EntityId, ReportCategory>,
        product_classes: &BTreeMap<EntityId, ProductClass>,
        choice_groups: &BTreeMap<EntityId, ChoiceGroup>,
        printer_logicals: &BTreeMap<EntityId, PrinterLogical>,
        price_levels: &BTreeMap<EntityId, PriceLevel>,
    ) -> Self {

        let mut column_visibility = HashMap::new();
        column_visibility.insert("Id".to_string(), true);
        column_visibility.insert("Name".to_string(), true);
        column_visibility.insert("Button1".to_string(), true);
        column_visibility.insert("Button2".to_string(), true);
        column_visibility.insert("Printer Text".to_string(), true);
        column_visibility.insert("Item Group".to_string(), true);
        column_visibility.insert("Product Class".to_string(), true);
        column_visibility.insert("Revenue Category".to_string(), true);
        column_visibility.insert("Tax Group".to_string(), true);
        column_visibility.insert("Security Level".to_string(), true);
        column_visibility.insert("Report Category".to_string(), true);
        column_visibility.insert("Cost Amount".to_string(), true);
        column_visibility.insert("Ask Price".to_string(), true);
        column_visibility.insert("Allow Price Override".to_string(), true);
        column_visibility.insert("Price Levels".to_string(), true);
        column_visibility.insert("Use Weight".to_string(), true);
        column_visibility.insert("Weight Amount".to_string(), true);
        column_visibility.insert("SKU".to_string(), true);
        column_visibility.insert("Bar Gun Code".to_string(), true);
        column_visibility.insert("Print On Check".to_string(), true);
        column_visibility.insert("Discountable".to_string(), true);
        column_visibility.insert("Voidable".to_string(), true);
        column_visibility.insert("Not Active".to_string(), false);
        column_visibility.insert("Tax Included".to_string(), true);
        column_visibility.insert("Stock Item".to_string(), true);
        column_visibility.insert("Customer Receipt Text".to_string(), true);
        column_visibility.insert("Kitchen Video Text".to_string(), true);
        column_visibility.insert("KDS Category".to_string(), true);
        column_visibility.insert("KDS Cooktime".to_string(), true);
        column_visibility.insert("KDS Department".to_string(), true);
        column_visibility.insert("Store ID".to_string(), false);
        column_visibility.insert("Covers".to_string(), false);
        column_visibility.insert("Image ID".to_string(), false);
        column_visibility.insert("Language ISO Code".to_string(), false);
        column_visibility.insert("Choice Groups".to_string(), true);
        column_visibility.insert("Printer Logicals".to_string(), true);

        let columns = create_columns();
        
        let rows: Vec<Row> = items.iter().map(
            |item| Row::generate(
                item.1, 
                item_groups,
                tax_groups,
                security_levels,
                revenue_categories,
                report_categories,
                product_classes,
                choice_groups,
                printer_logicals,
                price_levels,
            )
        ).collect();

        Self {
            columns,
            rows,
            header_id: scrollable::Id::unique(),
            body_id: scrollable::Id::unique(),
            column_visibility_enabled: true,
            column_visibility,
            show_diff: false,
        }
    }

    // New method to create a table with diff view
    pub fn new_with_diff(
        original_items: &BTreeMap<EntityId, Item>,
        modified_items: &BTreeMap<EntityId, Item>,
        item_groups: &BTreeMap<EntityId, ItemGroup>,
        tax_groups: &BTreeMap<EntityId, TaxGroup>,
        security_levels: &BTreeMap<EntityId, SecurityLevel>,
        revenue_categories: &BTreeMap<EntityId, RevenueCategory>,
        report_categories: &BTreeMap<EntityId, ReportCategory>,
        product_classes: &BTreeMap<EntityId, ProductClass>,
        choice_groups: &BTreeMap<EntityId, ChoiceGroup>,
        printer_logicals: &BTreeMap<EntityId, PrinterLogical>,
        price_levels: &BTreeMap<EntityId, PriceLevel>,
    ) -> Self {
        let mut table = Self::new(
            original_items,
            item_groups,
            tax_groups,
            security_levels,
            revenue_categories,
            report_categories,
            product_classes,
            choice_groups,
            printer_logicals,
            price_levels,
        );
        
        table.show_diff = true;
        
        // Generate rows with diff information
        table.rows = original_items.iter().map(|(id, original_item)| {
            if let Some(modified_item) = modified_items.get(id) {
                Row::generate_with_diff(
                    original_item,
                    modified_item,
                    item_groups,
                    tax_groups,
                    security_levels,
                    revenue_categories,
                    report_categories,
                    product_classes,
                    choice_groups,
                    printer_logicals,
                    price_levels,
                )
            } else {
                Row::generate(
                    original_item,
                    item_groups,
                    tax_groups,
                    security_levels,
                    revenue_categories,
                    report_categories,
                    product_classes,
                    choice_groups,
                    printer_logicals,
                    price_levels,
                )
            }
        }).collect();
        
        table
    }

    pub fn render(&self) -> Element<Message> {
        let table = responsive(|_size| {
            let mut table = table(
                self.header_id.clone(),
                self.body_id.clone(),
                &self.columns,
                &self.rows,
                Message::SyncHeader,
            );

            
            table = table.on_column_resize(Message::Resizing, Message::Resized);

            if self.column_visibility_enabled {
                table = table.on_column_visibility(Message::ColumnVisibility);
            }

            table.into()
        });


        let content = column![
            table,
        ];

        container(content).padding(20).center_x(Length::Fill).center_y(Length::Fill).into()
    }
}

fn create_columns() -> Vec<Column> {
    vec![
        Column::new(ColumnType::Id),
        Column::new(ColumnType::Name),
        Column::new(ColumnType::Button1),
        Column::new(ColumnType::Button2),
        Column::new(ColumnType::PrinterText),
        Column::new(ColumnType::ItemGroup),
        Column::new(ColumnType::ProductClass),
        Column::new(ColumnType::RevenueCategory),
        Column::new(ColumnType::TaxGroup),
        Column::new(ColumnType::SecurityLevel),
        Column::new(ColumnType::ReportCategory),
        Column::new(ColumnType::CostAmount),
        Column::new(ColumnType::AskPrice),
        Column::new(ColumnType::AllowPriceOverride),
        Column::new(ColumnType::PriceLevels),
        Column::new(ColumnType::UseWeight),
        Column::new(ColumnType::WeightAmount),
        Column::new(ColumnType::SKU),
        Column::new(ColumnType::BarGunCode),
        Column::new(ColumnType::PrintOnCheck),
        Column::new(ColumnType::Discountable),
        Column::new(ColumnType::Voidable),
        Column::new(ColumnType::NotActive),
        Column::new(ColumnType::TaxIncluded),
        Column::new(ColumnType::StockItem),
        Column::new(ColumnType::CustomerReceiptText),
        Column::new(ColumnType::KitchenVideoText),
        Column::new(ColumnType::KDSCategory),
        Column::new(ColumnType::KDSCooktime),
        Column::new(ColumnType::KDSDepartment),
        Column::new(ColumnType::StoreID),
        Column::new(ColumnType::Covers),
        Column::new(ColumnType::ImageID),
        Column::new(ColumnType::LanguageISOCode),
        Column::new(ColumnType::ChoiceGroups),
        Column::new(ColumnType::PrinterLogicals),
    ]
}

#[derive(Debug, Clone)]
pub struct Column {
    pub column_type: ColumnType,
    pub width: f32,
    pub resize_offset: Option<f32>,
    pub visible: bool,
}

impl Column {
    fn new(columntype: ColumnType) -> Self {
        let width = match columntype {
            ColumnType::Id =>  150.0,
            ColumnType::Name =>  200.0,
            ColumnType::Button1 =>  150.0,
            ColumnType::Button2 =>  150.0,
            ColumnType::PrinterText =>  150.0,
            ColumnType::ItemGroup =>  150.0,
            ColumnType::ProductClass =>  150.0,
            ColumnType::RevenueCategory =>  150.0,
            ColumnType::TaxGroup =>  150.0,
            ColumnType::SecurityLevel =>  150.0,
            ColumnType::ReportCategory =>  150.0,
            ColumnType::CostAmount =>  150.0,
            ColumnType::AskPrice =>  150.0,
            ColumnType::AllowPriceOverride =>  150.0,
            ColumnType::PriceLevels =>  500.0, // Wider for price list
            ColumnType::UseWeight =>  150.0,
            ColumnType::WeightAmount =>  150.0,
            ColumnType::SKU =>  150.0,
            ColumnType::BarGunCode =>  150.0,
            ColumnType::PrintOnCheck =>  150.0,
            ColumnType::Discountable =>  150.0,
            ColumnType::Voidable =>  150.0,
            ColumnType::NotActive =>  150.0,
            ColumnType::TaxIncluded =>  150.0,
            ColumnType::StockItem =>  150.0,
            ColumnType::CustomerReceiptText =>  150.0,
            ColumnType::KitchenVideoText =>  150.0,
            ColumnType::KDSCategory =>  150.0,
            ColumnType::KDSCooktime =>  150.0,
            ColumnType::KDSDepartment =>  150.0,
            ColumnType::StoreID =>  150.0,
            ColumnType::Covers =>  150.0,
            ColumnType::ImageID =>  150.0,
            ColumnType::LanguageISOCode =>  150.0,
            ColumnType::ChoiceGroups =>  500.0, // Wider for list
            ColumnType::PrinterLogicals =>  300.0, // Wider for list
        };

        let visible = match columntype { // Hidden by default
            ColumnType::SKU =>  false,
            ColumnType::BarGunCode =>  false,
            ColumnType::StockItem =>  false,
            ColumnType::KitchenVideoText =>  false,
            ColumnType::KDSCategory =>  false,
            ColumnType::KDSCooktime =>  false,
            ColumnType::KDSDepartment =>  false,
            ColumnType::NotActive => false, 
            ColumnType::StoreID => false,
            ColumnType::Covers => false,
            ColumnType::ImageID => false,
            ColumnType::LanguageISOCode => false,
            _ => true,
        };

        Self {
            column_type: columntype,
            width,
            resize_offset: None,
            visible,
        }
    }

    fn display_name(&self) -> &'static str {
        match self.column_type {
            ColumnType::Id =>  "Id",
            ColumnType::Name =>  "Name",
            ColumnType::Button1 =>  "Button1",
            ColumnType::Button2 =>  "Button2",
            ColumnType::PrinterText =>  "Printer Text",
            ColumnType::ItemGroup =>  "Item Group",
            ColumnType::ProductClass =>  "Product Class",
            ColumnType::RevenueCategory =>  "Revenue Category",
            ColumnType::TaxGroup =>  "Tax Group",
            ColumnType::SecurityLevel =>  "Security Level",
            ColumnType::ReportCategory =>  "Report Category",
            ColumnType::CostAmount =>  "Cost Amount",
            ColumnType::AskPrice =>  "Ask Price",
            ColumnType::AllowPriceOverride =>  "Allow Price Override",
            ColumnType::PriceLevels =>  "Price Levels",
            ColumnType::UseWeight =>  "Use Weight",
            ColumnType::WeightAmount =>  "Weight Amount",
            ColumnType::SKU => "SKU",
            ColumnType::BarGunCode =>  "Bar Gun Code",
            ColumnType::PrintOnCheck =>  "Print On Check",
            ColumnType::Discountable =>  "Discountable",
            ColumnType::Voidable =>  "Voidable",
            ColumnType::NotActive =>  "Not Active",
            ColumnType::TaxIncluded =>  "Tax Included",
            ColumnType::StockItem =>  "Stock Item",
            ColumnType::CustomerReceiptText =>  "Customer Receipt Text",
            ColumnType::KitchenVideoText =>  "Kitchen Video Text",
            ColumnType::KDSCategory =>  "KDS Category",
            ColumnType::KDSCooktime =>  "KDS Cooktime",
            ColumnType::KDSDepartment =>  "KDS Department",
            ColumnType::StoreID =>  "Store ID",
            ColumnType::Covers =>  "Covers",
            ColumnType::ImageID =>  "Image ID",
            ColumnType::LanguageISOCode =>  "Language ISO Code",
            ColumnType::ChoiceGroups =>  "Choice Groups",
            ColumnType::PrinterLogicals =>  "Printer Logicals",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ColumnType {
    Id,
    Name,
    Button1,
    Button2,
    PrinterText,
    ItemGroup,
    ProductClass,
    RevenueCategory,
    TaxGroup,
    SecurityLevel,
    ReportCategory,
    CostAmount,
    AskPrice,
    AllowPriceOverride,
    PriceLevels,
    UseWeight,
    WeightAmount,
    SKU,
    BarGunCode,
    PrintOnCheck,
    Discountable,
    Voidable,
    NotActive,
    TaxIncluded,
    StockItem,
    CustomerReceiptText,
    KitchenVideoText,
    KDSCategory,
    KDSCooktime,
    KDSDepartment,
    StoreID,
    Covers,
    ImageID,
    LanguageISOCode,
    ChoiceGroups,
    PrinterLogicals,
}

#[derive(Debug, Clone)]
pub struct Row {
    id: CellValue,
    name: CellValue,
    button1: CellValue,
    button2: CellValue,
    printerText: CellValue,
    itemGroup: CellValue,
    productClass: CellValue,
    revenueCategory: CellValue,
    taxGroup: CellValue,
    securityLevel: CellValue,
    reportCategory: CellValue,
    costAmount: CellValue,
    askPrice: CellValue,
    allowPriceOverride: CellValue,
    priceLevels: CellValue,
    useWeight: CellValue,
    weightAmount: CellValue,
    sKU: CellValue,
    barGunCode: CellValue,
    printOnCheck: CellValue,
    discountable: CellValue,
    voidable: CellValue,
    notActive: CellValue,
    taxIncluded: CellValue,
    stockItem: CellValue,
    customerReceiptText: CellValue,
    kitchenVideoText: CellValue,
    kDSCategory: CellValue,
    kDSCooktime: CellValue,
    kDSDepartment: CellValue,
    storeID: CellValue,
    covers: CellValue,
    imageID: CellValue,
    languageISOCode: CellValue,
    choiceGroups: CellValue,
    printerLogicals: CellValue,
}

impl Row {
    fn generate<'a>(
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
    ) -> Self {
        let id = CellValue::unchanged(item.id.to_string());
        let name = CellValue::unchanged(item.name.clone());
        let button1 = CellValue::unchanged(item.button1.clone());
        let button2 = CellValue::unchanged(item.button2.clone().unwrap_or("".to_string()));
        let printerText = CellValue::unchanged(item.printer_text.clone());
        let itemGroup = CellValue::unchanged(item.item_group
            .and_then(|id| item_groups.get(&id))
            .map(|ig| ig.name.as_str())
            .unwrap_or("None").to_string());
        let productClass = CellValue::unchanged(item.product_class
            .and_then(|id| product_classes.get(&id))
            .map(|product_class| product_class.name.as_str())
            .unwrap_or("None").to_string());
        let revenueCategory = CellValue::unchanged(item.revenue_category
            .and_then(|id| revenue_categories.get(&id))
            .map(|revenue_category| revenue_category.name.as_str())
            .unwrap_or("None").to_string());
        let taxGroup = CellValue::unchanged(item.tax_group
            .and_then(|id| tax_groups.get(&id))
            .map(|tax_group| tax_group.name.as_str())
            .unwrap_or("None").to_string());
        let securityLevel = CellValue::unchanged(item.security_level
            .and_then(|id| security_levels.get(&id))
            .map(|security_level| security_level.name.as_str())
            .unwrap_or("None").to_string());
        let reportCategory = CellValue::unchanged(item.report_category
            .and_then(|id| report_categories.get(&id))
            .map(|report_category| report_category.name.as_str())
            .unwrap_or("None").to_string());
        let costAmount = CellValue::unchanged(item.cost_amount.unwrap_or(Decimal::new(0,2)).to_string());
        let askPrice = CellValue::unchanged(item.ask_price.to_string());
        let allowPriceOverride = CellValue::unchanged(item.allow_price_override.to_string());
        let priceLevels = CellValue::unchanged(get_prices_string_with_names(
            item.default_price,
            item.item_prices.as_ref(), 
            price_levels
        ));
        let useWeight = CellValue::unchanged(item.use_weight.to_string());
        let weightAmount = CellValue::unchanged(item.weight_amount.to_string());
        let sKU = CellValue::unchanged(item.sku.clone().unwrap_or("".to_string()));
        let barGunCode = CellValue::unchanged(item.bar_gun_code.clone().unwrap_or("".to_string()));
        let printOnCheck = CellValue::unchanged(item.print_on_check.to_string());
        let discountable = CellValue::unchanged(item.discountable.to_string());
        let voidable = CellValue::unchanged(item.voidable.to_string());
        let notActive = CellValue::unchanged(item.not_active.to_string());
        let taxIncluded = CellValue::unchanged(item.tax_included.to_string());
        let stockItem = CellValue::unchanged(item.stock_item.to_string());
        let customerReceiptText = CellValue::unchanged(item.customer_receipt.clone());
        let kitchenVideoText = CellValue::unchanged(item.kitchen_video.clone());
        let kDSCategory = CellValue::unchanged(item.kds_category.clone());
        let kDSCooktime = CellValue::unchanged(item.kds_cooktime.to_string());
        let kDSDepartment = CellValue::unchanged(item.kds_dept.to_string());
        let storeID = CellValue::unchanged(item.store_id.to_string());
        let covers = CellValue::unchanged(item.covers.to_string());
        let imageID = CellValue::unchanged(item.image_id.to_string());
        let languageISOCode = CellValue::unchanged(item.language_iso_code.clone());
        let choiceGroups = CellValue::unchanged(get_choice_groups_string_with_names(
            item.choice_groups.as_ref(), 
            choice_groups
        ));
        let printerLogicals = CellValue::unchanged(get_printer_logicals_string_with_names(
            item.printer_logicals.as_ref(), 
            printer_logicals
        ));        

        Self {
            id,
            name,
            button1,
            button2,
            printerText,
            itemGroup,
            productClass,
            revenueCategory,
            taxGroup,
            securityLevel,
            reportCategory,
            costAmount,
            askPrice,
            allowPriceOverride,
            priceLevels,
            useWeight,
            weightAmount,
            sKU,
            barGunCode,
            printOnCheck,
            discountable,
            voidable,
            notActive,
            taxIncluded,
            stockItem,
            customerReceiptText,
            kitchenVideoText,
            kDSCategory,
            kDSCooktime,
            kDSDepartment,
            storeID,
            covers,
            imageID,
            languageISOCode,
            choiceGroups,
            printerLogicals,
        }
    }

    fn generate_with_diff<'a>(
        original: &'a Item,
        modified: &'a Item,
        item_groups: &'a BTreeMap<EntityId, ItemGroup>,
        tax_groups: &'a BTreeMap<EntityId, TaxGroup>,
        security_levels: &'a BTreeMap<EntityId, SecurityLevel>,
        revenue_categories: &'a BTreeMap<EntityId, RevenueCategory>,
        report_categories: &'a BTreeMap<EntityId, ReportCategory>,
        product_classes: &'a BTreeMap<EntityId, ProductClass>,
        choice_groups: &'a BTreeMap<EntityId, ChoiceGroup>,
        printer_logicals: &'a BTreeMap<EntityId, PrinterLogical>,
        price_levels: &'a BTreeMap<EntityId, PriceLevel>,
    ) -> Self {
        // Helper to create CellValue with diff
        let diff_value = |orig: String, modif: String| -> CellValue {
            if orig == modif {
                CellValue::unchanged(orig)
            } else {
                CellValue::modified(orig, modif)
            }
        };

        // Helper for optional entity lookups
        let lookup_entity_name = |id: Option<EntityId>, map: &BTreeMap<EntityId, &dyn HasName>| -> String {
            id.and_then(|id| map.get(&id))
                .map(|entity| entity.name().to_string())
                .unwrap_or_else(|| "None".to_string())
        };

        Self {
            id: CellValue::unchanged(original.id.to_string()),
            name: diff_value(original.name.clone(), modified.name.clone()),
            button1: diff_value(original.button1.clone(), modified.button1.clone()),
            button2: diff_value(
                original.button2.clone().unwrap_or_default(),
                modified.button2.clone().unwrap_or_default()
            ),
            printerText: diff_value(original.printer_text.clone(), modified.printer_text.clone()),
            itemGroup: diff_value(
                original.item_group
                    .and_then(|id| item_groups.get(&id))
                    .map(|ig| ig.name.clone())
                    .unwrap_or_else(|| "None".to_string()),
                modified.item_group
                    .and_then(|id| item_groups.get(&id))
                    .map(|ig| ig.name.clone())
                    .unwrap_or_else(|| "None".to_string())
            ),
            productClass: diff_value(
                original.product_class
                    .and_then(|id| product_classes.get(&id))
                    .map(|pc| pc.name.clone())
                    .unwrap_or_else(|| "None".to_string()),
                modified.product_class
                    .and_then(|id| product_classes.get(&id))
                    .map(|pc| pc.name.clone())
                    .unwrap_or_else(|| "None".to_string())
            ),
            revenueCategory: diff_value(
                original.revenue_category
                    .and_then(|id| revenue_categories.get(&id))
                    .map(|rc| rc.name.clone())
                    .unwrap_or_else(|| "None".to_string()),
                modified.revenue_category
                    .and_then(|id| revenue_categories.get(&id))
                    .map(|rc| rc.name.clone())
                    .unwrap_or_else(|| "None".to_string())
            ),
            taxGroup: diff_value(
                original.tax_group
                    .and_then(|id| tax_groups.get(&id))
                    .map(|tg| tg.name.clone())
                    .unwrap_or_else(|| "None".to_string()),
                modified.tax_group
                    .and_then(|id| tax_groups.get(&id))
                    .map(|tg| tg.name.clone())
                    .unwrap_or_else(|| "None".to_string())
            ),
            securityLevel: diff_value(
                original.security_level
                    .and_then(|id| security_levels.get(&id))
                    .map(|sl| sl.name.clone())
                    .unwrap_or_else(|| "None".to_string()),
                modified.security_level
                    .and_then(|id| security_levels.get(&id))
                    .map(|sl| sl.name.clone())
                    .unwrap_or_else(|| "None".to_string())
            ),
            reportCategory: diff_value(
                original.report_category
                    .and_then(|id| report_categories.get(&id))
                    .map(|rc| rc.name.clone())
                    .unwrap_or_else(|| "None".to_string()),
                modified.report_category
                    .and_then(|id| report_categories.get(&id))
                    .map(|rc| rc.name.clone())
                    .unwrap_or_else(|| "None".to_string())
            ),
            costAmount: diff_value(
                original.cost_amount.unwrap_or_else(|| Decimal::new(0, 2)).to_string(),
                modified.cost_amount.unwrap_or_else(|| Decimal::new(0, 2)).to_string()
            ),
            askPrice: diff_value(
                original.ask_price.to_string(),
                modified.ask_price.to_string()
            ),
            allowPriceOverride: diff_value(
                original.allow_price_override.to_string(),
                modified.allow_price_override.to_string()
            ),
            priceLevels: diff_value(
                get_prices_string_with_names(original.default_price, original.item_prices.as_ref(), price_levels),
                get_prices_string_with_names(modified.default_price, modified.item_prices.as_ref(), price_levels)
            ),
            useWeight: diff_value(
                original.use_weight.to_string(),
                modified.use_weight.to_string()
            ),
            weightAmount: diff_value(
                original.weight_amount.to_string(),
                modified.weight_amount.to_string()
            ),
            sKU: diff_value(
                original.sku.clone().unwrap_or_default(),
                modified.sku.clone().unwrap_or_default()
            ),
            barGunCode: diff_value(
                original.bar_gun_code.clone().unwrap_or_default(),
                modified.bar_gun_code.clone().unwrap_or_default()
            ),
            printOnCheck: diff_value(
                original.print_on_check.to_string(),
                modified.print_on_check.to_string()
            ),
            discountable: diff_value(
                original.discountable.to_string(),
                modified.discountable.to_string()
            ),
            voidable: diff_value(
                original.voidable.to_string(),
                modified.voidable.to_string()
            ),
            notActive: diff_value(
                original.not_active.to_string(),
                modified.not_active.to_string()
            ),
            taxIncluded: diff_value(
                original.tax_included.to_string(),
                modified.tax_included.to_string()
            ),
            stockItem: diff_value(
                original.stock_item.to_string(),
                modified.stock_item.to_string()
            ),
            customerReceiptText: diff_value(
                original.customer_receipt.clone(),
                modified.customer_receipt.clone()
            ),
            kitchenVideoText: diff_value(
                original.kitchen_video.clone(),
                modified.kitchen_video.clone()
            ),
            kDSCategory: diff_value(
                original.kds_category.clone(),
                modified.kds_category.clone()
            ),
            kDSCooktime: diff_value(
                original.kds_cooktime.to_string(),
                modified.kds_cooktime.to_string()
            ),
            kDSDepartment: diff_value(
                original.kds_dept.to_string(),
                modified.kds_dept.to_string()
            ),
            storeID: diff_value(
                original.store_id.to_string(),
                modified.store_id.to_string()
            ),
            covers: diff_value(
                original.covers.to_string(),
                modified.covers.to_string()
            ),
            imageID: diff_value(
                original.image_id.to_string(),
                modified.image_id.to_string()
            ),
            languageISOCode: diff_value(
                original.language_iso_code.clone(),
                modified.language_iso_code.clone()
            ),
            choiceGroups: diff_value(
                get_choice_groups_string_with_names(original.choice_groups.as_ref(), choice_groups),
                get_choice_groups_string_with_names(modified.choice_groups.as_ref(), choice_groups)
            ),
            printerLogicals: diff_value(
                get_printer_logicals_string_with_names(original.printer_logicals.as_ref(), printer_logicals),
                get_printer_logicals_string_with_names(modified.printer_logicals.as_ref(), printer_logicals)
            ),
        }
    }
}

fn get_prices_string_with_names(
    default_price: Option<Decimal>,
    item_prices: Option<&Vec<ItemPrice>>, 
    price_levels: &BTreeMap<EntityId, PriceLevel>
) -> String {
    let mut price_strings = Vec::new();
    
    // Add default price (usually price level 1)
    match default_price {
        Some(price) => {
            // Find price level 1's name, or use "Default" if not found
            let level_name = price_levels.values()
                .find(|pl| pl.id == 1) // Assuming price level 1 is the default
                .map(|pl| pl.name.as_str())
                .unwrap_or("Default");
            price_strings.push(format!("{}: ${:.2}", level_name, price));
        }
        None => {
            let level_name = price_levels.values()
                .find(|pl| pl.id == 1)
                .map(|pl| pl.name.as_str())
                .unwrap_or("Default");
            price_strings.push(format!("{}: $0.00", level_name));
        }
    }
    
    // Add additional price levels from item_prices
    if let Some(prices) = item_prices {
        for item_price in prices {
            // Find the price level name by ID
            if let Some(price_level) = price_levels.get(&item_price.price_level_id) {
                price_strings.push(format!("{}: ${:.2}", price_level.name, item_price.price));
            } else {
                // Fallback if price level not found
                price_strings.push(format!("Level {}: ${:.2}", item_price.price_level_id, item_price.price));
            }
        }
    }
    
    format!("[{}]", price_strings.join(", "))
}

fn get_choice_groups_string_with_names(
    choice_group_data: Option<&Vec<(EntityId, i32)>>, 
    choice_groups: &BTreeMap<EntityId, ChoiceGroup>
) -> String {
    match choice_group_data {
        Some(data) if !data.is_empty() => {
            let group_names: Vec<String> = data
                .iter()
                .filter_map(|(id, _)| choice_groups.get(id).map(|cg| cg.name.clone()))
                .collect();
            format!("[{}]", group_names.join(", "))
        }
        _ => "[]".to_string()
    }
}

fn get_printer_logicals_string_with_names(
    printer_logical_data: Option<&Vec<(EntityId, bool)>>, 
    printer_logicals: &BTreeMap<EntityId, PrinterLogical>
) -> String {
    match printer_logical_data {
        Some(data) if !data.is_empty() => {
            let printer_names: Vec<String> = data
                .iter()
                .filter_map(|(id, _)| printer_logicals.get(id).map(|pl| pl.name.clone()))
                .collect();
            format!("[{}]", printer_names.join(", "))
        }
        _ => "[]".to_string()
    }
}

// Helper function to parse list strings like "[Kitchen, Bar, Expo]" into Vec<String>
fn parse_list_string(s: &str) -> Vec<String> {
    if s.starts_with('[') && s.ends_with(']') {
        let inner = &s[1..s.len()-1];
        if inner.is_empty() {
            Vec::new()
        } else {
            inner.split(", ").map(|s| s.to_string()).collect()
        }
    } else {
        vec![s.to_string()]
    }
}

// Helper function to create a diff view for list-like fields
fn create_list_diff_view<'a>(original: &str, modified: &str) -> Element<'a, Message> {
    let orig_items = parse_list_string(original);
    let mod_items = parse_list_string(modified);

    let mut elements: Vec<Element<'a, Message>> = vec![
        text("[").size(14).style(Modern::primary_text()).into()
    ];
    
    let mut first = true;
    
    // Track which modified items have been used
    let mut used_mod_indices = std::collections::HashSet::new();
    
    // First, go through original items
    for orig_item in orig_items.iter() {
        // Check if this item exists in modified
        let found_in_modified = mod_items.iter()
            .enumerate()
            .find(|(idx, m)| m == &orig_item && !used_mod_indices.contains(idx));
        
        if !first {
            elements.push(text(", ").size(14).into());
        }
        first = false;
        
        if let Some((idx, _)) = found_in_modified {
            // Item still exists - show in normal color
            elements.push(text(orig_item.clone()).size(14).into());
            used_mod_indices.insert(idx);
        } else {
            // Item was removed - show in red with strikethrough
            elements.push(
                text(orig_item.clone())
                    .size(14)
                    .style(Modern::red_text())
                    .into()
            );
        }
    }
    
    // Then add any new items from modified that weren't in original
    for (idx, mod_item) in mod_items.iter().enumerate() {
        if !used_mod_indices.contains(&idx) {
            if !first {
                elements.push(text(", ").size(14).into());
            }
            first = false;
            
            // New item - show in green
            elements.push(
                text(mod_item.clone())
                    .size(14)
                    .style(Modern::green_text())
                    .into()
            );
        }
    }

    elements.push(text("]").size(14).style(Modern::primary_text()).into());

    let content = iced::widget::Row::from_vec(elements);
    
    container(content).into()
}

impl<'a> table::Column<'a, Message, Theme, Renderer> for Column {
    type Row = Row;

    fn header(&'a self, _col_index: usize) -> Element<'a, Message> {
        let content = self.display_name();
        container(text(content)).center_y(24).into()
    }

    fn cell(&'a self, _col_index: usize, _row_index: usize, row: &'a Row) -> Element<'a, Message> {
        let cell_value = match self.column_type {
            ColumnType::Id => &row.id,
            ColumnType::Name => &row.name,
            ColumnType::Button1 => &row.button1,
            ColumnType::Button2 => &row.button2,
            ColumnType::PrinterText => &row.printerText,
            ColumnType::ItemGroup => &row.itemGroup,
            ColumnType::ProductClass => &row.productClass,
            ColumnType::RevenueCategory => &row.revenueCategory,
            ColumnType::TaxGroup => &row.taxGroup,
            ColumnType::SecurityLevel => &row.securityLevel,
            ColumnType::ReportCategory => &row.reportCategory,
            ColumnType::CostAmount => &row.costAmount,
            ColumnType::AskPrice => &row.askPrice,
            ColumnType::AllowPriceOverride => &row.allowPriceOverride,
            ColumnType::PriceLevels => &row.priceLevels,
            ColumnType::UseWeight => &row.useWeight,
            ColumnType::WeightAmount => &row.weightAmount,
            ColumnType::SKU => &row.sKU,
            ColumnType::BarGunCode => &row.barGunCode,
            ColumnType::PrintOnCheck => &row.printOnCheck,
            ColumnType::Discountable => &row.discountable,
            ColumnType::Voidable => &row.voidable,
            ColumnType::NotActive => &row.notActive,
            ColumnType::TaxIncluded => &row.taxIncluded,
            ColumnType::StockItem => &row.stockItem,
            ColumnType::CustomerReceiptText => &row.customerReceiptText,
            ColumnType::KitchenVideoText => &row.kitchenVideoText,
            ColumnType::KDSCategory => &row.kDSCategory,
            ColumnType::KDSCooktime => &row.kDSCooktime,
            ColumnType::KDSDepartment => &row.kDSDepartment,
            ColumnType::StoreID => &row.storeID,
            ColumnType::Covers => &row.covers,
            ColumnType::ImageID => &row.imageID,
            ColumnType::LanguageISOCode => &row.languageISOCode,
            ColumnType::ChoiceGroups => &row.choiceGroups,
            ColumnType::PrinterLogicals => &row.printerLogicals,
        };

        let content: Element<_> = match &cell_value.change_type {
            CellChange::Modified => {
                let old_value = &cell_value.original;
                let new_value = cell_value.modified.as_ref().unwrap_or(&cell_value.original);
                
                // Check if this is a list-like field (contains brackets)
                let is_list_field = matches!(self.column_type, 
                    ColumnType::PriceLevels | 
                    ColumnType::ChoiceGroups | 
                    ColumnType::PrinterLogicals
                ) || (old_value.starts_with('[') && old_value.ends_with(']'));
                
                if is_list_field {
                    // Use special list diff view
                    create_list_diff_view(old_value, new_value)
                } else {
                    // For non-list fields, show inline diff
                    row![
                        text(old_value)
                            .size(14)
                            .style(|_| text::Style {
                                color: Some(Color::from_rgb(0.5, 0.5, 0.5)),
                            }),
                        text(" → ").size(14).style(|_| text::Style {
                            color: Some(Color::from_rgb(0.4, 0.4, 0.4)),
                        }),
                        text(new_value)
                            .size(14)
                            .style(|_| text::Style {
                                color: Some(Color::from_rgb(0.0, 0.7, 0.0)),
                            })
                    ]
                    .align_y(iced::Alignment::Center)
                    .spacing(4)
                    .into()
                }
            },
            CellChange::Added => {
                row![
                    text("+")
                        .size(14)
                        .style(|_| text::Style {
                            color: Some(Color::from_rgb(0.0, 0.7, 0.0)),
                        }),
                    text(cell_value.display())
                        .size(14)
                        .style(|_| text::Style {
                            color: Some(Color::from_rgb(0.0, 0.7, 0.0)),
                        })
                ]
                .spacing(4)
                .align_y(iced::Alignment::Center)
                .into()
            },
            CellChange::Removed => {
                row![
                    text("-")
                        .size(14)
                        .style(|_| text::Style {
                            color: Some(Color::from_rgb(0.7, 0.0, 0.0)),
                        }),
                    text(cell_value.display())
                        .size(14)
                        .style(|_| text::Style {
                            color: Some(Color::from_rgb(0.7, 0.0, 0.0)),
                        })
                ]
                .spacing(4)
                .align_y(iced::Alignment::Center)
                .into()
            },
            CellChange::None => {
                text(cell_value.display()).size(14).into()
            }
        };

        container(content)
            .width(Length::Fill)
            .center_y(32)
            .padding([2, 4])
            .into()
    }

    fn footer(&'a self, _col_index: usize, rows: &'a [Row]) -> Option<Element<'a, Message>> {
        let content = horizontal_space();

        Some(container(content).center_y(24).into())
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn resize_offset(&self) -> Option<f32> {
        self.resize_offset
    }

    // Implement the new trait methods for column visibility
    fn id(&self) -> String {
        self.display_name().to_string()
    }

    fn title(&self) -> String {
        self.display_name().to_string()
    }

    fn is_visible(&self) -> bool {
        self.visible
    }
}