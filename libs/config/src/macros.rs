use crate::{Item, Str};

/// Expands a macro into its resulting items
pub fn expand_macro(name: &Str, value: &Str) -> Vec<Item> {
    if name.value.starts_with("LIST_") {
        expand_list_macro(name, value)
    } else {
        // For unknown macros, just return the value as a single item
        vec![Item::Str(value.clone())]
    }
}

fn expand_list_macro(name: &Str, value: &Str) -> Vec<Item> {
    // Parse the count from LIST_X format
    let count = name.value
        .strip_prefix("LIST_")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1);

    // Create repeated items
    vec![Item::Str(value.clone()); count]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_macro_expansion() {
        let name = Str {
            value: "LIST_3".to_string(),
            span: 0..6,
        };
        let value = Str {
            value: "item".to_string(),
            span: 7..11,
        };

        let result = expand_macro(&name, &value);
        assert_eq!(result.len(), 3);
        for item in result {
            match item {
                Item::Str(s) => assert_eq!(s.value, "item"),
                _ => panic!("Expected string item"),
            }
        }
    }

    #[test]
    fn test_invalid_list_macro() {
        let name = Str {
            value: "LIST_invalid".to_string(),
            span: 0..11,
        };
        let value = Str {
            value: "item".to_string(),
            span: 12..16,
        };

        let result = expand_macro(&name, &value);
        assert_eq!(result.len(), 1); // Should default to 1 for invalid number
        match &result[0] {
            Item::Str(s) => assert_eq!(s.value, "item"),
            _ => panic!("Expected string item"),
        }
    }

    #[test]
    fn test_unknown_macro() {
        let name = Str {
            value: "UNKNOWN".to_string(),
            span: 0..7,
        };
        let value = Str {
            value: "value".to_string(),
            span: 8..13,
        };

        let result = expand_macro(&name, &value);
        assert_eq!(result.len(), 1);
        match &result[0] {
            Item::Str(s) => assert_eq!(s.value, "value"),
            _ => panic!("Expected string item"),
        }
    }
} 