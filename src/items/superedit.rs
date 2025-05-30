use iced::{Element, Task, Length};
use iced::widget::{button, column, combo_box, container, horizontal_space, pick_list, row, text, text_input};
use iced_modern_theme::Modern;
use std::collections::BTreeMap;
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
    UpdateActionPriceLevel(usize, PriceLevel),
}

#[derive(Debug, Clone)]
struct SuperEdit {
    conditions: Vec<FilterCondition>,
    actions: Vec<FilterAction>,
}

#[derive(Debug, Clone)]
struct FilterAction {
    category: FilterCategory,
    operation: ActionOperation,
    value: String,
    price_level: Option<PriceLevel>, // For price-related operations
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
            FilterOperator::Equals => write!(f, "equals"),
            FilterOperator::NotEquals => write!(f, "does not equal"),
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

impl SuperEdit {

    fn new() -> (Self, Task<Message>) {
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

        (
            Self {
                conditions: vec![default_condition],
                actions: vec![default_action],
            },
            Task::none()
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::AddCondition => {
                let new_condition = FilterCondition {
                    logic: ConditionLogic::And,
                    field: FilterCategory::Name,
                    operator: FilterOperator::Contains,
                    value: String::new(),
                };
                self.conditions.push(new_condition);
            }
            Message::RemoveCondition(index) => {
                if self.conditions.len() > 1 && index < self.conditions.len() {
                    self.conditions.remove(index);
                }
            }
            Message::UpdateConditionLogic(index, logic) => {
                if let Some(condition) = self.conditions.get_mut(index) {
                    condition.logic = logic;
                }
            }
            Message::UpdateConditionField(index, field) => {
                if let Some(condition) = self.conditions.get_mut(index) {
                    condition.field = field;
                }
            }
            Message::UpdateConditionOperator(index, operator) => {
                if let Some(condition) = self.conditions.get_mut(index) {
                    condition.operator = operator;
                }
            }
            Message::UpdateConditionValue(index, value) => {
                if let Some(condition) = self.conditions.get_mut(index) {
                    condition.value = value;
                }
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
            }
            Message::RemoveAction(index) => {
                if self.actions.len() > 1 && index < self.actions.len() {
                    self.actions.remove(index);
                }
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
                        action.price_level = Some(PriceLevel::Level1);
                    } else {
                        action.price_level = None;
                    }
                }
            }
            Message::UpdateActionOperation(index, operation) => {
                if let Some(action) = self.actions.get_mut(index) {
                    action.operation = operation;
                }
            }
            Message::UpdateActionValue(index, value) => {
                if let Some(action) = self.actions.get_mut(index) {
                    action.value = value;
                }
            }
            Message::UpdateActionPriceLevel(index, price_level) => {
                if let Some(action) = self.actions.get_mut(index) {
                    action.price_level = Some(price_level);
                }
            }
            // Legacy messages - keeping for compatibility but not implementing
            _ => {}
        }
        Task::none()
    }

    pub fn view<'a>(
        &'a self,
        items: &'a BTreeMap<EntityId, Item>,
        item_search: &'a String,
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
                            self.render_action(index, action)
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
        let action_buttons = row![
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
        .width(Length::Fill);

        let content = column![
            header,
            iced::widget::vertical_space().height(20),
            if_section,
            iced::widget::vertical_space().height(10),
            then_section,
            iced::widget::vertical_space().height(20),
            action_buttons,
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
            &FilterOperator::ALL[..],
            Some(&condition.operator),
            move |operator| Message::UpdateConditionOperator(index, operator)
        ).style(Modern::pick_list())
        .width(120);

        let temp = String::new();
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
        .align_y(iced::Alignment::Center)
        .into()
    }

    fn render_action<'a>(&'a self, index: usize, action: &'a FilterAction) -> Element<'a, Message> {
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
                row![
                    text_input("Amount", &action.value)
                        .on_input(move |value| Message::UpdateActionValue(index, value))
                        .style(Modern::inline_text_input())
                        .width(100),
                    iced::widget::horizontal_space().width(5),
                    text("at").style(Modern::secondary_text()),
                    iced::widget::horizontal_space().width(5),
                    pick_list(
                        &PriceLevel::ALL[..],
                        action.price_level.as_ref(),
                        move |price_level| Message::UpdateActionPriceLevel(index, price_level)
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FilterCategory {
    Name,
    ItemGroup,
    ProductClass,
    TaxGroup,
    RevenueCategory,
    PriceLevel,
    ChoiceGroup,
    PrinterLogical,
    ReportCategory,
    Price,
}

impl FilterCategory {
    const ALL: [FilterCategory; 10] = [
        FilterCategory::Name,
        FilterCategory::ItemGroup,
        FilterCategory::ProductClass,
        FilterCategory::TaxGroup,
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
