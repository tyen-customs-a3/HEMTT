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
// Basic array syntax tests covering the essential array patterns
//==============================================================================

test_config_parse!(
    test_arrays,
    r#"
    class Test {
        // Simple array with numbers
        simpleArray[] = {1, 2, 3};
        
        // Array with strings
        stringArray[] = {"one", "two", "three"};
        
        // Nested array
        nestedArray[] = {{1, 2}, {3, 4}};
        
        // Array expansion
        baseArray[] = {1, 2};
        expandArray[] += {3, 4};
    };
    "#,
    "Failed to parse basic arrays"
);

//==============================================================================
// MACRO IN ARRAYS TESTS
//==============================================================================
// Tests for arrays containing macros in various positions and combinations
//==============================================================================

test_config_parse!(
    test_macros_in_arrays,
    r#"
    class Test {
        // Array with a single macro
        macroArray[] = {MACRO(arg)};
        
        // Mixed array with numbers, macros and strings
        mixedArray[] = {1, MACRO(arg), "string"};
        
        // Nested array with macros
        nestedMacroArray[] = {{MACRO(a), MACRO(b)}, {1, 2}};
        
        // Array expansion with macros
        baseMacro[] = {MACRO(a), MACRO(b)};
        expandMacro[] += {MACRO(c), MACRO(d)};
    };
    "#,
    "Failed to parse arrays with macros"
);

//==============================================================================
// CLASS STRUCTURE TESTS
//==============================================================================
// Tests for class inheritance, nested classes, and class properties
//==============================================================================

test_config_parse!(
    test_class_structures,
    r#"
    class BaseClass {
        prop = 1;
    };
    
    class SubClass: BaseClass {
        subProp = 2;
    };
    
    class ParentClass {
        class NestedClass {
            nestedProp = 3;
        };
    };
    "#,
    "Failed to parse class structures"
);

//==============================================================================
// EVAL MACRO TESTS
//==============================================================================
// Tests for __EVAL macro in various contexts
//==============================================================================

test_config_parse!(
    test_eval_macros,
    r#"
    class Test {
        // Simple eval
        simpleEval = __EVAL(10*5);
        
        // Complex eval with nested operations
        complexEval = __EVAL(1.0*(0.5*120*(0.369^2)));
        
        // Eval in arrays
        evalArray[] = {__EVAL(750/5000), __EVAL(50/173)};
        
        // Edge case: extra parentheses in eval
        extraParensEval = __EVAL(45*(PI/180)));
    };
    "#,
    "Failed to parse __EVAL macros"
);

//==============================================================================
// PREPROCESSOR DIRECTIVE TESTS
//==============================================================================
// Tests for #define, #undef, #ifdef, #include and other preprocessor directives
//==============================================================================

test_config_parse!(
    test_preprocessor_directives,
    r#"
    // Simple define
    #define VERSION 1
    
    // Undefine and redefine
    #undef VERSION
    #define VERSION 2
    
    // Include directives
    #include "path/to/header1.h"
    #include "path/to/header2.h"
    
    // Conditional compilation
    #ifdef MACRO_DEFINED
        #define VALUE 1
    #else
        #define VALUE 2
    #endif
    
    class Test {
        version = VERSION;
        value = VALUE;
    };
    "#,
    "Failed to parse preprocessor directives"
);

//==============================================================================
// COMPLEX MACRO DEFINITIONS AND USAGE
//==============================================================================
// Tests for complex macro definitions with multiple parameters and usage patterns
//==============================================================================

