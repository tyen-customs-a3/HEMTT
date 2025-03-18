use chumsky::prelude::*;
use std::ops::Range;

use crate::{Str, MacroExpression, Value};

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

/// Parse a macro argument, which can be a nested macro, string, or raw text
fn macro_arg() -> impl Parser<char, Value, Error = Simple<char>> {
    recursive(|arg: Recursive<'_, char, Value, Simple<char>>| {
        let quoted_string = just('"')
            .ignore_then(filter(|c: &char| *c != '"').repeated())
            .then_ignore(just('"'))
            .collect::<String>()
            .map(|s| format!("\"{}\"", s))
            .map_with_span(|s, span| Value::Str(Str { value: s, span }));

        let raw_text = filter(|c: &char| !matches!(*c, ',' | '(' | ')' | '"'))
            .repeated()
            .collect::<String>()
            .map_with_span(|s, span| Value::Str(Str {
                value: s.trim().to_string(),
                span
            }));

        // For nested macros, parse them as actual macro expressions
        let nested_macro = macro_name()
            .then(
                arg.boxed()
                    .separated_by(just(',').padded())
                    .allow_trailing()
                    .delimited_by(just('('), just(')'))
            )
            .map_with_span(|(name, args), span| {
                let name_len = name.len();
                Value::Macro(MacroExpression {
                    name: Str {
                        value: name.clone(),
                        span: span.start..span.start + name_len
                    },
                    args: args.into_iter()
                        .map(|v| match v {
                            Value::Str(s) => s,
                            Value::Macro(m) => Str {
                                value: format!("{}({})",
                                    m.name.value,
                                    m.args.iter()
                                        .map(|a| a.value.as_str())
                                        .collect::<Vec<_>>()
                                        .join(",")
                                ),
                                span: m.span
                            },
                            _ => Str {
                                value: String::new(),
                                span: span.clone()
                            }
                        })
                        .collect(),
                    span
                })
            });

        choice((
            quoted_string,
            nested_macro,
            raw_text
        ))
    })
}

/// Parse a macro call with its arguments
fn macro_call() -> impl Parser<char, MacroExpression, Error = Simple<char>> {
    macro_name()
        .then(
            macro_arg()
                .separated_by(just(',').padded())
                .allow_trailing()
                .delimited_by(just('('), just(')'))
        )
        .map_with_span(|(name, args), span| {
            let name_len = name.len();
            MacroExpression {
                name: Str {
                    value: name,
                    span: span.start..span.start + name_len
                },
                args: args.into_iter()
                    .map(|v| match v {
                        Value::Str(s) => s,
                        Value::Macro(m) => Str {
                            value: format!("{}({})",
                                m.name.value,
                                m.args.iter()
                                    .map(|a| a.value.as_str())
                                    .collect::<Vec<_>>()
                                    .join(",")
                            ),
                            span: m.span
                        },
                        _ => Str {
                            value: String::new(),
                            span: span.clone()
                        }
                    })
                    .collect(),
                span
            }
        })
}

/// Parse a macro expression into its components
pub fn macro_expr() -> impl Parser<char, Value, Error = Simple<char>> {
    macro_call()
        .map_with_span(|result, _span| Value::Macro(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_macro_expr(value: Value) -> MacroExpression {
        match value {
            Value::Macro(m) => m,
            other => panic!("Expected Value::Macro, got {:?}", other),
        }
    }

    #[test]
    fn test_list_macro() {
        let result = macro_expr().parse("LIST_2(\"item\")").unwrap();
        let macro_expr = get_macro_expr(result);
        assert_eq!(macro_expr.name.value, "LIST_2");
        assert_eq!(macro_expr.args.len(), 1);
        assert_eq!(macro_expr.args[0].value, r#""item""#);
    }

    #[test]
    fn test_list_macro_large_number() {
        let result = macro_expr().parse("LIST_123(item)").unwrap();
        let macro_expr = get_macro_expr(result);
        assert_eq!(macro_expr.name.value, "LIST_123");
        assert_eq!(macro_expr.args.len(), 1);
        assert_eq!(macro_expr.args[0].value, "item");
    }

    #[test]
    fn test_path_with_backslashes() {
        let result = macro_expr().parse("QPATHTOF(\\x\\ace\\addons\\main\\data\\icon.paa)").unwrap();
        let macro_expr = get_macro_expr(result);
        assert_eq!(macro_expr.name.value, "QPATHTOF");
        assert_eq!(macro_expr.args.len(), 1);
        assert_eq!(macro_expr.args[0].value, "\\x\\ace\\addons\\main\\data\\icon.paa");
    }

    #[test]
    fn test_nested_macro_with_backslashes() {
        let result = macro_expr().parse("QUOTE(PATHTOF(\\x\\ace\\main.sqf))").unwrap();
        let macro_expr = get_macro_expr(result);
        assert_eq!(macro_expr.name.value, "QUOTE");
        assert_eq!(macro_expr.args.len(), 1);
        assert_eq!(macro_expr.args[0].value, "PATHTOF(\\x\\ace\\main.sqf)");
    }

    #[test]
    fn test_empty_args() {
        let result = macro_expr().parse("MACRO(,arg,)").unwrap();
        let macro_expr = get_macro_expr(result);
        assert_eq!(macro_expr.name.value, "MACRO");
        assert_eq!(macro_expr.args.len(), 3);
        assert_eq!(macro_expr.args[0].value, "");
        assert_eq!(macro_expr.args[1].value, "arg");
        assert_eq!(macro_expr.args[2].value, "");
    }

    #[test]
    fn test_quoted_strings() {
        let result = macro_expr().parse("FUNC(\"hello\", \"world\")").unwrap();
        let macro_expr = get_macro_expr(result);
        assert_eq!(macro_expr.name.value, "FUNC");
        assert_eq!(macro_expr.args.len(), 2);
        assert_eq!(macro_expr.args[0].value, r#""hello""#);
        assert_eq!(macro_expr.args[1].value, r#""world""#);
    }

    #[test]
    fn test_complex_nesting() {
        let result = macro_expr().parse("OUTER(INNER(NESTED(1,2),3),4)").unwrap();
        let macro_expr = get_macro_expr(result);
        assert_eq!(macro_expr.name.value, "OUTER");
        assert_eq!(macro_expr.args.len(), 2);
        assert_eq!(macro_expr.args[0].value, "INNER(NESTED(1,2),3)");
        assert_eq!(macro_expr.args[1].value, "4");
    }

    #[test]
    fn test_ecstring_macro() {
        let result = macro_expr().parse("ECSTRING(common,ACETeam)").unwrap();
        let macro_expr = get_macro_expr(result);
        assert_eq!(macro_expr.name.value, "ECSTRING");
        assert_eq!(macro_expr.args.len(), 2);
        assert_eq!(macro_expr.args[0].value, "common");
        assert_eq!(macro_expr.args[1].value, "ACETeam");
    }

    #[test]
    fn test_invalid_macro() {
        let result = macro_expr().parse("INVALID_MACRO(unclosed");
        assert!(result.is_err());
    }

    #[test]
    fn test_recovery_after_invalid() {
        let result = macro_expr().parse("INVALID(unclosed; VALID(arg)");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_nested_macro() {
        let result = macro_expr().parse("OUTER(INNER(unclosed,4)");
        assert!(result.is_err());
    }
} 