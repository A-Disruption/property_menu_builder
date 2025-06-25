use items::Item;
use std::collections::HashMap;
use iced::{Element, Length};
use iced::widget::{Column, column, row, scrollable, container};
use crate::EntityResolver;

enum Message {
    SyncHeader(scrollable::AbsoluteOffset),
    Resizing(usize, f32),
    Resized,
    ColumnVisibilityEnabled(bool),
    ColumnVisibility(ColumnVisibilityMessage),
}

struct Preview {
    columns: Vec<Column>,
    rows: Vec<Row>,
    header: scrollable::Id,
    body: scrollable::Id,
    footer: scrollable::Id,
    column_visibility_enabled: bool,
    column_visibility: HashMap<String, bool>,
}

impl Preview {

    fn new() -> Self { 
        Preview::default() 
    }

    fn update_column_visibility(&mut self) {
        // Update each column's visibility based on our state
        for column in &mut self.columns {
            if let Some(&visible) = self.column_visibility.get(column.id()) {
                column.visible = visible;
            }
        }
    }

    pub fn preview_items<R: EntityResolver>(items: &[Item], resolver: &R) -> Element<Message> {

        self.rows = items.map(|item|row.generate(item, resolver)).collect();

        let table = responsive(|size| {
            let mut table = table(
                self.header.clone(),
                self.body.clone(),
                &self.columns,
                &self.rows,
                Message::SyncHeader,
            );

            if self.resize_columns_enabled {
                table = table.on_column_resize(Message::Resizing, Message::Resized);
            }
            if self.column_visibility_enabled {
                table = table.on_column_visibility(Message::ColumnVisibility);
            }

            table.into()
        });

        let visible_columns_count = self.columns.iter().filter(|c| c.visible).count();

        let content = column![
            text("Items Preview"),
            table,
        ].spacing(6); 

        content.padding(20).center_x(Length::Fill).center_y(Length::Fill).into()
    }
}

impl Default for Preview {
    fn default() -> Self {
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

        Self {
            columns: vec![
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
            ],
            rows: (0..50).map(Row::generate).collect(),
            header: scrollable::Id::unique(),
            body: scrollable::Id::unique(),
            footer: scrollable::Id::unique(),
            column_visibility_enabled: true,
            column_visibility,
        }
    }
}




