use hemtt_sqm::{parse_sqm, parse_sqm_with_config, Class, Value,
          emit_diagnostics, ParallelConfig, IntegratedLexer};

#[test]
fn test_simple_class() {
    let input = r#"
        version = 54;
        class Intel {
            briefingName = "Test Mission";
            startWeather = 0.3;
        };
    "#;

    let result = parse_sqm(input).unwrap();
    
    // Check version
    assert_eq!(result.version, Some(54));

    // Check Intel class exists
    let intel = result.classes.get("Intel").unwrap();
    assert_eq!(intel.len(), 1);
    let intel = &intel[0];

    // Check Intel properties
    assert_eq!(
        intel.properties.get("briefingName").unwrap(),
        &Value::String("Test Mission".to_string())
    );
    assert_eq!(
        intel.properties.get("startWeather").unwrap(),
        &Value::Number(0.3)
    );
}

#[test]
fn test_array_values() {
    let input = r#"
        class Item0 {
            position[] = {1234.5, 67.8, 90.1};
            items = 3;
        };
    "#;

    // Debug the actual tokens 
    let mut lexer = hemtt_sqm::IntegratedLexer::new();
    let tokens = lexer.lex(input).to_vec(); // Convert to owned Vec to avoid borrow issues
    println!("Raw tokens from lexer:");
    for (i, token) in tokens.iter().enumerate() {
        println!("Token[{}]: {:?}", i, token);
    }
    
    // Add boundary map debugging
    let boundaries = lexer.scan_boundaries().expect("Failed to scan boundaries");
    println!("\nBoundary Map:");
    for (i, boundary) in boundaries.boundaries.iter().enumerate() {
        println!("Boundary {}: name={}, range={:?}, contents_range={:?}", 
                i, boundary.name, boundary.range, boundary.contents_range);
        
        // Print the tokens in the content range
        println!("Content tokens for {}:", boundary.name);
        for j in boundary.contents_range.clone() {
            if j < tokens.len() {
                println!("  Token[{}]: {:?}", j, tokens[j]);
            }
        }
    }

    // Use custom config with minimal parallelism for testing
    let config = ParallelConfig {
        min_tokens_for_parallel: 0, // Simple test, no need for parallelism
        max_parallel_tasks: 1,
        min_properties_for_parallel: 0,
    };
    
    let result = parse_sqm_with_config(input, config).unwrap();
    
    let item = result.classes.get("Item0").unwrap();
    assert_eq!(item.len(), 1);
    let item = &item[0];

    // Debug print all properties
    println!("Available properties: {:?}", item.properties);
    
    // Check array property
    match item.properties.get("position") {
        Some(Value::Array(arr)) => {
            println!("Found position array: {:?}", arr);
            println!("Array length: {}", arr.len());
            assert_eq!(arr.len(), 3, "Expected 3 elements in position array");
            assert_eq!(arr[0], Value::Number(1234.5));
            assert_eq!(arr[1], Value::Number(67.8));
            assert_eq!(arr[2], Value::Number(90.1));
        }
        Some(other) => panic!("Expected array value, got: {:?}", other),
        None => panic!("Position property not found in item properties"),
    }

    // Check if the 'items' property exists and has the correct value
    match item.properties.get("items") {
        Some(Value::Integer(val)) => {
            println!("Found items property with value: {}", val);
            assert_eq!(*val, 3);
        },
        Some(other) => {
            println!("Found items property with unexpected type: {:?}", other);
            panic!("Expected Integer(3), got: {:?}", other);
        },
        None => {
            println!("The 'items' property is STILL not being correctly parsed!");
            panic!("The 'items' property was not found in the parsed result");
        }
    }
}

#[test]
fn test_defines() {
    let input = r#"
        #define _ARMA_
        #define SOME_VALUE
        
        class Mission {
            version = 1;
        };
    "#;

    let result = parse_sqm(input).unwrap();
    
    assert_eq!(result.defines.len(), 2);
    assert!(result.defines.contains(&"_ARMA_".to_string()));
    assert!(result.defines.contains(&"SOME_VALUE".to_string()));
}

