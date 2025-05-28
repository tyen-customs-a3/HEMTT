use hemtt_common::version::Version;

use crate::{Class, Number, Property, Value, analyze::CfgPatch};

#[derive(Clone, Debug, PartialEq)]
/// A config file
pub struct Config(pub Vec<Property>);

impl Config {
    #[must_use]
    pub fn to_class(&self) -> Class {
        Class::Root {
            properties: self.0.clone(),
        }
    }
}

impl Config {
    #[must_use]
    /// Get the patches
    pub fn get_patches(&self) -> Vec<CfgPatch> {
        // Early return if no properties
        if self.0.is_empty() {
            return Vec::new();
        }
        
        // Find CfgPatches class efficiently
        for property in &self.0 {
            if let Property::Class(Class::Local {
                name, properties, ..
            }) = property
            {
                // Use eq_ignore_ascii_case instead of to_lowercase() to avoid allocation
                if name.as_str().eq_ignore_ascii_case("cfgpatches") {
                    // Pre-allocate with estimated capacity
                    let mut patches = Vec::with_capacity(properties.len());
                    
                    for patch in properties {
                        if let Property::Class(Class::Local {
                            name, properties, ..
                        }) = patch
                        {
                            let mut required_version = Version::new(0, 0, 0, None);
                            
                            // Look for requiredversion property
                            for property in properties {
                                if let Property::Entry { name: prop_name, value, .. } = property {
                                    if prop_name.as_str().eq_ignore_ascii_case("requiredversion") {
                                        if let Value::Number(Number::Float32 { value, .. }) = value {
                                            required_version = Version::from(*value);
                                            break; // Early exit once found
                                        }
                                    }
                                }
                            }
                            patches.push(CfgPatch::new(name.clone(), required_version));
                        }
                    }
                    return patches; // Early return once CfgPatches is processed
                }
            }
        }
        Vec::new() // No CfgPatches found
    }
}
