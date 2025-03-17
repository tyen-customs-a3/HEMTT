use std::ops::Range;

use crate::{Number, Str, MacroExpression};

#[derive(Debug, Clone, PartialEq)]
/// An array of entries
pub struct Array {
    pub expand: bool,
    pub items: Vec<Item>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
/// An array value
pub enum Item {
    /// A string value
    Str(Str),
    /// A number value
    Number(Number),
    /// A macro expression
    Macro(MacroExpression),
    /// An array value
    Array(Vec<Item>),
    /// An invalid value
    Invalid(Range<usize>),
}
