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
// BASIC PROPERTY SYNTAX TESTS
//==============================================================================
// Tests for basic property value syntax including different data types,
// assignment patterns, and simple value expressions
//==============================================================================

test_config_parse!(
    test_basic_properties,
    r#"
    class Test {
        // Integer values
        intValue = 42;
        
        // Decimal values
        decimalValue = 3.14;
        
        // String values with different quote styles
        stringValue = "Hello World";
        
        // Boolean values
        boolValue = true;
        
        // Multiple properties with different spacing
        prop1=1;
        prop2 = 2;
        prop3  =  3;
        
        // Property naming variations
        camelCaseProperty = 1;
        snake_case_property = 2;
        UPPERCASE_PROPERTY = 3;
        property_With_Mixed_Style = 4;
        property123 = 5;
    };
    "#,
    "Failed to parse basic property syntax"
);

//==============================================================================
// NUMERIC VALUE EDGE CASES
//==============================================================================
// Tests for various numeric value formats including scientific notation,
// hexadecimal values, and large numbers
//==============================================================================

test_config_parse!(
    test_numeric_values,
    r#"
    class Test {
        // Scientific notation
        sciNotation1 = 1.5e6;
        sciNotation2 = 1e+009;
        sciNotation3 = 1.5e-3;
        
        // Hexadecimal values
        hexValue1 = 0xAB;
        hexValue2 = 0xCDEF;
        hexValue3 = 0xFF00FF;
        
        // Hex values in arrays
        hexArray[] = {0xAB, 0xCD, 0xEF, 0xFF};
        
        // Large numeric values
        largeValue1 = 330000;
        largeValue2 = 1000000;
        largeValue3 = 2147483647;
        
        // Negative values
        negativeValue = -42;
        negativeFloat = -3.14;
        negativeExp = -1.5e3;
    };
    "#,
    "Failed to parse numeric value edge cases"
);

//==============================================================================
// STRING VALUE EDGE CASES
//==============================================================================
// Tests for various string value formats including escaped characters,
// path formats, and unquoted strings
//==============================================================================

test_config_parse!(
    test_string_values,
    r#"
    class Test {
        // Basic quoted strings
        quotedString = "Standard string";
        
        // Escaped characters in strings
        escapedBackslash = "path\to\file\name";
        escapedQuotes = "String with \"quotes\" inside";
        
        // Unquoted paths (special case in Arma configs)
        unquotedPath = \path\to\file\name;
        modelPath = \path\to\model.p3d;
        
        // Path variants
        backslashPath = "\path\to\file";
        forwardSlashPath = "/path/to/file";
        mixedPath = "\path/to\file";
        
        // Strings with expressions (interpreted as-is in Arma)
        expressionInString = "value * 0.5";
        functionInString = "call someFunction";
        
        // Complex paths
        complexPath1 = "\path\to\file.paa";
        complexPath2 = "pca\addons\module\data\image.paa";
        complexPath3 = "z\ace\addons\module\data\image.paa";
    };
    "#,
    "Failed to parse string value edge cases"
);

//==============================================================================
// CONSTANT AND SPECIAL VALUE TESTS
//==============================================================================
// Tests for constant values, special syntax, and non-standard assignments
//==============================================================================

test_config_parse!(
    test_constant_values,
    r#"
    class Test {
        // Unquoted constant values (often from #define)
        constantValue = SOME_CONSTANT;
        directionValue = DIRECTION_LEFT;
        
        // Special syntax combinations
        specialValue1 = db+5;
        specialValue2 = value factor[0,1];
        
        // Boolean constants
        boolTrue = true;
        boolFalse = false;
        
        // Mixed constants with operations
        mixedConstant = CONSTANT * 0.5;
        
        // Special db notation in array (common in sound configs)
        soundProperty[] = {"path\to\sound", db+8, 1, 25};
    };
    "#,
    "Failed to parse constant and special values"
);

//==============================================================================
// PROPERTY ASSIGNMENT PATTERNS
//==============================================================================
// Tests for various property assignment patterns including arrays,
// expressions, and special syntax structures
//==============================================================================

test_config_parse!(
    test_property_assignments,
    r#"
    class Test {
        // Standard assignment
        standardProp = 1;
        
        // Simple array assignment
        arrayProp[] = {1, 2, 3};
        
        // Array assignment with strings
        stringArray[] = {"one", "two", "three"};
        
        // Array expansion (append to existing array)
        baseArray[] = {1, 2};
        expandArray[] += {3, 4};
        
        // Mixed array types
        mixedArray[] = {1, "string", CONSTANT};
        
        // Nested arrays
        nestedArray[] = {{1, 2}, {3, 4}};
        
        // Array with empty element
        emptyElementArray[] = {
            {1, 2, 3},
            {},
            {4, 5, 6}
        };
        
        // Comma style variations
        commaArray[] = {
            "item1",
            "item2",
            "item3","item4",
            "item5"
        };
        
        // Expressions in arrays
        expressionArray[] = {0.5 + 0.1, 0.5 * 2.0};
    };
    "#,
    "Failed to parse property assignment patterns"
);

