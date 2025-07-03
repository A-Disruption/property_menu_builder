use iced::{Element, Task, Length};
use iced::widget::{button, column, container, horizontal_space, pick_list, row, text, text_input, scrollable};
use iced_modern_theme::Modern;
use std::collections::BTreeMap;
use rust_decimal::Decimal;
use crate::Action;
use crate::{
    items::{Item, ViewContext},
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
    
    // Action management
    AddAction,
    RemoveAction(usize),
    UpdateActionCategory(usize, FilterCategory),
    UpdateActionOperation(usize, ActionOperation),
    UpdateActionValue(usize, String),
    UpdateActionPriceLevel(usize, EntityId),

    //Save Changes to App State
    CommitChanges(Item),

    //Table Preview
    Preview(PreviewMessage),
}

#[derive(Debug, Clone)]
struct FilterAction {
    category: FilterCategory,
    operation: ActionOperation,
    value: String,
    price_level: Option<EntityId>, // For price-related operations
}

#[derive(Debug, Clone)]
struct FilterCondition {
    logic: ConditionLogic,      // And/Or
    field: FilterCategory,      // Name, ItemGroup, etc.
    operator: FilterOperator,   // begins with, contains, etc.
    value: String,             // user input value
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
    EndsWith,
    Equals,
    NotEquals,
    IsEmpty,
    IsNotEmpty,
    GreaterThan,    // For Price
    LessThan,       // For Price
    GreaterOrEqual, // For Price
    LessOrEqual,    // For Price
}

impl FilterOperator {
    // Get available operators based on the field type
    fn available_for_field(field: &FilterCategory) -> Vec<FilterOperator> {
        match field {
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
            _ => vec![
                FilterOperator::BeginsWith,
                FilterOperator::Contains,
                FilterOperator::EndsWith,
                FilterOperator::Equals,
                FilterOperator::NotEquals,
                FilterOperator::IsEmpty,
                FilterOperator::IsNotEmpty,
            ],
        }
    }
}

impl std::fmt::Display for FilterOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterOperator::BeginsWith => write!(f, "begins with"),
            FilterOperator::Contains => write!(f, "contains"),
            FilterOperator::EndsWith => write!(f, "ends with"),
            FilterOperator::Equals => write!(f, "is"),
            FilterOperator::NotEquals => write!(f, "is not"),
            FilterOperator::IsEmpty => write!(f, "is empty"),
            FilterOperator::IsNotEmpty => write!(f, "is not empty"),
            FilterOperator::GreaterThan => write!(f, "greater than"),
            FilterOperator::LessThan => write!(f, "less than"),
            FilterOperator::GreaterOrEqual => write!(f, "greater or equal"),
            FilterOperator::LessOrEqual => write!(f, "less or equal"),
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
    
    // For single-value fields (any category)
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
            FilterCategory::PrinterLogical | 
            FilterCategory::ChoiceGroup | 
            FilterCategory::PriceLevel => vec![
                ActionOperation::Add,
                ActionOperation::Remove,
                ActionOperation::SwapTo,
            ],
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
            ActionOperation::Add => write!(f, "Add"),
            ActionOperation::Remove => write!(f, "Remove"),
            ActionOperation::SwapTo => write!(f, "Swap"),
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
    preview_table: Option<ItemsTableView>, 
}

impl SuperEdit {

    pub fn new() -> Self {
        // Start with one default condition
        let default_condition = FilterCondition {
            logic: ConditionLogic::And,
            field: FilterCategory::Name,
            operator: FilterOperator::IsNotEmpty,
            value: String::new(),
        };

        let default_action = FilterAction {
            category: FilterCategory::Name,
            operation: ActionOperation::SwapTo,
            value: String::new(),
            price_level: None,
        };

        Self {
            conditions: vec![default_condition],
            actions: vec![default_action],
            filtered_items: None,
            preview_table: None,
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

            // Action messages
            Message::AddAction => {
                let new_action = FilterAction {
                    category: FilterCategory::Name,
                    operation: ActionOperation::SwapTo,
                    value: String::new(),
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
                    if !available_ops.contains(&action.operation) {
                        action.operation = available_ops[0].clone();
                    }
                    // Set default price level for price operations
                    if category == FilterCategory::Price {
                        action.price_level = price_levels
                            .keys()
                            .next()
                            .copied();
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
                            self.render_condition(index, condition)
                        })
                        .collect::<Vec<_>>()
                )
                .spacing(8),
                
                // Add condition button
                row![
                    button(
                        row![
                            text("+ Add condition").style(Modern::link_text())
                        ]
                        .align_y(iced::Alignment::Center)
                    )
                    .on_press(Message::AddCondition)
                    .style(Modern::plain_button())
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
                            self.render_action(index, action, price_levels) 
                        })
                        .collect::<Vec<_>>()
                )
                .spacing(8),
                
                // Add action button
                row![
                    button(
                        row![
                            text("+ Add action").style(Modern::link_text())
                        ]
                        .align_y(iced::Alignment::Center)
                    )
                    .on_press(Message::AddAction)
                    .style(Modern::plain_button())
                    .padding([5, 15])
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
                    text("Matching Items").style(Modern::primary_text()).size(16),
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
////        .max_width(800);

        container(content)
            .center_x(Length::Fill)
            .width(Length::Fill)
            .padding(20)
            .into()
    }

