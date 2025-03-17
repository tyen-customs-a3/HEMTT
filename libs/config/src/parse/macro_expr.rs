use chumsky::prelude::*;
use std::ops::Range;

use crate::{Str, MacroExpression};

/// Parse a macro name that can include numbers (e.g., LIST_2, LIST_10)
pub fn macro_name() -> impl Parser<char, String, Error = Simple<char>> {
    let ident_char = filter(|c: &char| c.is_ascii_alphabetic() || *c == '_');
    let ident_rest = filter(|c: &char| c.is_ascii_alphanumeric() || *c == '_');
    
    ident_char
        .then(ident_rest.repeated())
        .map(|(first, rest)| {
            let mut name = first.to_string();
            name.extend(rest);
            name
        })
}

/// Parse a macro expression into its components
pub fn macro_expr() -> impl Parser<char, MacroExpression, Error = Simple<char>> {
    // Parse macro arguments recursively to handle nested macros
    let arg = recursive(|arg| {
        let raw_text = filter(|c: &char| {
            // Allow most characters except those that would break the macro syntax
            !matches!(*c, ',' | '(' | ')' | '"' | ';' | '{' | '}' | '[' | ']')
        })
        .repeated()
        .collect::<String>();
            
        let nested_macro = macro_call(arg.clone());
            
        let quoted_string = just('"')
            .ignore_then(
                filter(|c: &char| *c != '"')
                    .repeated()
                    .collect::<String>()
            )
            .then_ignore(just('"'))
            .map(|s| format!("\"{}\"", s));
            
        choice((
            quoted_string.map(|s| Str {
                value: s,
                span: 0..0 // Updated by parent parser
            }),
            nested_macro.map(|m| Str {
                value: format!("{}({})", 
                    m.name.value,
                    m.args.iter()
                        .map(|a| a.value.clone())
                        .collect::<Vec<_>>()
                        .join(",")
                ),
                span: m.span
            }),
            raw_text.map(|s| Str {
                value: s.trim().to_string(),
                span: 0..0
            })
        ))
    });

    // Handle both standalone macros and macro calls
    macro_call(arg.or_not().map(|opt| opt.unwrap_or_else(|| Str {
        value: String::new(),
        span: 0..0
    })))
}

/// Parse a macro call with its arguments
fn macro_call(arg_parser: impl Parser<char, Str, Error = Simple<char>> + Clone) -> impl Parser<char, MacroExpression, Error = Simple<char>> {
    macro_name()
        .then(
            arg_parser
                .separated_by(just(',').padded())
                .allow_trailing()
                .delimited_by(just('('), just(')'))
        )
        .map_with_span(|(name, args), span| {
            MacroExpression {
                name: Str {
                    value: name.clone(),
                    span: span.start..span.start + name.len()
                },
                args,
                span
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_macro() {
        let result = macro_expr().parse("LIST_2(\"item\")").unwrap();
        assert_eq!(result.name.value, "LIST_2");
        assert_eq!(result.args.len(), 1);
        assert_eq!(result.args[0].value, r#""item""#);
    }

    #[test]
    fn test_list_macro_large_number() {
        let result = macro_expr().parse("LIST_123(item)").unwrap();
        assert_eq!(result.name.value, "LIST_123");
        assert_eq!(result.args.len(), 1);
        assert_eq!(result.args[0].value, "item");
    }

    #[test]
    fn test_path_with_backslashes() {
        let result = macro_expr().parse("QPATHTOF(\\x\\ace\\addons\\main\\data\\icon.paa)").unwrap();
        assert_eq!(result.name.value, "QPATHTOF");
        assert_eq!(result.args.len(), 1);
        assert_eq!(result.args[0].value, "\\x\\ace\\addons\\main\\data\\icon.paa");
    }

    #[test]
    fn test_nested_macro_with_backslashes() {
        let result = macro_expr().parse("QUOTE(PATHTOF(\\x\\ace\\main.sqf))").unwrap();
        assert_eq!(result.name.value, "QUOTE");
        assert_eq!(result.args.len(), 1);
        assert_eq!(result.args[0].value, "PATHTOF(\\x\\ace\\main.sqf)");
    }

    #[test]
    fn test_empty_args() {
        let result = macro_expr().parse("MACRO(,arg,)").unwrap();
        assert_eq!(result.name.value, "MACRO");
        assert_eq!(result.args.len(), 3);
        assert_eq!(result.args[0].value, "");
        assert_eq!(result.args[1].value, "arg");
        assert_eq!(result.args[2].value, "");
    }

    #[test]
    fn test_quoted_strings() {
        let result = macro_expr().parse("FUNC(\"hello\", \"world\")").unwrap();
        assert_eq!(result.name.value, "FUNC");
        assert_eq!(result.args.len(), 2);
        assert_eq!(result.args[0].value, r#""hello""#);
        assert_eq!(result.args[1].value, r#""world""#);
    }

    #[test]
    fn test_complex_nesting() {
        let result = macro_expr().parse("OUTER(INNER(NESTED(1,2),3),4)").unwrap();
        assert_eq!(result.name.value, "OUTER");
        assert_eq!(result.args.len(), 2);
        assert_eq!(result.args[0].value, "INNER(NESTED(1,2),3)");
        assert_eq!(result.args[1].value, "4");
    }

    #[test]
    fn test_ecstring_macro() {
        let result = macro_expr().parse("ECSTRING(common,ACETeam)").unwrap();
        assert_eq!(result.name.value, "ECSTRING");
        assert_eq!(result.args.len(), 2);
        assert_eq!(result.args[0].value, "common");
        assert_eq!(result.args[1].value, "ACETeam");
    }
} 