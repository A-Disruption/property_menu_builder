use iced::{Element, Task, Length};
use iced::widget::{button, column, container, horizontal_space, pick_list, row, text, text_input, scrollable};
use iced_modern_theme::Modern;
use std::collections::BTreeMap;
use rust_decimal::Decimal;
use crate::Action;
use crate::{
    items::Item,
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
    const ALL: [FilterOperator; 11] = [
        FilterOperator::BeginsWith,
        FilterOperator::Contains,
        FilterOperator::EndsWith,
        FilterOperator::Equals,
        FilterOperator::NotEquals,
        FilterOperator::IsEmpty,
        FilterOperator::IsNotEmpty,
        FilterOperator::GreaterThan,
        FilterOperator::LessThan,
        FilterOperator::GreaterOrEqual,
        FilterOperator::LessOrEqual,
    ];

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
pub struct SuperEdit {
    conditions: Vec<FilterCondition>,
    actions: Vec<FilterAction>,
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
        }
    }

    pub fn update(
        &mut self, 
        message: Message,
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
                }
                Action::none()
            }
            Message::UpdateConditionLogic(index, logic) => {
                if let Some(condition) = self.conditions.get_mut(index) {
                    condition.logic = logic;
                }
                Action::none()
            }
            Message::UpdateConditionField(index, field) => {
                if let Some(condition) = self.conditions.get_mut(index) {
                    condition.field = field;
                }
                Action::none()
            }
            Message::UpdateConditionOperator(index, operator) => {
                if let Some(condition) = self.conditions.get_mut(index) {
                    condition.operator = operator;
                }
                Action::none()
            }
            Message::UpdateConditionValue(index, value) => {
                if let Some(condition) = self.conditions.get_mut(index) {
                    condition.value = value;
                }
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
                crate::Action::operation(Operation::UpdateItem(item))
            }
        }
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

        // Action buttons