#[test]
fn test_nested_classes() {
    let input = r#"
        class Mission {
            class Entities {
                items = 2;
                class Item0 {
                    dataType = "Group";
                };
                class Item1 {
                    dataType = "Logic";
                };
            };
        };
    "#;

    // Use custom config with moderate parallelism for nested classes
    let config = ParallelConfig {
        min_tokens_for_parallel: 50,
        max_parallel_tasks: 2,
        min_properties_for_parallel: 5,
    };
    
    let result = parse_sqm_with_config(input, config).unwrap();
    
    let mission = result.classes.get("Mission").unwrap();
    assert_eq!(mission.len(), 1);
    let mission = &mission[0];

    let entities = mission.classes.get("Entities").unwrap();
    assert_eq!(entities.len(), 1);
    let entities = &entities[0];

    // Print all properties for debugging
    println!("Entities properties: {:?}", entities.properties);

    // Check if the items property exists (case insensitive)
    let items_key = entities.properties.keys()
        .find(|k| k.to_lowercase() == "items")
        .cloned()
        .unwrap_or_else(|| "items".to_string());
    
    assert_eq!(
        entities.properties.get(&items_key).unwrap(),
        &Value::Integer(2)
    );

    let items = entities.classes.get("Item0").unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(
        items[0].properties.get("dataType").unwrap(),
        &Value::String("Group".to_string())
    );

    let items = entities.classes.get("Item1").unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(
        items[0].properties.get("dataType").unwrap(),
        &Value::String("Logic".to_string())
    );
}

#[test]
fn test_error_reporting() {

    // Add a syntax that will certainly fail parsing
    let definitely_invalid = r#"
        class Test {
            property = "unclosed string;
        };
    "#;

    let result = parse_sqm(definitely_invalid);
    assert!(result.is_err(), "Parser should report an error on invalid input");
    
    if let Err(err) = result {
        // Generate detailed diagnostics for the error
        let diagnostics = emit_diagnostics(definitely_invalid, &err);
        println!("Error diagnostics:\n{}", diagnostics);
    }
}

#[test]
fn test_deep_nested_classes() {
    // Create a simple SQM sample with a similar nested structure but simpler
    let input = r#"
        class Mission {
            class Entities {
                items = 1;
                class Item0 {
                    dataType = "Unit";
                    class Attributes {
                        class Inventory {
                            class primaryWeapon {
                                name = "rhs_weap_ak74n";
                                muzzle = "rhs_acc_dtk1983";
                                firemode = "rhs_weap_ak74n:Single";
                                class primaryMuzzleMag {
                                    name = "rhs_30Rnd_545x39_7N10_AK";
                                    ammoLeft = 30;
                                };
                            };
                        };
                    };
                };
            };
        };
    "#;
    
    // Parse with minimal parallelism to ensure consistent results
    let config = ParallelConfig {
        min_tokens_for_parallel: 50,
        max_parallel_tasks: 2,
        min_properties_for_parallel: 10,
    };
    
    let result = parse_sqm_with_config(input, config).unwrap();
    
    // Navigate to the Inventory class - it's inside a nested structure
    // First, find the mission class
    assert!(result.classes.contains_key("Mission"));
    let mission = &result.classes["Mission"][0];
    
    // Find Entities in Mission
    assert!(mission.classes.contains_key("Entities"));
    let entities = &mission.classes["Entities"][0];
    
    // Find Item0 in Entities 
    assert!(entities.classes.contains_key("Item0"));
    let item0 = &entities.classes["Item0"][0];
    
    // Find Attributes in Item0
    assert!(item0.classes.contains_key("Attributes"));
    let attributes = &item0.classes["Attributes"][0];
    
    // Find Inventory in Attributes
    assert!(attributes.classes.contains_key("Inventory"));
    let inventory = &attributes.classes["Inventory"][0];
    
    // Find primaryWeapon in Inventory
    assert!(inventory.classes.contains_key("primaryWeapon"));
    let primary_weapon = &inventory.classes["primaryWeapon"][0];
    
    // Verify primaryWeapon properties 
    assert_eq!(primary_weapon.properties.get("name").unwrap(), &Value::String("rhs_weap_ak74n".to_string()));
    assert_eq!(primary_weapon.properties.get("muzzle").unwrap(), &Value::String("rhs_acc_dtk1983".to_string()));
    assert_eq!(primary_weapon.properties.get("firemode").unwrap(), &Value::String("rhs_weap_ak74n:Single".to_string()));
    
    // Find primaryMuzzleMag in primaryWeapon
    assert!(primary_weapon.classes.contains_key("primaryMuzzleMag"));
    let primary_muzzle_mag = &primary_weapon.classes["primaryMuzzleMag"][0];
    
    // Verify primaryMuzzleMag properties
    assert_eq!(primary_muzzle_mag.properties.get("name").unwrap(), &Value::String("rhs_30Rnd_545x39_7N10_AK".to_string()));
    assert_eq!(primary_muzzle_mag.properties.get("ammoLeft").unwrap(), &Value::Integer(30));
}

