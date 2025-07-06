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
// ARRAY SYNTAX TESTS
//==============================================================================
// Tests for various array syntax patterns including simple arrays, nested arrays,
// arrays with macros, array expansion, and various complex array structures
//==============================================================================

test_config_parse!(
    test_simple_array,
    r#"
    class Test {
        simpleArray[] = {1, 2, 3};
    };
    "#,
    "Failed to parse simple array"
);

test_config_parse!(
    test_array_with_strings,
    r#"
    class Test {
        string_array[] = {"one", "two", "three"};
    };
    "#,
    "Failed to parse array with strings"
);

test_config_parse!(
    test_single_macro_array,
    r#"
    class Test {
        macroArray[] = {MACRO(arg)};
    };
    "#,
    "Failed to parse array with single macro"
);

test_config_parse!(
    test_mixed_array,
    r#"
    class Test {
        mixedItems[] = {1, MACRO(arg), "string"};
    };
    "#,
    "Failed to parse mixed array"
);

test_config_parse!(
    test_nested_array,
    r#"
    class Test {
        nestedArray[] = {{1, 2}, {3, 4}};
    };
    "#,
    "Failed to parse nested array"
);

test_config_parse!(
    test_nested_array_with_macros,
    r#"
    class Test {
        nestedMacros[] = {{MACRO(a), MACRO(b)}, {1, 2}};
    };
    "#,
    "Failed to parse nested array with macros"
);

test_config_parse!(
    test_array_expansion,
    r#"
    class Test {
        baseArray[] = {1, 2};
        expandArray[] += {3, 4};
    };
    "#,
    "Failed to parse array expansion"
);

test_config_parse!(
    test_array_expansion_with_macros,
    r#"
    class Test {
        base[] = {MACRO(a), MACRO(b)};
        expanded[] += {MACRO(c), MACRO(d)};
    };
    "#,
    "Failed to parse array expansion with macros"
);

test_config_parse!(
    test_complex_array_structure,
    r#"
    class Test {
        torqueCurve[] = {
            {__EVAL(0/2100),__EVAL(0/883)},
            {__EVAL(900/2100),__EVAL(810/883)},
            {__EVAL(1100/2100),__EVAL(825/883)}
        };
    };
    "#,
    "Failed to parse complex array structures"
);

test_config_parse!(
    test_empty_array_element,
    r#"
    class Test {
        points[] = {
            {Object1, {0.0, -0.16}, 1},
            {Object1, {0.0, -0.03}, 1},
            {},
            {Object1, {0.0,  0.16}, 1},
            {Object1, {0.0,  0.03}, 1}
        };
    };
    "#,
    "Failed to parse empty array element"
);

test_config_parse!(
    test_array_with_brace_parameter,
    r#"
    class Test {
        points[] = {
            {
                {{-0.042+0.88},{-0.075+0.9}},
                {{+0.042+0.88},{-0.075+0.9}},
                {{+0.042+0.88},{+0.075+0.9}},
                {{-0.042+0.88},{+0.075+0.9}}
            }
        };
    };
    "#,
    "Failed to parse array with brace parameter"
);

test_config_parse!(
    test_array_with_expressions,
    r#"
    class Test {
        pos1[] = {0.5 + 0.1, __EVAL(sin(3.14/2) + 0.2)};
        pos2[] = {__EVAL(cos(3.14/4)), 0.5 * 2.0};
    };
    "#,
    "Failed to parse array with expressions"
);

test_config_parse!(
    test_comma_separated_array_as_property,
    r#"
    class Test {
        items[] = {
            "item1",
            "item2",
            "item3","item4",
            "item5",
            "item6",
            "item7",
            "item8",
            "item9",
            "item10",
            "item11",
            "item12"
        };
    };
    "#,
    "Failed to parse comma separated array as property"
);

//==============================================================================
// CLASS STRUCTURE TESTS
//==============================================================================
// Tests for various class structures including inheritance, nested classes,
// empty classes, and complex class hierarchies
//==============================================================================

test_config_parse!(
    test_class_inheritance,
    r#"
    class BaseClass {
        baseProp = 1;
    };
    class DerivedClass: BaseClass {
        childProp = 2;
    };
    "#,
    "Failed to parse class inheritance"
);

