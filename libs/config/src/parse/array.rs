use chumsky::prelude::*;
use std::borrow::Cow;

use crate::{Array, Item, Value, MacroExpression};

use super::value::math;

// Maximum number of arguments to prevent OOM
const MAX_MACRO_ARGS: usize = 256;

pub fn array(expand: bool) -> impl Parser<char, Array, Error = Simple<char>> {
    recursive(move |array_parser| {
        choice((
            // Empty array
            just('{')
                .padded()
                .ignore_then(just('}').padded())
                .map_with_span(move |_, span| Array {
                    expand,
                    items: vec![],
                    span,
                }),
            
            // Array with items
            choice((
                // Handle nested arrays
                array_parser.map(|nested_array: Array| Item::Array(nested_array.items)),
                
                // Handle basic array values
                array_value(),
            ))
            .padded()
            .separated_by(just(',').padded())
            .allow_trailing()
            .delimited_by(just('{').padded(), just('}').padded())
            .recover_with(nested_delimiters(
                '{',
                '}',
                [('[', ']'), ('(', ')')], // Also track these delimiter pairs
                |span| {
                    // Preserve error information instead of returning empty vec
                    vec![Item::Invalid(span)]
                }
            ))
            .map_with_span(move |mut items, span| {
                // Remove any trailing Invalid items that might have been added due to trailing commas
                while let Some(Item::Invalid(_)) = items.last() {
                    items.pop();
                }
                Array {
                    expand,
                    items,
                    span,
                }
            })
        ))
    })
}

fn array_value() -> impl Parser<char, Item, Error = Simple<char>> {
    choice((
        // String values
        super::str::string('"').map(Item::Str),
        
        // Math expressions
        math().map(Item::Number),
        
        // Number literals
        super::number::number().map(Item::Number),
        
        // Macros with parentheses
        super::macro_expr::macro_name()
            .then(
                super::macro_expr::macro_arg()
                    .separated_by(just(',').padded())
                    .allow_trailing()
                    .delimited_by(just('('), just(')'))
            )
            .try_map(|(name, args), span| {
                // Check bounds to prevent OOM
                if args.len() > MAX_MACRO_ARGS {
                    return Err(Simple::custom(span.clone(), format!("Too many macro arguments (max {})", MAX_MACRO_ARGS)));
                }
                
                // Use Cow to avoid unnecessary string allocations
                let arg_strings: Vec<String> = args.into_iter()
                    .map(|v| match v {
                        Value::Str(s) => Cow::Borrowed(s.value()).into_owned(),
                        Value::Macro(m) => m.to_string(),
                        _ => String::new()
                    })
                    .collect();
                
                MacroExpression::from_strings(name, arg_strings, span.clone())
                    .map(Item::Macro)
                    .map_err(|e| Simple::custom(span, e))
            }),
    ))
    .recover_with(skip_parser(
        none_of("},")
            .padded()
            .repeated()
            .at_least(1)
            .map_with_span(|_, span| Item::Invalid(span)),
    ))
}

#[cfg(test)]
mod tests {
    use crate::{Array, Item, Number};

    use super::*;

    #[test]
    fn empty() {
        assert_eq!(
            array(false).parse("{}"),
            Ok(Array {
                expand: false,
                items: vec![],
                span: 0..2,
            })
        );
    }

    #[test]
    fn single() {
        assert_eq!(
            array(false).parse("{1,2,3}"),
            Ok(Array {
                expand: false,
                items: vec![
                    Item::Number(Number::Int32 {
                        value: 1,
                        span: 1..2,
                    }),
                    Item::Number(Number::Int32 {
                        value: 2,
                        span: 3..4,
                    }),
                    Item::Number(Number::Int32 {
                        value: 3,
                        span: 5..6,
                    }),
                ],
                span: 0..7,
            })
        );
    }

    #[test]
    fn nested() {
        assert_eq!(
            array(false).parse("{{1,2},{3,4},5}"),
            Ok(Array {
                expand: false,
                items: vec![
                    Item::Array(vec![
                        Item::Number(Number::Int32 {
                            value: 1,
                            span: 2..3
                        }),
                        Item::Number(Number::Int32 {
                            value: 2,
                            span: 4..5
                        }),
                    ]),
                    Item::Array(vec![
                        Item::Number(Number::Int32 {
                            value: 3,
                            span: 8..9
                        }),
                        Item::Number(Number::Int32 {
                            value: 4,
                            span: 10..11
                        }),
                    ]),
                    Item::Number(Number::Int32 {
                        value: 5,
                        span: 13..14
                    }),
                ],
                span: 0..15
            })
        );
    }

    #[test]
    fn trailing() {
        assert_eq!(
            array(false).parse_recovery("{1,2,3,}").0,
            Some(Array {
                expand: false,
                items: vec![
                    Item::Number(Number::Int32 {
                        value: 1,
                        span: 1..2,
                    }),
                    Item::Number(Number::Int32 {
                        value: 2,
                        span: 3..4,
                    }),
                    Item::Number(Number::Int32 {
                        value: 3,
                        span: 5..6,
                    }),
                ],
                span: 0..8,
            })
        );
    }

    #[test]
    fn invalid_item() {
        assert_eq!(
            array(false).parse_recovery("{1,2,three,4}").0,
            Some(Array {
                expand: false,
                items: vec![
                    Item::Number(Number::Int32 {
                        value: 1,
                        span: 1..2,
                    }),
                    Item::Number(Number::Int32 {
                        value: 2,
                        span: 3..4,
                    }),
                    Item::Invalid(5..10),
                    Item::Number(Number::Int32 {
                        value: 4,
                        span: 11..12,
                    }),
                ],
                span: 0..13,
            })
        );
    }
}
