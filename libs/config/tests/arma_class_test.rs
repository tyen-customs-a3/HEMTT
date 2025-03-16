use hemtt_common::config::ProjectConfig;
use hemtt_preprocessor::Processor;
use hemtt_workspace::{LayerType, reporting::WorkspaceFiles};
use std::path::PathBuf;
use hemtt_config::{Property, Class, Value, Number};

// Test data structures
#[derive(Debug)]
struct ItemInfoData {
    parent: &'static str,
    mass: f64,
}

#[derive(Debug)]
struct TestItemData {
    name: &'static str,
    parent: &'static str,
    scope: Option<i32>,
    type_val: Option<i32>,
    ace_is_medical_item: i32,
    author: Option<&'static str>,
    display_name: Option<&'static str>,
    description_short: Option<&'static str>,
    description_use: Option<&'static str>,
    model: Option<&'static str>,
    picture: Option<&'static str>,
    item_info: ItemInfoData,
}

const TEST_ITEMS: &[TestItemData] = &[
    TestItemData {
        name: "FirstAidKit",
        parent: "ItemCore",
        scope: None,
        type_val: Some(0),
        ace_is_medical_item: 1,
        author: None,
        display_name: None,
        description_short: None,
        description_use: None,
        model: None,
        picture: None,
        item_info: ItemInfoData {
            parent: "InventoryFirstAidKitItem_Base_F",
            mass: 4.0,
        },
    },
    TestItemData {
        name: "Medikit",
        parent: "ItemCore",
        scope: None,
        type_val: Some(0),
        ace_is_medical_item: 1,
        author: None,
        display_name: None,
        description_short: None,
        description_use: None,
        model: None,
        picture: None,
        item_info: ItemInfoData {
            parent: "MedikitItem",
            mass: 60.0,
        },
    },
    TestItemData {
        name: "ACE_fieldDressing",
        parent: "ACE_ItemCore",
        scope: Some(2),
        type_val: None,
        ace_is_medical_item: 1,
        author: Some("ACE-Team"),
        display_name: Some("Field Dressing"),
        description_short: Some("Basic bandage for wounds"),
        description_use: Some("Used for basic treatment"),
        model: Some("data\\bandage.p3d"),
        picture: Some("ui\\fieldDressing_ca.paa"),
        item_info: ItemInfoData {
            parent: "CBA_MiscItem_ItemInfo",
            mass: 0.6,
        },
    },
    TestItemData {
        name: "ACE_packingBandage",
        parent: "ACE_ItemCore",
        scope: Some(2),
        type_val: None,
        ace_is_medical_item: 1,
        author: Some("ACE-Team"),
        display_name: Some("Packing Bandage"),
        description_short: Some("Bandage for deep wounds"),
        description_use: Some("Pack deep wounds"),
        model: Some("data\\packingbandage.p3d"),
        picture: Some("ui\\packingBandage_ca.paa"),
        item_info: ItemInfoData {
            parent: "CBA_MiscItem_ItemInfo",
            mass: 0.6,
        },
    },
];

const BASE_CLASSES: &[&str] = &[
    "ItemCore",
    "ACE_ItemCore",
    "CBA_MiscItem_ItemInfo",
    "InventoryFirstAidKitItem_Base_F",
    "MedikitItem",
];

// Helper function to find a class by name in a list of properties
fn find_class<'a>(properties: &'a [Property], name: &str) -> Option<&'a Class> {
    properties.iter().find_map(|p| {
        if let Property::Class(c) = p {
            if c.name().map_or(false, |n| n.as_str() == name) {
                Some(c)
            } else {
                None
            }
        } else {
            None
        }
    })
}

// Helper function to get property value by name from properties list
fn get_property_value_by_name<'a>(properties: &'a [Property], name: &str) -> Option<&'a Value> {
    properties.iter().find_map(|p| {
        if let Property::Entry { name: n, value, expected_array: false } = p {
            if n.as_str() == name {
                Some(value)
            } else {
                None
            }
        } else {
            None
        }
    })
}

