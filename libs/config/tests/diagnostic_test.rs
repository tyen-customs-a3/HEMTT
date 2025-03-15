use hemtt_common::config::ProjectConfig;
use hemtt_preprocessor::Processor;
use hemtt_workspace::{LayerType, reporting::WorkspaceFiles};
use std::path::PathBuf;

#[test]
fn test_parse_errors_diagnostic() {
    // Create a test config with various parsing errors
    let test_config = r#"
    class BadClass {
        missingQuotes = hello;  // Missing quotes around string
        missingBrace = {1, 2, 3  // Missing closing brace
        invalidNumber = 12.34.56;  // Invalid number format
        class NestedClass: NonExistentParent {  // Reference to non-existent parent
            array[] = {1, invalid, 3};  // Invalid array element
        };
    };
    "#;

    // Create a temporary directory for the test
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test.hpp");
    std::fs::write(&test_file, test_config).unwrap();

    // Set up workspace
    let workspace = hemtt_workspace::Workspace::builder()
        .physical(&PathBuf::from(temp_dir.path()), LayerType::Source)
        .finish(None, false, &hemtt_common::config::PDriveOption::Disallow)
        .unwrap();

    // Process and parse the config
    let source = workspace.join("test.hpp").unwrap();
    let processed = Processor::run(&source).unwrap();
    println!("Processed output:\n{}", processed.as_str());
    
    let parsed = hemtt_config::parse(None, &processed);
    let workspacefiles = WorkspaceFiles::new();

    // Get diagnostic output
    let diagnostic_output = match &parsed {
        Ok(config) => {
            // Even successful parse might have warnings/notes
            config
                .codes()
                .iter()
                .map(|e| e.diagnostic().unwrap().to_string(&workspacefiles))
                .collect::<Vec<_>>()
                .join("\n")
        }
        Err(errors) => {
            errors
                .iter()
                .map(|e| e.diagnostic().unwrap().to_string(&workspacefiles))
                .collect::<Vec<_>>()
                .join("\n")
        }
    };

    // Print the diagnostic output for inspection
    println!("\nFull diagnostic output:\n{}", diagnostic_output);
    println!("\nParse result: {:?}", parsed.is_ok());

    // The diagnostic output should contain:
    // 1. Error about missing quotes (L-C01)
    // 2. Error about invalid value for missing brace
    // 3. Error about invalid number format
    // 4. Error about non-existent parent class (L-C04)
    // 5. Error about invalid array element
    assert!(diagnostic_output.contains("[L-C01]") && diagnostic_output.contains("use quotes"), 
           "Should show error about missing quotes");
    assert!(diagnostic_output.contains("invalid value") && diagnostic_output.contains("missingBrace"), 
           "Should show error about invalid value for missing brace");
    assert!(diagnostic_output.contains("invalid value") && diagnostic_output.contains("12.34.56"), 
           "Should show error about invalid number format");
    assert!(diagnostic_output.contains("[L-C04]") && diagnostic_output.contains("NonExistentParent"), 
           "Should show error about non-existent parent");
    assert!(diagnostic_output.contains("invalid value") && diagnostic_output.contains("array[]"), 
           "Should show error about invalid array element");
}