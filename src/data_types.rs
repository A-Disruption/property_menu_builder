use std::ops::Range;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

// Custom type for IDs to make it easier to change the underlying type if needed
pub type EntityId = i32;

// Custom type for currency values
pub type Currency = Decimal;

//Convert String to Decimal
pub fn string_to_decimal(input: &str) -> Result<Decimal, String> {
    Decimal::from_str(input)
        .map_err(|e| format!("Failed to convert '{}' to Decimal: {}", input, e))
}

//Struct to handle PriceLevel: Price pairs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ItemPrice {
    pub price_level_id: EntityId,
    pub price: Decimal,
}

// Common validation error type for all modules
#[derive(Debug, Clone)]
pub enum ValidationError {
    InvalidId(String),
    DuplicateId(String),
    EmptyName(String),
    NameTooLong(String),
    InvalidRange(String),
    RangeOverlap(String),  // Added this variant
    InvalidValue(String),
    InvalidReference(String),
    InvalidRate(String),
    InvalidPrice(String),
    MissingItemGroup(String),
    MissingRevenueCategory(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InvalidRange(msg) => write!(f, "Invalid range: {}", msg),
            ValidationError::RangeOverlap(msg) => write!(f, "Range overlap: {}", msg),
            ValidationError::InvalidId(msg) => write!(f, "Invalid ID: {}", msg),
            ValidationError::DuplicateId(msg) => write!(f, "Duplicate ID: {}", msg),
            ValidationError::EmptyName(msg) => write!(f, "Empty name: {}", msg),
            ValidationError::NameTooLong(msg) => write!(f, "Too Long: {}", msg),
            ValidationError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
            ValidationError::InvalidReference(msg) => write!(f, "Invalid refernce: {}", msg),
            ValidationError::InvalidRate(msg) => write!(f, "Invalid rate: {}", msg),
            ValidationError::InvalidPrice(msg)=> write!(f, "Invalid Price: {}", msg),
            ValidationError::MissingItemGroup(msg)=> write!(f, "Missing Item Group: {}", msg),
            ValidationError::MissingRevenueCategory(msg)=> write!(f, "Missing Revenue Group: {}", msg),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeletionInfo {
    pub entity_type: String,
    pub entity_id: EntityId,
    pub affected_items: Vec<String>,
}

impl DeletionInfo {
    pub fn new() -> Self {
        Self {
            entity_type: String::new(),
            entity_id: 1,
            affected_items: Vec::new(),
        }
    }
}