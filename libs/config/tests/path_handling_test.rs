use hemtt_preprocessor::Processor;
use hemtt_workspace::{LayerType, Workspace};
use hemtt_common::config::PDriveOption;
use tempfile::tempdir;
use std::path::PathBuf;

/// Test that paths with backslashes are handled correctly
#[test]
fn test_path_with_backslashes() {
    // Test cases with different backslash patterns
    let test_paths = [
        ("model", r#""data\bandage.p3d""#, "data\\bandage.p3d"),
        ("picture", r#""ui\fieldDressing_ca.paa""#, "ui\\fieldDressing_ca.paa"),
        ("singleSlash", r#""data\folder\file.p3d""#, "data\\folder\\file.p3d"),
        ("doubleSlash", r#""data\\folder\\file.p3d""#, "data\\\\folder\\\\file.p3d"),
        ("mixedSlash", r#""data\folder\\file.p3d""#, "data\\folder\\\\file.p3d"),
    ];

    // Create config content with all test cases
    let config_content = format!(r#"
    class CfgWeapons {{
        class ACE_fieldDressing {{
            {entries}
        }};
    }};
    "#, entries = test_paths.iter()
           .map(|(name, raw_path, _)| format!("{} = {};", name, raw_path))
           .collect::<Vec<_>>()
           .join("\n            "));

    // Create a temporary directory
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let temp_path = PathBuf::from(temp_dir.path());
    
    let workspace = Workspace::builder()
        .physical(&temp_path, LayerType::Source)
        .finish(None, false, &PDriveOption::Disallow)
        .unwrap();

    // Write the content to a temporary file
    let temp_file = temp_dir.path().join("path_test.hpp");
    std::fs::write(&temp_file, config_content).unwrap();

    // Process and parse the file
    let source = workspace.join("path_test.hpp").unwrap();
    let processed = Processor::run(&source).unwrap();
    let parsed = hemtt_config::parse(None, &processed).unwrap();
    let config = parsed.into_config();

    // Get the ACE_fieldDressing class
    let cfg_weapons = config.0.iter()
        .find_map(|p| {
            if let hemtt_config::Property::Class(c) = p {
                if c.name().map_or(false, |n| n.as_str() == "CfgWeapons") {
                    Some(c)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .expect("CfgWeapons class not found");

    let field_dressing = cfg_weapons.properties().iter()
        .find_map(|p| {
            if let hemtt_config::Property::Class(c) = p {
                if c.name().map_or(false, |n| n.as_str() == "ACE_fieldDressing") {
                    Some(c)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .expect("ACE_fieldDressing class not found");

    // Verify each test case
    for (name, raw_path, expected) in test_paths.iter() {
        let value = field_dressing.properties().iter()
            .find_map(|p| {
                if let hemtt_config::Property::Entry { name: prop_name, value, .. } = p {
                    if prop_name.as_str() == *name {
                        Some(value)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .unwrap_or_else(|| panic!("{} property not found", name));

        // Print actual vs expected for debugging
        if let hemtt_config::Value::Str(s) = value {
            println!("{} path:", name);
            println!("  Raw in config: {}", raw_path);
            println!("  Parsed value: {}", s.value());
            println!("  Expected: {}", expected);
            assert_eq!(s.value(), *expected, 
                "Path '{}' was not preserved correctly.\nExpected: {}\nGot: {}", 
                name, expected, s.value()
            );
        } else {
            panic!("{} is not a string", name);
        }
    }
} 