#[test]
fn test_scientific_notation() {
    // Create a sample with scientific notation
    let input = r#"
        class ScientificValues {
            positive_e = 1.23e+10;
            negative_e = 4.56e-3;
            large_negative = -3.4028235e+38;
            simple_e = 7.8e5;
        };
    "#;
    
    let result = parse_sqm(input).unwrap();
    
    // Verify scientific notation parsing
    assert!(result.classes.contains_key("ScientificValues"));
    let sci_values = &result.classes["ScientificValues"][0];
    
    // Verify all values are correctly parsed
    // Note: these will be parsed as strings in NumberLit, then converted to f64 values
    assert!(sci_values.properties.contains_key("positive_e"));
    assert!(sci_values.properties.contains_key("negative_e"));
    assert!(sci_values.properties.contains_key("large_negative"));
    assert!(sci_values.properties.contains_key("simple_e"));
    
    if let Value::Number(positive_e) = sci_values.properties["positive_e"] {
        assert!((positive_e - 1.23e+10).abs() < 0.0001);
    } else {
        panic!("Expected Number type for 'positive_e'");
    }
    
    if let Value::Number(negative_e) = sci_values.properties["negative_e"] {
        assert!((negative_e - 4.56e-3).abs() < 0.0001);
    } else {
        panic!("Expected Number type for 'negative_e'");
    }
    
    if let Value::Number(large_negative) = sci_values.properties["large_negative"] {
        // Use a relative error percentage for comparison due to floating-point precision
        let expected: f64 = -3.4028235e+38;
        let relative_error = ((large_negative - expected) / expected).abs();
        assert!(relative_error < 0.0001);
    } else {
        panic!("Expected Number type for 'large_negative'");
    }
    
    if let Value::Number(simple_e) = sci_values.properties["simple_e"] {
        assert!((simple_e - 7.8e5).abs() < 0.0001);
    } else {
        panic!("Expected Number type for 'simple_e'");
    }
}

#[test]
fn test_find_primary_weapon_recursive() {
    // Read the mission file
    let fixture_path = "tests/fixtures/mission_full_simple.sqm";
    let input = std::fs::read_to_string(fixture_path).expect("Failed to read fixture file");
    
    // Configure parser
    let config = ParallelConfig {
        min_tokens_for_parallel: 1000,
        max_parallel_tasks: 4,
        min_properties_for_parallel: 20,
    };
    
    // Parse the file
    let result = parse_sqm_with_config(input, config).unwrap();
    
    // Define a recursive function to search for the primaryWeapon class
    fn find_primary_weapon(class: &Class) -> Option<&Class> {
        // Check if this is the class we're looking for
        if class.name == "primaryWeapon" && 
           class.properties.get("name").map(|v| v == &Value::String("rhs_weap_ak74n".to_string())).unwrap_or(false) {
            return Some(class);
        }
        
        // Search in all nested classes
        for classes in class.classes.values() {
            for nested_class in classes {
                if let Some(found) = find_primary_weapon(nested_class) {
                    return Some(found);
                }
            }
        }
        
        None
    }
    
    // Start the search from the top-level classes
    let mut found_weapon = None;
    
    for classes in result.classes.values() {
        for class in classes {
            if let Some(weapon) = find_primary_weapon(class) {
                found_weapon = Some(weapon);
                break;
            }
        }
        if found_weapon.is_some() {
            break;
        }
    }
    
    // Verify we found the primaryWeapon
    assert!(found_weapon.is_some(), "Failed to find primaryWeapon class with rhs_weap_ak74n");
    
    let weapon = found_weapon.unwrap();
    assert_eq!(weapon.name, "primaryWeapon");
    assert_eq!(weapon.properties.get("name").unwrap(), &Value::String("rhs_weap_ak74n".to_string()));
    assert_eq!(weapon.properties.get("muzzle").unwrap(), &Value::String("rhs_acc_dtk1983".to_string()));
    assert_eq!(weapon.properties.get("firemode").unwrap(), &Value::String("rhs_weap_ak74n:Single".to_string()));
    
    // Check if there's a primaryMuzzleMag subclass
    assert!(weapon.classes.contains_key("primaryMuzzleMag"));
    let muzzle_mag = &weapon.classes["primaryMuzzleMag"][0];
    assert_eq!(muzzle_mag.properties.get("name").unwrap(), &Value::String("rhs_30Rnd_545x39_7N10_AK".to_string()));
    assert_eq!(muzzle_mag.properties.get("ammoLeft").unwrap(), &Value::Integer(30));
}

