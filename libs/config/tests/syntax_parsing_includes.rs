use hemtt_config::{Class, Property, Value, Item, Config};

//------------------------------------------------------------------------------
// GUIDE: HOW TO ADD NEW TEST COVERAGE
//------------------------------------------------------------------------------
// 1. CAPTURE REAL SYNTAX PATTERNS:
//    - Tests should preserve the exact syntax structure found in real config files
//    - Transfer the pattern faithfully, maintaining spacing, structure, and style
//    - When adding a new test, examine actual config files to find unique patterns
//
// 2. USE GENERIC PROPERTY NAMES:
//    - Replace domain-specific names with generic ones (foo, bar, baz, etc.)
//    - Maintain the original syntax style (camelCase, snake_case, etc.)
//    - The focus is on testing the syntax pattern, not specific property names
//
// 3. TEST COVERAGE SHOULD BE MINIMAL BUT COMPLETE:
//    - Each test should focus on a single syntax pattern
//    - Include only the necessary code to test that pattern
//    - Be thorough - cover edge cases found in the wild
//
// 4. CREATING A NEW TEST:
//    - Use the test_config_parse! macro
//    - Name the test clearly (test_feature_being_tested)
//    - Provide a minimal example that isolates the syntax feature
//    - Include a descriptive error message
//
// 5. EXAMPLE PROCESS:
//    - Find interesting syntax in real config: `suspTravelDirection[] = SUSPTRAVELDIR_LEFT;`
//    - Create test with generic name: `directionArray[] = CONST_VALUE;`
//    - Preserve exact syntax structure while making names generic
//
//------------------------------------------------------------------------------

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

macro_rules! test_config_parse {
    ($test_name:ident, $config:expr, $message:expr) => {
        #[test]
        fn $test_name() {
            let config = $config;
            let result = parse_config(config);
            assert!(result.is_ok(), $message);
        }
    };
}


//==============================================================================
// INCLUDE STATEMENT SYNTAX TESTS
//==============================================================================
// Tests for #include statements with various paths and formatting
//==============================================================================

test_config_parse!(
    test_basic_includes,
    r#"
    // Basic include at the file root
    #include "basic_include.hpp"
    
    // Include with different extensions
    #include "path/to/config.h"
    #include "path/to/settings.sqf"
    
    // Include with path variations
    #include "\absolute\path\file.hpp"
    #include "/forward/slash/path.hpp"
    #include "relative/path.hpp"
    
    class BasicConfig {
        property = 1;
    };
    "#,
    "Failed to parse basic include statements"
);

//==============================================================================
// RELATIVE PATH INCLUDE TESTS
//==============================================================================
// Tests for includes with various relative path formats and notations
//==============================================================================

test_config_parse!(
    test_relative_path_includes,
    r#"
    // Parent directory reference with backslash
    #include "..\parent_directory.hpp"
    
    // Parent directory reference with forward slash
    #include "../another_parent_file.hpp"
    
    // Multiple parent directories
    #include "..\..\two_levels_up.hpp"
    #include "../../alternative_two_levels.hpp"
    
    // Mixed slash styles in path
    #include "..\mixed/style/path.hpp"
    
    // Current directory with explicit notation
    #include ".\current_directory.hpp"
    #include "./alternative_current.hpp"
    
    // Deeply nested relative paths
    #include "..\..\..\..\deep\relative\path.hpp"
    
    class Config {
        value = 100;
    };
    "#,
    "Failed to parse includes with relative paths"
);

//==============================================================================
// INCLUDE STATEMENT FORMATTING VARIATIONS
//==============================================================================
// Tests for various formatting styles in include statements
//==============================================================================

test_config_parse!(
    test_include_formatting,
    r#"
    // Standard include
    #include "standard.hpp"

    // Include with extra whitespace
    #include     "extra_space.hpp"

    // Include with tab indentation
    	#include "tabbed_include.hpp"

    // Include with unusual spacing around path
    #include      "spaced_path.hpp"     

    // Include with line breaks
    #include
    "broken_line.hpp"

    // Multiple includes each on their own line
    #include "first.hpp"
    #include "second.hpp"

    // Include with trailing comments
    #include "commented.hpp" // This is a comment

    class Config {
        property = 1;
    };
    "#,
    "Failed to parse includes with unusual formatting"
);

//==============================================================================
// INCLUDES INSIDE CLASS DEFINITIONS
//==============================================================================
// Tests for #include statements appearing inside class bodies
//==============================================================================