struct Column {
    ColumnType: ColumnType,
    width: f32,
    resize_offset: Option<f32>,
    visible: bool,
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
            ColumnType::PriceLevels =>  150.0,
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
            ColumnType::ChoiceGroups =>  150.0,
            ColumnType::PrinterLogicals =>  150.0,
        };

        let visible = match kind { // Hidden by default
            ColumnType::NotActive => false, 
            ColumnType::StoreID => false,
            ColumnType::Covers => false,
            ColumnType::ImageID => false,
            ColumnType::LanguageISOCode => false,
            _ => true,
        };

        Self {
            columntype,
            width,
            resize_offset: None,
            visible,
        }
    }

    fn display_name(&self) -> &'static str {
        match self.columntype {
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

#[derive(Clone, Copy)]
enum ColumnType {
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

struct Row {
    id: String,
    name: String,
    button1: String,
    button2: String,
    printerText: String,
    itemGroup: String,
    productClass: String,
    revenueCategory: String,
    taxGroup: String,
    securityLevel: String,
    reportCategory: String,
    costAmount: String,
    askPrice: String,
    allowPriceOverride: String,
    priceLevels: String,
    useWeight: String,
    weightAmount: String,
    sKU: String,
    barGunCode: String,
    printOnCheck: String,
    discountable: String,
    voidable: String,
    notActive: String,
    taxIncluded: String,
    stockItem: String,
    customerReceiptText: String,
    kitchenVideoText: String,
    kDSCategory: String,
    kDSCooktime: String,
    kDSDepartment: String,
    storeID: String,
    covers: String,
    imageID: String,
    languageISOCode: String,
    choiceGroups: String,
    printerLogicals: String,
}

impl Row {
    fn generate<R: EntityResolver>(item: Item, resolver: &R) -> Self {
        let id = item.id.to_string();
        let name = item.name.clone();
        let button1 = item.button1.clone();
        let button2 = Some(item.button2).expect("".to_string());
        let printerText = item.printer_text.to_clone();
        let itemGroup = resolver.get_item_group_name(item.item_group);
        let productClass = resolver.get_product_class_name(item.product_class);
        let revenueCategory = resolver.get_revenue_category_name(item.revenue_category);
        let taxGroup = resolver.get_tax_group_name(item.tax_group);
        let securityLevel = resolver.get_security_level_name(item.security_level);
        let reportCategory = resolver.get_report_category_name(item.report_category);
        let costAmount = item.cost_amount.clone();
        let askPrice = item.ask_price.to_string();
        let allowPriceOverride = item.allow_price_override.to_string();
        let priceLevels = get_prices_string(item.default_price, item.item_prices);
        let useWeight = item.use_weight.to_string();
        let weightAmount = item.weight_amount.clone();
        let sKU = item.SKU.clone();
        let barGunCode = item.bar_gun_code.clone();
        let printOnCheck = item.print_on_check.to_string();
        let discountable = item.discountable.to_string();
        let voidable = item.voidable.to_string();
        let notActive = item.not_active.to_string();
        let taxIncluded = item.tax_included.to_string();
        let stockItem = item.stock_item.to_string();
        let customerReceiptText = item.customer_receipt.clone();
        let kitchenVideoText = item.kitchen_video.clone();
        let kDSCategory = item.kds_category.clone();
        let kDSCooktime = item.kds_cooktime.clone();
        let kDSDepartment = item.kds_dept.clone();
        let storeID = item.store_id.to_string();
        let covers = item.covers.to_string();
        let imageID = item.image_id.clone();
        let languageISOCode = item.language_iso_code.clone();
        let choiceGroups = get_choice_groups_string(item.choice_groups);
        let printerLogicals = get_printer_logicals_string(item.printer_logicals);        

        Self {
            id,
            name,
            button1,
            button2,
            printer_text,
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

    fn get_prices_string(default_price: Option<Decimal>, prices: Option<Vec<ItemPrice>>) -> String {
        let mut price_string = String::new();
        
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
                price_string = price_string.trim_end_matches(',').to_string();
                price_string.push_str("}");
            }
        };

        price_string
    }

    fn get_choice_groups_string(choice_groups: Option<Vec<(i32, i32)>>) -> String {
        let mut choice_group_string = String::new();

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

        choice_group_string
    }

    fn get_printer_logicals_string(printer_logicals: Option<Vec<(i32, bool)>>) -> String {
        let mut printer_logicals_string = String::new();

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

        printer_logicals_string
    }
}

impl<'a> table::Column<'a, Message, Theme, Renderer> for Column {
    type Row = Row;

    fn header(&'a self, _col_index: usize) -> Element<'a, Message> {
        let content = self.display_name();
        container(text(content)).center_y(24).into()
    }

    fn cell(&'a self, row: &'a Row) -> Element<'a, Message> {
        let content: Element<_> = match self.columntype {
            ColumnType::Id =>  text(row.id).into(),
            ColumnType::Name =>  text(row.name).into(),
            ColumnType::Button1 =>  text(row.button1).into(),
            ColumnType::Button2 =>  text(row.button2).into(),
            ColumnType::PrinterText =>  text(row.printer_text).into(),
            ColumnType::ItemGroup =>  text(row.itemGroup).into(),
            ColumnType::ProductClass =>  text(row.productClass).into(),
            ColumnType::RevenueCategory =>  text(row.revenueCategory).into(),
            ColumnType::TaxGroup =>  text(row.taxGroup).into(),
            ColumnType::SecurityLevel =>  text(row.securityLevel).into(),
            ColumnType::ReportCategory =>  text(row.reportCategory).into(),
            ColumnType::CostAmount =>  text(row.costAmount).into(),
            ColumnType::AskPrice =>  text(row.askPrice).into(),
            ColumnType::AllowPriceOverride =>  text(row.allowPriceOverride).into(),
            ColumnType::PriceLevels =>  text(row.priceLevels).into(),
            ColumnType::UseWeight =>  text(row.useWeight).into(),
            ColumnType::WeightAmount =>  text(row.weightAmount).into(),
            ColumnType::SKU => text(row.sKU).into(),
            ColumnType::BarGunCode =>  text(row.barGunCode).into(),
            ColumnType::PrintOnCheck =>  text(row.printOnCheck).into(),
            ColumnType::Discountable =>  text(row.discountable).into(),
            ColumnType::Voidable =>  text(row.voidable).into(),
            ColumnType::NotActive =>  text(row.notActive).into(),
            ColumnType::TaxIncluded =>  text(row.taxIncluded).into(),
            ColumnType::StockItem =>  text(row.stockItem).into(),
            ColumnType::CustomerReceiptText =>  text(row.customerReceiptText).into(),
            ColumnType::KitchenVideoText =>  text(row.kitchenVideoText).into(),
            ColumnType::KDSCategory =>  text(row.kDSCategory).into(),
            ColumnType::KDSCooktime =>  text(row.kDSCooktime).into(),
            ColumnType::KDSDepartment =>  text(row.kDSDepartment).into(),
            ColumnType::StoreID =>  text(row.storeID).into(),
            ColumnType::Covers =>  text(row.covers).into(),
            ColumnType::ImageID =>  text(row.imageID).into(),
            ColumnType::LanguageISOCode =>  text(row.languageISOCode).into(),
            ColumnType::ChoiceGroups => text(row.choiceGroups).into(),
            ColumnType::PrinterLogicals =>  text(row.printerLogicals).into(),
        };

        container(content).width(Length::Fill).center_y(32).into()
    }

    fn footer(&'a self, _col_index: usize, rows: &'a [Row]) -> Option<Element<'a, Message>> {
        let content = horizontal_space().into();

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
        Column::id(self).to_string()
    }

    fn title(&self) -> String {
        self.display_name().to_string()
    }

    fn is_visible(&self) -> bool {
        self.visible
    }
}