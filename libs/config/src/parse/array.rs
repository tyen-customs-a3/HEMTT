use chumsky::prelude::*;

use crate::{Array, Item};

use super::value::math;

pub fn array(expand: bool) -> impl Parser<char, Array, Error = Simple<char>> {
    recursive(|value| {
        value
            .map(Item::Array)
            .or(array_value().recover_with(skip_parser(
                none_of("},")
                    .padded()
                    .repeated()
                    .at_least(1)
                    .map_with_span(move |_, span| Item::Invalid(span)),
            )))
            .padded()
            .separated_by(just(',').padded())
            .allow_trailing()
            .delimited_by(just('{'), just('}'))
    })
    .map_with_span(move |items, span| Array {
        expand,
        items,
        span: span.start..span.end,
    })
}

fn array_value() -> impl Parser<char, Item, Error = Simple<char>> {
    choice((
        super::str::string('"').padded().map(Item::Str),
        math().padded().map(Item::Number),
        super::number::number().padded().map(Item::Number),
        super::macro_expr::macro_expr().padded().map(Item::Macro),
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

    #[test]
    fn macro_list() {
        assert_eq!(
            array(false).parse("{LIST_2(\"item\")}"),
            Ok(Array {
                expand: false,
                items: vec![
                    Item::Macro((
                        crate::Str {
                            value: "LIST_2".to_string(),
                            span: 1..7,
                        },
                        crate::Str {
                            value: "item".to_string(),
                            span: 8..14,
                        },
                        1..15,
                    )),
                ],
                span: 0..16,
            })
        );
    }

    #[test]
    fn mixed_array() {
        assert_eq!(
            array(false).parse("{1, LIST_2(\"item\"), \"string\"}"),
            Ok(Array {
                expand: false,
                items: vec![
                    Item::Number(Number::Int32 {
                        value: 1,
                        span: 1..2,
                    }),
                    Item::Macro((
                        crate::Str {
                            value: "LIST_2".to_string(),
                            span: 4..10,
                        },
                        crate::Str {
                            value: "item".to_string(),
                            span: 11..17,
                        },
                        4..18,
                    )),
                    Item::Str(crate::Str {
                        value: "string".to_string(),
                        span: 20..28,
                    }),
                ],
                span: 0..29,
            })
        );
    }
}
