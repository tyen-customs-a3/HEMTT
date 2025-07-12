use chumsky::prelude::*;

use crate::Ident;

pub fn ident() -> impl Parser<char, Ident, Error = Simple<char>> {
    // Identifiers can start with a letter, underscore, or digit (for Arma 3 configs like "30Rnd_...")
    // Then allow alphanumeric and underscore
    filter(|c: &char| c.is_ascii_alphanumeric() || *c == '_')
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
        // Test numeric identifiers (common in Arma 3 configs)
        assert_eq!(
            super::ident().parse("30Rnd_9x21_Mag_SMG_02"),
            Ok(Ident {
                value: "30Rnd_9x21_Mag_SMG_02".to_string(),
                span: 0..21,
            })
        );
    }
}
