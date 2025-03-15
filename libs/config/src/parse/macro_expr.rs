use chumsky::prelude::*;
use std::ops::Range;

use crate::Str;

/// Parse a macro expression into its name and arguments, returning with its span.
pub fn macro_expr() -> impl Parser<char, (Str, Vec<Str>, Range<usize>), Error = Simple<char>> {
    let ident = text::ident::<char, Simple<char>>();
    
    // Main parser for all macro patterns
    ident
        .then(
            super::str::string('"')
                .separated_by(just(',').padded())
                .delimited_by(just('('), just(')'))
                .recover_with(nested_delimiters(
                    '(',
                    ')',
                    [('[', ']'), ('{', '}')],
                    |_| vec![]
                ))
        )
        .map_with_span(|(name, args), span| {
            let name_str = Str {
                value: name.clone(),
                span: span.start..span.start + name.len(),
            };
            
            let args_strs = args.iter().enumerate().map(|(i, arg)| {
                // Calculate approximate spans for each argument
                let arg_str_len = arg.value.len();
                let prev_args_len: usize = if i == 0 { 0 } else {
                    args[0..i].iter().map(|s: &crate::Str| s.value.len() + 4).sum::<usize>() // +4 for quotes and comma
                };
                
                let arg_start = span.start + name.len() + 1 + prev_args_len + if i > 0 { i * 2 } else { 0 };
                let arg_end = arg_start + arg_str_len + 2; // +2 for quotes
                
                Str {
                    value: arg.value.clone(),
                    span: arg_start..arg_end,
                }
            }).collect();
            
            (name_str, args_strs, span)
        })
        .recover_with(nested_delimiters(
            '(',
            ')',
            [('[', ']'), ('{', '}')],
            |span: Range<usize>| {
                // Fallback for recovery
                (
                    Str {
                        value: "UNKNOWN".to_string(),
                        span: span.start..span.start + 7,
                    },
                    vec![Str {
                        value: "PARSE_ERROR".to_string(),
                        span: span.start + 8..span.end - 1,
                    }],
                    span
                )
            }
        ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_macro() {
        let result = macro_expr().parse("LIST_2(\"item\")").unwrap();
        assert_eq!(result.0.value, "LIST_2");
        assert_eq!(result.1.len(), 1);
        assert_eq!(result.1[0].value, "item");
        assert_eq!(result.2, 0..14);
    }

    #[test]
    fn list_macro_large_number() {
        let result = macro_expr().parse("LIST_123(\"item\")").unwrap();
        assert_eq!(result.0.value, "LIST_123");
        assert_eq!(result.1.len(), 1);
        assert_eq!(result.1[0].value, "item");
        assert_eq!(result.2, 0..16);
    }

    #[test]
    fn eval_macro() {
        let result = macro_expr().parse("EVAL(\"MyClass\", \"1 + 2\")").unwrap();
        assert_eq!(result.0.value, "EVAL");
        assert_eq!(result.1.len(), 2);
        assert_eq!(result.1[0].value, "MyClass");
        assert_eq!(result.1[1].value, "1 + 2");
        assert_eq!(result.2, 0..24);
    }

    #[test]
    fn generic_macro_single_arg() {
        let result = macro_expr().parse("GVAR(\"value\")").unwrap();
        assert_eq!(result.0.value, "GVAR");
        assert_eq!(result.1.len(), 1);
        assert_eq!(result.1[0].value, "value");
        assert_eq!(result.2, 0..13);
    }

    #[test]
    fn generic_macro_multiple_args() {
        let result = macro_expr().parse("ARR_3(\"one\", \"two\", \"three\")").unwrap();
        assert_eq!(result.0.value, "ARR_3");
        assert_eq!(result.1.len(), 3);
        assert_eq!(result.1[0].value, "one");
        assert_eq!(result.1[1].value, "two");
        assert_eq!(result.1[2].value, "three");
        assert_eq!(result.2, 0..28);
    }

    #[test]
    fn invalid_macro_recovers() {
        // This should recover and return a generic macro
        let result = macro_expr().parse_recovery("UNKNOWN(\"value\" error)");
        assert!(!result.1.is_empty()); // We do expect errors
        // We don't assert anything specific about result.0 since recovery behavior may vary
        // But we do expect the parser to handle the error gracefully without crashing
    }

    #[test]
    fn invalid_syntax_recovers() {
        // Missing closing parenthesis should recover
        let result = macro_expr().parse_recovery("GVAR(\"test\"");
        assert!(!result.1.is_empty()); // We expect errors
        // We don't assert anything about result.0 since recovery behavior may vary
        // But we do expect the parser to handle the error gracefully without crashing
    }
} 