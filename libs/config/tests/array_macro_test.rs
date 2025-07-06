use hemtt_preprocessor::Processor;
use hemtt_workspace::{LayerType, Workspace, reporting::WorkspaceFiles};
use hemtt_common::config::PDriveOption;
use std::path::PathBuf;
use hemtt_config::{Class, Property, Value, Item};
use tempfile::tempdir;

#[test]
fn test_array_macro_handling() {
    // Create test config with arrays containing macros
    let config_content = r#"
    class TestConfig {
        // Simple array with macros
        simpleArray[] = {
            MACRO(arg1),
            MACRO(arg2),
            "normal string",
            123
        };

        // Nested array with macros
        nestedArray[] = {
            {MACRO(inner1), MACRO(inner2)},
            {QPATHTOF(path\to\file), QUOTE(something)},
            {1, MACRO(value), 3}
        };

        // Array expansion with macros
        baseArray[] = {1, MACRO(base), 3};
        expandArray[] += {MACRO(expand1), MACRO(expand2)};

        // Mixed macro types
        mixedArray[] = {
            QUOTE(quoted_value),
            CONCAT(part1,part2),
            FORMAT_2("%1_%2",arg1,arg2),
            PATH(some\path)
        };
    };
    "#;

    // Create a temporary directory and workspace
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = PathBuf::from(temp_dir.path());
    
    let workspace = Workspace::builder()
        .physical(&temp_path, LayerType::Source)
        .finish(None, false, &PDriveOption::Disallow)
        .unwrap();

    // Write the content to a temporary file
    let temp_file = temp_dir.path().join("array_test.hpp");
    std::fs::write(&temp_file, config_content).unwrap();

    // Process and parse the file
    let source = workspace.join("array_test.hpp").unwrap();
    let processed = Processor::run(&source).unwrap();
    println!("Processed output:\n{}", processed.as_str());
    
    let parsed = hemtt_config::parse(None, &processed);
    let workspacefiles = WorkspaceFiles::new();

    // Print diagnostic output
    match &parsed {
        Ok(config) => {
            println!("\nWarnings/Notes:");
            for code in config.codes() {
                if let Some(diag) = code.diagnostic() {
                    println!("{}", diag.to_string(&workspacefiles));
                }
            }
        }
        Err(errors) => {
            println!("\nErrors:");
            for error in errors {
                if let Some(diag) = error.diagnostic() {
                    println!("{}", diag.to_string(&workspacefiles));
                }
            }
            panic!("Failed to parse config");
        }
    }

    let config = parsed.unwrap().into_config();

    // Get TestConfig class
    let test_config = config.0.iter()
        .find_map(|p| {
            if let Property::Class(c) = p {
                if c.name().map_or(false, |n| n.as_str() == "TestConfig") {
                    Some(c)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .expect("TestConfig class not found");

    // Test simple array with macros
    let simple_array = get_array_property(test_config, "simpleArray");
    assert_eq!(simple_array.items.len(), 4, "Simple array should have 4 items");
    verify_array_item(&simple_array.items[0], "MACRO(arg1)");
    verify_array_item(&simple_array.items[1], "MACRO(arg2)");
    assert!(matches!(&simple_array.items[2], Item::Str(_)), "Third item should be a string");
    assert!(matches!(&simple_array.items[3], Item::Number(_)), "Fourth item should be a number");

    // Test nested array with macros
    let nested_array = get_array_property(test_config, "nestedArray");
    assert_eq!(nested_array.items.len(), 3, "Nested array should have 3 subarrays");
    
    if let Item::Array(inner1) = &nested_array.items[0] {
        assert_eq!(inner1.len(), 2, "First subarray should have 2 items");
        verify_array_item(&inner1[0], "MACRO(inner1)");
        verify_array_item(&inner1[1], "MACRO(inner2)");
    } else {
        panic!("First item should be an array");
    }

    if let Item::Array(inner2) = &nested_array.items[1] {
        assert_eq!(inner2.len(), 2, "Second subarray should have 2 items");
        verify_array_item(&inner2[0], "QPATHTOF(path\\to\\file)");
        verify_array_item(&inner2[1], "QUOTE(something)");
    } else {
        panic!("Second item should be an array");
    }

    // Test array expansion
    let base_array = get_array_property(test_config, "baseArray");
    assert_eq!(base_array.expand, false, "Base array should not be marked for expansion");
    
    let expand_array = get_array_property(test_config, "expandArray");
    assert_eq!(expand_array.expand, true, "Expand array should be marked for expansion");
    assert_eq!(expand_array.items.len(), 2, "Expand array should have 2 items");
    verify_array_item(&expand_array.items[0], "MACRO(expand1)");
    verify_array_item(&expand_array.items[1], "MACRO(expand2)");

    // Test mixed macro types
    let mixed_array = get_array_property(test_config, "mixedArray");
    assert_eq!(mixed_array.items.len(), 4, "Mixed array should have 4 items");
    verify_array_item(&mixed_array.items[0], "QUOTE(quoted_value)");
    verify_array_item(&mixed_array.items[1], "CONCAT(part1,part2)");
    verify_array_item(&mixed_array.items[2], "FORMAT_2(\"%1_%2\",arg1,arg2)");
    verify_array_item(&mixed_array.items[3], "PATH(some\\path)");
}

// Helper function to get an array property
fn get_array_property<'a>(class: &'a Class, name: &str) -> &'a hemtt_config::Array {
    let prop = class.properties().iter()
        .find_map(|p| {
            if let Property::Entry { name: prop_name, value, .. } = p {
                if prop_name.as_str() == name {
                    Some(value)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap_or_else(|| panic!("{} property not found", name));

    match prop {
        Value::Array(a) => a,
        _ => panic!("{} is not an array", name),
    }
}

// Helper function to verify a macro in an array item
fn verify_array_item(item: &Item, expected: &str) {
    match item {
        Item::Str(s) => assert_eq!(s.value(), expected, "String value mismatch"),
        Item::Number(n) => assert_eq!(n.to_string(), expected, "Number value mismatch"),
        Item::Macro(m) => {
            let macro_str = format!("{}({})", 
                m.name().value(),
                m.args().iter()
                    .map(|a| a.value().to_string())
                    .collect::<Vec<_>>()
                    .join(","));
            assert_eq!(macro_str, expected, "Macro value mismatch");
        },
        Item::Array(_a) => panic!("Unexpected array when expecting {}", expected),
        Item::Invalid(_) => panic!("Invalid item when expecting {}", expected),
    }
} 