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

/// Parse a token concatenation operator (##) and the token that follows it
pub fn token_concat() -> impl Parser<char, String, Error = Simple<char>> {
    just("##")
        .ignore_then(
            filter(|c: &char| c.is_ascii_alphanumeric() || *c == '_')
                .repeated()
                .at_least(1)
                .collect::<String>()
        )
}

/// Parse a macro argument, which can be a nested macro, string, raw text, or token concatenation
pub fn macro_arg() -> impl Parser<char, Value, Error = Simple<char>> {
    recursive(|arg: Recursive<'_, char, Value, Simple<char>>| {
        let quoted_string = just('"')
            .ignore_then(filter(|c: &char| *c != '"').repeated())
            .then_ignore(just('"'))
            .collect::<String>()
            .map(|s| format!("\"{}\"", s))
            .map_with_span(|s, span| Value::Str(Str { value: s, span }));

        // Handle token concatenation with ## operator, treating it as part of raw text
        let token_concatenation = token_concat()
            .map(|token| format!("##{}##", token));

        let raw_text = choice((
            token_concatenation,
            filter(|c: &char| !matches!(*c, ',' | '(' | ')' | '"' | '#'))
                .repeated()
                .collect::<String>()
        ))
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
                // Convert args to strings
                let arg_strings = args.into_iter()
                    .map(|v| match v {
                        Value::Str(s) => s.value().to_string(),
                        Value::Macro(m) => m.to_string(),
                        _ => String::new()
                    })
                    .collect();
                
                Value::Macro(MacroExpression::new(name.clone(), arg_strings, span))
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
    let with_args = macro_name()
        .then(
            macro_arg()
                .separated_by(just(',').padded())
                .allow_trailing()
                .delimited_by(just('('), just(')'))
        )
        .map_with_span(|(name, args), span| {
            // Convert args to strings
            let arg_strings = args.into_iter()
                .map(|v| match v {
                    Value::Str(s) => s.value().to_string(),
                    Value::Macro(m) => m.to_string(),
                    _ => String::new()
                })
                .collect();
            
            MacroExpression::new(name, arg_strings, span)
        });

    // For macros without parentheses at all (e.g. WEAPON_FIRE_BEGIN)
    let no_args = macro_name()
        .map_with_span(|name, span| {
            MacroExpression::new(name, vec![], span)
        })
        .then_ignore(end());

    // Try parsing with arguments first, fall back to no-args only if there are no parentheses
    with_args.or(no_args)
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
    fn test_no_args_macro() {
        let result = macro_expr().parse("WEAPON_FIRE_BEGIN").unwrap();
        let parsed = get_macro_expr(result);
        assert_eq!(parsed.name.value, "WEAPON_FIRE_BEGIN");
        assert_eq!(parsed.args.len(), 0);

        let result = macro_expr().parse("WEAPON_FIRE_END").unwrap();
        let parsed = get_macro_expr(result);
        assert_eq!(parsed.name.value, "WEAPON_FIRE_END");
        assert_eq!(parsed.args.len(), 0);
        
        // Also test with empty parentheses
        let result = macro_expr().parse("WEAPON_FIRE_BEGIN()").unwrap();
        let parsed = get_macro_expr(result);
        assert_eq!(parsed.name.value, "WEAPON_FIRE_BEGIN");
        assert_eq!(parsed.args.len(), 1);
        assert_eq!(parsed.args[0].value, "");
        
        let result = macro_expr().parse("WEAPON_FIRE_END()").unwrap();
        let parsed = get_macro_expr(result);
        assert_eq!(parsed.name.value, "WEAPON_FIRE_END");
        assert_eq!(parsed.args.len(), 1);
        assert_eq!(parsed.args[0].value, "");
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