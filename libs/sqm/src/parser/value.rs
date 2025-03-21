// This module is only used for testing
use chumsky::prelude::*;
use crate::lexer::Token;
use crate::Value;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;
    use chumsky::Parser;

    // Helper function for tests only
    fn number_value() -> impl Parser<Token, Value, Error = Simple<Token>> {
        select! {
            Token::NumberLit(n) => {
                if n.contains('.') {
                    // Parse as float if it contains a decimal point
                    Value::Number(n.parse().unwrap_or(0.0))
                } else {
                    // Parse as integer otherwise
                    Value::Integer(n.parse().unwrap_or(0))
                }
            }
        }
    }

    // Helper function for tests only
    fn value() -> impl Parser<Token, Value, Error = Simple<Token>> {
        recursive(|value| {
            let string = select! { Token::StringLit(s) => Value::String(s.clone()) };
            let number = number_value();
            let array = just(Token::OpenBrace)
                .ignore_then(value.separated_by(just(Token::Comma)))
                .then_ignore(just(Token::CloseBrace))
                .map(Value::Array);
            
            choice((string, number, array))
        })
    }

    #[test]
    fn test_number_parsing() {
        let test_cases = vec![
            ("2.0", Value::Number(2.0)),
            ("0.2617994", Value::Number(0.2617994)),
            ("8745", Value::Integer(8745)),
            ("-0.6729841", Value::Number(-0.6729841)),
        ];

        for (input, expected) in test_cases {
            let tokens = tokenize(input).0;
            let result = number_value().parse(tokens).unwrap();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_string_parsing() {
        let input = r#""test string""#;
        let tokens = tokenize(input).0;
        let result = value().parse(tokens).unwrap();
        assert_eq!(result, Value::String("test string".to_string()));
    }
} 