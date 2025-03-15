use std::ops::Range;

use crate::Str;

/// A #define preprocessor directive
#[derive(Debug, Clone, PartialEq)]
pub enum Define {
    /// A simple define directive with a value
    /// ```cpp
    /// #define NAME value
    /// ```
    Simple {
        /// The name of the define
        name: String,
        /// The value of the define
        value: String,
        /// The span of the define
        span: Range<usize>,
    },
    /// A macro define directive with parameters
    /// ```cpp
    /// #define NAME(var1, var2) expression
    /// ```
    Macro {
        /// The name of the macro
        name: String,
        /// The parameters of the macro
        params: Vec<String>,
        /// The body/expression of the macro
        body: String,
        /// The span of the macro
        span: Range<usize>,
    },
}

impl Define {
    /// Get the name of the define
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Simple { name, .. } | Self::Macro { name, .. } => name,
        }
    }

    /// Get the span of the define
    #[must_use]
    pub fn span(&self) -> Range<usize> {
        match self {
            Self::Simple { span, .. } | Self::Macro { span, .. } => span.clone(),
        }
    }
} 