use chumsky::prelude::*;
use std::ops::Range;

use crate::Str;

pub fn macro_expr() -> impl Parser<char, (Str, Str, Range<usize>), Error = Simple<char>> {
    let ident = text::ident();
    let number = text::int(10);
    
    let macro_name = ident
        .then(just('_').then(number).map(|(_, n)| n).or_not())
        .map(|(name, count)| match count {
            Some(n) => format!("{}_{}", name, n),
            None => name,
        })
        .map_with_span(|name, span| Str {
            value: name,
            span,
        });

    let macro_arg = super::str::string('"');

    macro_name
        .then_ignore(just('('))
        .then(macro_arg)
        .then_ignore(just(')'))
        .map_with_span(|(name, arg), span| (name, arg, span))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_macro() {
        assert_eq!(
            macro_expr().parse("LIST_2(\"item\")"),
            Ok((
                Str {
                    value: "LIST_2".to_string(),
                    span: 0..6,
                },
                Str {
                    value: "item".to_string(),
                    span: 7..13,
                },
                0..14,
            ))
        );
    }

    #[test]
    fn macro_without_number() {
        assert_eq!(
            macro_expr().parse("LIST(\"item\")"),
            Ok((
                Str {
                    value: "LIST".to_string(),
                    span: 0..4,
                },
                Str {
                    value: "item".to_string(),
                    span: 5..11,
                },
                0..12,
            ))
        );
    }
} 