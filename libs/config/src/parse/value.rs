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
    fn str() {
        assert_eq!(
            value().parse("\"\""),
            Ok(Value::Str(Str {
                value: String::new(),
                span: 0..2
            }))
        );
        assert_eq!(
            value().parse("\"abc\""),
            Ok(Value::Str(Str {
                value: "abc".to_string(),
                span: 0..5
            }))
        );
        assert_eq!(
            value().parse("\"abc\"\"def\"\"\""),
            Ok(Value::Str(Str {
                value: "abc\"def\"".to_string(),
                span: 0..12
            }))
        );
        assert_eq!(
            value().parse("\"abc\ndef\""),
            Ok(Value::Str(Str {
                value: "abc\ndef".to_string(),
                span: 0..9
            }))
        );
    }

    #[test]
    fn eval() {
        assert_eq!(
            super::eval().parse("__EVAL(1 + 2)"),
            Ok(Expression {
                value: "1 + 2".to_string(),
                span: 0..13
            })
        );
        assert_eq!(
            super::eval().parse("__EVAL(2 * (1 + 1))"),
            Ok(Expression {
                value: "2 * (1 + 1)".to_string(),
                span: 0..19
            })
        );
    }

    #[test]
    fn number() {
        assert_eq!(
            value().parse("123"),
            Ok(Value::Number(Number::Int32 {
                value: 123,
                span: 0..3
            }))
        );
        assert_eq!(
            value().parse("123.456"),
            Ok(Value::Number(Number::Float32 {
                value: 123.456,
                span: 0..7
            }))
        );
        assert_eq!(
            value().parse("123.456e-7"),
            Ok(Value::Number(Number::Float32 {
                value: 0.000_012_345_6,
                span: 0..10
            }))
        );
        assert_eq!(
            value().parse("123.456e+7"),
            Ok(Value::Number(Number::Float32 {
                value: 1_234_560_000.0,
                span: 0..10
            }))
        );
        assert_eq!(
            value().parse("123.456e7"),
            Ok(Value::Number(Number::Float32 {
                value: 1_234_560_000.0,
                span: 0..9
            }))
        );
        assert_eq!(
            value().parse("123.456e+"),
            Ok(Value::Number(Number::Float32 {
                value: 123.456,
                span: 0..7
            }))
        );
        assert_eq!(
            value().parse("123.456e-"),
            Ok(Value::Number(Number::Float32 {
                value: 123.456,
                span: 0..7
            }))
        );
        assert_eq!(
            value().parse("123.456e"),
            Ok(Value::Number(Number::Float32 {
                value: 123.456,
                span: 0..7
            }))
        );
        assert_eq!(
            value().parse("123.456e+abc"),
            Ok(Value::Number(Number::Float32 {
                value: 123.456,
                span: 0..7
            }))
        );
        assert_eq!(
            value().parse("123.456e-abc"),
            Ok(Value::Number(Number::Float32 {
                value: 123.456,
                span: 0..7
            }))
        );
        assert_eq!(
            value().parse("123.456eabc"),
            Ok(Value::Number(Number::Float32 {
                value: 123.456,
                span: 0..7
            }))
        );
    }

    #[test]
    fn math() {
        assert_eq!(
            value().parse("1 + 2"),
            Ok(Value::Number(Number::Int32 {
                value: 3,
                span: 0..5
            }))
        );
        assert_eq!(
            value().parse("1 - 2"),
            Ok(Value::Number(Number::Int32 {
                value: -1,
                span: 0..5
            }))
        );
        assert_eq!(
            value().parse("1 * 2"),
            Ok(Value::Number(Number::Int32 {
                value: 2,
                span: 0..5
            }))
        );
        assert_eq!(
            value().parse("1 / 2"),
            Ok(Value::Number(Number::Float32 {
                value: 0.5,
                span: 0..5
            }))
        );
        assert_eq!(
            value().parse("1 % 2"),
            Ok(Value::Number(Number::Int32 {
                value: 1,
                span: 0..5
            }))
        );
        assert_eq!(
            value().parse("1 ^ 2"),
            Ok(Value::Number(Number::Int32 {
                value: 1,
                span: 0..5
            }))
        );
        assert_eq!(
            value().parse("1 + 2 * 3"),
            Ok(Value::Number(Number::Int32 {
                value: 7,
                span: 0..9
            }))
        );
        assert_eq!(
            value().parse("(1 + 2) * 3"),
            Ok(Value::Number(Number::Int32 {
                value: 9,
                span: 0..11
            }))
        );
        assert_eq!(
            value().parse("1 + 2 * 3 + 4"),
            Ok(Value::Number(Number::Int32 {
                value: 11,
                span: 0..13
            }))
        );
        assert_eq!(
            value().parse("1 + 2 * (3 + 4)"),
            Ok(Value::Number(Number::Int32 {
                value: 15,
                span: 0..15
            }))
        );
        assert_eq!(
            value().parse("2 ^ 3"),
            Ok(Value::Number(Number::Int32 {
                value: 8,
                span: 0..5
            }))
        );
        assert_eq!(
            value().parse("10 % 3"),
            Ok(Value::Number(Number::Int32 {
                value: 1,
                span: 0..6
            }))
        );
        assert_eq!(
            value().parse("10 % 3 + 2"),
            Ok(Value::Number(Number::Int32 {
                value: 3,
                span: 0..10
            }))
        );
        assert_eq!(
            value().parse("-0.01*0.5"),
            Ok(Value::Number(Number::Float32 {
                value: -0.005,
                span: 0..9
            }))
        );
    }

    #[test]
    fn macro_expressions() {
        
        // Test simple macro without arguments
        let result = value().parse("SIMPLE_MACRO").unwrap();
        if let Value::Macro(m) = result {
            assert_eq!(m.name().value(), "SIMPLE_MACRO");
            assert!(m.args().is_empty());
        } else {
            panic!("Expected macro value");
        }

        // Test macro with single argument
        let result = value().parse("FUNC(arg1)").unwrap();
        if let Value::Macro(m) = result {
            assert_eq!(m.name().value(), "FUNC");
            assert_eq!(m.args().len(), 1);
            assert_eq!(m.args()[0].value(), "arg1");
        } else {
            panic!("Expected macro value");
        }

        // Test macro with multiple arguments
        let result = value().parse("FORMAT_2(arg1,arg2)").unwrap();
        if let Value::Macro(m) = result {
            assert_eq!(m.name().value(), "FORMAT_2");
            assert_eq!(m.args().len(), 2);
            assert_eq!(m.args()[0].value(), "arg1");
            assert_eq!(m.args()[1].value(), "arg2");
        } else {
            panic!("Expected macro value");
        }

        // Test macro with quoted string arguments
        let result = value().parse("ECSTRING(\"common\",\"ACETeam\")").unwrap();
        if let Value::Macro(m) = result {
            assert_eq!(m.name().value(), "ECSTRING");
            assert_eq!(m.args().len(), 2);
            assert_eq!(m.args()[0].value(), "\"common\"");
            assert_eq!(m.args()[1].value(), "\"ACETeam\"");
        } else {
            panic!("Expected macro value");
        }
    }
}
