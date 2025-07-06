use hemtt_preprocessor::Processor;
use hemtt_workspace::{LayerType, Workspace, reporting::WorkspaceFiles};
use hemtt_common::config::PDriveOption;
use std::path::PathBuf;
use hemtt_config::{Class, Property, Value};

/// Test that ACE config file can be parsed correctly
#[test]
fn test_ace_config_parsing() {
    // Set up workspace with the fixtures directory
    let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    
    let workspace = Workspace::builder()
        .physical(&fixtures_dir, LayerType::Source)
        .finish(None, false, &PDriveOption::Disallow)
        .unwrap();

    // Process and parse the file
    let source = workspace.join("ace_cfg.hpp").unwrap();
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

    // Get CfgWeapons class
    let cfg_weapons = config.0.iter()
        .find_map(|p| {
            if let Property::Class(c) = p {
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

    // Verify base class declarations
    let base_classes = ["ItemCore", "ACE_ItemCore", "CBA_MiscItem_ItemInfo", 
                       "InventoryFirstAidKitItem_Base_F", "MedikitItem"];
    for class_name in base_classes {
        assert!(has_base_class(cfg_weapons.properties(), class_name), 
                "Base class {} not found", class_name);
    }

    // Test FirstAidKit class
    let first_aid_kit = find_class(cfg_weapons.properties(), "FirstAidKit")
        .expect("FirstAidKit class not found");
    assert_eq!(get_parent_class(first_aid_kit), Some("ItemCore"), 
              "FirstAidKit should inherit from ItemCore");
    verify_property(first_aid_kit, "type", "0");
    verify_property(first_aid_kit, "ACE_isMedicalItem", "1");
    
    let fak_item_info = find_class(first_aid_kit.properties(), "ItemInfo")
        .expect("ItemInfo class not found in FirstAidKit");
    assert_eq!(get_parent_class(fak_item_info), Some("InventoryFirstAidKitItem_Base_F"),
              "FirstAidKit ItemInfo should inherit from InventoryFirstAidKitItem_Base_F");
    verify_property(fak_item_info, "mass", "4");

    // Test Medikit class
    let medikit = find_class(cfg_weapons.properties(), "Medikit")
        .expect("Medikit class not found");
    assert_eq!(get_parent_class(medikit), Some("ItemCore"),
              "Medikit should inherit from ItemCore");
    verify_property(medikit, "type", "0");
    verify_property(medikit, "ACE_isMedicalItem", "1");
    
    let medikit_item_info = find_class(medikit.properties(), "ItemInfo")
        .expect("ItemInfo class not found in Medikit");
    assert_eq!(get_parent_class(medikit_item_info), Some("MedikitItem"),
              "Medikit ItemInfo should inherit from MedikitItem");
    verify_property(medikit_item_info, "mass", "60");

    // Test ACE_fieldDressing class
    let field_dressing = find_class(cfg_weapons.properties(), "ACE_fieldDressing")
        .expect("ACE_fieldDressing class not found");
    assert_eq!(get_parent_class(field_dressing), Some("ACE_ItemCore"),
              "ACE_fieldDressing should inherit from ACE_ItemCore");

    // Verify all field dressing properties
    let field_dressing_props = [
        ("scope", "2"),
        ("author", "ECSTRING(common,ACETeam)"),
        ("model", "QPATHTOF(data\\bandage.p3d)"),
        ("picture", "QPATHTOF(ui\\fieldDressing_ca.paa)"),
        ("displayName", "CSTRING(Bandage_Basic_Display)"),
        ("descriptionShort", "CSTRING(Bandage_Basic_Desc_Short)"),
        ("descriptionUse", "CSTRING(Bandage_Basic_Desc_Use)"),
        ("ACE_isMedicalItem", "1"),
    ];
    for (name, expected) in field_dressing_props.iter() {
        verify_property(field_dressing, name, expected);
    }

    let fd_item_info = find_class(field_dressing.properties(), "ItemInfo")
        .expect("ItemInfo class not found in ACE_fieldDressing");
    assert_eq!(get_parent_class(fd_item_info), Some("CBA_MiscItem_ItemInfo"),
              "ACE_fieldDressing ItemInfo should inherit from CBA_MiscItem_ItemInfo");
    verify_property(fd_item_info, "mass", "0.6");

    // Test ACE_packingBandage class
    let packing_bandage = find_class(cfg_weapons.properties(), "ACE_packingBandage")
        .expect("ACE_packingBandage class not found");
    assert_eq!(get_parent_class(packing_bandage), Some("ACE_ItemCore"),
              "ACE_packingBandage should inherit from ACE_ItemCore");

    // Verify all packing bandage properties
    let packing_bandage_props = [
        ("scope", "2"),
        ("author", "ECSTRING(common,ACETeam)"),
        ("displayName", "CSTRING(Packing_Bandage_Display)"),
        ("picture", "QPATHTOF(ui\\packingBandage_ca.paa)"),
        ("model", "QPATHTOF(data\\packingbandage.p3d)"),
        ("descriptionShort", "CSTRING(Packing_Bandage_Desc_Short)"),
        ("descriptionUse", "CSTRING(Packing_Bandage_Desc_Use)"),
        ("ACE_isMedicalItem", "1"),
    ];
    for (name, expected) in packing_bandage_props.iter() {
        verify_property(packing_bandage, name, expected);
    }

    let pb_item_info = find_class(packing_bandage.properties(), "ItemInfo")
        .expect("ItemInfo class not found in ACE_packingBandage");
    assert_eq!(get_parent_class(pb_item_info), Some("CBA_MiscItem_ItemInfo"),
              "ACE_packingBandage ItemInfo should inherit from CBA_MiscItem_ItemInfo");
    verify_property(pb_item_info, "mass", "0.6");

    // Verify total number of classes to ensure we haven't missed any
    let total_classes = count_classes(cfg_weapons.properties());
    assert_eq!(total_classes, 9, "Expected exactly 9 classes in CfgWeapons");
}

// Helper function to find a class by name
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

// Helper function to verify a property value
fn verify_property(class: &Class, name: &str, expected: &str) {
    let value = class.properties().iter()
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

    match value {
        Value::Number(n) => assert_eq!(n.to_string(), expected, "Property {} has incorrect value", name),
        Value::Str(s) => assert_eq!(s.value(), expected, "Property {} has incorrect value", name),
        Value::Macro(m) => assert_eq!(format!("{}({})", 
                                            m.name().value(), 
                                            m.args().iter()
                                                .map(|a| a.value().to_string())
                                                .collect::<Vec<_>>()
                                                .join(",")), 
                                    expected, 
                                    "Property {} has incorrect macro value", name),
        _ => panic!("Property {} has unexpected type", name),
    }
}

// Helper function to check if a base class exists
fn has_base_class(properties: &[Property], name: &str) -> bool {
    properties.iter().any(|p| {
        if let Property::Class(Class::External { name: class_name }) = p {
            class_name.as_str() == name
        } else {
            false
        }
    })
}

// Helper function to get parent class name
fn get_parent_class(class: &Class) -> Option<&str> {
    match class {
        Class::Local { parent, .. } => parent.as_ref().map(|p| p.as_str()),
        _ => None
    }
}

// Helper function to count total number of classes
fn count_classes(properties: &[Property]) -> usize {
    properties.iter().filter(|p| matches!(p, Property::Class(_))).count()
} 