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
// BASIC ARRAY TESTS
//==============================================================================
// Tests for basic array syntax and simple array structures
//==============================================================================

test_config_parse!(
    test_basic_arrays,
    r#"
    class Test {
        // Simple array with numbers
        numericArray[] = {1, 2, 3};
        
        // Array with strings
        stringArray[] = {"one", "two", "three"};
        
        // Array with booleans
        booleanArray[] = {true, false, true};
        
        // Mixed type array
        mixedArray[] = {1, "two", true};
        
        // Array with a single element
        singleElementArray[] = {1};
        
        // Empty array
        emptyArray[] = {};
    };
    "#,
    "Failed to parse basic arrays"
);

//==============================================================================
// ARRAY FORMATTING TESTS
//==============================================================================
// Tests for various array formatting styles
//==============================================================================

test_config_parse!(
    test_array_formatting,
    r#"
    class Test {
        // Single line array
        singleLineArray[] = {1, 2, 3, 4, 5};
        
        // Multi-line array
        multiLineArray[] = {
            1,
            2,
            3,
            4,
            5
        };
        
        // Mixed spacing and line breaks
        mixedFormatArray[] = {1, 2,
            3, 4,
            5
        };
        
        // Array with inconsistent spacing
        inconsistentSpacingArray[] = {1,2,   3,  4,5};
        
        // Array with trailing comma
        trailingCommaArray[] = {1, 2, 3,};
        
        // Array with items on the same line and different lines
        mixedLineArray[] = {
            "item1", "item2", "item3",
            "item4",
            "item5", "item6"
        };
    };
    "#,
    "Failed to parse array formatting variations"
);

//==============================================================================
// NESTED ARRAY TESTS
//==============================================================================
// Tests for arrays containing other arrays at various nesting levels
//==============================================================================

test_config_parse!(
    test_nested_arrays,
    r#"
    class Test {
        // Simple nested array
        nestedArray[] = {{1, 2}, {3, 4}};
        
        // Deeply nested array (3 levels)
        deeplyNestedArray[] = {{{1, 2}, {3, 4}}, {{5, 6}, {7, 8}}};
        
        // Nested array with mixed nesting depths
        mixedNestingArray[] = {{1, 2}, 3, {4, {5, 6}}};
        
        // Nested array with formatting
        formattedNestedArray[] = {
            {1, 2},
            {3, 4},
            {
                5, 
                6
            }
        };
        
        // Nested array with empty elements
        nestedEmptyArray[] = {{}, {1, 2}, {}};
        
        // Nested array with single empty element
        singleNestedEmptyArray[] = {{}};
    };
    "#,
    "Failed to parse nested arrays"
);

//==============================================================================
// ARRAY WITH SPECIAL VALUES TESTS
//==============================================================================
// Tests for arrays containing special values like constants, macros, and expressions
//==============================================================================

test_config_parse!(
    test_arrays_with_special_values,
    r#"
    class Test {
        // Array with constants
        constantArray[] = {CONST_VALUE1, CONST_VALUE2, CONST_VALUE3};
        
        // Array with macros
        macroArray[] = {MACRO(arg1), MACRO(arg2), MACRO(arg3)};
        
        // Array with mixed macros and values
        mixedMacroArray[] = {1, MACRO(arg), "string"};
        
        // Array with unquoted paths
        pathArray[] = {\path\to\file1.paa, \path\to\file2.paa};
        
        // Array with hex values
        hexArray[] = {0xAB, 0xCD, 0xEF, 0xFF};
        
        // Array with special constants
        specialConstantArray[] = {db+5, db+10, factor[0,1]};
        
        // Inheriting a constant array
        arrayInheritance[] = PARENT_ARRAY;
    };
    "#,
    "Failed to parse arrays with special values"
);

//==============================================================================
// ARRAY WITH EXPRESSIONS TESTS
//==============================================================================
// Tests for arrays containing expressions and calculations
//==============================================================================

