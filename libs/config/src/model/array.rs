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
    /// A generic expression like LIST_2("item") or EVAL("class", "expression") or MACRO("name")
    Macro {
        /// The macro name
        name: Str,
        /// The arguments to the macro
        args: Vec<Str>,
        /// The span of the entire macro
        span: Range<usize>,
    },
    /// An invalid value
    Invalid(Range<usize>),
}
