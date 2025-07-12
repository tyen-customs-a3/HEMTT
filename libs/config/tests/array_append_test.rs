use hemtt_config::{Config, Property, Class};
use chumsky::Parser;

#[test]
fn test_array_append_syntax() {
    let config_text = r#"
class CfgMagazineWells
{
    class CBA_45ACP_Thompson_Stick
    {
        sp_fwa_Magazines[] += {"sp_fwa_30Rnd_45acp_thompson_m1a1","sp_fwa_30Rnd_45acp_thompson_m1a1_Tracer"};
    };
};
"#;
    
    let (config, errors) = hemtt_config::parse::config().parse_recovery(config_text);
    
    if !errors.is_empty() {
        eprintln!("Failed to parse config with += syntax:");
        for error in errors {
            eprintln!("  Error: {:?}", error);
        }
        panic!("Parser should handle += array syntax");
    }
    
    if let Some(config) = config {
        println!("Successfully parsed config with += syntax");
        
        // Verify the parsed structure
        let cfgmagazinewells = config.0.iter()
            .find(|p| p.name().value == "CfgMagazineWells")
            .expect("Should find CfgMagazineWells class");
        
        match cfgmagazinewells {
            Property::Class(Class::Local { properties, .. }) => {
                assert!(!properties.is_empty(), "Should have properties in CfgMagazineWells");
                
                // Check the inner class
                let inner_class = properties.iter()
                    .find(|p| p.name().value == "CBA_45ACP_Thompson_Stick")
                    .expect("Should find CBA_45ACP_Thompson_Stick class");
                
                match inner_class {
                    Property::Class(Class::Local { properties: inner_props, .. }) => {
                        let mag_prop = inner_props.iter()
                            .find(|p| p.name().value == "sp_fwa_Magazines")
                            .expect("Should find sp_fwa_Magazines property");
                        
                        match mag_prop {
                            Property::Entry { value, expected_array, .. } => {
                                assert!(expected_array, "Should be marked as array");
                                if let hemtt_config::Value::Array(arr) = value {
                                    assert!(arr.expand, "Array should have expand flag set for += syntax");
                                    assert_eq!(arr.items.len(), 2, "Array should have 2 items");
                                } else {
                                    panic!("Expected array value");
                                }
                            }
                            _ => panic!("Expected Entry property for sp_fwa_Magazines"),
                        }
                    }
                    _ => panic!("Expected local class for CBA_45ACP_Thompson_Stick"),
                }
            }
            _ => panic!("Expected local class for CfgMagazineWells"),
        }
    } else {
        panic!("Parser returned None");
    }
}

#[test]
fn test_soundshaders_array() {
    let config_text = r#"
class SMGVermin_Shot_SoundSet;
class sp_fwa_thompson_Shot_SoundSet: SMGVermin_Shot_SoundSet
{
    soundShaders[] = {"sp_fwa_thompson_Closure_SoundShader","sp_fwa_thompson_closeShot_SoundShader","sp_fwa_thompson_midShot_SoundShader","sp_fwa_thompson_distShot_SoundShader"};
};
"#;
    
    let (config, errors) = hemtt_config::parse::config().parse_recovery(config_text);
    
    if !errors.is_empty() {
        eprintln!("Failed to parse soundShaders array:");
        for error in errors {
            eprintln!("  Error: {:?}", error);
        }
        panic!("Parser should handle regular array syntax");
    }
    
    if let Some(config) = config {
        println!("Successfully parsed soundShaders array");
        
        // Verify the parsed structure
        let sound_class = config.0.iter()
            .find(|p| p.name().value == "sp_fwa_thompson_Shot_SoundSet")
            .expect("Should find sp_fwa_thompson_Shot_SoundSet class");
        
        match sound_class {
            Property::Class(Class::Local { properties, .. }) => {
                let sound_shaders = properties.iter()
                    .find(|p| p.name().value == "soundShaders")
                    .expect("Should find soundShaders property");
                
                match sound_shaders {
                    Property::Entry { value, expected_array, .. } => {
                        assert!(expected_array, "Should be marked as array");
                        if let hemtt_config::Value::Array(arr) = value {
                            assert!(!arr.expand, "Regular array should not have expand flag");
                            assert_eq!(arr.items.len(), 4, "Array should have 4 items");
                        } else {
                            panic!("Expected array value");
                        }
                    }
                    _ => panic!("Expected Entry property for soundShaders"),
                }
            }
            _ => panic!("Expected local class for sp_fwa_thompson_Shot_SoundSet"),
        }
    } else {
        panic!("Parser returned None");
    }
}