    fn render_condition<'a>(&'a self, index: usize, condition: &'a FilterCondition) -> Element<'a, Message> {
        let logic_picker = if index == 0 {
            // First condition shows "If" with same width as pick_list
            container(
                text("If").style(Modern::primary_text())
            )
            .width(60)
            .center_x(60)
        } else {
            // Subsequent conditions show And/Or picker
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
            &FilterCategory::ALL[..],
            Some(&condition.field),
            move |field| Message::UpdateConditionField(index, field)
        ).style(Modern::pick_list())
        .width(140);

        let operator_picker = pick_list(
            FilterOperator::available_for_field(&condition.field),
            Some(&condition.operator),
            move |operator| Message::UpdateConditionOperator(index, operator)
        ).style(Modern::pick_list())
        .width(120);

        // Determine if value input should be shown based on operator
        let needs_value_input = !matches!(
            condition.operator,
            FilterOperator::IsEmpty | FilterOperator::IsNotEmpty
        );

        let value_input = if needs_value_input {
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
        } else {
            // Show disabled/empty input for operators that don't need values
            text_input("", "")
                .style(Modern::inline_text_input())
                .width(150)
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
        price_levels: &'a BTreeMap<EntityId, PriceLevel>,
    ) -> Element<'a, Message> {
        let category_picker = pick_list(
            &FilterCategory::ALL[..],
            Some(&action.category),
            move |category| Message::UpdateActionCategory(index, category)
        ).style(Modern::pick_list())
        .width(140);

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

                // Convert BTreeMap to Vec for pick_list
                let price_level_options: Vec<PriceLevel> = price_levels.values().cloned().collect();

                let selected_price_level = action.price_level
                    .and_then(|id| price_levels.get(&id).cloned());

                row![
                    text_input("Amount", &action.value)
                        .on_input(move |value| Message::UpdateActionValue(index, value))
                        .style(Modern::inline_text_input())
                        .width(100),
                    iced::widget::horizontal_space().width(5),
                    text("at").style(Modern::secondary_text()),
                    iced::widget::horizontal_space().width(5),
                    pick_list(
                        price_level_options,
                        selected_price_level,
                        move |price_level| Message::UpdateActionPriceLevel(index, price_level.id)
                    ).style(Modern::pick_list())
                    .width(80)
                ]
            }
            // Regular value input for other operations
            _ => {
                row![
                    text_input("Value", &action.value)
                        .on_input(move |value| Message::UpdateActionValue(index, value))
                        .style(Modern::inline_text_input())
                        .width(185)
                ]
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
            FilterCategory::ItemGroup => {
                self.evaluate_optional_entity_field(
                    item.item_group,
                    item_groups,
                    &condition.operator,
                    &condition.value
                )
            }
            FilterCategory::TaxGroup => {
                self.evaluate_optional_entity_field(
                    item.tax_group,
                    tax_groups,
                    &condition.operator,
                    &condition.value
                )
            }
            FilterCategory::SecurityLevel => {
                self.evaluate_optional_entity_field(
                    item.security_level,
                    security_levels,
                    &condition.operator,
                    &condition.value
                )
            }
            FilterCategory::RevenueCategory => {
                self.evaluate_optional_entity_field(
                    item.revenue_category,
                    revenue_categories,
                    &condition.operator,
                    &condition.value
                )
            }
            FilterCategory::ReportCategory => {
                self.evaluate_optional_entity_field(
                    item.report_category,
                    report_categories,
                    &condition.operator,
                    &condition.value
                )
            }
            FilterCategory::ProductClass => {
                self.evaluate_optional_entity_field(
                    item.product_class,
                    product_classes,
                    &condition.operator,
                    &condition.value
                )
            }
            FilterCategory::ChoiceGroup => {
                self.evaluate_multi_entity_field(
                    item.choice_groups.as_ref(),
                    choice_groups,
                    &condition.operator,
                    &condition.value
                )
            }
            FilterCategory::PrinterLogical => {
                self.evaluate_printer_logical_field(
                    item.printer_logicals.as_ref(),
                    printer_logicals,
                    &condition.operator,
                    &condition.value
                )
            }
            FilterCategory::PriceLevel => {
                self.evaluate_price_level_field(
                    item.price_levels.as_ref(),
                    price_levels,
                    &condition.operator,
                    &condition.value
                )
            }
            FilterCategory::Price => {
                self.evaluate_price_field(
                    item.price_levels.as_ref(),
                    price_levels,
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
        price_level_ids: Option<&Vec<EntityId>>,
        price_levels: &BTreeMap<EntityId, PriceLevel>,
        operator: &FilterOperator,
        condition_value: &str
    ) -> bool {
        if let Some(ids) = price_level_ids {
            if let Some(first_id) = ids.first() {
                if let Some(price_level) = price_levels.get(first_id) {
                    if let Ok(condition_price) = condition_value.parse::<Decimal>() {
                        let item_price = price_level.price;
                        match operator {
                            FilterOperator::GreaterThan => item_price > condition_price,
                            FilterOperator::LessThan => item_price < condition_price,
                            FilterOperator::GreaterOrEqual => item_price >= condition_price,
                            FilterOperator::LessOrEqual => item_price <= condition_price,
                            FilterOperator::Equals => (item_price - condition_price).abs() < Decimal::new(1, 2),
                            FilterOperator::NotEquals => (item_price - condition_price).abs() >= Decimal::new(1, 2),
                            FilterOperator::IsEmpty => false,
                            FilterOperator::IsNotEmpty => true,
                            _ => false,
                        }
                    } else {
                        match operator {
                            FilterOperator::IsEmpty => false,
                            FilterOperator::IsNotEmpty => true,
                            _ => false,
                        }
                    }
                } else {
                    *operator == FilterOperator::IsEmpty
                }
            } else {
                *operator == FilterOperator::IsEmpty
            }
        } else {
            *operator == FilterOperator::IsEmpty
        }
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum FilterCategory {
    Name,
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
    const ALL: [FilterCategory; 11] = [
        FilterCategory::Name,
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