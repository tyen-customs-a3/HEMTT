use std::ops::Range;
use crate::Str;

/// A macro expression with name and arguments
#[derive(Debug, Clone, PartialEq)]
pub struct MacroExpression {
    /// The name of the macro (e.g., "QUOTE", "FUNC", etc.)
    pub name: Str,
    /// The arguments passed to the macro
    pub args: Vec<Str>,
    /// The span of the entire macro expression
    pub span: Range<usize>
}

impl MacroExpression {
    /// Get a reference to the macro name
    pub fn name(&self) -> &Str {
        &self.name
    }

    /// Get a reference to the macro arguments
    pub fn args(&self) -> &[Str] {
        &self.args
    }

    /// Get the span of the macro expression
    pub fn span(&self) -> &Range<usize> {
        &self.span
    }
} 