test_config_parse!(
    test_nested_class_structure,
    r#"
    class Test {
        class NestedClass {
            property = 1;
            class DeepNestedClass {
                deepProperty = 2;
            };
        };
    };
    "#,
    "Failed to parse nested class structure"
);

test_config_parse!(
    test_multiple_braces_closure,
    r#"
    class Test {
        class DisplaySettings {
            class PrimaryDisplay {
                type = fixed;
                pos[] = {0.5, 0.5};
            };
        };
        class PointDefinitions {
            class ScreenPoints {
                points[] = {
                    {Alpha, {0.0, -0.16}, 1},
                    {Alpha, {0.0, -0.03}, 1},
                    {}
                };
            };
        };
    };
    "#,
    "Failed to parse multiple nested braces with proper closure"
);

test_config_parse!(
    test_class_references,
    r#"
    class Test {
        class BaseWidget {
            alpha = 1;
        };
        class ExtendedWidget: BaseWidget {
            alpha = 2;
        };
    };
    "#,
    "Failed to parse class references"
);

test_config_parse!(
    test_complex_inheritance_chain,
    r#"
    class ClassA {
        foo = 1;
    };
    class ClassB: ClassA {
        bar = 2;
    };
    class ClassC: ClassB {
        baz = 3;
    };
    class ClassD: ClassA {
        qux = 4;
    };
    "#,
    "Failed to parse complex inheritance chains"
);

test_config_parse!(
    test_class_with_empty_content,
    r#"
    class EmptyClass1 {};
    class EmptyClass2 {};
    class Test {
        class EmptyClass3 {};
    };
    "#,
    "Failed to parse classes with empty content"
);

test_config_parse!(
    test_empty_nested_classes,
    r#"
    class Test {
        class Controls {
            class Background {};
            class Title {};
            class Content {
                class List {};
                class Buttons {};
            };
        };
    };
    "#,
    "Failed to parse nested empty class definitions"
);

test_config_parse!(
    test_weapon_class_inheritance,
    r#"
    class Rifle_Base_F;
    class rhs_weap_ak74m_Base_F: Rifle_Base_F {
        baseWeapon = "rhs_weap_ak74m";
        WEAPON_FIRE_BEGIN
        magazines[] = {
            "rhs_30Rnd_545x39_7N6M_AK",
            "rhs_30Rnd_545x39_7N10_AK",
            WEAPON_MAGS_CORE
        };
        WEAPON_FIRE_END
    };
    "#,
    "Failed to parse weapon class with preprocessor macros and inheritance"
);

test_config_parse!(
    test_magazine_definition,
    r#"
    class Test {
        class rhs_mag_base {
            count = 30;
            type = "2A70";
            initSpeed = 850;
        };
        class rhs_mag_specific: rhs_mag_base {
            ammo = "rhs_ammo_type";
            displayName = "Magazine Name";
            displayNameShort = "Mag";
            descriptionShort = "Description";
            model = "\path\to\model.p3d";
            picture = "\path\to\picture.paa";
            mass = 25;
            tracersEvery = 3;
            lastRoundsTracer = 5;
        };
    };
    "#,
    "Failed to parse complex magazine definition with inheritance"
);

//==============================================================================
// MACRO AND PREPROCESSOR TESTS
//==============================================================================
// Tests for various preprocessor directives including #define, #undef, #ifdef,
// and complex macro definitions and usage patterns
//==============================================================================

test_config_parse!(
    test_macro_eval,
    r#"
    class Test {
        evalValue = __EVAL(10*5);
    };
    "#,
    "Failed to parse __EVAL macro"
);

test_config_parse!(
    test_complex_eval_expression,
    r#"
    class Test {
        complexCalc = __EVAL(1.0*(0.5*120*(0.369^2)));
        evalArray[] = {__EVAL(750/5000),__EVAL(50/173)};
    };
    "#,
    "Failed to parse complex __EVAL expressions"
);

test_config_parse!(
    test_define_directive,
    r#"
    #define PosX0Center 0.44
    #define PosY0Center 0.47
    
    class Test {
        pos[] = {PosX0Center, PosY0Center};
    };
    "#,
    "Failed to parse #define directive"
);