test_config_parse!(
    test_arrays_with_expressions,
    r#"
    class Test {
        // Array with simple expressions
        expressionArray[] = {0.5 + 0.1, 0.5 * 2.0, 10 / 2};
        
        // Array with nested expressions
        nestedExpressionArray[] = {(0.5 + 0.1) * 2, 0.5 + (0.1 * 2)};
        
        // Array with __EVAL expressions
        evalArray[] = {__EVAL(10*5), __EVAL(sin(3.14/2))};
        
        // Array with complex __EVAL expressions
        complexEvalArray[] = {
            {__EVAL(0/2100), __EVAL(0/883)},
            {__EVAL(900/2100), __EVAL(810/883)},
            {__EVAL(1100/2100), __EVAL(825/883)}
        };
        
        // Array with math expressions in positions
        positionArray[] = {
            {{-0.042+0.88}, {-0.075+0.9}},
            {{+0.042+0.88}, {-0.075+0.9}},
            {{+0.042+0.88}, {+0.075+0.9}}
        };
    };
    "#,
    "Failed to parse arrays with expressions"
);

//==============================================================================
// ARRAY MODIFICATION TESTS
//==============================================================================
// Tests for array expansion and other modification patterns
//==============================================================================

test_config_parse!(
    test_array_modifications,
    r#"
    class Test {
        // Basic array expansion
        baseArray[] = {1, 2};
        expandArray[] += {3, 4};
        
        // Expanded array with strings
        baseStringArray[] = {"a", "b"};
        expandedStringArray[] += {"c", "d"};
        
        // Array expansion with macros
        baseMacroArray[] = {MACRO(a), MACRO(b)};
        expandedMacroArray[] += {MACRO(c), MACRO(d)};
        
        // Multiple expansions
        multiBase[] = {1};
        multiExpand1[] += {2};
        multiExpand2[] += {3};
        multiExpand3[] += {4};
    };
    "#,
    "Failed to parse array modifications"
);

//==============================================================================
// ARRAY STRUCTURE PATTERN TESTS
//==============================================================================
// Tests for common array structure patterns found in real configs
//==============================================================================

test_config_parse!(
    test_array_structure_patterns,
    r#"
    class Test {
        // Key-value pair pattern (common in gearbox ratios, etc.)
        keyValueArray[] = {
            "Key1", 1.5,
            "Key2", 2.5,
            "Key3", 3.5
        };
        
        // Array with empty elements
        emptyElementArray[] = {
            {Object1, {0.0, -0.16}, 1},
            {},
            {Object1, {0.0, 0.16}, 1}
        };
        
        // Sound array pattern
        soundArray[] = {"path\to\sound", db+5, 1, 9};
        
        // Color array pattern
        colorArray[] = {1, 1, 1, 0.5};
        
        // Position array pattern
        posArray[] = {{0.5, 0.5}, 1};
        
        // Control list pattern
        controlsArray[] = {"Control1", "Control2", "Control3"};
        
        // Node array pattern
        nodeArray[] = {{"Node1", 1}, {"Node2", 2}, {"Node3", 3}};
    };
    "#,
    "Failed to parse common array structure patterns"
);

//==============================================================================
// COMPLEX ARRAY EDGE CASE TESTS
//==============================================================================
// Tests for complex array syntax edge cases found in real configs
//==============================================================================

test_config_parse!(
    test_array_edge_cases,
    r#"
    class Test {
        // Array with deeply nested braces in expressions
        complexBraceArray[] = {
            {
                {{-0.042+0.88},{-0.075+0.9},1},
                {{+0.042+0.88},{-0.075+0.9},1},
                {{+0.042+0.88},{+0.075+0.9},1},
                {{-0.042+0.88},{+0.075+0.9},1}
            }
        };
        
        // Array with profile namespace variables
        namespaceArray[] = {
            "(profilenamespace getvariable ['COLOR_R',0])",
            "(profilenamespace getvariable ['COLOR_G',1])",
            "(profilenamespace getvariable ['COLOR_B',1])",
            "(profilenamespace getvariable ['COLOR_A',0.8])"
        };
        
        // Array with irregular nesting
        irregularNestingArray[] = {
            {0, 0},
            {{0, 0}, {1, 1}},
            {{{0, 0}, {1, 1}}, {{2, 2}, {3, 3}}}
        };
        
        // Diary record array pattern
        diaryArray[] = {
            "Diary",
            ["Title", "Content with \"quotes\" and special characters"]
        };
        
        // Array with macro-generated content
        macroContentArray[] = {MACRO_CONTENT(param1, param2)};
        
        // Magazine array pattern from RHS configs
        magazinesArray[] = {
            "magazine1",
            "magazine2",
            MACRO_MAGAZINES
        };
    };
    "#,
    "Failed to parse array edge cases"
);