// Helper function to validate a medical item against test data
fn validate_medical_item(item_class: &Class, test_data: &TestItemData) {
    // Validate parent class
    assert_eq!(item_class.parent().map(|p| p.as_str()), Some(test_data.parent),
        "{}: parent should be {}", test_data.name, test_data.parent);

    // Validate scope if present
    if let Some(scope_val) = test_data.scope {
        if let Some(Value::Number(num)) = get_property_value_by_name(item_class.properties(), "scope") {
            assert_eq!(num.to_string(), scope_val.to_string(),
                "{}: scope should be {}", test_data.name, scope_val);
        } else {
            panic!("{}: scope property not found or invalid", test_data.name);
        }
    }

    // Validate type if present
    if let Some(type_val) = test_data.type_val {
        if let Some(Value::Number(num)) = get_property_value_by_name(item_class.properties(), "type") {
            assert_eq!(num.to_string(), type_val.to_string(),
                "{}: type should be {}", test_data.name, type_val);
        } else {
            panic!("{}: type property not found or invalid", test_data.name);
        }
    }

    // Validate ACE_isMedicalItem
    if let Some(Value::Number(num)) = get_property_value_by_name(item_class.properties(), "ACE_isMedicalItem") {
        assert_eq!(num.to_string(), test_data.ace_is_medical_item.to_string(),
            "{}: ACE_isMedicalItem should be {}", test_data.name, test_data.ace_is_medical_item);
    } else {
        panic!("{}: ACE_isMedicalItem property not found or invalid", test_data.name);
    }

    // Validate string properties if present
    let string_properties = [
        ("author", test_data.author),
        ("displayName", test_data.display_name),
        ("descriptionShort", test_data.description_short),
        ("descriptionUse", test_data.description_use),
        ("model", test_data.model),
        ("picture", test_data.picture),
    ];

    for (prop_name, expected_value) in string_properties.iter() {
        if let Some(expected) = expected_value {
            if let Some(Value::Str(s)) = get_property_value_by_name(item_class.properties(), prop_name) {
                assert_eq!(s.value(), *expected,
                    "{}: {} should be '{}'", test_data.name, prop_name, expected);
            } else {
                panic!("{}: {} property not found or invalid", test_data.name, prop_name);
            }
        }
    }

    // Validate ItemInfo
    let item_info = find_class(item_class.properties(), "ItemInfo")
        .unwrap_or_else(|| panic!("{}: ItemInfo class not found", test_data.name));
    
    assert_eq!(item_info.parent().map(|p| p.as_str()), Some(test_data.item_info.parent),
        "{}: ItemInfo parent should be {}", test_data.name, test_data.item_info.parent);

    if let Some(Value::Number(num)) = get_property_value_by_name(item_info.properties(), "mass") {
        assert!((num.to_string().parse::<f64>().unwrap() - test_data.item_info.mass).abs() < f64::EPSILON,
            "{}: mass should be {}", test_data.name, test_data.item_info.mass);
    } else {
        panic!("{}: mass property not found or invalid", test_data.name);
    }
}

#[test]
fn test_ace_medical_base_classes() {
    let test_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures");
    
    let workspace = hemtt_workspace::Workspace::builder()
        .physical(&test_dir, LayerType::Source)
        .finish(None, false, &hemtt_common::config::PDriveOption::Disallow)
        .unwrap();

    let source = workspace.join("ace_medical.hpp").unwrap();
    let processed = Processor::run(&source).unwrap();
    let parsed = hemtt_config::parse(None, &processed).unwrap();
    let config = parsed.config();

    let cfg_weapons = find_class(config.properties(), "CfgWeapons")
        .expect("CfgWeapons class not found");

    // Verify all base classes exist
    for base_class in BASE_CLASSES {
        assert!(find_class(cfg_weapons.properties(), base_class).is_some(),
            "{} base class not found", base_class);
    }
}

#[test]
fn test_ace_medical_items() {
    let test_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures");
    
    let workspace = hemtt_workspace::Workspace::builder()
        .physical(&test_dir, LayerType::Source)
        .finish(None, false, &hemtt_common::config::PDriveOption::Disallow)
        .unwrap();

    let source = workspace.join("ace_medical.hpp").unwrap();
    let processed = Processor::run(&source).unwrap();
    let parsed = hemtt_config::parse(None, &processed).unwrap();
    let config = parsed.config();

    let cfg_weapons = find_class(config.properties(), "CfgWeapons")
        .expect("CfgWeapons class not found");

    // Test all medical items
    for test_item in TEST_ITEMS {
        let item_class = find_class(cfg_weapons.properties(), test_item.name)
            .unwrap_or_else(|| panic!("{} class not found", test_item.name));
        validate_medical_item(item_class, test_item);
    }
}