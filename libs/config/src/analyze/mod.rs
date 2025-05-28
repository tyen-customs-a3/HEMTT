use std::sync::{Arc, Mutex};

use hemtt_common::config::ProjectConfig;
use hemtt_workspace::{
    lint::LintManager,
    lint_manager,
    position::Position,
    reporting::{Codes, Processed},
};

mod cfgpatch;
mod chumsky;

pub mod lints {
    automod::dir!(pub "src/analyze/lints");
}

pub struct LintData {
    pub(crate) path: String,
    pub(crate) localizations: Arc<Mutex<Vec<(String, Position)>>>,
}

lint_manager!(config, vec![]);

impl LintData {
    pub fn new(path: String) -> Self {
        Self {
            path,
            localizations: Arc::new(Mutex::new(Vec::with_capacity(16))),
        }
    }
    
    /// Optimized path joining method
    pub fn with_child_path(&self, name: &str) -> Self {
        let mut path = String::with_capacity(self.path.len() + name.len() + 1);
        path.push_str(&self.path);
        path.push('/');
        path.push_str(name);
        
        Self {
            path,
            localizations: self.localizations.clone(),
        }
    }
}

pub use cfgpatch::CfgPatch;
pub use chumsky::ChumskyCode;

use crate::{Array, Class, Config, Expression, Item, Number, Property, Str, Value, MacroExpression};

/// Trait for analyzing objects with optimized implementations
pub trait Analyze: Sized + 'static {
    fn analyze(
        &self,
        data: &LintData,
        project: Option<&ProjectConfig>,
        processed: &Processed,
        manager: &LintManager<LintData>,
    ) -> Codes {
        // Pre-allocate with reasonable capacity based on typical usage
        let mut codes = Vec::with_capacity(4);
        codes.extend(manager.run(data, project, Some(processed), self));
        codes
    }
}

impl Analyze for Str {}
impl Analyze for Number {}
impl Analyze for Expression {}

impl Analyze for Config {
    fn analyze(
        &self,
        data: &LintData,
        project: Option<&ProjectConfig>,
        processed: &Processed,
        manager: &LintManager<LintData>,
    ) -> Codes {
        // Estimate capacity based on config size and typical lint rules
        let estimated_capacity = self.0.len().max(8);
        let mut codes = Vec::with_capacity(estimated_capacity);
        
        codes.extend(manager.run(data, project, Some(processed), self));
        codes.extend(manager.run(data, project, Some(processed), &self.to_class()));
        codes.extend(
            self.0
                .iter()
                .flat_map(|p| p.analyze(data, project, processed, manager)),
        );
        codes
    }
}

impl Analyze for Class {
    fn analyze(
        &self,
        data: &LintData,
        project: Option<&ProjectConfig>,
        processed: &Processed,
        manager: &LintManager<LintData>,
    ) -> Codes {
        let mut codes = Vec::with_capacity(8);
        codes.extend(manager.run(data, project, Some(processed), self));
        codes.extend(match self {
            Self::External { .. } => Vec::new(),
            Self::Local { properties, .. } | Self::Root { properties, .. } => {
                // Use optimized path construction
                let data = if let Some(name) = self.name() {
                    data.with_child_path(&name.value)
                } else {
                    LintData {
                        path: data.path.clone(),
                        localizations: data.localizations.clone(),
                    }
                };
                
                // Pre-allocate with estimated capacity
                let mut property_codes = Vec::with_capacity(properties.len() * 2);
                for property in properties {
                    property_codes.extend(property.analyze(&data, project, processed, manager));
                }
                property_codes
            }
        });
        codes
    }
}

impl Analyze for Property {
    fn analyze(
        &self,
        data: &LintData,
        project: Option<&ProjectConfig>,
        processed: &Processed,
        manager: &LintManager<LintData>,
    ) -> Codes {
        let mut codes = Vec::with_capacity(4);
        codes.extend(manager.run(data, project, Some(processed), self));
        codes.extend(match self {
            Self::Class(class) => class.analyze(data, project, processed, manager),
            Self::Entry { .. } => Vec::new(),
            Self::Delete(_) | Self::MissingSemicolon(_, _) => Vec::new(),
            Self::Enum(_) => Vec::new(),
        });
        codes
    }
}

impl Analyze for Value {
    fn analyze(
        &self,
        data: &LintData,
        project: Option<&ProjectConfig>,
        processed: &Processed,
        manager: &LintManager<LintData>,
    ) -> Codes {
        let mut codes = Vec::with_capacity(4);
        codes.extend(manager.run(data, project, Some(processed), self));
        codes.extend(match self {
            Self::Str(s) => s.analyze(data, project, processed, manager),
            Self::Number(n) => n.analyze(data, project, processed, manager),
            Self::Expression(e) => e.analyze(data, project, processed, manager),
            Self::Array(a) | Self::UnexpectedArray(a) => {
                a.analyze(data, project, processed, manager)
            }
            Self::Macro(m) => m.analyze(data, project, processed, manager),
            Self::Invalid(_) => Vec::new(),
        });
        codes
    }
}

impl Analyze for Array {
    fn analyze(
        &self,
        data: &LintData,
        project: Option<&ProjectConfig>,
        processed: &Processed,
        manager: &LintManager<LintData>,
    ) -> Codes {
        let mut codes = Vec::with_capacity(4 + self.items.len());
        codes.extend(manager.run(data, project, Some(processed), self));
        codes.extend(
            self.items
                .iter()
                .flat_map(|i| i.analyze(data, project, processed, manager)),
        );
        codes
    }
}

impl Analyze for Item {
    fn analyze(
        &self,
        data: &LintData,
        project: Option<&ProjectConfig>,
        processed: &Processed,
        manager: &LintManager<LintData>,
    ) -> Codes {
        let mut codes = Vec::with_capacity(4);
        codes.extend(manager.run(data, project, Some(processed), self));
        codes.extend(match self {
            Self::Str(s) => s.analyze(data, project, processed, manager),
            Self::Number(n) => n.analyze(data, project, processed, manager),
            Self::Macro(m) => m.analyze(data, project, processed, manager),
            Self::Array(a) => {
                let mut array_codes = Vec::with_capacity(a.len() * 2);
                for item in a {
                    array_codes.extend(item.analyze(data, project, processed, manager));
                }
                array_codes
            },
            Self::Invalid(_) => Vec::new(),
        });
        codes
    }
}

impl Analyze for MacroExpression {
    fn analyze(
        &self,
        data: &LintData,
        project: Option<&ProjectConfig>,
        processed: &Processed,
        manager: &LintManager<LintData>,
    ) -> Codes {
        let mut codes = Vec::with_capacity(4 + self.args.len());
        codes.extend(manager.run(data, project, Some(processed), self));
        // Analyze macro arguments with pre-allocated capacity
        for arg in &self.args {
            codes.extend(arg.analyze(data, project, processed, manager));
        }
        codes
    }
}
