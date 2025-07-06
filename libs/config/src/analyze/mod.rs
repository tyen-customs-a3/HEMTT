use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use hemtt_common::config::{ProjectConfig, RuntimeArguments};
use hemtt_workspace::{
    addons::{Addon, DefinedFunctions, MagazineWellInfo},
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
    pub(crate) functions_defined: Arc<Mutex<DefinedFunctions>>,
    pub(crate) magazine_well_info: Arc<Mutex<MagazineWellInfo>>,
}

lint_manager!(config, vec![]);

impl LintData {
    pub fn new(path: String) -> Self {
        Self {
            path,
            localizations: Arc::new(Mutex::new(Vec::new())),
            functions_defined: Arc::new(Mutex::new(HashSet::new())),
            magazine_well_info: Arc::new(Mutex::new((Vec::new(), Vec::new()))),
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
            functions_defined: self.functions_defined.clone(),
            magazine_well_info: self.magazine_well_info.clone(),
        }
    }
}

pub use cfgpatch::CfgPatch;
pub use chumsky::ChumskyCode;

use crate::{Array, Class, Config, Expression, Item, Number, Property, Str, Value, MacroExpression, EnumDef, EnumEntry};

/// Trait for analyzing objects
pub trait Analyze: Sized + 'static {
    fn analyze(
        &self,
        data: &LintData,
        project: Option<&ProjectConfig>,
        processed: &Processed,
        manager: &LintManager<LintData>,
    ) -> Codes {
        let mut codes = Vec::new();
        codes.extend(manager.run(data, project, Some(processed), self));
        codes
    }
}

impl Analyze for Str {}
impl Analyze for Number {}
impl Analyze for Expression {}
impl Analyze for MacroExpression {}
impl Analyze for EnumDef {
    fn analyze(
        &self,
        data: &LintData,
        project: Option<&ProjectConfig>,
        processed: &Processed,
        manager: &LintManager<LintData>,
    ) -> Codes {
        let mut codes = Vec::with_capacity(4 + self.entries().len());
        codes.extend(manager.run(data, project, Some(processed), self));
        codes.extend(
            self.entries()
                .iter()
                .flat_map(|entry| entry.analyze(data, project, processed, manager)),
        );
        codes
    }
}

impl Analyze for EnumEntry {
    fn analyze(
        &self,
        data: &LintData,
        project: Option<&ProjectConfig>,
        processed: &Processed,
        manager: &LintManager<LintData>,
    ) -> Codes {
        let mut codes = Vec::with_capacity(4);
        codes.extend(manager.run(data, project, Some(processed), self));
        
        // Create a new data context for the enum entry
        let entry_data = LintData {
            path: format!("{}.{}", data.path, self.name().value),
            localizations: data.localizations.clone(),
            functions_defined: data.functions_defined.clone(),
            magazine_well_info: data.magazine_well_info.clone(),
        };
        
        codes.extend(self.value().analyze(&entry_data, project, processed, manager));
        codes
    }
}

impl Analyze for Config {
    fn analyze(
        &self,
        data: &LintData,
        project: Option<&ProjectConfig>,
        processed: &Processed,
        manager: &LintManager<LintData>,
    ) -> Codes {
        let mut codes = Vec::new();
        
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
        let mut codes = Vec::new();
        codes.extend(manager.run(data, project, Some(processed), self));
        codes.extend(match self {
            Self::External { .. } => Vec::new(),
            Self::Local { properties, .. } | Self::Root { properties, .. } => {
                let data = if let Some(name) = self.name() {
                    data.with_child_path(&name.value)
                } else {
                    LintData {
                        path: data.path.clone(),
                        localizations: data.localizations.clone(),
                        functions_defined: data.functions_defined.clone(),
                        magazine_well_info: data.magazine_well_info.clone(),
                    }
                };
                
                let mut property_codes = Vec::new();
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
        let mut codes = Vec::new();
        codes.extend(manager.run(data, project, Some(processed), self));
        codes.extend(match self {
            Self::Entry { value, .. } => {
                let data = LintData {
                    path: format!("{}.{}", data.path, self.name().value),
                    localizations: data.localizations.clone(),
                    functions_defined: data.functions_defined.clone(),
                    magazine_well_info: data.magazine_well_info.clone(),
                };
                value.analyze(&data, project, processed, manager)
            }
            Self::Class(c) => c.analyze(data, project, processed, manager),
            Self::Delete(_) | Self::MissingSemicolon(_, _) => vec![],
            Self::Enum(e) => e.analyze(data, project, processed, manager),
            Self::Macro { .. } => vec![], // Macros don't need analysis at this level
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
        let mut codes = Vec::new();
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
        let mut codes = Vec::new();
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
        let mut codes = Vec::new();
        codes.extend(manager.run(data, project, Some(processed), self));
        codes.extend(match self {
            Self::Str(s) => s.analyze(data, project, processed, manager),
            Self::Number(n) => n.analyze(data, project, processed, manager),
            Self::Macro(m) => m.analyze(data, project, processed, manager),
            Self::Array(a) => {
                let mut array_codes = Vec::new();
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

#[must_use]
#[allow(clippy::ptr_arg)]
pub fn lint_all(project: Option<&ProjectConfig>, addons: &Vec<Addon>) -> Codes {
    let mut manager = LintManager::new(
        project.map_or_else(Default::default, |project| project.lints().config().clone()),
        project.map_or_else(RuntimeArguments::default, |p| p.runtime().clone()),
    );
    let _e = manager.extend(
        crate::analyze::CONFIG_LINTS
            .iter()
            .map(|l| (**l).clone())
            .collect::<Vec<_>>(),
    );

    manager.run(
        &LintData {
            path: String::new(),
            localizations: Arc::new(Mutex::new(vec![])),
            functions_defined: Arc::new(Mutex::new(HashSet::new())),
            magazine_well_info: Arc::new(Mutex::new((Vec::new(), Vec::new()))),
        },
        project,
        None,
        addons,
    )
}
