use std::ops::Range;
use rust_decimal::Decimal;

// Custom type for IDs to make it easier to change the underlying type if needed
pub type EntityId = i32;

// Custom type for currency values
pub type Currency = Decimal;

// Common validation error type for all modules
#[derive(Debug, Clone)]
pub enum ValidationError {
    InvalidId(String),
    DuplicateId(String),
    EmptyName(String),
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
            ValidationError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
            ValidationError::InvalidReference(msg) => write!(f, "Invalid refernce: {}", msg),
            ValidationError::InvalidRate(msg) => write!(f, "Invalid rate: {}", msg),
            ValidationError::InvalidPrice(msg)=> write!(f, "Invalid Price: {}", msg),
            ValidationError::MissingItemGroup(msg)=> write!(f, "Missing Item Group: {}", msg),
            ValidationError::MissingRevenueCategory(msg)=> write!(f, "Missing Revenue Group: {}", msg),
        }
    }
}

// Common export error type
#[derive(Debug)]
pub enum ExportError {
    InvalidFormat(String),
    InvalidValue(String),
    IoError(std::io::Error),
}

// Enum for validation ranges
#[derive(Debug, Clone)]
pub enum IdRange {
    Item(Range<EntityId>),
    PriceLevel(Range<EntityId>),
    StorePriceLevel(Range<EntityId>),
    TaxGroup(Range<EntityId>),        // 1-99
    SecurityLevel(Range<EntityId>),    // 0-9
    RevenueCategory(Range<EntityId>),  // 1-99
    ReportCategory(Range<EntityId>),   // 1-255
    ItemGroup(Range<EntityId>),        // User defined
    ProductClass(Range<EntityId>),     // 1-999
    ChoiceGroup(Range<EntityId>),      // 1-9999
    PrinterLogical(Range<EntityId>),   // 0-25
}

impl IdRange {
    pub fn is_valid(&self, id: EntityId) -> bool {
        match self {
            IdRange::TaxGroup(_) => (1..=99).contains(&id),
            IdRange::SecurityLevel(_) => (0..=9).contains(&id),
            IdRange::RevenueCategory(_) => (1..=99).contains(&id),
            IdRange::ReportCategory(_) => (1..=255).contains(&id),
            IdRange::ProductClass(_) => (1..=999).contains(&id),
            IdRange::ChoiceGroup(_) => (1..=9999).contains(&id),
            IdRange::PrinterLogical(_) => (0..=25).contains(&id),
            IdRange::PriceLevel(_) => (1..=999).contains(&id),
            IdRange::StorePriceLevel(_) => (1..=99999).contains(&id),
            IdRange::ItemGroup(range) => range.contains(&id),
            IdRange::Item(range) => range.contains(&id),
        }
    }
}

// Validation trait
pub trait Validatable {
    fn validate(&self) -> Result<(), ValidationError>;
}

// Common price level type used across modules
#[derive(Debug, Clone, PartialEq)]
pub enum PriceLevelType {
    Item,     // Valid range: 1-999
    Store,    // Valid range: 1-99999
}