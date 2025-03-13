use chumsky::prelude::*;
use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub enum MacroType {
    List {
        count: u32,
        item: String,
    },
    Eval {
        class: String,
        expression: String,
    },
}

pub fn macro_expr() -> impl Parser<char, (MacroType, Range<usize>), Error = Simple<char>> {
    let ident = text::ident::<char, Simple<char>>();
    let number = text::int::<char, _>(10).map(|s: String| s.parse::<u32>().unwrap());
    
    let list_macro = just("LIST")
        .then(just('_').ignore_then(number))
        .then(
            super::str::string('"')
                .delimited_by(just('('), just(')'))
        )
        .map_with_span(|((_, count), arg), span| {
            (MacroType::List {
                count,
                item: arg.value,
            }, span)
        });

    let eval_macro = just("EVAL")
        .then(
            super::str::string('"')
                .then_ignore(just(',').padded())
                .then(super::str::string('"'))
                .delimited_by(just('('), just(')'))
        )
        .try_map(|(_, (class, expr)), span| {
            if class.value.is_empty() || expr.value.is_empty() {
                Err(Simple::custom(span, "EVAL macro arguments cannot be empty"))
            } else {
                Ok((MacroType::Eval {
                    class: class.value,
                    expression: expr.value,
                }, span))
            }
        });

    choice((
        list_macro,
        eval_macro,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_macro() {
        assert_eq!(
            macro_expr().parse("LIST_2(\"item\")"),
            Ok((
                MacroType::List {
                    count: 2,
                    item: "item".to_string(),
                },
                0..14,
            ))
        );
    }

    #[test]
    fn list_macro_large_number() {
        assert_eq!(
            macro_expr().parse("LIST_123(\"item\")"),
            Ok((
                MacroType::List {
                    count: 123,
                    item: "item".to_string(),
                },
                0..16,
            ))
        );
    }

    #[test]
    fn eval_macro() {
        assert_eq!(
            macro_expr().parse("EVAL(\"MyClass\", \"1 + 2\")"),
            Ok((
                MacroType::Eval {
                    class: "MyClass".to_string(),
                    expression: "1 + 2".to_string(),
                },
                0..24,
            ))
        );
    }

    #[test]
    fn invalid_macro() {
        assert!(macro_expr().parse("UNKNOWN(\"value\")").is_err());
    }

    #[test]
    fn invalid_list_macro_no_number() {
        assert!(macro_expr().parse("LIST(\"item\")").is_err());
    }

    #[test]
    fn invalid_list_macro_invalid_number() {
        assert!(macro_expr().parse("LIST_abc(\"item\")").is_err());
    }

    #[test]
    fn invalid_eval_macro_missing_comma() {
        assert!(macro_expr().parse("EVAL(\"MyClass\" \"1 + 2\")").is_err());
    }

    #[test]
    fn invalid_eval_macro_missing_quotes() {
        assert!(macro_expr().parse("EVAL(MyClass, \"1 + 2\")").is_err());
    }

    #[test]
    fn invalid_eval_macro_extra_args() {
        assert!(macro_expr().parse("EVAL(\"MyClass\", \"1 + 2\", \"extra\")").is_err());
    }

    #[test]
    fn invalid_eval_macro_missing_args() {
        assert!(macro_expr().parse("EVAL(\"MyClass\")").is_err());
    }

    #[test]
    fn invalid_eval_macro_empty_args() {
        assert!(macro_expr().parse("EVAL(\"\", \"\")").is_err());
    }
} 