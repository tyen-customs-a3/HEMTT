use hemtt_config::{Class, Property, Value, Item, Config};

mod parse_helpers {
    use super::*;
    use std::sync::Arc;
    use hemtt_preprocessor::Processor;
    use hemtt_workspace::{LayerType, Workspace, reporting::WorkspaceFiles};
    use hemtt_common::config::PDriveOption;
    use std::path::PathBuf;
    use tempfile::tempdir;

    pub fn parse_config(content: &str) -> Result<Config, Vec<Arc<dyn hemtt_workspace::reporting::Code>>> {
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let temp_path = PathBuf::from(temp_dir.path());
        
        let workspace = Workspace::builder()
            .physical(&temp_path, LayerType::Source)
            .finish(None, false, &PDriveOption::Disallow)
            .unwrap();

        let temp_file = temp_dir.path().join("test.hpp");
        std::fs::write(&temp_file, content).unwrap();

        let source = workspace.join("test.hpp").unwrap();
        let processed = Processor::run(&source).unwrap();
        println!("\nProcessed output:\n{}", processed.as_str());
        
        let parsed = hemtt_config::parse(None, &processed);
        let workspacefiles = WorkspaceFiles::new();

        // Print diagnostic output
        match parsed {
            Ok(config) => {
                if !config.codes().is_empty() {
                    println!("\nWarnings/Notes:");
                    for code in config.codes() {
                        if let Some(diag) = code.diagnostic() {
                            println!("{}", diag.to_string(&workspacefiles));
                        }
                    }
                }
                Ok(config.into_config())
            }
            Err(errors) => {
                println!("\nErrors:");
                for error in &errors {
                    if let Some(diag) = error.diagnostic() {
                        println!("{}", diag.to_string(&workspacefiles));
                    }
                }
                Err(errors)
            }
        }
    }
}

use parse_helpers::parse_config;

#[test]
fn test_simple_array() {
    let config = r#"
    class Test {
        simple[] = {1, 2, 3};
    };
    "#;
    let result = parse_config(config);
    assert!(result.is_ok(), "Failed to parse simple array");
}

#[test]
fn test_array_with_strings() {
    let config = r#"
    class Test {
        strings[] = {"one", "two", "three"};
    };
    "#;
    let result = parse_config(config);
    assert!(result.is_ok(), "Failed to parse array with strings");
}

#[test]
fn test_single_macro_array() {
    let config = r#"
    class Test {
        macro[] = {MACRO(arg)};
    };
    "#;
    let result = parse_config(config);
    assert!(result.is_ok(), "Failed to parse array with single macro");
}

#[test]
fn test_mixed_array() {
    let config = r#"
    class Test {
        mixed[] = {1, MACRO(arg), "string"};
    };
    "#;
    let result = parse_config(config);
    assert!(result.is_ok(), "Failed to parse mixed array");
}

#[test]
fn test_nested_array() {
    let config = r#"
    class Test {
        nested[] = {{1, 2}, {3, 4}};
    };
    "#;
    let result = parse_config(config);
    assert!(result.is_ok(), "Failed to parse nested array");
}

#[test]
fn test_nested_array_with_macros() {
    let config = r#"
    class Test {
        nested[] = {{MACRO(a), MACRO(b)}, {1, 2}};
    };
    "#;
    let result = parse_config(config);
    assert!(result.is_ok(), "Failed to parse nested array with macros");
}

#[test]
fn test_array_expansion() {
    let config = r#"
    class Test {
        base[] = {1, 2};
        expand[] += {3, 4};
    };
    "#;
    let result = parse_config(config);
    assert!(result.is_ok(), "Failed to parse array expansion");
}

#[test]
fn test_array_expansion_with_macros() {
    let config = r#"
    class Test {
        base[] = {MACRO(a), MACRO(b)};
        expand[] += {MACRO(c), MACRO(d)};
    };
    "#;
    let result = parse_config(config);
    assert!(result.is_ok(), "Failed to parse array expansion with macros");
} 