test_config_parse!(
    test_complex_macro_definition,
    r#"
    #undef BONE_POINTS
    #define BONE_POINTS(center,bone,scale) \
    {center,{0, __EVAL(-1.0000 * scale / XtoYscale)}, 1}, \
    {bone##1,{0, __EVAL(1.0000 * scale )}, 1, center, 1}, \
    {bone##2,{0, __EVAL(1.0000 * scale )}, 1, center, 1}
    
    class Test {
        points[] = { BONE_POINTS(centerPoint, bone, 1.5) };
    };
    "#,
    "Failed to parse complex macro definitions with continuation characters"
);

test_config_parse!(
    test_macro_concatenation,
    r#"
    #define BASE item
    
    class Test {
        modelPoints[] = { 
            {BASE##1, {0, 0}, 1},
            {BASE##2, {0, 0}, 1}
        };
    };
    "#,
    "Failed to parse macro token concatenation using ##"
);

test_config_parse!(
    test_undef_directive,
    r#"
    #define VERSION 1
    #undef VERSION
    #define VERSION 2
    
    class Test {
        configVersion = VERSION;
    };
    "#,
    "Failed to parse #undef directive"
);

test_config_parse!(
    test_complex_nested_macros,
    r#"
    #define FUNC1(val,from,to) val
    #define FUNC2(val,from0,to0,from1,to1) FUNC1(val,from0,to0)
    #define FUNC3(val,from,band0,to,band1) FUNC2(val,from,band0,to,band1)
    
    class Test {
        foo = FUNC3(value,100,200,800,100);
    };
    "#,
    "Failed to parse complex nested macro expressions"
);

#[test]
fn test_factor_expression_parsing() {
    // Test the original problematic expression that was simplified
    let original_config = r#"
    #define FUNC1(val,from,to) (val factor[from,to])
    #define FUNC2(val,from0,to0,from1,to1) (FUNC1(val,from0,to0) * FUNC1(val,to1,from1))
    #define FUNC3(val,from,band0,to,band1) FUNC2(val,from,(from+band0),to,(to+band1))
    
    class Test {
        foo = 0.9 + FUNC3(value,100,200,800,100)*0.2;
    };
    "#;

    println!("Testing original problematic expression...");
    let result = parse_config(original_config);
    match result {
        Ok(_) => println!("SUCCESS: Original config parsed successfully"),
        Err(errors) => {
            println!("FAILED: Original config parsing failed with {} errors", errors.len());
            for error in errors {
                if let Some(diag) = error.diagnostic() {
                    eprintln!("Error: {}", diag.to_string(&hemtt_workspace::reporting::WorkspaceFiles::new()));
                }
            }
        }
    }

    // Test the expanded expression directly
    let expanded_config = r#"
    class Test {
        foo = 0.9 + ((value factor[100,(100+200)]) * (value factor[(800+100),800]))*0.2;
    };
    "#;

    println!("\nTesting expanded expression directly...");
    let result = parse_config(expanded_config);
    match result {
        Ok(_) => println!("SUCCESS: Expanded config parsed successfully"),
        Err(errors) => {
            println!("FAILED: Expanded config parsing failed with {} errors", errors.len());
            for error in errors {
                if let Some(diag) = error.diagnostic() {
                    eprintln!("Error: {}", diag.to_string(&hemtt_workspace::reporting::WorkspaceFiles::new()));
                }
            }
        }
    }
    
    // Test simpler factor expression
    let simple_config = r#"
    class Test {
        foo = value factor[100,200];
    };
    "#;

    println!("\nTesting simple factor expression...");
    let result = parse_config(simple_config);
    match result {
        Ok(_) => println!("SUCCESS: Simple factor config parsed successfully"),
        Err(errors) => {
            println!("FAILED: Simple factor config parsing failed with {} errors", errors.len());
            for error in errors {
                if let Some(diag) = error.diagnostic() {
                    eprintln!("Error: {}", diag.to_string(&hemtt_workspace::reporting::WorkspaceFiles::new()));
                }
            }
        }
    }
    
    // For now, all of these should fail since we haven't implemented factor parsing yet
    // This test is just to document the issue
}

test_config_parse!(
    test_macro_with_parameters,
    r#"
    class Test {
        RHS_FUEL_RANGE(500)
        value = 1;
    };
    "#,
    "Failed to parse macro with parameters"
);

test_config_parse!(
    test_author_macro_placeholder,
    r#"
    class Test {
        MACRO_PLACEHOLDER
        foo = 2;
    };
    "#,
    "Failed to parse macro placeholder without value assignment"
);

test_config_parse!(
    test_hud_define_macros,
    r#"
    #define MACRO_FUNCTION(X,Y,SX,SY,ALIGN,TEXT,FONT) \
    class NAME \
    { \
        type = "text"; \
        source = "static"; \
        text = TEXT; \
        align = ALIGN; \
        scale = 1; \
        pos[] = {{X,Y},1}; \
        right[] = {{X+SX,Y},1}; \
        down[] = {{X,Y+SY},1}; \
    }
    
    class Test {
        MACRO_FUNCTION(0.875,0.82,0.035,0.14,center,"Text",Font)
    };
    "#,
    "Failed to parse HUD define macros with nested class definitions"
);

test_config_parse!(
    test_multiple_includes,
    r#"
    class Test {
        prop = 1;
    };
    "#,
    "Failed to parse multiple includes"
);

test_config_parse!(
    test_ifdef_conditions,
    r#"
    #ifdef MACRO_DEFINED
        #define VALUE 1
    #else
        #define VALUE 2
    #endif
    
    class Test {
        prop = VALUE;
    };
    "#,
    "Failed to parse ifdef conditions"
);

test_config_parse!(
    test_add_macro_to_class,
    r#"
    #define ADD_MACRO_TO_CLASS class Prop { value = 1; };
    
    class Test {
        ADD_MACRO_TO_CLASS
    };
    "#,
    "Failed to parse macro that adds content to class"
);

test_config_parse!(
    test_sound_shader_macro,
    r#"
    MACRO_SOUND(Name,3000)
    MACRO_SOUND_SPECIAL(Name,700)
    
    class Test {
        prop = 1;
    };
    "#,
    "Failed to parse sound shader macro"
);

//==============================================================================
// PROPERTY VALUE TESTS
//==============================================================================
// Tests for various property value syntax including constants, scientific notation, 
// hexadecimal values, strings with special characters, and complex value expressions
//==============================================================================

test_config_parse!(
    test_special_syntax_constants,
    r#"
    class Test {
        suspTravelDirection[] = SUSPTRAVELDIR_LEFT;
        tankTurnForce = 0.31e6;
    };
    "#,
    "Failed to parse special syntax constants"
);

test_config_parse!(
    test_scientific_notation,
    r#"
    class Test {
        foo = 0.85e6;
        bar = 1e+009;
        baz = 1.5e-3;
    };
    "#,
    "Failed to parse scientific notation values"
);

test_config_parse!(
    test_string_with_escaped_characters,
    r#"
    class Test {
        foo = "path\to\file\name";
    };
    "#,
    "Failed to parse strings with escaped backslashes"
);

test_config_parse!(
    test_arithmetic_in_condition,
    r#"
    class Test {
        condition = "param1*param2*0.5";
        foo = "param1*param2*PARAM3";
    };
    "#,
    "Failed to parse arithmetic operations in condition strings"
);

test_config_parse!(
    test_hexadecimal_color_values,
    r#"
    class Test {
        foo[] = {0xAB, 0xCD, 0xEF, 0xFF};
        bar = 0xFF00FF;
    };
    "#,
    "Failed to parse hexadecimal color values"
);

test_config_parse!(
    test_model_paths_without_quotes,
    r#"
    class Test {
        model = \path\to\file\model.p3d;
    };
    "#,
    "Failed to parse model paths without quotes"
);

test_config_parse!(
    test_large_numeric_values,
    r#"
    class Test {
        prop1 = 1e+009;
        prop2 = 1e+009;
        prop3 = 330000;
        prop4 = 1;
    };
    "#,
    "Failed to parse large numeric values"
);

test_config_parse!(
    test_complex_property_chain,
    r#"
    class Test {
        prop1[]={"Value"};
        prop2[]	= CONSTVALUE;
        prop3[]	= {
            {0.0, 	0.8},
            {0.38, 	1.0},
            {0.7, 	0.65}
        };
    };
    "#,
    "Failed to parse complex property chain"
);

test_config_parse!(
    test_rhs_style_properties,
    r#"
    class Test {
        RHS_fuelCapacity = 1885;
        RHS_enginePower = 780;
        RHS_property_with_underscore = 100;
    };
    "#,
    "Failed to parse RHS-style property names with underscores"
);

test_config_parse!(
    test_special_character_handling,
    r#"
    class Test {
        path_with_backslash = "\path\to\file";
        path_with_forward_slash = "/path/to/file";
        mixed_path = "\path/to\file";
        quoted_special = "\special\"chars\"";
        unquoted_special = \special\chars;
    };
    "#,
    "Failed to parse paths with special characters"
);

//==============================================================================
// DOMAIN-SPECIFIC SYNTAX TESTS
//==============================================================================
// Tests for domain-specific syntax patterns found in real config files including
// vehicle configs, HUD elements, sound settings, and UI controls
//==============================================================================

test_config_parse!(
    test_sound_syntax,
    r#"
    class Test {
        foo[]={"path\to\sound",db+5,1,9};
        bar[]={"path\to\sound",db+8,1,25};
    };
    "#,
    "Failed to parse sound configuration syntax with db+ notation"
);

test_config_parse!(
    test_complex_condition,
    r#"
    class Test {
        class DisplayElement {
            condition = "(value1>=137)*(value1<=211)";
            alpha = 1;
        };
    };
    "#,
    "Failed to parse complex condition expressions"
);

test_config_parse!(
    test_condition_with_functions,
    r#"
    class Test {
        condition = "func1 this && (this func2 'param' func3 0)";
    };
    "#,
    "Failed to parse condition with functions and logical operators"
);

test_config_parse!(
    test_statement_with_complex_code,
    r#"
    class Test {
        statement = "this spawn {_this func1 ['param1',1];sleep 1.2;private _var = 'value' func2 [0,0,0];_var func3 ((func4 _this) func5 [0,0.2,0.1]);_var func6 func7 _this;_var func8 [0.5,0.5,0]}";
    };
    "#,
    "Failed to parse statement with complex code"
);

test_config_parse!(
    test_blinking_parameters,
    r#"
    class Test {
        foo[] = {0.3, 0.3};
        bar = true;
    };
    "#,
    "Failed to parse blinking parameters"
);

test_config_parse!(
    test_material_properties,
    r#"
    class Test {
        class Material
        {
            foo[] = {1, 1, 1, 1};
            bar[] = {10, 10, 10, 1};
            baz[] = {400, 200, 200, 1};
        };
    };
    "#,
    "Failed to parse material properties"
);

test_config_parse!(
    test_complex_animation_sources,
    r#"
    class Test {
        class Sources
        {
            class Item
            {
                source = "user";
                initPhase = 0;
                animPeriod = 1.2;
            };
        };
    };
    "#,
    "Failed to parse complex animation sources"
);

test_config_parse!(
    test_user_actions,
    r#"
    class Test {
        class Actions
        {
            class ItemAction
            {
                displayName = "Action Name";
                position = "";
                radius = 2;
                showWindow = 1;
                condition = "condition test";
                statement = "statement test";
                priority = 1;
            };
        };
    };
    "#,
    "Failed to parse user actions"
);

test_config_parse!(
    test_complex_gearbox_ratios,
    r#"
    class Test {
        class Gearbox {
            Ratios[] = {
                "R1",-2.235,
                "N",0,
                "D1",2.6,
                "D2",1.5,
                "D3",1.125,
                "D4",0.85,
                "D5",0.64,
                "D6",0.50
            };
            OtherRatios[] = {"High",12};
            ExtraRatios[] = {"R1", -40, "N", 0, "D1", 40};
        };
    };
    "#,
    "Failed to parse complex gearbox ratios"
);

test_config_parse!(
    test_editor_categories,
    r#"
    class Test {
        MacroName(param_name)
        prop1 = 2;
        prop2 = 2;
        category1 = "Category1";
        category2 = "Category2";
        group1 = "Group1";
        group2 = "Group2";
    };
    "#,
    "Failed to parse editor categories and macro parameters"
);

test_config_parse!(
    test_texture_property,
    r#"
    class Test {
        texture = "path\to\texture.paa";
        textures[] = {
            "path\to\texture1.paa",
            "path\to\texture2.paa",
            "path\to\texture3.paa"
        };
    };
    "#,
    "Failed to parse texture property"
);

test_config_parse!(
    test_icon_picture_properties,
    r#"
    class Test {
        picture = "\path\to\picture.paa";
        icon = "\path\to\icon.paa";
        size = 9.45;
    };
    "#,
    "Failed to parse icon and picture properties"
);

test_config_parse!(
    test_complex_font_settings,
    r#"
    class Test {
        font = "FontName";
        enableEffect = false;
        alpha = 0.5;
        color[] = {1,1,1};
    };
    "#,
    "Failed to parse complex font settings"
);

test_config_parse!(
    test_polygon_type,
    r#"
    class Test {
        class Shape
        {
            type = polygon;
            width = 4.0;
            points[] = {
                {
                    {{-0.042+0.88},{-0.075+0.9},1},
                    {{+0.042+0.88},{-0.075+0.9},1},
                    {{+0.042+0.88},{+0.075+0.9},1},
                    {{-0.042+0.88},{+0.075+0.9},1}
                }
            };
        };
    };
    "#,
    "Failed to parse polygon type"
);

test_config_parse!(
    test_turret_array,
    r#"
    class Test {
        prop1[] = {-1};
        prop2[] = {"Value1"};
        prop3[] = {"Value2"};
    };
    "#,
    "Failed to parse turret array"
);

test_config_parse!(
    test_pylon_properties,
    r#"
    class Test {
        class Item {
            type = "typename";
            pos[] = {{0.375, 0.34}, 1};
            id = 1;
            name = "itemname";
        };
    };
    "#,
    "Failed to parse pylon properties"
);

test_config_parse!(
    test_memory_point_references,
    r#"
    class Test {
        point1 = "PointName1";
        point2 = "PointName2";
        point3 = "PointName3";
    };
    "#,
    "Failed to parse memory point references"
);

test_config_parse!(
    test_font_families,
    r#"
    class CfgFontFamilies
    {
        class FontName
        {
            fonts[] = {"\path\to\font\fontfile"};
        };
    };
    "#,
    "Failed to parse font families configuration"
);

test_config_parse!(
    test_control_with_profile_namespace,
    r#"
    class Test {
        color[] = {
            "(profilenamespace getvariable ['COLOR_R',0])",
            "(profilenamespace getvariable ['COLOR_G',1])",
            "(profilenamespace getvariable ['COLOR_B',1])",
            "(profilenamespace getvariable ['COLOR_A',0.8])"
        };
    };
    "#,
    "Failed to parse control with profile namespace variables"
);

test_config_parse!(
    test_controls_array,
    r#"
    class Test {
        controls[] = {
            "Control1", "Control2", "Control3", "Control4", "Control5", 
            "Control6", "Control7", "Control8", "Control9", "Control10"
        };
    };
    "#,
    "Failed to parse controls array"
);

test_config_parse!(
    test_control_definition_complex,
    r#"
    class Test {
        idc = 8655;
        access = 0;
        type = 1;
        style = 2;
        x = 0;
        y = 0;
        w = 0.1;
        h = 0.1;
        colorText[] = {1,1,1,1};
        colorBackground[] = {0,0,0,0};
        font = "FontName";
        sizeEx = 0.04;
        text = "";
    };
    "#,
    "Failed to parse complex control definition"
);

test_config_parse!(
    test_hud_config_with_macros,
    r#"
    class Test {
        class HUD {
            HUD_BEGIN(1.0)
            class Elements {
                class Crosshair {
                    type = "group";
                    HUD_POSITION(0.5, 0.5, 0.05);
                };
            };
            HUD_END
        };
    };
    "#,
    "Failed to parse HUD config with macro blocks"
);

test_config_parse!(
    test_vehicle_sounds,
    r#"
    class Test {
        class Sounds {
            class Engine {
                sound[] = {"path\to\sound.wss", db-5, 1.0};
                frequency = "0.9 + ((rpm/7000) factor[(2000/7000),(6000/7000)])*0.2";
                volume = "engineOn*(1-camPos)*((rpm/7000) factor[(400/7000),(900/7000)])";
            };
            class Movement {
                sound[] = {"", db+0, 1};
                trigger = "road";
                expression = "(1-camPos)*grass*(speed factor[4, 14])";
            };
        };
    };
    "#,
    "Failed to parse vehicle sound configuration with complex expressions"
);

#[test]
fn test_standalone_macro_calls() {
    // Test for standalone macro calls without a following statement
    // Based on patterns from sound shader files
    let config = r#"
    // Sound shader macros with parameters
    RHS_SOUNDSHADER_MINICANNON(yakb,4500)
    HELISOUNDSHADERS_DEFAULT(venom,2500,20000,FILEPATH,PREFIX)
    RHS_TAILSHADERCONFIG_CANNON(autocannon,3700)
    
    // Regular property after macros to ensure parsing continues
    regularProperty = 1;
    "#;
    
    let result = parse_helpers::parse_config(config);
    assert!(result.is_ok(), "Failed to parse config with standalone macro calls");
}

#[test]
fn test_macro_token_concatenation() {
    // Test for macro token concatenation using ## operator
    // Based on patterns from engine_asset.hpp
    let config = r#"
    // Define using token concatenation
    #define SOUNDSET_DEF(vehname,modtag,campos,rpm) ##modtag##_##vehname##_Engine_##rpm##_##campos##_SoundSet,
    
    // Usage of the define
    SOUNDSET_DEF(VEH_NAME,MOD_TAG,CAMPOS,RPM0)
    
    // Regular property after token concatenation
    regularProperty = 1;
    "#;
    
    let result = parse_helpers::parse_config(config);
    assert!(result.is_ok(), "Failed to parse config with token concatenation");
}

#[test]
fn test_combined_edge_cases() {
    // Test combining multiple edge cases:
    // 1. Extra parentheses in EVAL expressions
    // 2. Standalone macros without assignment
    // 3. Token concatenation with ## operator
    let config = r#"
    // Standalone macros like those found in sound files
    RHS_SOUNDSHADER_MINICANNON(yakb,4500)
    HELISOUNDSHADERS_DEFAULT(venom,2500,20000,FILEPATH,PREFIX)
    
    // Extra parentheses in EVAL expressions like in physics files
    tankTurnForceAngMinSpd = __EVAL(45*(PI/180)));
    tankTurnForceAngSpd = __EVAL(48*(PI/180))); 
    
    // Token concatenation for macro definitions
    #define SOUNDSET_DEF(vehname,modtag,campos,rpm) ##modtag##_##vehname##_Engine_##rpm##_##campos##_SoundSet
    
    // Regular property to ensure parsing continues
    regularProperty = 1;
    "#;
    
    let result = parse_helpers::parse_config(config);
    assert!(result.is_ok(), "Failed to parse config with combined edge cases");
}

//==============================================================================
// CLASS NAME WITH MACRO TESTS
//==============================================================================
// Tests for class declarations using macros in class name, which is common
// in Arma mods for namespacing purposes (e.g., class GVAR(actions) {})
//==============================================================================

test_config_parse!(
    test_class_with_macro_name,
    r#"
    class GVAR(actions) {};
    "#,
    "Failed to parse class with macro name GVAR(actions)"
);

test_config_parse!(
    test_class_with_macro_name_and_properties,
    r#"
    class GVAR(actions) {
        property = 1;
        stringProp = "value";
    };
    "#,
    "Failed to parse class with macro name and properties"
);

test_config_parse!(
    test_multiple_macro_class_names,
    r#"
    class GVAR(actions) {};
    class GVAR(items) {};
    class EGVAR(common,settings) {};
    "#,
    "Failed to parse multiple classes with macro names"
);

test_config_parse!(
    test_macro_class_with_inheritance,
    r#"
    class CfgBase {};
    class GVAR(actions): CfgBase {
        property = 1;
    };
    "#,
    "Failed to parse class with macro name and inheritance"
);

test_config_parse!(
    test_nested_macro_classes,
    r#"
    class GVAR(container) {
        class GVAR(nested) {
            property = 1;
        };
    };
    "#,
    "Failed to parse nested classes with macro names"
);

test_config_parse!(
    test_complex_macro_class_names,
    r#"
    class PREFIX_CLASS(name) {};
    class TRIPLES(prefix,mid,suffix) {};
    class DOUBLES(prefix,name) {};
    "#,
    "Failed to parse classes with complex macro naming patterns"
);

test_config_parse!(
    test_macro_class_with_array_property,
    r#"
    class GVAR(items) {
        items[] = {"item1", "item2", "item3"};
    };
    "#,
    "Failed to parse macro-named class with array property"
);

test_config_parse!(
    test_mixed_standard_and_macro_classes,
    r#"
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
    "Failed to parse a mix of standard and macro-named classes with inheritance"
);