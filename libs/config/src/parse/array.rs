use chumsky::prelude::*;

use crate::{Array, Item};

use super::{value::math, macro_expr::MacroType};

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
        super::macro_expr::macro_expr().padded().map(|(macro_type, span)| {
            match macro_type {
                MacroType::List { count, item } => Item::Macro((
                    crate::Str {
                        value: format!("LIST_{}", count),
                        span: span.start..span.start + format!("LIST_{}", count).len(),
                    },
                    crate::Str {
                        value: item,
                        span: span.start + format!("LIST_{}", count).len() + 1..span.end - 1,
                    },
                    span,
                )),
                MacroType::Eval { class, expression } => Item::Eval {
                    class: crate::Str {
                        value: class.clone(),
                        span: span.start + 6..span.start + 6 + class.len() + 2,
                    },
                    expression: crate::Str {
                        value: expression,
                        span: span.start + 8 + class.len() + 2..span.end - 1,
                    },
                    span,
                },
            }
        }),
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

    #[test]
    fn eval_macro_with_class() {
        assert_eq!(
            array(false).parse("{EVAL(\"MyClass\", \"1 + 2\")}"),
            Ok(Array {
                expand: false,
                items: vec![
                    Item::Eval {
                        class: crate::Str {
                            value: "MyClass".to_string(),
                            span: 7..16,
                        },
                        expression: crate::Str {
                            value: "1 + 2".to_string(),
                            span: 18..24,
                        },
                        span: 1..25,
                    },
                ],
                span: 0..26,
            })
        );
    }

    #[test]
    fn invalid_eval_macro_in_array() {
        assert_eq!(
            array(false).parse_recovery("{EVAL(\"MyClass\" \"1 + 2\")}").0,
            Some(Array {
                expand: false,
                items: vec![Item::Invalid(1..24)],
                span: 0..25,
            })
        );
    }

    #[test]
    fn invalid_list_macro_in_array() {
        assert_eq!(
            array(false).parse_recovery("{LIST_abc(\"item\")}").0,
            Some(Array {
                expand: false,
                items: vec![Item::Invalid(1..17)],
                span: 0..18,
            })
        );
    }

    #[test]
    fn mixed_array_with_invalid_macros() {
        assert_eq!(
            array(false).parse_recovery("{1, EVAL(), LIST_2, \"string\"}").0,
            Some(Array {
                expand: false,
                items: vec![
                    Item::Number(Number::Int32 {
                        value: 1,
                        span: 1..2,
                    }),
                    Item::Invalid(4..10),
                    Item::Invalid(12..18),
                    Item::Str(crate::Str {
                        value: "string".to_string(),
                        span: 20..28,
                    }),
                ],
                span: 0..29,
            })
        );
    }

    #[test]
    fn nested_array_with_invalid_macros() {
        assert_eq!(
            array(false).parse_recovery("{{1, EVAL()}, {LIST_2}, {\"string\"}}").0,
            Some(Array {
                expand: false,
                items: vec![
                    Item::Array(vec![
                        Item::Number(Number::Int32 {
                            value: 1,
                            span: 2..3,
                        }),
                        Item::Invalid(5..11),
                    ]),
                    Item::Array(vec![Item::Invalid(15..21)]),
                    Item::Array(vec![Item::Str(crate::Str {
                        value: "string".to_string(),
                        span: 25..33,
                    })]),
                ],
                span: 0..35,
            })
        );
    }
}
