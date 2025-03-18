use chumsky::prelude::*;

use crate::{Array, Item, Value};

use super::value::math;
use super::macro_expr;

pub fn array(expand: bool) -> impl Parser<char, Array, Error = Simple<char>> {
    recursive(|value| {
        choice((
            just('{')
                .padded()
                .ignore_then(just('}').padded())
                .map(|_| vec![]),
            value
                .map(Item::Array)
                .or(array_value())
                .padded()
                .separated_by(just(',').padded())
                .allow_trailing()
                .delimited_by(just('{').padded(), just('}').padded())
                .map(|mut items| {
                    // Remove any trailing Invalid items that might have been added due to trailing commas
                    while let Some(Item::Invalid(_)) = items.last() {
                        items.pop();
                    }
                    items
                })
        ))
    })
    .map_with_span(move |items, span| Array {
        expand,
        items,
        span,
    })
}

fn array_value() -> impl Parser<char, Item, Error = Simple<char>> {
    choice((
        super::str::string('"').map(Item::Str),
        math().map(Item::Number),
        super::number::number().map(Item::Number),
        super::macro_expr::macro_expr().map(|v| match v {
            Value::Macro(m) => Item::Macro(m),
            Value::Invalid(span) => Item::Invalid(span),
            _ => Item::Invalid(0..0), // Fallback case
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
    use crate::Number;

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
