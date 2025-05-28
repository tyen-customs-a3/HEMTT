use std::ops::Range;

use chumsky::prelude::*;

use crate::{Expression, Number, Value};

pub fn value() -> impl Parser<char, Value, Error = Simple<char>> {
    choice((
        eval().map(Value::Expression),
        super::array::array(false).map(Value::UnexpectedArray),
        super::str::string('"').map(Value::Str),
        math().map(Value::Number),
        super::number::number().map(Value::Number),
        super::macro_expr::macro_expr(),
    ))
}

/// Handles simple math expressions
/// This is a simpler implementation that doesn't try to be too clever
/// but handles basic arithmetic expressions
pub fn math() -> impl Parser<char, Number, Error = Simple<char>> {
    choice((
        super::number::number().map(|n| n.to_string()),
        just("-".to_string()),
        just("+".to_string()),
        just("*".to_string()),
        just("/".to_string()),
        just("%".to_string()),
        just("^".to_string()),
        just("(".to_string()),
        just(")".to_string()),
        just(" ".to_string()),
    ))
    .repeated()
    .at_least(2)
    .collect::<String>()
    .map(|s| s.trim().to_string())
    .try_map(|expr, span: Range<usize>| {
        let number = Number::try_evaulation(&expr, span.clone());
        number.map_or_else(
            || {
                // Pre-allocate error message capacity for optimization
                let mut msg = String::with_capacity(expr.len() + 32);
                msg.push_str(&expr);
                msg.push_str(" is not a valid math expression");
                Err(Simple::custom(span, msg))
            },
            Ok,
        )
    })
}

pub fn eval() -> impl Parser<char, Expression, Error = Simple<char>> {
    just("__EVAL".to_string())
        .ignore_then(recursive(|eval| {
            eval.repeated()
                .at_least(1)
                .collect::<String>()
                .map(|mut s| {
                    s.insert(0, '(');
                    s.push(')');
                    s
                })
                .delimited_by(just("(".to_string()), just(")".to_string()))
                .or(none_of("()".to_string())
                    .repeated()
                    .at_least(1)
                    .collect::<String>())
        }))
        .map_with_span(|expr, span| Expression {
            value: expr
                .strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .expect("eval should be wrapped in brackets")
                .to_string(),
            span,
        })
}

#[cfg(test)]
mod tests {
    use crate::{Number, Str, Value};
    use super::*;

    #[test]
    fn math() {
        assert_eq!(
            super::math().parse("1 + 2"),
            Ok(Number::Int32 {
                value: 3,
                span: 0..5
            })
        );
    }

    #[test]
    fn str() {
        assert_eq!(
            value().parse("\"hello\""),
            Ok(Value::Str(Str {
                value: "hello".to_string(),
                span: 0..7  // Fixed: should span the entire string including quotes
            }))
        );
    }

    #[test]
    fn number() {
        assert_eq!(
            value().parse("42"),
            Ok(Value::Number(Number::Int32 {
                value: 42,
                span: 0..2
            }))
        );
    }

    #[test]
    fn eval() {
        assert_eq!(
            value().parse("__EVAL(1 + 2)"),
            Ok(Value::Expression(Expression {
                value: "1 + 2".to_string(),
                span: 0..13
            }))
        );
    }
}