#[test]
fn test_parse_mission_files() {
    let test_files = ["tests/fixtures/mission_test1.sqm", "tests/fixtures/mission_test2.sqm"];
    
    for file_path in test_files {
        // Read the mission file
        let input = std::fs::read_to_string(file_path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", file_path, e));
        
        println!("\nTesting file: {}", file_path);
        
        // Create a lexer instance for diagnostic information
        let mut lexer = IntegratedLexer::new();
        let tokens = lexer.lex(&input);
        
        // Print token information if in verbose mode
        if std::env::var("RUST_TEST_NOCAPTURE").is_ok() {
            println!("Token count: {}", tokens.len());
            println!("First few tokens:");
            for (i, token) in tokens.iter().take(5).enumerate() {
                println!("  Token[{}]: {:?}", i, token);
            }
        }
        
        // Scan for class boundaries
        match lexer.scan_boundaries() {
            Ok(boundaries) => {
                println!("Successfully scanned class boundaries");
                println!("Found {} class boundaries", boundaries.boundaries.len());
            }
            Err(err) => {
                panic!("Failed to scan boundaries in {}: {:?}", file_path, err);
            }
        }
        
        // Try to parse the file
        match parse_sqm(&input) {
            Ok(result) => {
                println!("Successfully parsed {}", file_path);
                println!("Found {} top-level classes", result.classes.len());
                if let Some(version) = result.version {
                    println!("SQM Version: {}", version);
                }
                println!("Define directives: {}", result.defines.len());
                
                // Print top-level class names
                println!("Top-level classes:");
                for class_name in result.classes.keys() {
                    println!("  - {}", class_name);
                }
            }
            Err(err) => {
                // Generate and print detailed diagnostics
                let diagnostics = emit_diagnostics(&input, &err);
                panic!("Failed to parse {}\nDiagnostic output:\n{}", file_path, diagnostics);
            }
        }
    }
}

#[test]
fn test_string_with_script() {
    let input = r#"
        class Item0 {
            init = "this addAction [""Flip vehicle"",{ \n params [""_vehicle"", ""_caller"", ""_actionId"", ""_arguments""]; \n _normalVec = surfaceNormal getPos _vehicle; \n if (!local _vehicle) then { \n [_vehicle,_normalVec] remoteExec [""setVectorUp"",_vehicle]; \n } else { \n _vehicle setVectorUp _normalVec; \n }; \n _vehicle setPosATL [getPosATL _vehicle select 0, getPosATL _vehicle select 1, 0]; \n},[],1.5,true,true,"""",""(vectorUp _target) vectorCos (surfaceNormal getPos _target) <0.5"",5];";
        };
    "#;

    let result = parse_sqm(input).unwrap();
    
    let item = result.classes.get("Item0").unwrap();
    assert_eq!(item.len(), 1);
    let item = &item[0];

    // Verify the init script is preserved exactly as provided
    match item.properties.get("init") {
        Some(Value::String(script)) => {
            println!("Parsed script: {}", script);
            // Check for exact string patterns as they should appear in SQM format
            assert!(script.contains(r#"this addAction"#));
            assert!(script.contains(r#"[""Flip vehicle"","#));
            assert!(script.contains(r#"params [""_vehicle"""#));
            assert!(script.contains(r#"_normalVec = surfaceNormal getPos _vehicle"#));
            assert!(script.contains(r#"\n"#)); // Should preserve \n literally
            assert!(script.contains(r#"[""setVectorUp"""#));
            assert!(script.contains(r#"_vehicle setVectorUp _normalVec"#));
            
            // Print the exact string for debugging
            println!("String patterns to match:");
            println!("1. {}", r#"[""Flip vehicle"","#);
            println!("2. {}", r#"params [""_vehicle"""#);
            println!("3. {}", r#"\n"#);
            println!("Found in script:");
            println!("1. {}", script.contains(r#"[""Flip vehicle"","#));
            println!("2. {}", script.contains(r#"params [""_vehicle"""#));
            println!("3. {}", script.contains(r#"\n"#));
            println!("Full script:");
            println!("{}", script);
        },
        other => panic!("Expected init property to be a string, got: {:?}", other),
    }
}