//==============================================================================
// COMPLEX ARRAY STRUCTURES
//==============================================================================
// Tests for sophisticated array structures with nested elements,
// expressions, and special formatting
//==============================================================================

test_config_parse!(
    test_complex_arrays,
    r#"
    class Test {
        // Deeply nested arrays
        deepNested[] = {{{1, 2}, {3, 4}}, {{5, 6}, {7, 8}}};
        
        // Arrays with expressions
        expressionArray[] = {
            {__EVAL(0/2100), __EVAL(0/883)},
            {__EVAL(900/2100), __EVAL(810/883)}
        };
        
        // Arrays with nested braces in expressions
        braceArray[] = {
            {
                {{-0.042+0.88}, {-0.075+0.9}},
                {{+0.042+0.88}, {-0.075+0.9}},
                {{+0.042+0.88}, {+0.075+0.9}}
            }
        };
        
        // Mixed format arrays (common in configs)
        mixedFormatArray[] = {
            "Label1", 10,
            "Label2", 20,
            "Label3", 30
        };
        
        // Arrays with alternating string/value pairs (like in gearbox configs)
        pairArray[] = {
            "P1", 1.5,
            "P2", 2.5,
            "P3", 3.5
        };
        
        // Arrays with database notation (common in sound configs)
        soundArray[] = {"path\to\sound", db+5, 1, 9};
    };
    "#,
    "Failed to parse complex array structures"
);

//==============================================================================
// CONDITION AND EXPRESSION PROPERTIES
//==============================================================================
// Tests for properties containing code-like expressions, conditions,
// and formula patterns
//==============================================================================

test_config_parse!(
    test_expressions,
    r#"
    class Test {
        // Simple arithmetic in conditions
        condition1 = "param1 * param2 * 0.5";
        
        // Complex conditions with parentheses
        condition2 = "(value1 >= 137) * (value1 <= 211)";
        
        // Conditions with functions
        condition3 = "func1 this && (this func2 'param')";
        
        // Complex statement with multiple operations
        statement = "this spawn {sleep 1.2; private _var = 'value'}";
        
        // Profile namespace references
        profileValue = "(profilenamespace getvariable ['COLOR_R', 0])";
        
        // Positioning with complex expressions
        position = "safezoneX + (safezoneW * 0.5)";
        
        // Arrays of expressions
        expressionArray[] = {
            "(profilenamespace getvariable ['COLOR_R', 0])",
            "(profilenamespace getvariable ['COLOR_G', 1])"
        };
    };
    "#,
    "Failed to parse conditions and expressions"
);

//==============================================================================
// UI PROPERTY PATTERNS
//==============================================================================
// Tests for properties commonly found in UI configurations
// such as controls, fonts, and visual elements
//==============================================================================

test_config_parse!(
    test_ui_properties,
    r#"
    class Test {
        // Basic UI properties
        idc = 1234;
        type = 0;
        style = 16;
        
        // Positioning properties
        x = 0.1;
        y = 0.2;
        w = 0.3;
        h = 0.4;
        
        // Color arrays
        colorText[] = {1, 1, 1, 1};
        colorBackground[] = {0, 0, 0, 0.5};
        
        // Font properties
        font = "FontName";
        sizeEx = 0.04;
        
        // Style combinations
        combinedStyle = ST_LEFT + ST_MULTI;
        
        // Sound properties
        soundClick[] = {"\path\to\sound.ogg", 0.09, 1};
        
        // Control lists
        controls[] = {"Control1", "Control2", "Control3"};
        
        // Complex position expressions
        complexX = "safezoneX + (safezoneW * 0.5)";
        complexY = "(safezoneY + safezoneH) - (1 * ((((safezoneW / safezoneH) min 1.2) / 1.2) / 25))";
    };
    "#,
    "Failed to parse UI property patterns"
);

//==============================================================================
// PROPERTY NAME EDGE CASES
//==============================================================================
// Tests for unusual property naming patterns
//==============================================================================

test_config_parse!(
    test_property_names,
    r#"
    class Test {
        // Property names with underscores
        under_score_name = 1;
        
        // Property names with prefixes
        RHS_propertyName = 2;
        ACE_setting = 3;
        
        // Mixed case property names
        mixedCASE_property = 4;
        
        // Property names with numbers
        property123 = 5;
        property_456 = 6;
        
        // Long property names
        thisIsAReallyLongPropertyNameThatShouldStillWorkFine = 7;
        
        // Property names with adjacent underscores
        property__with__multiple_underscores = 8;
    };
    "#,
    "Failed to parse property name edge cases"
);

//==============================================================================
// MIXED PROPERTY SYNTAX PATTERNS
//==============================================================================
// Tests combining multiple property syntax patterns in a single class
//==============================================================================

