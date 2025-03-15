use hemtt_common::version::Version;

use crate::{Class, Define, Number, Property, Value, analyze::CfgPatch};

#[derive(Clone, Debug, PartialEq)]
/// A config file
pub struct Config(pub Vec<Property>, pub Vec<Define>);

impl Config {
    /// Create a new config
    #[must_use]
    pub fn new(properties: Vec<Property>, defines: Vec<Define>) -> Self {
        Self(properties, defines)
    }

    #[must_use]
    pub fn to_class(&self) -> Class {
        Class::Root {
            properties: self.0.clone(),
        }
    }
    
    /// Get the properties in the config
    #[must_use]
    pub fn properties(&self) -> &[Property] {
        &self.0
    }
    
    /// Get the defines in the config
    #[must_use]
    pub fn get_defines(&self) -> &[Define] {
        &self.1
    }
}

impl Config {
    #[must_use]
    /// Get the patches
    pub fn get_patches(&self) -> Vec<CfgPatch> {
        let mut patches = Vec::new();
        for property in &self.0 {
            if let Property::Class(Class::Local {
                name, properties, ..
            }) = property
            {
                if name.as_str().to_lowercase() == "cfgpatches" {
                    for patch in properties {
                        if let Property::Class(Class::Local {
                            name, properties, ..
                        }) = patch
                        {
                            let mut required_version = Version::new(0, 0, 0, None);
                            for property in properties {
                                if let Property::Entry { name, value, .. } = property {
                                    if name.as_str().to_lowercase() == "requiredversion" {
                                        if let Value::Number(Number::Float32 { value, .. }) = value
                                        {
                                            required_version = Version::from(*value);
                                        }
                                    }
                                }
                            }
                            patches.push(CfgPatch::new(name.clone(), required_version));
                        }
                    }
                }
            }
        }
        patches
    }
}
