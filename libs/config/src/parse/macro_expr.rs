use chumsky::prelude::*;

use crate::{Str, MacroExpression, Value};

// Maximum nesting depth to prevent stack overflow
const MAX_MACRO_DEPTH: usize = 32;
// Maximum number of arguments to prevent OOM
const MAX_MACRO_ARGS: usize = 256;

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
    macro_arg_with_depth(0)
}

/// Parse a macro argument with depth tracking to prevent stack overflow
fn macro_arg_with_depth(depth: usize) -> impl Parser<char, Value, Error = Simple<char>> {
    recursive(move |_| {
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

        // For nested macros, parse them as actual macro expressions with depth checking
        let nested_macro = if depth >= MAX_MACRO_DEPTH {
            // If we've exceeded max depth, treat as raw text to prevent stack overflow
            filter(|c: &char| !matches!(*c, ',' | '(' | ')' | '"'))
                .repeated()
                .collect::<String>()
                .map_with_span(|s, span| Value::Str(Str {
                    value: s.trim().to_string(),
                    span
                }))
                .boxed()
        } else {
            macro_name()
                .map_with_span(|name, name_span| (name, name_span))
                .then(
                    macro_arg_with_depth(depth + 1)
                        .separated_by(just(',').padded())
                        .allow_trailing()
                        .delimited_by(just('('), just(')'))
                )
                .try_map(|((name, name_span), args), span| {
                    // Check argument count to prevent OOM
                    if args.len() > MAX_MACRO_ARGS {
                        return Err(Simple::custom(span.clone(), format!("Too many macro arguments (max {})", MAX_MACRO_ARGS)));
                    }
                    
                    // Convert args to Str with proper spans, avoiding unnecessary allocations
                    let mut arg_strs = Vec::with_capacity(args.len());
                    for v in args {
                        let arg_str = match v {
                            Value::Str(s) => s,
                            Value::Macro(m) => {
                                // Use Cow to avoid unnecessary string allocation
                                let macro_str = m.to_string();
                                let macro_span = m.span().clone();
                                Str { value: macro_str, span: macro_span }
                            },
                            _ => {
                                // Fallback for other types
                                Str { value: String::new(), span: span.clone() }
                            }
                        };
                        arg_strs.push(arg_str);
                    }

                    let name_str = Str { value: name, span: name_span };
                    MacroExpression::new(name_str, arg_strs, span.clone())
                        .map(Value::Macro)
                        .map_err(|e| Simple::custom(span, e))
                })
                .boxed()
        };

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
        .map_with_span(|name, name_span| (name, name_span))
        .then(
            macro_arg()
                .separated_by(just(',').padded())
                .allow_trailing()
                .delimited_by(just('('), just(')'))
        )
        .try_map(|((name, name_span), args), span| {
            // Check bounds to prevent OOM
            if args.len() > MAX_MACRO_ARGS {
                return Err(Simple::custom(span.clone(), format!("Too many macro arguments (max {})", MAX_MACRO_ARGS)));
            }
            
            // Convert args to Str with proper spans
            let mut arg_strs = Vec::with_capacity(args.len());
            for v in args {
                let arg_str = match v {
                    Value::Str(s) => s,
                    Value::Macro(m) => {
                        // Create a Str from the macro's string representation
                        let macro_str = m.to_string();
                        let macro_span = m.span().clone();
                        Str { value: macro_str, span: macro_span }
                    },
                    _ => {
                        // Fallback for other types
                        Str { value: String::new(), span: span.clone() }
                    }
                };
                arg_strs.push(arg_str);
            }

            let name_str = Str { value: name, span: name_span };
            MacroExpression::new(name_str, arg_strs, span.clone())
                .map_err(|e| Simple::custom(span, e))
        });

    // For macros without parentheses at all (e.g. WEAPON_FIRE_BEGIN)
    let no_args = macro_name()
        .map_with_span(|name, span| {
            let name_str = Str { value: name.clone(), span: span.clone() };
            MacroExpression::new(name_str, vec![], span.clone())
                .unwrap_or_else(|_| {
                    // Fallback using from_strings
                    MacroExpression::from_strings(name, vec![], span).unwrap()
                })
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
        assert_eq!(macro_expr.name().value, "LIST_2");
        assert_eq!(macro_expr.args().len(), 1);
        assert_eq!(macro_expr.args()[0].value, r#""item""#);
    }

    #[test]
    fn test_list_macro_large_number() {
        let result = macro_expr().parse("LIST_123(item)").unwrap();
        let macro_expr = get_macro_expr(result);
        assert_eq!(macro_expr.name().value, "LIST_123");
        assert_eq!(macro_expr.args().len(), 1);
        assert_eq!(macro_expr.args()[0].value, "item");
    }

    #[test]
    fn test_path_with_backslashes() {
        let result = macro_expr().parse("QPATHTOF(\\x\\ace\\addons\\main\\data\\icon.paa)").unwrap();
        let macro_expr = get_macro_expr(result);
        assert_eq!(macro_expr.name().value, "QPATHTOF");
        assert_eq!(macro_expr.args().len(), 1);
        assert_eq!(macro_expr.args()[0].value, "\\x\\ace\\addons\\main\\data\\icon.paa");
    }

    #[test]
    fn test_nested_macro_with_backslashes() {
        let result = macro_expr().parse("QUOTE(PATHTOF(\\x\\ace\\main.sqf))").unwrap();
        let macro_expr = get_macro_expr(result);
        assert_eq!(macro_expr.name().value, "QUOTE");
        assert_eq!(macro_expr.args().len(), 1);
        assert_eq!(macro_expr.args()[0].value, "PATHTOF(\\x\\ace\\main.sqf)");
    }

    #[test]
    fn test_empty_args() {
        let result = macro_expr().parse("MACRO(,arg,)").unwrap();
        let macro_expr = get_macro_expr(result);
        assert_eq!(macro_expr.name().value, "MACRO");
        assert_eq!(macro_expr.args().len(), 3);
        assert_eq!(macro_expr.args()[0].value, "");
        assert_eq!(macro_expr.args()[1].value, "arg");
        assert_eq!(macro_expr.args()[2].value, "");
    }

    #[test]
    fn test_quoted_strings() {
        let result = macro_expr().parse("FUNC(\"hello\", \"world\")").unwrap();
        let macro_expr = get_macro_expr(result);
        assert_eq!(macro_expr.name().value, "FUNC");
        assert_eq!(macro_expr.args().len(), 2);
        assert_eq!(macro_expr.args()[0].value, r#""hello""#);
        assert_eq!(macro_expr.args()[1].value, r#""world""#);
    }

    #[test]
    fn test_complex_nesting() {
        let result = macro_expr().parse("OUTER(INNER(NESTED(1,2),3),4)").unwrap();
        let macro_expr = get_macro_expr(result);
        assert_eq!(macro_expr.name().value, "OUTER");
        assert_eq!(macro_expr.args().len(), 2);
        assert_eq!(macro_expr.args()[0].value, "INNER(NESTED(1,2),3)");
        assert_eq!(macro_expr.args()[1].value, "4");
    }

    #[test]
    fn test_ecstring_macro() {
        let result = macro_expr().parse("ECSTRING(common,ACETeam)").unwrap();
        let macro_expr = get_macro_expr(result);
        assert_eq!(macro_expr.name().value, "ECSTRING");
        assert_eq!(macro_expr.args().len(), 2);
        assert_eq!(macro_expr.args()[0].value, "common");
        assert_eq!(macro_expr.args()[1].value, "ACETeam");
    }

    #[test]
    fn test_no_args_macro() {
        let result = macro_expr().parse("WEAPON_FIRE_BEGIN").unwrap();
        let parsed = get_macro_expr(result);
        assert_eq!(parsed.name().value, "WEAPON_FIRE_BEGIN");
        assert_eq!(parsed.args().len(), 0);

        let result = macro_expr().parse("WEAPON_FIRE_END").unwrap();
        let parsed = get_macro_expr(result);
        assert_eq!(parsed.name().value, "WEAPON_FIRE_END");
        assert_eq!(parsed.args().len(), 0);

        // Also test with empty parentheses
        let result = macro_expr().parse("WEAPON_FIRE_BEGIN()").unwrap();
        let parsed = get_macro_expr(result);
        assert_eq!(parsed.name().value, "WEAPON_FIRE_BEGIN");
        assert_eq!(parsed.args().len(), 1);
        assert_eq!(parsed.args()[0].value, "");

        let result = macro_expr().parse("WEAPON_FIRE_END()").unwrap();
        let parsed = get_macro_expr(result);
        assert_eq!(parsed.name().value, "WEAPON_FIRE_END");
        assert_eq!(parsed.args().len(), 1);
        assert_eq!(parsed.args()[0].value, "");
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