//==============================================================================
// DOMAIN-SPECIFIC ARRAY PATTERNS
//==============================================================================
// Tests for domain-specific array patterns found in real configs
//==============================================================================

test_config_parse!(
    test_domain_specific_arrays,
    r#"
    class Test {
        // Vehicle suspension array pattern
        suspensionArray[] = {0.42, 0.75, 0.51, 2.0, 6.7};
        
        // Physics dampening array
        dampeningArray[] = {0.1, 0.3, 0.0, 0.0, 0.0, 0.0, 0.0};
        
        // Weapon magazines array
        magazinesArray[] = {
            "magazine1",
            "magazine2",
            "magazine3",
            WEAPON_MAGAZINES_MACRO
        };
        
        // Gear box ratios
        gearboxArray[] = {
            "R1", -2.235,
            "N", 0,
            "D1", 2.6,
            "D2", 1.5,
            "D3", 1.125
        };
        
        // Model selection array
        selectionArray[] = {
            "piece1",
            "piece2",
            "piece3"
        };
        
        // UI controls array
        controlsArray[] = {
            "Control1", "Control2", "Control3", "Control4", "Control5", 
            "Control6", "Control7", "Control8", "Control9", "Control10"
        };
        
        // Animation array
        animationArray[] = {
            "Animation1", 0,
            "Animation2", 1
        };
    };
    "#,
    "Failed to parse domain-specific array patterns"
);

//==============================================================================
// PREPROCESSOR AND MACRO ARRAY TESTS
//==============================================================================
// Tests for arrays with preprocessor directives and macros
//==============================================================================