/*         let action_buttons = row![
            button("Run rule")
                .style(Modern::secondary_button())
                .padding([8, 20]),
            iced::widget::horizontal_space(),
            button("Cancel")
                .style(Modern::secondary_button())
                .padding([8, 20]),
            button("Save rule")
                .style(Modern::primary_button())
                .padding([8, 20]),
        ]
        .spacing(10)
        .width(Length::Fill); */

        // Filter items based on conditions
        let filtered_items: Vec<&Item> = items
            .values()
            .filter(|item| self.applies_to_item(
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
            .collect();

        // Items preview section
        let items_section = container(
            column![
                row![
                    text("Matching Items").style(Modern::primary_text()).size(16),
                    iced::widget::horizontal_space(),
                    text(format!("{} items", filtered_items.len())).style(Modern::secondary_text()),
                ],
                // Header row
                row![
                    text("Name").width(Length::Fixed(200.0)),
                    text("Item Group").width(Length::Fixed(150.0)),
                    text("Price").width(Length::Fixed(100.0)),
                    text("Status").width(Length::Fixed(100.0)),
                ]
                .padding(5),
                
                // Items list
                scrollable(
                    column(
                        filtered_items
                            .iter()
                            //.take(50) // Limit to first 50 items for performance
                            .map(|item| {
                                let item_group_name = item.item_group
                                    .and_then(|id| item_groups.get(&id))
                                    .map(|ig| ig.name.as_str())
                                    .unwrap_or("None");
                                
                                let price_display = item.price_levels
                                    .as_ref()
                                    .and_then(|levels| levels.first())
                                    .and_then(|id| price_levels.get(id))
                                    .map(|pl| format!("${:.2}", pl.price))
                                    .unwrap_or("$0.00".to_string());

                                row![
                                    text(&item.name).width(Length::Fixed(200.0)),
                                    text(item_group_name).width(Length::Fixed(150.0)),
                                    text(price_display).width(Length::Fixed(100.0)),
                                ]
                                .padding(5)
                                .spacing(10)
                                .into()
                            })
                            .collect::<Vec<_>>()
                    )
                    .spacing(2)
                )
                .height(Length::Fill),
            ]
            .spacing(10)
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
            //action_buttons,
        ]
        .spacing(0)
        .width(Length::Fill)
        .max_width(800);

        container(content)
            .center_x(Length::Fill)
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
                .style(Modern::inline_text_input()) // You might need to add this style
                .width(150)
        };

/*         let temp = String::new();
        let value_input = match condition.operator {
            FilterOperator::IsEmpty | FilterOperator::IsNotEmpty => {
                text_input(
                    "",
                    &temp
                ).style(Modern::inline_text_input())
                .width(150)
            }
            _ => {
                text_input(
                    "Value",
                    &condition.value
                ).style(Modern::inline_text_input())
                .on_input(move |value| Message::UpdateConditionValue(index, value))
                .width(150)
            }
        }; */

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

    fn applies_to_item(
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
        for condition in &self.conditions {
        let matches_condition = match condition.field {
            FilterCategory::Name => {
                match condition.operator {
                    FilterOperator::Contains => item.name.to_lowercase().contains(&condition.value.to_lowercase()),
                    FilterOperator::BeginsWith => item.name.to_lowercase().starts_with(&condition.value.to_lowercase()),
                    FilterOperator::EndsWith => item.name.to_lowercase().ends_with(&condition.value.to_lowercase()),
                    FilterOperator::Equals => item.name.to_lowercase() == condition.value.to_lowercase(),
                    FilterOperator::NotEquals => item.name.to_lowercase() != condition.value.to_lowercase(),
                    FilterOperator::IsEmpty => item.name.is_empty(),
                    FilterOperator::IsNotEmpty => !item.name.is_empty(),
                    _ => false,
                }
            }
            FilterCategory::ItemGroup => {
                if let Some(ig_id) = item.item_group {
                    if let Some(ig) = item_groups.get(&ig_id) {
                        match condition.operator {
                            FilterOperator::Contains => ig.name.to_lowercase().contains(&condition.value.to_lowercase()),
                            FilterOperator::BeginsWith => ig.name.to_lowercase().starts_with(&condition.value.to_lowercase()),
                            FilterOperator::EndsWith => ig.name.to_lowercase().ends_with(&condition.value.to_lowercase()),
                            FilterOperator::Equals => ig.name.to_lowercase() == condition.value.to_lowercase(),
                            FilterOperator::NotEquals => ig.name.to_lowercase() != condition.value.to_lowercase(),
                            FilterOperator::IsEmpty => false, // Has a value, so not empty
                            FilterOperator::IsNotEmpty => true, // Has a value
                            _ => false,
                        }
                    } else { 
                        condition.operator == FilterOperator::IsEmpty 
                    }
                } else { 
                    condition.operator == FilterOperator::IsEmpty 
                }
            }
            FilterCategory::TaxGroup => {
                if let Some(tg_id) = item.tax_group {
                    if let Some(tg) = tax_groups.get(&tg_id) {
                        match condition.operator {
                            FilterOperator::Contains => tg.name.to_lowercase().contains(&condition.value.to_lowercase()),
                            FilterOperator::BeginsWith => tg.name.to_lowercase().starts_with(&condition.value.to_lowercase()),
                            FilterOperator::EndsWith => tg.name.to_lowercase().ends_with(&condition.value.to_lowercase()),
                            FilterOperator::Equals => tg.name.to_lowercase() == condition.value.to_lowercase(),
                            FilterOperator::NotEquals => tg.name.to_lowercase() != condition.value.to_lowercase(),
                            FilterOperator::IsEmpty => false,
                            FilterOperator::IsNotEmpty => true,
                            _ => false,
                        }
                    } else { 
                        condition.operator == FilterOperator::IsEmpty 
                    }
                } else { 
                    condition.operator == FilterOperator::IsEmpty 
                }
            }
            FilterCategory::SecurityLevel => {
                if let Some(sl_id) = item.security_level {
                    if let Some(sl) = security_levels.get(&sl_id) {
                        match condition.operator {
                            FilterOperator::Contains => sl.name.to_lowercase().contains(&condition.value.to_lowercase()),
                            FilterOperator::BeginsWith => sl.name.to_lowercase().starts_with(&condition.value.to_lowercase()),
                            FilterOperator::EndsWith => sl.name.to_lowercase().ends_with(&condition.value.to_lowercase()),
                            FilterOperator::Equals => sl.name.to_lowercase() == condition.value.to_lowercase(),
                            FilterOperator::NotEquals => sl.name.to_lowercase() != condition.value.to_lowercase(),
                            FilterOperator::IsEmpty => false,
                            FilterOperator::IsNotEmpty => true,
                            _ => false,
                        }
                    } else { 
                        condition.operator == FilterOperator::IsEmpty 
                    }
                } else { 
                    condition.operator == FilterOperator::IsEmpty 
                }
            }
            FilterCategory::RevenueCategory => {
                if let Some(rc_id) = item.revenue_category {
                    if let Some(rc) = revenue_categories.get(&rc_id) {
                        match condition.operator {
                            FilterOperator::Contains => rc.name.to_lowercase().contains(&condition.value.to_lowercase()),
                            FilterOperator::BeginsWith => rc.name.to_lowercase().starts_with(&condition.value.to_lowercase()),
                            FilterOperator::EndsWith => rc.name.to_lowercase().ends_with(&condition.value.to_lowercase()),
                            FilterOperator::Equals => rc.name.to_lowercase() == condition.value.to_lowercase(),
                            FilterOperator::NotEquals => rc.name.to_lowercase() != condition.value.to_lowercase(),
                            FilterOperator::IsEmpty => false,
                            FilterOperator::IsNotEmpty => true,
                            _ => false,
                        }
                    } else { 
                        condition.operator == FilterOperator::IsEmpty 
                    }
                } else { 
                    condition.operator == FilterOperator::IsEmpty 
                }
            }
            FilterCategory::ReportCategory => {
                if let Some(rc_id) = item.report_category {
                    if let Some(rc) = report_categories.get(&rc_id) {
                        match condition.operator {
                            FilterOperator::Contains => rc.name.to_lowercase().contains(&condition.value.to_lowercase()),
                            FilterOperator::BeginsWith => rc.name.to_lowercase().starts_with(&condition.value.to_lowercase()),
                            FilterOperator::EndsWith => rc.name.to_lowercase().ends_with(&condition.value.to_lowercase()),
                            FilterOperator::Equals => rc.name.to_lowercase() == condition.value.to_lowercase(),
                            FilterOperator::NotEquals => rc.name.to_lowercase() != condition.value.to_lowercase(),
                            FilterOperator::IsEmpty => false,
                            FilterOperator::IsNotEmpty => true,
                            _ => false,
                        }
                    } else { 
                        condition.operator == FilterOperator::IsEmpty 
                    }
                } else { 
                    condition.operator == FilterOperator::IsEmpty 
                }
            }
            FilterCategory::ProductClass => {
                if let Some(pc_id) = item.product_class {
                    if let Some(pc) = product_classes.get(&pc_id) {
                        match condition.operator {
                            FilterOperator::Contains => pc.name.to_lowercase().contains(&condition.value.to_lowercase()),
                            FilterOperator::BeginsWith => pc.name.to_lowercase().starts_with(&condition.value.to_lowercase()),
                            FilterOperator::EndsWith => pc.name.to_lowercase().ends_with(&condition.value.to_lowercase()),
                            FilterOperator::Equals => pc.name.to_lowercase() == condition.value.to_lowercase(),
                            FilterOperator::NotEquals => pc.name.to_lowercase() != condition.value.to_lowercase(),
                            FilterOperator::IsEmpty => false,
                            FilterOperator::IsNotEmpty => true,
                            _ => false,
                        }
                    } else { 
                        condition.operator == FilterOperator::IsEmpty 
                    }
                } else { 
                    condition.operator == FilterOperator::IsEmpty 
                }
            }
            FilterCategory::ChoiceGroup => {
                if let Some(cg_ids) = &item.choice_groups {
                    if cg_ids.is_empty() {
                        condition.operator == FilterOperator::IsEmpty
                    } else {
                        match condition.operator {
                            FilterOperator::IsEmpty => false,
                            FilterOperator::IsNotEmpty => true,
                            _ => {
                                // Check if any choice group matches the condition
                                cg_ids.iter().any(|cg_id| {
                                    if let Some(cg) = choice_groups.get(&cg_id.0) {
                                        match condition.operator {
                                            FilterOperator::Contains => cg.name.to_lowercase().contains(&condition.value.to_lowercase()),
                                            FilterOperator::BeginsWith => cg.name.to_lowercase().starts_with(&condition.value.to_lowercase()),
                                            FilterOperator::EndsWith => cg.name.to_lowercase().ends_with(&condition.value.to_lowercase()),
                                            FilterOperator::Equals => cg.name.to_lowercase() == condition.value.to_lowercase(),
                                            FilterOperator::NotEquals => cg.name.to_lowercase() != condition.value.to_lowercase(),
                                            _ => false,
                                        }
                                    } else {
                                        false
                                    }
                                })
                            }
                        }
                    }
                } else {
                    condition.operator == FilterOperator::IsEmpty
                }
            }
            FilterCategory::PrinterLogical => {
                if let Some(pl_ids) = &item.printer_logicals {
                    if pl_ids.is_empty() {
                        condition.operator == FilterOperator::IsEmpty
                    } else {
                        match condition.operator {
                            FilterOperator::IsEmpty => false,
                            FilterOperator::IsNotEmpty => true,
                            _ => {
                                // Check if any printer logical matches the condition
                                pl_ids.iter().any(|(pl_id, _)| {
                                    if let Some(pl) = printer_logicals.get(pl_id) {
                                        match condition.operator {
                                            FilterOperator::Contains => pl.name.to_lowercase().contains(&condition.value.to_lowercase()),
                                            FilterOperator::BeginsWith => pl.name.to_lowercase().starts_with(&condition.value.to_lowercase()),
                                            FilterOperator::EndsWith => pl.name.to_lowercase().ends_with(&condition.value.to_lowercase()),
                                            FilterOperator::Equals => pl.name.to_lowercase() == condition.value.to_lowercase(),
                                            FilterOperator::NotEquals => pl.name.to_lowercase() != condition.value.to_lowercase(),
                                            _ => false,
                                        }
                                    } else {
                                        false
                                    }
                                })
                            }
                        }
                    }
                } else {
                    condition.operator == FilterOperator::IsEmpty
                }
            }
            FilterCategory::PriceLevel => {
                if let Some(pl_ids) = &item.price_levels {
                    if pl_ids.is_empty() {
                        condition.operator == FilterOperator::IsEmpty
                    } else {
                        match condition.operator {
                            FilterOperator::IsEmpty => false,
                            FilterOperator::IsNotEmpty => true,
                            _ => {
                                // Check if any price level matches the condition
                                pl_ids.iter().any(|pl_id| {
                                    if let Some(pl) = price_levels.get(pl_id) {
                                        match condition.operator {
                                            FilterOperator::Contains => pl.name.to_lowercase().contains(&condition.value.to_lowercase()),
                                            FilterOperator::BeginsWith => pl.name.to_lowercase().starts_with(&condition.value.to_lowercase()),
                                            FilterOperator::EndsWith => pl.name.to_lowercase().ends_with(&condition.value.to_lowercase()),
                                            FilterOperator::Equals => pl.name.to_lowercase() == condition.value.to_lowercase(),
                                            FilterOperator::NotEquals => pl.name.to_lowercase() != condition.value.to_lowercase(),
                                            _ => false,
                                        }
                                    } else {
                                        false
                                    }
                                })
                            }
                        }
                    }
                } else {
                    condition.operator == FilterOperator::IsEmpty
                }
            }
            FilterCategory::Price => {
                if let Some(price_levels_vec) = &item.price_levels {
                    if let Some(first_price_id) = price_levels_vec.first() {
                        if let Some(price_level) = price_levels.get(first_price_id) {
                            if let Ok(condition_value) = condition.value.parse::<Decimal>() {
                                let price_value = price_level.price;
                                match condition.operator {
                                    FilterOperator::GreaterThan => price_value > condition_value,
                                    FilterOperator::LessThan => price_value < condition_value,
                                    FilterOperator::GreaterOrEqual => price_value >= condition_value,
                                    FilterOperator::LessOrEqual => price_value <= condition_value,
                                    FilterOperator::Equals => (price_value - condition_value).abs() < Decimal::new(1, 2),
                                    FilterOperator::NotEquals => (price_value - condition_value).abs() >= Decimal::new(1, 2),
                                    FilterOperator::IsEmpty => false, // Has a price
                                    FilterOperator::IsNotEmpty => true, // Has a price
                                    _ => false,
                                }
                            } else { 
                                // If condition value can't be parsed as number
                                match condition.operator {
                                    FilterOperator::IsEmpty => false,
                                    FilterOperator::IsNotEmpty => true,
                                    _ => false,
                                }
                            }
                        } else { 
                            condition.operator == FilterOperator::IsEmpty 
                        }
                    } else { 
                        condition.operator == FilterOperator::IsEmpty 
                    }
                } else { 
                    condition.operator == FilterOperator::IsEmpty 
                }
            }
        };

        // Handle And/Or logic
        // For simplicity, let's assume all conditions must be met (AND logic)
        // TODO: Implement proper And/Or logic based on condition.logic
        if !matches_condition {
            return false;
        }
    }
    true
    }
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