test_config_parse!(
    test_complex_macros,
    r#"
    // Multi-line macro definition with continuation characters
    #define FUNCTION_MACRO(x, y, text) \
    class MacroClass_##x { \
        pos[] = {x, y}; \
        text = text; \
    }
    
    // Nested macro definitions
    #define FUNC1(val,from,to) (val factor[from,to])
    #define FUNC2(val,from0,to0,from1,to1) (FUNC1(val,from0,to0) * FUNC1(val,to1,from1))
    
    // Token concatenation
    #define BASE item
    
    class Test {
        // Using function macro
        FUNCTION_MACRO(0.5, 0.5, "Center")
        
        // Using nested macros
        complexValue = 0.9 + FUNC2(value,100,200,800,100)*0.2;
        
        // Using token concatenation
        tokenPoints[] = { 
            {BASE##1, {0, 0}, 1},
            {BASE##2, {0, 0}, 1}
        };
    };
    "#,
    "Failed to parse complex macro definitions and usage"
);

//==============================================================================
// STANDALONE MACRO USAGE
//==============================================================================
// Tests for standalone macros without assignment and macros as placeholders
//==============================================================================

test_config_parse!(
    test_standalone_macros,
    r#"
    // Standalone macros at file level
    MACRO_GLOBAL(param1, param2)
    
    class Test {
        // Standalone macro inside class
        MACRO_LOCAL(500)
        
        // Macro placeholder
        MACRO_PLACEHOLDER
        
        // Normal property
        value = 1;
    };
    "#,
    "Failed to parse standalone macros"
);

//==============================================================================
// CLASS NAMES WITH MACROS
//==============================================================================
// Tests for class declarations using macros in the class name
//==============================================================================

test_config_parse!(
    test_macro_class_names,
    r#"
    // Simple macro class name
    class GVAR(actions) {};
    
    // Multiple macro class names
    class GVAR(items) {};
    class EGVAR(common,settings) {};

    // Macro class name with token concatenation
    class FGVAR( common ,  settings) {  };
    class FGVAR  (common ,  settings) {    };
    
    // Macro class name with inheritance
    class BaseClass {};
    class GVAR(derived): BaseClass {
        prop = 1;
    };
    
    // Nested macro classes
    class GVAR(container) {
        class GVAR(nested) {
            property = 1;
        };
    };
    
    // Mixed standard and macro classes
    class NormalClass {
        prop = 1;
    };
    class GVAR(macroClass) {
        prop = 2;
    };
    class AnotherNormal: GVAR(macroClass) {
        prop = 3;
    };
    "#,
    "Failed to parse class names with macros"
);

//==============================================================================
// IRREGULAR SPACING AND FORMATTING TESTS
//==============================================================================
// Tests for configs with unusual spacing, indentation, and formatting patterns
// to ensure the parser is robust against non-uniform formatting
//==============================================================================

test_config_parse!(
    test_irregular_formatting,
    r#"
    class   IrregularSpacing    {
        // Property with extra spaces around equals sign
        normalProp     =      100;
        
        // Property with tab indentation and no spaces around equals
        	tabProp=200;
        
        // Mixed spacing in array declaration
        spacedArray [ ]={1,2,    3};
        
        // Irregular array with spaces everywhere
        verySpacedArray  [   ]    =     {   10  ,   20  ,  30   }  ;
        
        // No spaces in array declaration
        compactArray[]={4,5,6};
        
        // Irregular spacing in nested arrays
        nestedSpacing[ ]={{  7,8  }, {  9  ,  10}   };
        
        // Property with excessive line breaks and spacing
        multiLine
            = 
                300;
        
        // Irregular array expansion
        baseValues[]=  {1,2};
        expandValues[ ] 
            += 
                {3,  4};
                
        // Non-uniform spacing in string values
        weirdString    =    "text with    internal   spaces"    ;
        
        // Extremely sparse formatting with nested classes
        class    VerySpacedClass 
            {
                prop  
                    =   
                        400    ;
                        
                class  NestedSpacedClass{
                nestedProp=500;
                }      ;
            }    
            ;
            
        // Irregular spacing in macro usages
        strangeFormat  =  MACRO  (  arg1  ,   arg2  )  ;
        
        // Irregular array with macros and weird spacing
        macroMess [ ] = {   MACRO(  a  )  ,   MACRO  (  b  ) ,   1   };
    };
    
    // Class inheritance with unusual spacing
    class   BaseClass{};
    class   Child   :    BaseClass
    {
        value   =  1  ;
    }   ;
    "#,
    "Failed to parse config with irregular spacing and formatting"
);

//==============================================================================
// EDGE CASES
//==============================================================================
// Tests for various edge cases found in real-world configs
//==============================================================================

test_config_parse!(
    test_edge_cases,
    r#"
    // Class that adds content via a macro
    #define ADD_CLASS_CONTENT class Property { value = 1; };
    
    class Test1 {
        ADD_CLASS_CONTENT
        prop = 1;
    };
    
    // Macro token concatenation in a define
    #define CONCAT_TOKENS(prefix,name) ##prefix##_##name##_Suffix
    
    class Test2 {
        token = CONCAT_TOKENS(foo,bar);
    };
    
    // HUD-style macro with positioning
    #define HUD_ELEMENT(x,y,text) \
    class Element { \
        pos[] = {{x,y},1}; \
        text = text; \
    }
    
    class Test3 {
        HUD_ELEMENT(0.5, 0.5, "Center")
    };
    
    // Complex class with inheritance and macros
    class Base;
    class Derived: Base {
        MACRO_BEGIN
        properties[] = {
            "prop1",
            "prop2",
            MACRO_VALUES
        };
        MACRO_END
    };
    "#,
    "Failed to parse edge cases"
);

//==============================================================================
// EXTREME FORMATTING TESTS
//==============================================================================
// Tests with extremely unusual syntax formatting to push the parser to its limits
//==============================================================================

test_config_parse!(
    test_extreme_formatting,
    r#"
    class 
    
    
    ExtremeFormatting 
    
    
    {
        // Properties with no space before semicolon
        prop1= 100;prop2=200;prop3 = 300;
        
        // Multi-line property name
        very
        long
        property
        name = 400;
        
        // Arrays with scattered brackets
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
        
        // Nested arrays with bizarre formatting
        array2 [
        
        ]={
        {1    ,
            
                2},
            {    3,4
        }
            };
            
        // Array with excessive newlines between elements
        sparseArray[]={
            5,
            
            
            
            6,
            
            
            7
        };
        
        // Mixed styles within a single class
        compact[]={8,9,10};array3 = {
        // Comment in the middle of array definition
        11,
        
        12  ,13};
        
        // Complex nested structure with inconsistent formatting
        class 
            NestedClass1
                {
                    a = 1;
                    
                    class 
                        
                        DeeperClass 
                        
                        {
                            b 
                            = 
                            2;
                            
                            array 
                            
                            [   
                            ]   =   
                            {   
                                3, 
                                4   
                            }   ;
                        }   
                        ;
                        
                    // Mixed property and class
                    c=3;class AnotherClass{d=4;}
                }   ;
    }
    ;
    
    // Class with no space after colon for inheritance
    class ChildClass:
    ExtremeFormatting{
        value
        =
        1000
        ;
        
        array
        []=
        {   1000,
            2000   };
    };
    "#,
    "Failed to parse config with extreme formatting variations"
);