test_config_parse!(
    test_preprocessor_arrays,
    r#"
    #define ARRAY_MACRO(value) {value, value*2, value*3}
    #define COLOR_ARRAY(r,g,b,a) {r, g, b, a}
    
    class Test {
        // Array using define macro
        macroArray[] = ARRAY_MACRO(10);
        
        // Color array using macro
        colorArray[] = COLOR_ARRAY(1,1,1,0.5);
        
        // Nested array with macros
        nestedMacroArray[] = {{MACRO(a), MACRO(b)}, {1, 2}};
        
        // Array with preprocessor concat
        concatArray[] = {TOKEN##1, TOKEN##2, TOKEN##3};
        
        // Array with quoted macros
        quotedMacroArray[] = {
            QUOTE(MACRO_VALUE1),
            QUOTE(MACRO_VALUE2),
            QUOTE(MACRO_VALUE3)
        };
        
        // Array with conditional content
        #ifdef CONDITION
            conditionalArray[] = {1, 2, 3};
        #else
            conditionalArray[] = {4, 5, 6};
        #endif
    };
    "#,
    "Failed to parse preprocessor and macro arrays"
);

//==============================================================================
// ARRAY WITH CODE/STRING PATTERNS
//==============================================================================
// Tests for arrays containing code snippets, complex strings, and expressions
//==============================================================================

test_config_parse!(
    test_code_in_arrays,
    r#"
    class Test {
        // Array with SQF code snippets
        codeArray[] = {
            "if (true) then {hint 'Hello';}",
            "player setDamage 0;",
            "systemChat str _this;"
        };
        
        // Array with function calls
        functionArray[] = {
            "[player, 'action1'] call fnc_doAction;",
            "['param1', 'param2'] call fnc_otherAction;"
        };
        
        // Array with complex path strings
        pathArray[] = {
            "\path\to\file.sqf",
            "pca\addons\module\functions\fnc_name.sqf",
            "z\ace\addons\module\data\image.paa"
        };
        
        // Array with complex strings containing quotes
        complexStringArray[] = {
            "String with ""nested"" quotes",
            "Another string with ""more"" nested ""quotes"""
        };
        
        // Array with expressions in strings
        expressionArray[] = {
            "safezoneX + (safezoneW * 0.5)",
            "(safezoneY + safezoneH) - (1 * ((((safezoneW / safezoneH) min 1.2) / 1.2) / 25))"
        };
    };
    "#,
    "Failed to parse arrays with code and string patterns"
);

//==============================================================================
// IRREGULAR ARRAY FORMATTING TESTS
//==============================================================================
// Tests for arrays with unusual spacing, indentation, and formatting patterns
// to ensure the parser is robust against non-uniform array formatting
//==============================================================================

test_config_parse!(
    test_irregular_array_formatting,
    r#"
    class   IrregularArrays    {
        // Arrays with extra spaces around brackets and equals sign
        normalArray     [ ]     =      {1, 2, 3};
        
        // Arrays with no spaces
        compactArray[]={4,5,6};
        
        // Arrays with unusual spacing between elements
        spacedArray[] = {7,   8,        9};
        
        // Arrays with tab indentation
        	tabArray[] = {10, 11, 12};
            	deepTabArray[]={13,14,15};
        
        // Arrays with excessive line breaks and spacing
        multiLine
            [
            ] 
                = 
                    {
                    16, 
                    17, 
                    18
                    };
        
        // Array with no spaces before semicolon and next array
        noSpaceArray[]={19,20,21};nextArray[]={22,23,24};
        
        // Nested arrays with unusual spacing
        nestedArray[ ]={{  25,26  }, {  27  ,  28}   };
        
        // Extremely sparse formatting with nested arrays
        verySpaced  [   ]   =   {
                {
                    29  ,  
                    
                    30
                }   ,
                
                
                {
                    31,
                    
                    32
                }
            }   ;
            
        // Array with irregular macro spacing
        macroArray  [  ]  =  {   MACRO  (  a  )  ,   MACRO  (  b  )   }  ;
        
        // Mixed styles (compact and verbose)
        compactArray[]={33,34,35};verboseArray  [  ]  =  {  36  ,  37  ,  38  }  ;
        
        // Array with comments inside
        commentedArray[] = {
            39, // Comment after element
            
            // Comment before element
            40,
            
            /* Block comment before element */
            41
        };
    };
    "#,
    "Failed to parse arrays with irregular spacing and formatting"
);

//==============================================================================
// EXTREME ARRAY FORMATTING TESTS
//==============================================================================
// Tests with extremely unusual array syntax formatting to push the parser to its limits
//==============================================================================

test_config_parse!(
    test_extreme_array_formatting,
    r#"
    class 
    
    
    ExtremeArrayFormatting 
    
    
    {
        // Arrays with scattered brackets and elements
        array1
        [
        
        ]
        =
        {
            1,
            
            
            2,
            
            
            
            3
        }
        ;
        
        // Arrays with no space before semicolon and next array
        array2[]={4,5,6};array3[]={7,8,9};
        
        // Nested arrays with bizarre spacing
        array4 [
        
        ]={
        {10    ,
            
                11},
            {    12,13
        }
            };
            
        // Multiple arrays on a single line with no spaces
        single1[]={14};single2[]={15};single3[]={16};
        
        // Multiple arrays on a single line with mixed spacing
        mixed1[]=  {17};  mixed2  [  ]={18};mixed3[]  =  {  19  };
        
        // Arrays with excessive newlines between elements
        sparseArray[]={
            20,
            
            
            
            21,
            
            
            22
        };
        
        // Complex nested structure with inconsistent formatting
        nestedComplex[
        
        ] 
          = 
            {
                {
                    {
                        23  ,   24
                    }   ,
                    
                    {
                        25,
                        
                        
                        26
                    }
                }   ,
                
                {
                    {27,28},
                    {   29   ,    30   }
                }
            }   ;
            
        // Mixed styles within a single array definition
        compactElements[]={31,32,
        
        33,
        
        34,35};
        
        // Very compact array declarations
        veryCompact[]={{{{36,37},{38,39}},{{40,41},{42,43}}}};
        
        // Array elements on same line with comment inside braces
        commentInArray[]={44 /* inline comment */, 45, /* another comment */ 46};
        
        // Array with columns aligned oddly (like in some real configs)
        columnArray[] = {
            47,     48,     49,
            50,     51,     52,
            53,     54,     55
        };
    };
    "#,
    "Failed to parse arrays with extreme formatting variations"
);