use std::ops::Range;

use crate::{Number, Str};

#[derive(Debug, Clone, PartialEq)]
/// An array of entries
pub struct Array {
    pub expand: bool,
    pub items: Vec<Item>,
    pub span: Range<usize>,
}

impl Array {
    /// Get a reference to the array items
    pub fn items(&self) -> &[Item] {
        &self.items
    }
}

#[derive(Debug, Clone, PartialEq)]
/// An array value
pub enum Item {
    /// A string value
    Str(Str),
    /// A number value
    Number(Number),
    /// An array value
    Array(Vec<Item>),
    /// A macro expression like LIST_2("item")
    Macro((Str, Str, Range<usize>)),
    /// An invalid value
    Invalid(Range<usize>),
}
