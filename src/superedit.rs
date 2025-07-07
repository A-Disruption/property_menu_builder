use iced::{Element, Task, Length};
use iced::widget::{button, column, container, horizontal_space, pick_list, row, text, text_input, scrollable};
use iced_modern_theme::Modern;
use std::collections::BTreeMap;
use rust_decimal::Decimal;
use crate::Action;
use crate::{
    items::{Item, ViewContext},
    data_types::{EntityId, ItemPrice},
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
use crate::items::preview_changes::{ItemsTableView, Message as PreviewMessage};
use iced_table::{ColumnVisibilityMessage, table::Column};

#[derive(Debug, Clone)]
pub enum Message {
    // Rule management
    AddCondition,
    RemoveCondition(usize),
    UpdateConditionLogic(usize, ConditionLogic),
    UpdateConditionField(usize, FilterCategory),
    UpdateConditionOperator(usize, FilterOperator),
    UpdateConditionValue(usize, String),
    UpdateConditionEntity(usize, EntityId), // For entity dropdowns
    
    // Action management
    AddAction,
    RemoveAction(usize),
    UpdateActionCategory(usize, FilterCategory),
    UpdateActionOperation(usize, ActionOperation),
    UpdateActionValue(usize, String),
    UpdateActionPriceLevel(usize, EntityId),
    UpdateActionSwapFrom(usize, EntityId),

    // Entity Selection
    UpdateActionEntity(usize, EntityId),
    AcceptChanges,
    CancelPreview,

    //Save Changes to App State
    CommitChanges(Item),

    //Table Preview
    Preview(PreviewMessage),
    PreviewChanges, 
}

#[derive(Debug, Clone)]
struct FilterAction {
    category: FilterCategory,
    operation: ActionOperation,
    value: String,
    entity_id: Option<EntityId>, // For entity selections
    swap_from_id: Option<EntityId>, // For swap from selections
    price_level: Option<EntityId>, // For price-related operations
}

#[derive(Debug, Clone)]
struct FilterCondition {
    logic: ConditionLogic,      // And/Or
    field: FilterCategory,      // Name, ItemGroup, etc.
    operator: FilterOperator,   // begins with, contains, etc.
    value: String,             // user input value for text fields
    entity_id: Option<EntityId>, // For entity dropdowns
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ConditionLogic {
    And,
    Or,
}

impl ConditionLogic {
    const ALL: [ConditionLogic; 2] = [ConditionLogic::And, ConditionLogic::Or];
}

impl std::fmt::Display for ConditionLogic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConditionLogic::And => write!(f, "And"),
            ConditionLogic::Or => write!(f, "Or"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FilterOperator {
    BeginsWith,
    Contains,
    DoesNotContain,
    EndsWith,
    Equals,
    NotEquals,
    IsEmpty,
    IsNotEmpty,
    GreaterThan,    // For Price and ID
    LessThan,       // For Price and ID
    GreaterOrEqual, // For Price
    LessOrEqual,    // For Price
    Between,        // For ID
}

impl FilterOperator {
    // Get available operators based on the field type
    fn available_for_field(field: &FilterCategory) -> Vec<FilterOperator> {
        match field {
            FilterCategory::Name => vec![
                FilterOperator::Contains,
                FilterOperator::BeginsWith,
                FilterOperator::EndsWith,
                FilterOperator::IsEmpty,
                FilterOperator::IsNotEmpty,
            ],
            FilterCategory::ItemGroup | 
            FilterCategory::ProductClass |
            FilterCategory::TaxGroup |
            FilterCategory::SecurityLevel |
            FilterCategory::RevenueCategory |
            FilterCategory::ReportCategory => vec![
                FilterOperator::Contains,
                FilterOperator::Equals,
                FilterOperator::NotEquals,
                FilterOperator::IsEmpty,
                FilterOperator::IsNotEmpty,
            ],
            FilterCategory::PriceLevel |
            FilterCategory::ChoiceGroup |
            FilterCategory::PrinterLogical => vec![
                FilterOperator::Contains,
                FilterOperator::DoesNotContain,
                FilterOperator::Equals,
                FilterOperator::NotEquals,
                FilterOperator::IsEmpty,
                FilterOperator::IsNotEmpty,
            ],
            FilterCategory::Price => vec![
                FilterOperator::Equals,
                FilterOperator::NotEquals,
                FilterOperator::GreaterThan,
                FilterOperator::LessThan,
                FilterOperator::GreaterOrEqual,
                FilterOperator::LessOrEqual,
                FilterOperator::IsEmpty,
                FilterOperator::IsNotEmpty,
            ],
            FilterCategory::Id => vec![
                FilterOperator::GreaterThan,
                FilterOperator::LessThan,
                FilterOperator::Between,
            ],
        }
    }
}

impl std::fmt::Display for FilterOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterOperator::BeginsWith => write!(f, "begins with"),
            FilterOperator::Contains => write!(f, "contains"),
            FilterOperator::DoesNotContain => write!(f, "does not contain"),
            FilterOperator::EndsWith => write!(f, "ends with"),
            FilterOperator::Equals => write!(f, "is"),
            FilterOperator::NotEquals => write!(f, "is not"),
            FilterOperator::IsEmpty => write!(f, "is empty"),
            FilterOperator::IsNotEmpty => write!(f, "is not empty"),
            FilterOperator::GreaterThan => write!(f, "greater than"),
            FilterOperator::LessThan => write!(f, "less than"),
            FilterOperator::GreaterOrEqual => write!(f, "greater or equal"),
            FilterOperator::LessOrEqual => write!(f, "less or equal"),
            FilterOperator::Between => write!(f, "between"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ActionOperation {
    // For Price
    AddToPrice,
    SubtractFromPrice,
    SetPrice,

    // For multi-value fields (Printer Logical, Choice Group, Price Level)
    Add,
    Remove,
    
    // For all entity fields
    SwapTo,
}

impl ActionOperation {
    // Get available operations based on the category
    fn available_for_category(category: &FilterCategory) -> Vec<ActionOperation> {
        match category {
            FilterCategory::Price => vec![
                ActionOperation::AddToPrice,
                ActionOperation::SubtractFromPrice,
                ActionOperation::SetPrice,
            ],
            FilterCategory::PrinterLogical | FilterCategory::PriceLevel | FilterCategory::ChoiceGroup => vec![
                ActionOperation::Add,
                ActionOperation::Remove,
                ActionOperation::SwapTo,
            ],
            FilterCategory::Name | FilterCategory::Id => vec![], // These shouldn't appear in actions
            _ => vec![
                ActionOperation::SwapTo,
            ],
        }
    }
}

impl std::fmt::Display for ActionOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionOperation::AddToPrice => write!(f, "Add to price"),
            ActionOperation::SubtractFromPrice => write!(f, "Subtract from price"),
            ActionOperation::SetPrice => write!(f, "Set price to"),
            ActionOperation::SwapTo => write!(f, "Swap"),
            ActionOperation::Add => write!(f, "Add"),
            ActionOperation::Remove => write!(f, "Remove"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    UpdateItem(Item),
}

#[derive(Debug, Clone)]
pub struct SuperEdit{
    conditions: Vec<FilterCondition>,
    actions: Vec<FilterAction>,
    filtered_items: Option<BTreeMap<EntityId, Item>>,
    modified_items: Option<BTreeMap<EntityId, Item>>,
    preview_table: Option<ItemsTableView>,
    show_preview: bool,
    changed_item_ids: Vec<EntityId>, // Track which items were actually changed
}

impl SuperEdit {
    pub fn new() -> Self {
        // Start with one default condition
        let default_condition = FilterCondition {
            logic: ConditionLogic::And,
            field: FilterCategory::Name,
            operator: FilterOperator::IsNotEmpty,
            value: String::new(),
            entity_id: None,
        };

        let default_action = FilterAction {
            category: FilterCategory::ItemGroup,
            operation: ActionOperation::SwapTo,
            value: String::new(),
            entity_id: None,
            swap_from_id: None,
            price_level: None,
        };

        Self {
            conditions: vec![default_condition],
            actions: vec![default_action],
            filtered_items: None,
            modified_items: None,
            preview_table: None,
            show_preview: false,
            changed_item_ids: Vec::new(),
        }
    }

    pub fn update(
        &mut self, 
        message: Message,
        items: &mut BTreeMap<EntityId, Item>,
        item_groups: &BTreeMap<EntityId, ItemGroup>,
        tax_groups: &BTreeMap<EntityId, TaxGroup>,
        security_levels: &BTreeMap<EntityId, SecurityLevel>,
        revenue_categories: &BTreeMap<EntityId, RevenueCategory>,
        report_categories: &BTreeMap<EntityId, ReportCategory>,
        product_classes: &BTreeMap<EntityId, ProductClass>,
        choice_groups: &BTreeMap<EntityId, ChoiceGroup>,
        printer_logicals: &BTreeMap<EntityId, PrinterLogical>,
        price_levels: &BTreeMap<EntityId, PriceLevel>,
    ) -> Action<Operation, Message> {
        match message {
            Message::AddCondition => {
                let new_condition = FilterCondition {
                    logic: ConditionLogic::And,
                    field: FilterCategory::Name,
                    operator: FilterOperator::Contains,
                    value: String::new(),
                    entity_id: None,
                };
                self.conditions.push(new_condition);
                Action::none()
            }
            Message::RemoveCondition(index) => {
                if self.conditions.len() > 1 && index < self.conditions.len() {
                    self.conditions.remove(index);
                    self.refresh_filtered_items(items, item_groups, tax_groups, security_levels,
                        revenue_categories, report_categories, product_classes, choice_groups,
                        printer_logicals, price_levels);
                }
                Action::none()
            }
            Message::UpdateConditionLogic(index, logic) => {
                if let Some(condition) = self.conditions.get_mut(index) {
                    condition.logic = logic;
                    self.refresh_filtered_items(items, item_groups, tax_groups, security_levels,
                        revenue_categories, report_categories, product_classes, choice_groups,
                        printer_logicals, price_levels);
                }
                Action::none()
            }
            Message::UpdateConditionField(index, field) => {
                if let Some(condition) = self.conditions.get_mut(index) {
                    condition.field = field.clone();
                    
                    // Update operator if current one is not valid for new field
                    let available_ops = FilterOperator::available_for_field(&field);
                    if !available_ops.contains(&condition.operator) {
                        condition.operator = available_ops[0].clone();
                    }
                    
                    // Clear entity_id for text fields, clear value for entity fields
                    match field {
                        FilterCategory::Name | FilterCategory::Price | FilterCategory::Id => {
                            condition.entity_id = None;
                        }
                        _ => {
                            condition.value = String::new();
                        }
                    }
                    
                    self.refresh_filtered_items(items, item_groups, tax_groups, security_levels,
                        revenue_categories, report_categories, product_classes, choice_groups,
                        printer_logicals, price_levels);
                }
                Action::none()
            }
            Message::UpdateConditionOperator(index, operator) => {
                if let Some(condition) = self.conditions.get_mut(index) {
                    condition.operator = operator;
                    self.refresh_filtered_items(items, item_groups, tax_groups, security_levels,
                        revenue_categories, report_categories, product_classes, choice_groups,
                        printer_logicals, price_levels);
                }
                Action::none()
            }
            Message::UpdateConditionValue(index, value) => {
                if let Some(condition) = self.conditions.get_mut(index) {
                    condition.value = value;
                }

                self.refresh_filtered_items(items, item_groups, tax_groups, security_levels,
                    revenue_categories, report_categories, product_classes, choice_groups,
                    printer_logicals, price_levels);

                Action::none()
            }
            Message::UpdateConditionEntity(index, entity_id) => {
                if let Some(condition) = self.conditions.get_mut(index) {
                    condition.entity_id = Some(entity_id);
                }

                self.refresh_filtered_items(items, item_groups, tax_groups, security_levels,
                    revenue_categories, report_categories, product_classes, choice_groups,
                    printer_logicals, price_levels);

                Action::none()
            }

            // Action messages
            Message::AddAction => {
                let new_action = FilterAction {
                    category: FilterCategory::ItemGroup,
                    operation: ActionOperation::SwapTo,
                    value: String::new(),
                    entity_id: None,
                    swap_from_id: None,
                    price_level: None,
                };
                self.actions.push(new_action);
                Action::none()
            }
            Message::RemoveAction(index) => {
                if self.actions.len() > 1 && index < self.actions.len() {
                    self.actions.remove(index);
                }
                Action::none()
            }
            Message::UpdateActionCategory(index, category) => {
                if let Some(action) = self.actions.get_mut(index) {
                    action.category = category.clone();

                    // Update available operations when category changes
                    let available_ops = ActionOperation::available_for_category(&category);
                    if !available_ops.is_empty() && !available_ops.contains(&action.operation) {
                        action.operation = available_ops[0].clone();
                    }
                    
                    // Reset fields based on category
                    action.entity_id = None;
                    action.swap_from_id = None;
                    action.value = String::new();
                    
                    // Set default price level for price operations
                    if category == FilterCategory::Price {
                        action.price_level = Some(0); // Default price level
                    } else {
                        action.price_level = None;
                    }
                }
                Action::none()
            }
            Message::UpdateActionOperation(index, operation) => {
                if let Some(action) = self.actions.get_mut(index) {
                    action.operation = operation;
                }
                Action::none()
            }
            Message::UpdateActionValue(index, value) => {
                if let Some(action) = self.actions.get_mut(index) {
                    action.value = value;
                }
                Action::none()
            }
            Message::UpdateActionPriceLevel(index, price_level_id) => {
                if let Some(action) = self.actions.get_mut(index) {
                    action.price_level = Some(price_level_id);
                }
                Action::none()
            }
            Message::CommitChanges(item) => {
                Action::operation(Operation::UpdateItem(item))
            }
            Message::UpdateActionEntity(index, entity_id) => {
                if let Some(action) = self.actions.get_mut(index) {
                    action.entity_id = Some(entity_id);
                }
                Action::none()
            }

            Message::PreviewChanges => {
                self.preview_changes(items, item_groups, tax_groups, security_levels,
                    revenue_categories, report_categories, product_classes, choice_groups,
                    printer_logicals, price_levels);
                Action::none()
            }
            Message::UpdateActionSwapFrom(index, entity_id) => {
                if let Some(action) = self.actions.get_mut(index) {
                    action.swap_from_id = Some(entity_id);
                }
                Action::none()
            }
            Message::CancelPreview => {
                self.show_preview = false;
                self.modified_items = None;
                self.changed_item_ids.clear();
                
                // Refresh the table with filtered view
                self.refresh_filtered_items(items, item_groups, tax_groups, security_levels,
                    revenue_categories, report_categories, product_classes, choice_groups,
                    printer_logicals, price_levels);
                Action::none()
            }
            Message::AcceptChanges => {
                if let Some(modified_items) = &self.modified_items {
                    // Update only the changed items
                    for id in &self.changed_item_ids {
                        if let Some(modified_item) = modified_items.get(id) {
                            if let Some(original) = items.get_mut(id) {
                                *original = modified_item.clone();
                            }
                        }
                    }
                    
                    // Reset to initial state
                    self.conditions = vec![FilterCondition {
                        logic: ConditionLogic::And,
                        field: FilterCategory::Name,
                        operator: FilterOperator::IsNotEmpty,
                        value: String::new(),
                        entity_id: None,
                    }];
                    
                    self.actions = vec![FilterAction {
                        category: FilterCategory::ItemGroup,
                        operation: ActionOperation::SwapTo,
                        value: String::new(),
                        entity_id: None,
                        swap_from_id: None,
                        price_level: None,
                    }];
                    
                    self.show_preview = false;
                    self.modified_items = None;
                    self.changed_item_ids.clear();
                    
                    // Refresh the table with all items
                    self.refresh_filtered_items(items, item_groups, tax_groups, security_levels,
                        revenue_categories, report_categories, product_classes, choice_groups,
                        printer_logicals, price_levels);
                }
                Action::none()
            }
            Message::Preview(preview_msg) => {
                match preview_msg {
                    PreviewMessage::SyncHeader(offset) => {
                        if let Some(preview) = &mut self.preview_table {
                            let task = iced::widget::scrollable::scroll_to(preview.header_id.clone(), offset);
                            return Action::task(task)
                        }

                        Action::none()
                    }
                    PreviewMessage::Resizing(index, offset) => {
                        if let Some(preview) = &mut self.preview_table {
                            if let Some(column) = preview.columns.get_mut(index) {
                                    column.resize_offset = Some(offset);
                                }
                        }
                        Action::none()
                    }
                    PreviewMessage::Resized => {
                        if let Some(preview) = &mut self.preview_table {
                            preview.columns.iter_mut().for_each(|column| {
                                if let Some(offset) = column.resize_offset.take() {
                                    column.width += offset;
                                }
                            });
                        }
                        Action::none()
                    }
                    PreviewMessage::ColumnVisibility(visibility_msg) => {
                        if let Some(preview) = &mut self.preview_table {
                            match visibility_msg {
                                ColumnVisibilityMessage::ToggleColumn(column_id) => {
                                    if let Some(visible) = preview.column_visibility.get_mut(&column_id) {
                                        *visible = !*visible;
                                        
                                        // Update the corresponding column
                                        if let Some(column) = preview.columns.iter_mut().find(|c| c.id() == column_id) {
                                            column.visible = *visible;
                                        }
                                    }
                                }
                                ColumnVisibilityMessage::HideContextMenu => {
                                    // Context menu was closed, no action needed
                                }
                            }
                        }
                        Action::none()
                    }
                    PreviewMessage::ColumnVisibilityEnabled(enabled) => {
                        if let Some(preview) = &mut self.preview_table {
                            preview.column_visibility_enabled = enabled;
                        }
                        Action::none()
                    }
                }
            }
        }
    }

    // Helper method to refresh filtered items
    fn refresh_filtered_items(
        &mut self,
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
    ) {
        self.filtered_items = Some(items
            .iter()
            .filter(|(_, item)| self.applies_to_item(
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
            ))
            .map(|(id, item)| (*id, item.clone()))
            .collect());

        let table = match &self.filtered_items {
            Some(filtered_items) => {
                ItemsTableView::new(
                    &filtered_items,
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
            None => {
                ItemsTableView::new(
                    &items,
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
        };
        self.preview_table = Some(table);
    }

    pub fn view<'a>(
        &'a self,
        items: &'a BTreeMap<EntityId, Item>,
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
        let header = row![
            text("Super Editor").style(Modern::primary_text()).size(18),
        ];

        // If section (conditions)
        let if_section = container(
            column![
                text("Filters").style(Modern::primary_text()).size(16),
                // Conditions
                column(
                    self.conditions
                        .iter()
                        .enumerate()
                        .map(|(index, condition)| {
                            self.render_condition(index, condition, item_groups, tax_groups,
                                security_levels, revenue_categories, report_categories,
                                product_classes, choice_groups, printer_logicals, price_levels)
                        })
                        .collect::<Vec<_>>()
                )
                .spacing(8),
                
                // Add condition button
                row![
                    button("+ Add condition")
                        .on_press(Message::AddCondition)
                        .style(Modern::primary_button())
                        .padding([5, 15])
                ]
                .width(Length::Fill)
            ]
            .spacing(15)
        )
        .style(Modern::card_container())
        .width(Length::Fill)
        .padding(15);

        // Then section
        let then_section = container(
            column![
                text("Editor").style(Modern::primary_text()).size(16),
                // Actions
                column(
                    self.actions
                        .iter()
                        .enumerate()
                        .map(|(index, action)| {
                            self.render_action(index, action, item_groups, tax_groups,
                                security_levels, revenue_categories, report_categories,
                                product_classes, choice_groups, printer_logicals, price_levels) 
                        })
                        .collect::<Vec<_>>()
                )
                .spacing(8),
                
                // Add action button
                row![
                    button("+ Add action")
                        .on_press(Message::AddAction)
                        .style(Modern::primary_button())
                        .padding([5, 15]),
                    
                    horizontal_space(),
                    
                    if self.show_preview {
                        row![
                            button("Cancel Preview")
                                .on_press(Message::CancelPreview)
                                .style(Modern::secondary_button())
                                .padding([8, 20]),
                            button("Accept Changes")
                                .on_press(Message::AcceptChanges)
                                .style(Modern::primary_button())
                                .padding([8, 20])
                        ].spacing(10)
                    } else {
                        row![
                            button("Preview Changes")
                                .on_press(Message::PreviewChanges)
                                .style(Modern::secondary_button())
                                .padding([8, 20]),
                        ]
                    }
                ]
                .width(Length::Fill)
            ]
            .spacing(15)
        )
        .style(Modern::card_container())
        .width(Length::Fill)
        .padding(15);

        // Items preview section
        let items_section = container(
            column![
                row![
                    text(if self.show_preview { "Changed Items" } else { "Matching Items" })
                        .style(Modern::primary_text()).size(16),
                ],
                
                // Items list   
                if let Some(table) = &self.preview_table {
                    table.render().map(Message::Preview)
                } else {
                    text("Loading table...").into()
                }
            ]
            //.spacing(10)
        )
        .style(Modern::card_container())
        .width(Length::Fill)
        .padding(15);

        let content = column![
            header,
            iced::widget::vertical_space().height(20),
            if_section,
            iced::widget::vertical_space().height(10),
            then_section,
            iced::widget::vertical_space().height(20),
            items_section
        ]
        .spacing(0)
        .width(Length::Fill);

        container(content)
            .center_x(Length::Fill)
            .width(Length::Fill)
            .padding(20)
            .into()
    }

    fn render_condition<'a>(
        &'a self, 
        index: usize, 
        condition: &'a FilterCondition,
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
        let logic_picker = if index == 0 {
            container(
                text("If").style(Modern::primary_text())
            )
            .width(60)
            .center_x(60)
        } else {
            container(
                pick_list(
                    &ConditionLogic::ALL[..],
                    Some(&condition.logic),
                    move |logic| Message::UpdateConditionLogic(index, logic)
                ).style(Modern::pick_list())
            )
            .width(60)
        };

        let field_picker = pick_list(
            &FilterCategory::ALL_CONDITIONS[..],
            Some(&condition.field),
            move |field| Message::UpdateConditionField(index, field)
        ).style(Modern::pick_list())
        .width(150);

        let operator_picker = pick_list(
            FilterOperator::available_for_field(&condition.field),
            Some(&condition.operator),
            move |operator| Message::UpdateConditionOperator(index, operator)
        ).style(Modern::pick_list())
        .width(150);

        // Determine if value input should be shown based on operator
        let needs_value_input = !matches!(
            condition.operator,
            FilterOperator::IsEmpty | FilterOperator::IsNotEmpty
        );

        let value_input = if needs_value_input {
            match condition.field {
                FilterCategory::Name | FilterCategory::Price => {
                    text_input(
                        match condition.field {
                            FilterCategory::Price => "Amount",
                            _ => "Value",
                        },
                        &condition.value
                    )
                    .on_input(move |s| Message::UpdateConditionValue(index, s))
                    .style(Modern::inline_text_input())
                    .width(150)
                    .into()
                }
                FilterCategory::Id => {
                    if condition.operator == FilterOperator::Between {
                        // For between, we need two inputs
                        let values: Vec<&str> = condition.value.split('-').collect();
                        let from_value = values.get(0).unwrap_or(&"").to_string();
                        let to_value = values.get(1).unwrap_or(&"").to_string();
                        
                        row![
                            text_input("From", &from_value)
                                .on_input(move |s| {
                                    let values: Vec<&str> = condition.value.split('-').collect();
                                    let to_value = values.get(1).unwrap_or(&"").to_string();
                                    Message::UpdateConditionValue(index, format!("{}-{}", s, to_value))
                                })
                                .style(Modern::inline_text_input())
                                .width(70),
                            text(" to ").size(14),
                            text_input("To", &to_value)
                                .on_input(move |s| {
                                    let values: Vec<&str> = condition.value.split('-').collect();
                                    let from_value = values.get(0).unwrap_or(&"").to_string();
                                    Message::UpdateConditionValue(index, format!("{}-{}", from_value, s))
                                })
                                .style(Modern::inline_text_input())
                                .width(70),
                        ].into()
                    } else {
                        text_input("ID", &condition.value)
                            .on_input(move |s| Message::UpdateConditionValue(index, s))
                            .style(Modern::inline_text_input())
                            .width(150)
                            .into()
                    }
                }
                // Entity dropdowns for all entity-based filters
                FilterCategory::ItemGroup => {
                    create_condition_entity_dropdown(index, condition, item_groups)
                }
                FilterCategory::TaxGroup => {
                    create_condition_entity_dropdown(index, condition, tax_groups)
                }
                FilterCategory::SecurityLevel => {
                    create_condition_entity_dropdown(index, condition, security_levels)
                }
                FilterCategory::RevenueCategory => {
                    create_condition_entity_dropdown(index, condition, revenue_categories)
                }
                FilterCategory::ReportCategory => {
                    create_condition_entity_dropdown(index, condition, report_categories)
                }
                FilterCategory::ProductClass => {
                    create_condition_entity_dropdown(index, condition, product_classes)
                }
                FilterCategory::ChoiceGroup => {
                    create_condition_entity_dropdown(index, condition, choice_groups)
                }
                FilterCategory::PrinterLogical => {
                    create_condition_entity_dropdown(index, condition, printer_logicals)
                }
                FilterCategory::PriceLevel => {
                    create_condition_entity_dropdown(index, condition, price_levels)
                }
            }
        } else {
            // Show disabled/empty input for operators that don't need values
            text_input("", "")
                .style(Modern::inline_text_input())
                .width(150)
                .into()
        };

        let remove_button: Element<Message> = if index != 0 {
            button(text("×").size(16))
                .on_press(Message::RemoveCondition(index))
                .style(Modern::danger_button())
                .width(30)
                .height(30).into()
        } else {
            horizontal_space().width(30).into()
        };

        row![
            logic_picker,
            iced::widget::horizontal_space().width(10),
            field_picker,
            iced::widget::horizontal_space().width(10),
            operator_picker,
            iced::widget::horizontal_space().width(10),
            value_input,
            iced::widget::horizontal_space().width(10),
            remove_button,
        ]
        .width(Length::Fill)
        .align_y(iced::Alignment::Center)
        .into()
    }

    fn render_action<'a>(
        &'a self, 
        index: usize, 
        action: &'a FilterAction,
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
        let category_picker = pick_list(
            &FilterCategory::ALL_ACTIONS[..],
            Some(&action.category),
            move |category| Message::UpdateActionCategory(index, category)
        ).style(Modern::pick_list())
        .width(150);

        let operation_picker = pick_list(
            ActionOperation::available_for_category(&action.category),
            Some(&action.operation),
            move |operation| Message::UpdateActionOperation(index, operation)
        ).style(Modern::pick_list())
        .width(150);

        // Build the value section based on category and operation
        let value_section = match (&action.category, &action.operation) {
            // Price operations that need a price level selector
            (FilterCategory::Price, ActionOperation::AddToPrice | 
            ActionOperation::SubtractFromPrice | ActionOperation::SetPrice) => {
                // Create price level options including "Default"
                let mut price_level_options: Vec<(EntityId, String)> = vec![(0, "Default".to_string())];
                price_level_options.extend(
                    price_levels.iter().map(|(id, pl)| (*id, pl.name.clone()))
                );

                let selected = action.price_level.unwrap_or(0);
                let selected_name = price_level_options.iter()
                    .find(|(id, _)| *id == selected)
                    .map(|(_, name)| name.clone());

                row![
                    text_input("Amount", &action.value)
                        .on_input(move |value| Message::UpdateActionValue(index, value))
                        .style(Modern::inline_text_input())
                        .width(100),
                    iced::widget::horizontal_space().width(5),
                    text("at").style(Modern::secondary_text()),
                    iced::widget::horizontal_space().width(5),
                    pick_list(
                        price_level_options.iter().map(|(_, name)| name.clone()).collect::<Vec<_>>(),
                        selected_name,
                        move |name| {
                            let id = price_level_options.iter()
                                .find(|(_, n)| n == &name)
                                .map(|(id, _)| *id)
                                .unwrap_or(0);
                            Message::UpdateActionPriceLevel(index, id)
                        }
                    ).style(Modern::pick_list())
                    .width(100)
                ].into()
            }

            // Regular entity selections for Add/Remove operations on multi-value fields
            (FilterCategory::PrinterLogical, ActionOperation::Add | ActionOperation::Remove) => {
                create_entity_dropdown(index, action, printer_logicals)
            }
            (FilterCategory::ChoiceGroup, ActionOperation::Add | ActionOperation::Remove) => {
                create_entity_dropdown(index, action, choice_groups)
            }
            (FilterCategory::PriceLevel, ActionOperation::Add | ActionOperation::Remove) => {
                create_entity_dropdown(index, action, price_levels)
            }
            
            // All entity swap operations now use swap dropdowns
            (FilterCategory::ItemGroup, ActionOperation::SwapTo) => {
                create_swap_dropdowns(index, action, item_groups, self.filtered_items.as_ref())
            }
            (FilterCategory::TaxGroup, ActionOperation::SwapTo) => {
                create_swap_dropdowns(index, action, tax_groups, self.filtered_items.as_ref())
            }
            (FilterCategory::SecurityLevel, ActionOperation::SwapTo) => {
                create_swap_dropdowns(index, action, security_levels, self.filtered_items.as_ref())
            }
            (FilterCategory::RevenueCategory, ActionOperation::SwapTo) => {
                create_swap_dropdowns(index, action, revenue_categories, self.filtered_items.as_ref())
            }
            (FilterCategory::ReportCategory, ActionOperation::SwapTo) => {
                create_swap_dropdowns(index, action, report_categories, self.filtered_items.as_ref())
            }
            (FilterCategory::ProductClass, ActionOperation::SwapTo) => {
                create_swap_dropdowns(index, action, product_classes, self.filtered_items.as_ref())
            }
            (FilterCategory::ChoiceGroup, ActionOperation::SwapTo) => {
                create_swap_dropdowns(index, action, choice_groups, self.filtered_items.as_ref())
            }
            (FilterCategory::PrinterLogical, ActionOperation::SwapTo) => {
                create_swap_dropdowns(index, action, printer_logicals, self.filtered_items.as_ref())
            }
            (FilterCategory::PriceLevel, ActionOperation::SwapTo) => {
                create_swap_dropdowns(index, action, price_levels, self.filtered_items.as_ref())
            }
            
            // This shouldn't happen with current constraints
            _ => {
                row![
                    text("Invalid combination").style(Modern::red_text())
                ].into()
            }
        };

        let remove_button: Element<Message> = if index != 0 {
            button(text("×").size(16))
                .on_press(Message::RemoveAction(index))
                .style(Modern::danger_button())
                .width(30)
                .height(30).into()
        } else {
            horizontal_space().width(30).into()
        };

        row![
            text("Then").style(Modern::primary_text()).width(60),
            iced::widget::horizontal_space().width(10),
            category_picker,
            iced::widget::horizontal_space().width(10),
            operation_picker,
            iced::widget::horizontal_space().width(10),
            value_section,
            iced::widget::horizontal_space().width(10),
            remove_button,
        ]
        .align_y(iced::Alignment::Center)
        .into()
    }

    pub fn preview_changes(
        &mut self,
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
    ) {
        let mut modified_items = items.clone();
        self.changed_item_ids.clear();
        
        // Apply actions to filtered items and track changes
        for (id, item) in &mut modified_items {
            if self.applies_to_item(item, item_groups, tax_groups, security_levels,
                revenue_categories, report_categories, product_classes, choice_groups,
                printer_logicals, price_levels) {
                
                let original_item = items.get(id).unwrap().clone();
                
                // Apply each action
                for action in &self.actions {
                    self.apply_action_to_item(item, action);
                }
                
                // Check if item actually changed
                if item != &original_item {
                    self.changed_item_ids.push(*id);
                }
            }
        }
        
        self.modified_items = Some(modified_items.clone());
        
        // Create diff table showing only changed items
        let items_to_show: BTreeMap<EntityId, Item> = self.changed_item_ids.iter()
            .filter_map(|id| items.get(id).map(|item| (*id, item.clone())))
            .collect();
        
        let modified_items_to_show: BTreeMap<EntityId, Item> = self.changed_item_ids.iter()
            .filter_map(|id| modified_items.get(id).map(|item| (*id, item.clone())))
            .collect();
        
        let table = ItemsTableView::new_with_diff(
            &items_to_show,
            &modified_items_to_show,
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
        
        self.preview_table = Some(table);
        self.show_preview = true;
    }

    fn apply_action_to_item(&self, item: &mut Item, action: &FilterAction) {
        match (&action.category, &action.operation) {
            // Single entity fields (ItemGroup, TaxGroup, etc.)
            (FilterCategory::ItemGroup, ActionOperation::SwapTo) => {
                if let (Some(from_id), Some(to_id)) = (action.swap_from_id, action.entity_id) {
                    if item.item_group == Some(from_id) {
                        item.item_group = Some(to_id);
                    }
                }
            }
            (FilterCategory::TaxGroup, ActionOperation::SwapTo) => {
                if let (Some(from_id), Some(to_id)) = (action.swap_from_id, action.entity_id) {
                    if item.tax_group == Some(from_id) {
                        item.tax_group = Some(to_id);
                    }
                }
            }
            (FilterCategory::SecurityLevel, ActionOperation::SwapTo) => {
                if let (Some(from_id), Some(to_id)) = (action.swap_from_id, action.entity_id) {
                    if item.security_level == Some(from_id) {
                        item.security_level = Some(to_id);
                    }
                }
            }
            (FilterCategory::RevenueCategory, ActionOperation::SwapTo) => {
                if let (Some(from_id), Some(to_id)) = (action.swap_from_id, action.entity_id) {
                    if item.revenue_category == Some(from_id) {
                        item.revenue_category = Some(to_id);
                    }
                }
            }
            (FilterCategory::ReportCategory, ActionOperation::SwapTo) => {
                if let (Some(from_id), Some(to_id)) = (action.swap_from_id, action.entity_id) {
                    if item.report_category == Some(from_id) {
                        item.report_category = Some(to_id);
                    }
                }
            }
            (FilterCategory::ProductClass, ActionOperation::SwapTo) => {
                if let (Some(from_id), Some(to_id)) = (action.swap_from_id, action.entity_id) {
                    if item.product_class == Some(from_id) {
                        item.product_class = Some(to_id);
                    }
                }
            }
            
            // Multi-value entity fields
            (FilterCategory::PrinterLogical, ActionOperation::SwapTo) => {
                if let (Some(from_id), Some(to_id)) = (action.swap_from_id, action.entity_id) {
                    if let Some(ref mut logicals) = item.printer_logicals {
                        for (id, _) in logicals.iter_mut() {
                            if *id == from_id {
                                *id = to_id;
                                break;
                            }
                        }
                    }
                }
            }
            (FilterCategory::ChoiceGroup, ActionOperation::SwapTo) => {
                if let (Some(from_id), Some(to_id)) = (action.swap_from_id, action.entity_id) {
                    if let Some(ref mut groups) = item.choice_groups {
                        for (id, _) in groups.iter_mut() {
                            if *id == from_id {
                                *id = to_id;
                                break;
                            }
                        }
                    }
                }
            }
            (FilterCategory::PriceLevel, ActionOperation::SwapTo) => {
                if let (Some(from_id), Some(to_id)) = (action.swap_from_id, action.entity_id) {
                    if let Some(ref mut levels) = item.price_levels {
                        for id in levels.iter_mut() {
                            if *id == from_id {
                                *id = to_id;
                                break;
                            }
                        }
                    }
                }
            }
            
            // Price operations
            (FilterCategory::Price, ActionOperation::SetPrice) => {
                if let Ok(new_price) = action.value.parse::<Decimal>() {
                    if action.price_level == Some(0) || action.price_level.is_none() {
                        // Update default price
                        item.default_price = Some(new_price);
                    } else if let Some(price_level_id) = action.price_level {
                        // Update specific price level
                        if let Some(ref mut prices) = item.item_prices {
                            if let Some(price) = prices.iter_mut().find(|p| p.price_level_id == price_level_id) {
                                price.price = new_price;
                            } else {
                                // Add new price entry if it doesn't exist
                                prices.push(ItemPrice {
                                    price_level_id,
                                    price: new_price,
                                });
                            }
                        } else {
                            // Create new item_prices vector with this price
                            item.item_prices = Some(vec![ItemPrice {
                                price_level_id,
                                price: new_price,
                            }]);
                        }
                    }
                }
            }
            (FilterCategory::Price, ActionOperation::AddToPrice) => {
                if let Ok(add_amount) = action.value.parse::<Decimal>() {
                    if action.price_level == Some(0) || action.price_level.is_none() {
                        // Update default price
                        if let Some(current_price) = item.default_price {
                            item.default_price = Some(current_price + add_amount);
                        } else {
                            item.default_price = Some(add_amount);
                        }
                    } else if let Some(price_level_id) = action.price_level {
                        // Update specific price level
                        if let Some(ref mut prices) = item.item_prices {
                            if let Some(price) = prices.iter_mut().find(|p| p.price_level_id == price_level_id) {
                                price.price += add_amount;
                            } else {
                                // Add new price if it doesn't exist
                                prices.push(ItemPrice {
                                    price_level_id,
                                    price: add_amount,
                                });
                            }
                        } else {
                            item.item_prices = Some(vec![ItemPrice {
                                price_level_id,
                                price: add_amount,
                            }]);
                        }
                    }
                }
            }
            (FilterCategory::Price, ActionOperation::SubtractFromPrice) => {
                if let Ok(sub_amount) = action.value.parse::<Decimal>() {
                    if action.price_level == Some(0) || action.price_level.is_none() {
                        // Update default price
                        if let Some(current_price) = item.default_price {
                            item.default_price = Some(current_price - sub_amount);
                        }
                    } else if let Some(price_level_id) = action.price_level {
                        // Update specific price level
                        if let Some(ref mut prices) = item.item_prices {
                            if let Some(price) = prices.iter_mut().find(|p| p.price_level_id == price_level_id) {
                                price.price -= sub_amount;
                            }
                        }
                    }
                }
            }
            
            // Catch-all for invalid combinations
            _ => {}
        }
    }

    // Helper function to evaluate a single condition
    fn evaluate_condition(
        &self,
        condition: &FilterCondition,
        item: &Item,
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
        match condition.field {
            FilterCategory::Name => {
                self.evaluate_string_field(&item.name, &condition.operator, &condition.value)
            }
            FilterCategory::Id => {
                self.evaluate_id_field(item.id, &condition.operator, &condition.value)
            }
            FilterCategory::ItemGroup => {
                if condition.entity_id.is_some() {
                    self.evaluate_entity_by_id(item.item_group, condition.entity_id, &condition.operator)
                } else {
                    self.evaluate_optional_entity_field(
                        item.item_group,
                        item_groups,
                        &condition.operator,
                        &condition.value
                    )
                }
            }
            FilterCategory::TaxGroup => {
                if condition.entity_id.is_some() {
                    self.evaluate_entity_by_id(item.tax_group, condition.entity_id, &condition.operator)
                } else {
                    self.evaluate_optional_entity_field(
                        item.tax_group,
                        tax_groups,
                        &condition.operator,
                        &condition.value
                    )
                }
            }
            FilterCategory::SecurityLevel => {
                if condition.entity_id.is_some() {
                    self.evaluate_entity_by_id(item.security_level, condition.entity_id, &condition.operator)
                } else {
                    self.evaluate_optional_entity_field(
                        item.security_level,
                        security_levels,
                        &condition.operator,
                        &condition.value
                    )
                }
            }
            FilterCategory::RevenueCategory => {
                if condition.entity_id.is_some() {
                    self.evaluate_entity_by_id(item.revenue_category, condition.entity_id, &condition.operator)
                } else {
                    self.evaluate_optional_entity_field(
                        item.revenue_category,
                        revenue_categories,
                        &condition.operator,
                        &condition.value
                    )
                }
            }
            FilterCategory::ReportCategory => {
                if condition.entity_id.is_some() {
                    self.evaluate_entity_by_id(item.report_category, condition.entity_id, &condition.operator)
                } else {
                    self.evaluate_optional_entity_field(
                        item.report_category,
                        report_categories,
                        &condition.operator,
                        &condition.value
                    )
                }
            }
            FilterCategory::ProductClass => {
                if condition.entity_id.is_some() {
                    self.evaluate_entity_by_id(item.product_class, condition.entity_id, &condition.operator)
                } else {
                    self.evaluate_optional_entity_field(
                        item.product_class,
                        product_classes,
                        &condition.operator,
                        &condition.value
                    )
                }
            }
            FilterCategory::ChoiceGroup => {
                if condition.entity_id.is_some() {
                    self.evaluate_multi_entity_by_id(
                        item.choice_groups.as_ref().map(|v| v.iter().map(|(id, _)| *id).collect()),
                        condition.entity_id,
                        &condition.operator
                    )
                } else {
                    self.evaluate_multi_entity_field(
                        item.choice_groups.as_ref(),
                        choice_groups,
                        &condition.operator,
                        &condition.value
                    )
                }
            }
            FilterCategory::PrinterLogical => {
                if condition.entity_id.is_some() {
                    self.evaluate_multi_entity_by_id(
                        item.printer_logicals.as_ref().map(|v| v.iter().map(|(id, _)| *id).collect()),
                        condition.entity_id,
                        &condition.operator
                    )
                } else {
                    self.evaluate_printer_logical_field(
                        item.printer_logicals.as_ref(),
                        printer_logicals,
                        &condition.operator,
                        &condition.value
                    )
                }
            }
            FilterCategory::PriceLevel => {
                if condition.entity_id.is_some() {
                    self.evaluate_multi_entity_by_id(
                        item.price_levels.as_ref().map(|v| v.clone()),
                        condition.entity_id,
                        &condition.operator
                    )
                } else {
                    self.evaluate_price_level_field(
                        item.price_levels.as_ref(),
                        price_levels,
                        &condition.operator,
                        &condition.value
                    )
                }
            }
            FilterCategory::Price => {
                self.evaluate_price_field(
                    &item,
                    &condition.operator,
                    &condition.value
                )
            }
        }
    }

    // Helper methods for evaluating different field types
    fn evaluate_string_field(&self, field_value: &str, operator: &FilterOperator, condition_value: &str) -> bool {
        match operator {
            FilterOperator::Contains => field_value.to_lowercase().contains(&condition_value.to_lowercase()),
            FilterOperator::BeginsWith => field_value.to_lowercase().starts_with(&condition_value.to_lowercase()),
            FilterOperator::EndsWith => field_value.to_lowercase().ends_with(&condition_value.to_lowercase()),
            FilterOperator::Equals => field_value.to_lowercase() == condition_value.to_lowercase(),
            FilterOperator::NotEquals => field_value.to_lowercase() != condition_value.to_lowercase(),
            FilterOperator::IsEmpty => field_value.is_empty(),
            FilterOperator::IsNotEmpty => !field_value.is_empty(),
            _ => false,
        }
    }

    fn evaluate_id_field(&self, id: EntityId, operator: &FilterOperator, condition_value: &str) -> bool {
        match operator {
            FilterOperator::GreaterThan => {
                if let Ok(value) = condition_value.parse::<EntityId>() {
                    id > value
                } else {
                    false
                }
            }
            FilterOperator::LessThan => {
                if let Ok(value) = condition_value.parse::<EntityId>() {
                    id < value
                } else {
                    false
                }
            }
            FilterOperator::Between => {
                let values: Vec<&str> = condition_value.split('-').collect();
                if values.len() == 2 {
                    if let (Ok(from), Ok(to)) = (values[0].parse::<EntityId>(), values[1].parse::<EntityId>()) {
                        id >= from && id <= to
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn evaluate_entity_by_id(
        &self,
        field_id: Option<EntityId>,
        condition_entity_id: Option<EntityId>,
        operator: &FilterOperator
    ) -> bool {
        match operator {
            FilterOperator::Equals => field_id == condition_entity_id,
            FilterOperator::NotEquals => field_id != condition_entity_id,
            FilterOperator::IsEmpty => field_id.is_none(),
            FilterOperator::IsNotEmpty => field_id.is_some(),
            _ => false,
        }
    }

    fn evaluate_multi_entity_by_id(
        &self,
        field_ids: Option<Vec<EntityId>>,
        condition_entity_id: Option<EntityId>,
        operator: &FilterOperator
    ) -> bool {
        if let Some(ids) = field_ids {
            match operator {
                FilterOperator::Contains | FilterOperator::Equals => {
                    condition_entity_id.map_or(false, |id| ids.contains(&id))
                }
                FilterOperator::DoesNotContain | FilterOperator::NotEquals => {
                    condition_entity_id.map_or(true, |id| !ids.contains(&id))
                }
                FilterOperator::IsEmpty => ids.is_empty(),
                FilterOperator::IsNotEmpty => !ids.is_empty(),
                _ => false,
            }
        } else {
            *operator == FilterOperator::IsEmpty
        }
    }

    fn evaluate_optional_entity_field<T: HasName>(
        &self,
        field_id: Option<EntityId>,
        entities: &BTreeMap<EntityId, T>,
        operator: &FilterOperator,
        condition_value: &str
    ) -> bool {
        if let Some(id) = field_id {
            if let Some(entity) = entities.get(&id) {
                self.evaluate_string_field(entity.name(), operator, condition_value)
            } else {
                *operator == FilterOperator::IsEmpty
            }
        } else {
            *operator == FilterOperator::IsEmpty
        }
    }

    fn evaluate_multi_entity_field<T: HasName>(
        &self,
        field_ids: Option<&Vec<(EntityId, i32)>>,
        entities: &BTreeMap<EntityId, T>,
        operator: &FilterOperator,
        condition_value: &str
    ) -> bool {
        if let Some(ids) = field_ids {
            if ids.is_empty() {
                *operator == FilterOperator::IsEmpty
            } else {
                match operator {
                    FilterOperator::IsEmpty => false,
                    FilterOperator::IsNotEmpty => true,
                    FilterOperator::DoesNotContain => {
                        !ids.iter().any(|(id, _)| {
                            if let Some(entity) = entities.get(id) {
                                entity.name().to_lowercase().contains(&condition_value.to_lowercase())
                            } else {
                                false
                            }
                        })
                    }
                    _ => {
                        ids.iter().any(|(id, _)| {
                            if let Some(entity) = entities.get(id) {
                                self.evaluate_string_field(entity.name(), operator, condition_value)
                            } else {
                                false
                            }
                        })
                    }
                }
            }
        } else {
            *operator == FilterOperator::IsEmpty
        }
    }

    fn evaluate_printer_logical_field(
        &self,
        field_ids: Option<&Vec<(EntityId, bool)>>,
        entities: &BTreeMap<EntityId, PrinterLogical>,
        operator: &FilterOperator,
        condition_value: &str
    ) -> bool {
        if let Some(ids) = field_ids {
            if ids.is_empty() {
                *operator == FilterOperator::IsEmpty
            } else {
                match operator {
                    FilterOperator::IsEmpty => false,
                    FilterOperator::IsNotEmpty => true,
                    FilterOperator::DoesNotContain => {
                        !ids.iter().any(|(id, _)| {
                            if let Some(entity) = entities.get(id) {
                                entity.name.to_lowercase().contains(&condition_value.to_lowercase())
                            } else {
                                false
                            }
                        })
                    }
                    _ => {
                        ids.iter().any(|(id, _)| {
                            if let Some(entity) = entities.get(id) {
                                self.evaluate_string_field(&entity.name, operator, condition_value)
                            } else {
                                false
                            }
                        })
                    }
                }
            }
        } else {
            *operator == FilterOperator::IsEmpty
        }
    }

    fn evaluate_price_level_field(
        &self,
        field_ids: Option<&Vec<EntityId>>,
        entities: &BTreeMap<EntityId, PriceLevel>,
        operator: &FilterOperator,
        condition_value: &str
    ) -> bool {
        if let Some(ids) = field_ids {
            if ids.is_empty() {
                *operator == FilterOperator::IsEmpty
            } else {
                match operator {
                    FilterOperator::IsEmpty => false,
                    FilterOperator::IsNotEmpty => true,
                    FilterOperator::DoesNotContain => {
                        !ids.iter().any(|id| {
                            if let Some(entity) = entities.get(id) {
                                entity.name.to_lowercase().contains(&condition_value.to_lowercase())
                            } else {
                                false
                            }
                        })
                    }
                    _ => {
                        ids.iter().any(|id| {
                            if let Some(entity) = entities.get(id) {
                                self.evaluate_string_field(&entity.name, operator, condition_value)
                            } else {
                                false
                            }
                        })
                    }
                }
            }
        } else {
            *operator == FilterOperator::IsEmpty
        }
    }

    fn evaluate_price_field(
        &self,
        item: &Item,
        operator: &FilterOperator,
        condition_value: &str
    ) -> bool {
        // First check default_price
        if let Some(default_price) = item.default_price {
            if let Ok(condition_price) = condition_value.parse::<Decimal>() {
                let matches = match operator {
                    FilterOperator::GreaterThan => default_price > condition_price,
                    FilterOperator::LessThan => default_price < condition_price,
                    FilterOperator::GreaterOrEqual => default_price >= condition_price,
                    FilterOperator::LessOrEqual => default_price <= condition_price,
                    FilterOperator::Equals => (default_price - condition_price).abs() < Decimal::new(1, 2),
                    FilterOperator::NotEquals => (default_price - condition_price).abs() >= Decimal::new(1, 2),
                    FilterOperator::IsEmpty => false,
                    FilterOperator::IsNotEmpty => true,
                    _ => false,
                };
                
                if matches {
                    return true;
                }
            }
        }
        
        // Then check item_prices
        if let Some(item_prices) = &item.item_prices {
            for item_price in item_prices {
                if let Ok(condition_price) = condition_value.parse::<Decimal>() {
                    let matches = match operator {
                        FilterOperator::GreaterThan => item_price.price > condition_price,
                        FilterOperator::LessThan => item_price.price < condition_price,
                        FilterOperator::GreaterOrEqual => item_price.price >= condition_price,
                        FilterOperator::LessOrEqual => item_price.price <= condition_price,
                        FilterOperator::Equals => (item_price.price - condition_price).abs() < Decimal::new(1, 2),
                        FilterOperator::NotEquals => (item_price.price - condition_price).abs() >= Decimal::new(1, 2),
                        FilterOperator::IsEmpty => false,
                        FilterOperator::IsNotEmpty => true,
                        _ => false,
                    };
                    
                    if matches {
                        return true;
                    }
                }
            }
        }
        
        // If no prices exist
        *operator == FilterOperator::IsEmpty
    }

    pub fn applies_to_item(
        &self,
        item: &Item,
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
        if self.conditions.is_empty() {
            return true;
        }

        // Start with the first condition
        let mut result = self.evaluate_condition(
            &self.conditions[0],
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
        );

        // Apply AND/OR logic for subsequent conditions
        for (index, condition) in self.conditions.iter().enumerate().skip(1) {
            let condition_result = self.evaluate_condition(
                condition,
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
            );

            match condition.logic {
                ConditionLogic::And => result = result && condition_result,
                ConditionLogic::Or => result = result || condition_result,
            }
        }

        result
    }

    pub fn fill_table(
        &mut self,
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
    ) {
        let table = ItemsTableView::new(
            &items,
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
        self.preview_table = Some(table);
    }
}

// Trait for entities that have a name field
pub trait HasName {
    fn name(&self) -> &str;
}

// Implement HasName for all your entity types
impl HasName for ItemGroup {
    fn name(&self) -> &str { &self.name }
}

impl HasName for TaxGroup {
    fn name(&self) -> &str { &self.name }
}

impl HasName for SecurityLevel {
    fn name(&self) -> &str { &self.name }
}

impl HasName for RevenueCategory {
    fn name(&self) -> &str { &self.name }
}

impl HasName for ReportCategory {
    fn name(&self) -> &str { &self.name }
}

impl HasName for ProductClass {
    fn name(&self) -> &str { &self.name }
}

impl HasName for ChoiceGroup {
    fn name(&self) -> &str { &self.name }
}

impl HasName for PrinterLogical {
    fn name(&self) -> &str { &self.name }
}

impl HasName for PriceLevel  {
    fn name (&self) -> &str { &self.name }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FilterCategory {
    Name,
    Id,
    ItemGroup,
    ProductClass,
    TaxGroup,
    SecurityLevel,
    RevenueCategory,
    PriceLevel,
    ChoiceGroup,
    PrinterLogical,
    ReportCategory,
    Price,
}

impl FilterCategory {
    // Categories available for conditions
    const ALL_CONDITIONS: [FilterCategory; 12] = [
        FilterCategory::Name,
        FilterCategory::Id,
        FilterCategory::ItemGroup,
        FilterCategory::ProductClass,
        FilterCategory::TaxGroup,
        FilterCategory::SecurityLevel,
        FilterCategory::RevenueCategory,
        FilterCategory::PriceLevel,
        FilterCategory::ChoiceGroup,
        FilterCategory::PrinterLogical,
        FilterCategory::ReportCategory,
        FilterCategory::Price,
    ];
    
    // Categories available for actions (excludes Name and Id)
    const ALL_ACTIONS: [FilterCategory; 10] = [
        FilterCategory::ItemGroup,
        FilterCategory::ProductClass,
        FilterCategory::TaxGroup,
        FilterCategory::SecurityLevel,
        FilterCategory::RevenueCategory,
        FilterCategory::PriceLevel,
        FilterCategory::ChoiceGroup,
        FilterCategory::PrinterLogical,
        FilterCategory::ReportCategory,
        FilterCategory::Price,
    ];
}

impl std::fmt::Display for FilterCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FilterCategory::Name => "Name",
                FilterCategory::Id => "ID",
                FilterCategory::ItemGroup => "Item Group",
                FilterCategory::ProductClass => "Product Class",
                FilterCategory::TaxGroup => "Tax Group",
                FilterCategory::SecurityLevel => "Security Level",
                FilterCategory::RevenueCategory => "Revenue Category",
                FilterCategory::PriceLevel => "Price Level",
                FilterCategory::ChoiceGroup => "Choice Group",
                FilterCategory::PrinterLogical => "Printer Logical",
                FilterCategory::ReportCategory => "Report Category",
                FilterCategory::Price => "Price",
            }
        )
    }
}

// Helper function to create entity dropdown for conditions
fn create_condition_entity_dropdown<'a, T: HasName + Clone>(
    index: usize,
    condition: &FilterCondition,
    entities: &'a BTreeMap<EntityId, T>
) -> Element<'a, Message> {
    let options: Vec<(EntityId, String)> = entities.iter()
        .map(|(id, entity)| (*id, entity.name().to_string()))
        .collect();
    
    let selected_name = condition.entity_id
        .and_then(|id| entities.get(&id))
        .map(|e| e.name().to_string());
    
    row![
        pick_list(
            options.iter().map(|(_, name)| name.clone()).collect::<Vec<_>>(),
            selected_name,
            move |name| {
                let id = options.iter()
                    .find(|(_, n)| n == &name)
                    .map(|(id, _)| *id)
                    .unwrap_or(0);
                Message::UpdateConditionEntity(index, id)
            }
        )
        .style(Modern::pick_list())
        .width(150)
    ].into()
}

// Helper function to create swap from/to dropdowns
fn create_swap_dropdowns<'a, T: HasName + Clone>(
    index: usize,
    action: &FilterAction,
    all_entities: &'a BTreeMap<EntityId, T>,
    filtered_items: Option<&BTreeMap<EntityId, Item>>,
) -> Element<'a, Message> {
    // Get entities that appear in filtered items
    let mut used_entity_ids = std::collections::HashSet::new();
    
    if let Some(items) = filtered_items {
        for item in items.values() {
            match std::any::type_name::<T>() {
                name if name.contains("ItemGroup") => {
                    if let Some(id) = item.item_group {
                        used_entity_ids.insert(id);
                    }
                }
                name if name.contains("ProductClass") => {
                    if let Some(id) = item.product_class {
                        used_entity_ids.insert(id);
                    }
                }
                name if name.contains("TaxGroup") => {
                    if let Some(id) = item.tax_group {
                        used_entity_ids.insert(id);
                    }
                }
                name if name.contains("SecurityLevel") => {
                    if let Some(id) = item.security_level {
                        used_entity_ids.insert(id);
                    }
                }
                name if name.contains("RevenueCategory") => {
                    if let Some(id) = item.revenue_category {
                        used_entity_ids.insert(id);
                    }
                }
                name if name.contains("ReportCategory") => {
                    if let Some(id) = item.report_category {
                        used_entity_ids.insert(id);
                    }
                }
                name if name.contains("PrinterLogical") => {
                    if let Some(logicals) = &item.printer_logicals {
                        for (id, _) in logicals {
                            used_entity_ids.insert(*id);
                        }
                    }
                }
                name if name.contains("ChoiceGroup") => {
                    if let Some(groups) = &item.choice_groups {
                        for (id, _) in groups {
                            used_entity_ids.insert(*id);
                        }
                    }
                }
                name if name.contains("PriceLevel") => {
                    if let Some(levels) = &item.price_levels {
                        for id in levels {
                            used_entity_ids.insert(*id);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    
    // Create options for swap from (only entities used in filtered items)
    let swap_from_options: Vec<(EntityId, String)> = all_entities.iter()
        .filter(|(id, _)| used_entity_ids.contains(id))
        .map(|(id, entity)| (*id, entity.name().to_string()))
        .collect();
    
    let swap_from_selected = action.swap_from_id
        .and_then(|id| all_entities.get(&id))
        .map(|e| e.name().to_string());
    
    // Create options for swap to (all available entities)
    let swap_to_options: Vec<(EntityId, String)> = all_entities.iter()
        .map(|(id, entity)| (*id, entity.name().to_string()))
        .collect();
    
    let swap_to_selected = action.entity_id
        .and_then(|id| all_entities.get(&id))
        .map(|e| e.name().to_string());
    
    row![
        pick_list(
            swap_from_options.iter().map(|(_, name)| name.clone()).collect::<Vec<_>>(),
            swap_from_selected,
            move |name| {
                let id = swap_from_options.iter()
                    .find(|(_, n)| n == &name)
                    .map(|(id, _)| *id)
                    .unwrap_or(0);
                Message::UpdateActionSwapFrom(index, id)
            }
        )
        .style(Modern::pick_list())
        .width(150),
        iced::widget::horizontal_space().width(5),
        text("to").style(Modern::secondary_text()).center(),
        iced::widget::horizontal_space().width(5),
        pick_list(
            swap_to_options.iter().map(|(_, name)| name.clone()).collect::<Vec<_>>(),
            swap_to_selected,
            move |name| {
                let id = swap_to_options.iter()
                    .find(|(_, n)| n == &name)
                    .map(|(id, _)| *id)
                    .unwrap_or(0);
                Message::UpdateActionEntity(index, id)
            }
        )
        .style(Modern::pick_list())
        .width(150)
    ].into()
}

fn create_entity_dropdown<'a, T: HasName + Clone>(
    index: usize,
    action: &FilterAction,
    entities: &'a BTreeMap<EntityId, T>
) -> Element<'a, Message> {
    let options: Vec<(EntityId, String)> = entities.iter()
        .map(|(id, entity)| (*id, entity.name().to_string()))
        .collect();
    
    let selected_name = action.entity_id
        .and_then(|id| entities.get(&id))
        .map(|e| e.name().to_string());
    
    row![
        pick_list(
            options.iter().map(|(_, name)| name.clone()).collect::<Vec<_>>(),
            selected_name,
            move |name| {
                let id = options.iter()
                    .find(|(_, n)| n == &name)
                    .map(|(id, _)| *id)
                    .unwrap_or(0);
                Message::UpdateActionEntity(index, id)
            }
        )
        .style(Modern::pick_list())
        .width(185)
    ].into()
}