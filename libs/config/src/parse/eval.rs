use chumsky::prelude::*;

use crate::Expression;

/// Parse an EVAL expression, allowing for extra closing parentheses
pub fn eval() -> impl Parser<char, Expression, Error = Simple<char>> {
    // First parse the __EVAL token followed by an opening parenthesis
    just("__EVAL").padded().then(just('('))
        .then(
            // Parse the content until we find the matching closing parenthesis
            // This handles nested parentheses correctly
            recursive(|inner| {
                choice((
                    // Handle nested parentheses by recursively parsing their content
                    just('(')
                        .ignore_then(inner)
                        .then_ignore(just(')')),
                    
                    // Handle any character except parentheses
                    filter(|c: &char| *c != '(' && *c != ')')
                        .repeated()
                        .at_least(1)
                        .collect::<String>()
                ))
                .repeated()
                .collect::<String>()
            })
            .delimited_by(empty(), just(')'))
            // Consume any extra closing parentheses
            .then_ignore(just(')').repeated().ignored())
        )
        .map_with_span(|((_, _), content), span| {
            Expression {
                value: content,
                span,
            }
        })
} 