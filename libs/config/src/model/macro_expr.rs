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
    /// Create a new MacroExpression from name string, value strings and a span
    pub fn new(name: String, args: Vec<String>, span: Range<usize>) -> Self {
        let name_len = name.len();
        let name_span = span.start..span.start + name_len;
        
        Self {
            name: Str { 
                value: name, 
                span: name_span.clone() 
            },
            args: args.into_iter()
                .enumerate()
                .map(|(i, value)| {
                    // Create approximate spans for args - not precise but functional
                    let arg_start = name_span.end + 1 + i;
                    let arg_end = arg_start + value.len();
                    Str { 
                        value, 
                        span: arg_start..arg_end 
                    }
                })
                .collect(),
            span
        }
    }
    
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
    
    /// Convert the macro to a string representation
    pub fn to_string(&self) -> String {
        if self.args.is_empty() {
            self.name.value().to_string()
        } else {
            format!("{}({})",
                self.name.value(),
                self.args.iter()
                    .map(|a| a.value())
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
    }
} 