test_config_parse!(
    test_includes_inside_classes,
    r#"
    class Vehicle {
        // Include at the start of a class
        #include "vehicle_properties.hpp"
        
        // Properties mixed with includes
        enginePower = 1000;
        
        // Include in the middle of properties
        #include "vehicle_armor.hpp"
        
        maxSpeed = 120;
        
        // Nested class with include
        class Engine {
            #include "engine_properties.hpp"
            type = "V8";
        };
        
        // Include at the end of a class
        #include "vehicle_animations.hpp"
    };
    "#,
    "Failed to parse includes inside class definitions"
);

//==============================================================================
// INCLUDES IN ARRAYS AND COMPLEX STRUCTURES
//==============================================================================
// Tests for #include statements used within arrays and other structures
//==============================================================================

test_config_parse!(
    test_includes_in_arrays,
    r#"
    class SoundConfig {
        // Include in array initialization
        soundSets[] = {
            #include "common_sounds.hpp",

            "AdditionalSound1",
            "AdditionalSound2"
        };
        
        // Multiple includes in an array
        effectSets[] = {
            #include "effect_set1.hpp",

            #include "effect_set2.hpp",

            "ManualEffect"
        };
        
        // Nested arrays with includes
        complexSounds[] = {
            {
                #include "engine_sounds.hpp",

                "EngineExtra"
            },
            {
                #include "movement_sounds.hpp"

            }
        };
        
        // Include with trailing comma in array
        optionalSounds[] = {
            #include "optional_sound_list.hpp",

        };
    };
    "#,
    "Failed to parse includes within arrays"
);

//==============================================================================
// REAL WORLD INCLUDE PATTERNS
//==============================================================================
// Tests based on real-world examples from actual config files like 2S1.hpp
//==============================================================================

test_config_parse!(
    test_real_world_include_patterns,
    r#"
    // Vehicle config based on real patterns from 2S1.hpp
    attenuationType = "StandardAttenuation";
    
    engineSound[] = {"path\to\sound", db-5, 1.0};
    
    // Include outside of any class
    #include "..\collisions_standard.hpp"

    class Sounds
    {
        // Include at the start of class body with different spacing patterns
        #include "settings.hpp"

        
        soundSetsInt[] =
        {
            // Include inside array with comma
            #include "..\general_vehicle_int.hpp",

            
            Movement_Int_SoundSet,
            Surface_Int_SoundSet,
            //Commented_SoundSet,
            Extra_Int_SoundSet,
        };
        
        
        soundSetsExt[] =
        {
            // Include inside array followed by other elements
            #include "..\general_vehicle_ext.hpp",

            
            Movement_Ext_SoundSet,
            Surface_Ext_SoundSet,
            Collision_SoundSet
        };
    };
    "#,
    "Failed to parse real-world include patterns"
);

//==============================================================================
// COMPLEX MIXED INCLUDE SCENARIOS
//==============================================================================
// Tests with a mix of different include styles and edge cases
//==============================================================================

test_config_parse!(
    test_complex_mixed_includes,
    r#"
    // Root level include
    #include "root_config.hpp"

    
    // Base vehicle properties
    maxSpeed = 100;
    
    class Vehicle {
        // Standard properties
        enginePower = 1000;
        
        // Include inside class
        #include "vehicle_core.hpp"

        
        // Nested class with includes
        class Turret {
            // Include at start of nested class
            #include "turret_base.hpp"

            
            elevation = 45;
            
            // Multiple consecutive includes
            #include "turret_animations.hpp"

            #include "turret_weapons.hpp"

            
            // Include with parent directory path
            #include "..\weapons\ammunition.hpp"

        };
        
        // Complex array with includes
        animations[] = {
            // Include in array with leading comma
            #include "anim_set1.hpp",

            
            // Standalone animation entries
            "Anim1",
            "Anim2",
            
            // Include at the end of array
            #include "anim_set2.hpp"

        };
        
        // Extreme inclusion pattern from real configs
        class Sounds {
            soundSets[] = {
                // Multiple nested includes in arrays
                #include "..\common\basic_sounds.hpp",

                #include "..\common\advanced_sounds.hpp",

                
                // Normal entries mixed with includes
                "CustomSound1",
                #include "special_sounds.hpp",

                "CustomSound2"
            };
        };
    };
    "#,
    "Failed to parse complex mixed include patterns"
);