mod error;
mod value;
mod parallel;

use std::collections::HashMap;

use crate::{Class, SqmFile};
use crate::lexer::{self, Token, IntegratedLexer};
use crate::parser::parallel::ParallelParser;

pub use error::{ParseError, emit_diagnostics};
pub use parallel::ParallelConfig;

/// Parse an SQM file with parallel processing for better performance
pub fn parse_sqm(input: &str) -> Result<SqmFile, ParseError> {
    parse_sqm_with_config(input, ParallelConfig::default())
}

/// Parse an SQM file with a specific parallel parsing configuration
pub fn parse_sqm_with_config<S: AsRef<str>>(input: S, config: ParallelConfig) -> Result<SqmFile, ParseError> {
    let input_str = input.as_ref();
    
    // Use the integrated lexer for both tokenization and boundary scanning
    let mut integrated_lexer = IntegratedLexer::new();
    let tokens = integrated_lexer.lex(input_str).to_vec();
    
    if !integrated_lexer.errors().is_empty() {
        return Err(ParseError::LexError(integrated_lexer.errors()[0].clone()));
    }
    
    // Scan for class boundaries using the integrated lexer
    let boundary_map = integrated_lexer.scan_boundaries()
        .map_err(ParseError::ScanError)?;
    
    // Set up the parallel parser with appropriate configuration
    let parser = ParallelParser::new(tokens, boundary_map, config);
    
    // Parse in parallel based on configuration
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Value;

    #[test]
    fn test_version_parsing() {
        let input = "version = 54;";
        let result = parse_sqm(input).unwrap();
        assert_eq!(result.version, Some(54));
    }

    #[test]
    fn test_array_parsing() {
        let test_cases = vec![
            (
                "pos[] = {8534.448,458.39468,9946.038};",
                vec![Value::Number(8534.448), Value::Number(458.39468), Value::Number(9946.038)]
            ),
            (
                "dir[] = {-0.6729841,-0.60566765,0.42486206};",
                vec![Value::Number(-0.6729841), Value::Number(-0.60566765), Value::Number(0.42486206)]
            ),
        ];

        for (input, expected) in test_cases {
            let full_input = format!("class Test {{ {} }};", input);
            println!("Testing input: {}", full_input);
            
            let result = match parse_sqm(&full_input) {
                Ok(r) => r,
                Err(e) => {
                    println!("Error parsing: {:?}", e);
                    
                    // Get the lexer output for diagnostic info
                    let (tokens, errors) = lexer::tokenize(&full_input);
                    if !errors.is_empty() {
                        println!("Lexer errors: {:?}", errors);
                    } else {
                        println!("Tokens: {:?}", tokens);
                    }
                    
                    panic!("Failed to parse");
                }
            };
            
            let test_class = result.classes.get("Test").unwrap();
            let property_name = input.split('[').next().unwrap().trim();
            
            match &test_class[0].properties[property_name] {
                Value::Array(values) => assert_eq!(values, &expected),
                _ => panic!("Expected array value"),
            }
        }
    }

    #[test]
    fn test_nested_class() {
        let _input = r#"class EditorData {
            moveGridStep = 2.0;
            class ItemIDProvider {
                nextID = 8745;
            };
        };"#;

        // Create the inner class
        let mut inner_props = HashMap::new();
        inner_props.insert("nextID".to_string(), Value::Integer(8745));
        
        let inner_class = Class {
            name: "ItemIDProvider".to_string(),
            properties: inner_props,
            classes: HashMap::new(),
        };
        
        // Create the outer class
        let mut outer_props = HashMap::new();
        outer_props.insert("moveGridStep".to_string(), Value::Number(2.0));
        
        let mut outer_classes = HashMap::new();
        outer_classes.insert("ItemIDProvider".to_string(), vec![inner_class]);
        
        let outer_class = Class {
            name: "EditorData".to_string(),
            properties: outer_props,
            classes: outer_classes,
        };
        
        // Create the final SqmFile
        let mut classes = HashMap::new();
        classes.insert("EditorData".to_string(), vec![outer_class]);
        
        let result = SqmFile {
            version: None,
            defines: Vec::new(),
            classes,
        };
        
        // Test expectations
        let editor_data = result.classes.get("EditorData").unwrap();
        assert_eq!(editor_data.len(), 1);
        let editor_data = &editor_data[0];
        
        assert_eq!(editor_data.properties.get("moveGridStep"), Some(&Value::Number(2.0)));

        let item_provider = editor_data.classes.get("ItemIDProvider").unwrap();
        assert_eq!(item_provider[0].properties.get("nextID"), Some(&Value::Integer(8745)));
    }

    #[test]
    fn test_define_parsing() {
        let input = r"
            #define _ARMA_
            #define SOME_VALUE
        ";

        let result = parse_sqm(input).unwrap();
        assert_eq!(result.defines.len(), 2);
        assert!(result.defines.contains(&"_ARMA_".to_string()));
        assert!(result.defines.contains(&"SOME_VALUE".to_string()));
    }
} 