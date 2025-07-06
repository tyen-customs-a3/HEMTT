use std::ops::Range;
use crate::Str;

/// A macro expression with name and arguments
#[derive(Debug, Clone, PartialEq)]
pub struct MacroExpression {
    /// The name of the macro (e.g., "QUOTE", "FUNC", etc.)
    name: Str,
    /// The arguments passed to the macro
    args: Vec<Str>,
    /// The span of the entire macro expression
    span: Range<usize>
}

impl MacroExpression {
    /// Create a new MacroExpression with pre-calculated spans
    /// 
    /// # Arguments
    /// * `name` - The macro name as a Str with proper span
    /// * `args` - Vector of arguments as Str with proper spans
    /// * `span` - The span of the entire macro expression
    /// 
    /// # Errors
    /// Returns an error if the macro name is invalid
    pub fn new(name: Str, args: Vec<Str>, span: Range<usize>) -> Result<Self, String> {
        // Validate macro name
        Self::validate_macro_name(&name.value)?;
        
        Ok(Self {
            name,
            args,
            span
        })
    }
    
    /// Create a new MacroExpression from strings (legacy compatibility)
    /// This method should be used only when proper spans are not available
    pub fn from_strings(name: String, args: Vec<String>, span: Range<usize>) -> Result<Self, String> {
        // Validate macro name
        Self::validate_macro_name(&name)?;
        
        // Create approximate spans for backward compatibility
        let name_len = name.len();
        let name_span = span.start..span.start + name_len;
        
        let name_str = Str { 
            value: name, 
            span: name_span.clone() 
        };
        
        let args_str = args.into_iter()
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
            .collect();
            
        Ok(Self {
            name: name_str,
            args: args_str,
            span
        })
    }
    
    /// Validate that a macro name follows the expected format
    fn validate_macro_name(name: &str) -> Result<(), String> {
        if name.is_empty() {
            return Err("Macro name cannot be empty".to_string());
        }
        
        let first_char = name.chars().next().unwrap();
        if !first_char.is_ascii_alphabetic() && first_char != '_' {
            return Err(format!("Macro name '{}' must start with a letter or underscore", name));
        }
        
        for ch in name.chars() {
            if !ch.is_ascii_alphanumeric() && ch != '_' {
                return Err(format!("Macro name '{}' contains invalid character '{}'", name, ch));
            }
        }
        
        Ok(())
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
            let mut result = String::with_capacity(
                self.name.value().len() + 
                self.args.iter().map(|a| a.value().len() + 1).sum::<usize>() + 2
            );
            result.push_str(self.name.value());
            result.push('(');
            
            for (i, arg) in self.args.iter().enumerate() {
                if i > 0 {
                    result.push(',');
                }
                result.push_str(arg.value());
            }
            
            result.push(')');
            result
        }
    }
    
    /// Convert this macro expression to an Ident
    pub fn to_ident(&self) -> crate::Ident {
        crate::Ident {
            value: self.to_string(),
            span: self.span.clone(),
        }
    }
}

impl std::fmt::Display for MacroExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}