test_config_parse!(
    test_mixed_property_patterns,
    r#"
    class Test {
        // Regular properties
        normalProp = 1;
        stringProp = "value";
        
        // Array properties
        arrayProp[] = {1, 2, 3};
        
        // Properties with constants
        constantProp = SOME_CONSTANT;
        
        // Properties with paths
        pathProp = "\path\to\file.paa";
        unquotedPath = \path\to\file;
        
        // Complex expressions
        expression = "a*b+c";
        
        // Nested with properties of different styles
        class Nested {
            prop1 = 1;
            prop2[] = {1, 2};
            prop3 = "string";
            prop4 = CONSTANT;
        };
    };
    "#,
    "Failed to parse mixed property patterns"
);

//==============================================================================
// PROPERTY EDGE CASES
//==============================================================================
// Tests for rare but valid property syntax patterns found in real configs
//==============================================================================

test_config_parse!(
    test_edge_cases,
    r#"
    class Test {
        // Extra parentheses in expressions (common error but must parse)
        extraParens = (1 + 2));
        
        // Trailing commas in arrays
        trailingComma[] = {1, 2, 3,};
        
        // Strange spacing around operators
        strangeSpacing =1+  2*3;
        
        // Unterminated value (no semicolon) followed by new property
        unterminated = 1
        nextProp = 2;
        
        // Empty array
        emptyArray[] = {};
        
        // Array with single empty element
        singleEmptyArray[] = {{}};
        
        // Property with namespace path
        namespaceProperty = "missionNamespace getVariable ['varName', 0]";
        
        // Property value with nested quotes
        nestedQuotes = "Property with ""nested"" quotes";
        
        // Uncommon operators in property values
        uncommonOps = value % 10;
    };
    "#,
    "Failed to parse property edge cases"
);

//==============================================================================
// IRREGULAR PROPERTY FORMATTING TESTS
//==============================================================================
// Tests for properties with unusual spacing, indentation, and formatting patterns
// to ensure the parser is robust against non-uniform property formatting
//==============================================================================

test_config_parse!(
    test_irregular_property_formatting,
    r#"
    class   IrregularProperties    {
        // Properties with unusual spacing around equals sign
        normalProp     =      100;
        compactProp=200;
        spacedProp   =   300   ;
        
        // Properties with tab indentation
        	tabProp = 400;
            	deepTabProp=500;
        
        // Properties with excessive line breaks
        multiLine
            = 
                600;
                
        // Properties with no space before semicolon
        noSpaceBefore=700;noSpaceAfter = 800;
        
        // Multi-line property name
        very
        long
        property
        name = 900;
        
        // String values with unusual spacing
        stringValue    =    "text with    internal   spaces"    ;
        
        // Extremely sparse formatting
        singleProperty  
                    =   
                        1000    ;
                        
        // Multiple properties on a single line with mixed spacing
        prop1=1;  prop2  =  2;prop3=   3;   prop4   =4;
        
        // Properties with unusual array spacing
        array1[]=  {1,2,3};
        array2  [   ]    =     {   10  ,   20  ,  30   }  ;
        
        // Property with trailing space and next property on same line
        trailingSpace = 1;   nextProp = 2;
        
        // Properties with unusual macro spacing
        macroValue  =  MACRO  (  arg1  ,   arg2  )  ;
        
        // Mixed compact and verbose properties
        compactSection={value=1;prop=2;};verboseProp   =   3;
    };
    "#,
    "Failed to parse properties with irregular spacing and formatting"
);

//==============================================================================
// EXTREME PROPERTY FORMATTING TESTS
//==============================================================================
// Tests for properties with extremely unusual formatting to push the parser to its limits
//==============================================================================

test_config_parse!(
    test_extreme_property_formatting,
    r#"
    class 
    
    
    ExtremePropertyFormatting 
    
    
    {
        // Properties on separate lines with excessive whitespace
        prop1
        
        
        
        = 
        
        
        100;
        
        // Properties with no space before semicolon and next property
        prop2=200;prop3=300;
        
        // Properties with mixed styles in a single declaration
        compact=400;    spaced   =   500;
        
        // Nested properties with bizarre indentation
        class 
            NestedClass
                {
                    a 
                    
                    = 
                    
                    1;
                    
                    // Mix of compact and verbose style
                    b=2;    c     =     3;
                    
                    // Property and class on same line
                    d=4;class InnerClass{e=5;}
                }   
                ;
                
        // Array with highly irregular spacing
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
        
        // String with excessive internal spacing
        string     =     "this    string    has    many    spaces"     ;
        
        // Nested property path with irregular spacing
        nestedPath   .   subPath    .   deepPath   =    100;
        
        // Mixed property endings
        prop4 = 600
        prop5 = 700;
    };
    "#,
    "Failed to parse properties with extreme formatting variations"
);