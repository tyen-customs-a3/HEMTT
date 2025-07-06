use chumsky::prelude::*;

use crate::Ident;

pub fn ident() -> impl Parser<char, Ident, Error = Simple<char>> {
    // Identifiers should start with a letter or underscore, then allow alphanumeric and underscore
    filter(|c: &char| c.is_ascii_alphabetic() || *c == '_')
        .then(filter(|c: &char| c.is_ascii_alphanumeric() || *c == '_').repeated())
        .map(|(first, rest)| {
            let mut ident = first.to_string();
            ident.extend(rest);
            ident
        })
        .map_with_span(|value, span| Ident { value, span })
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::Ident;

    #[test]
    fn ident() {
        assert_eq!(
            super::ident().parse("abc"),
            Ok(Ident {
                value: "abc".to_string(),
                span: 0..3,
            })
        );
        assert_eq!(
            super::ident().parse("abc123"),
            Ok(Ident {
                value: "abc123".to_string(),
                span: 0..6,
            })
        );
        assert_eq!(
            super::ident().parse("abc_123"),
            Ok(Ident {
                value: "abc_123".to_string(),
                span: 0..7,
            })
        );
    }
}
