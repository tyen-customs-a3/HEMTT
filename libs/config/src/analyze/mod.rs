use std::sync::{Arc, Mutex};
use std::collections::HashMap;

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

pub use cfgpatch::CfgPatch;
pub use chumsky::ChumskyCode;

use crate::{Array, Class, Config, Expression, Item, Number, Property, Str, Value};

/// Trait for rapifying objects
pub trait Analyze: Sized + 'static {
    fn analyze(
        &self,
        data: &LintData,
        project: Option<&ProjectConfig>,
        processed: &Processed,
        manager: &LintManager<LintData>,
    ) -> Codes {
        let mut codes = vec![];
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
        let mut codes = vec![];
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
        let mut codes = vec![];
        codes.extend(manager.run(data, project, Some(processed), self));
        codes.extend(match self {
            Self::External { .. } => vec![],
            Self::Local { properties, .. } | Self::Root { properties, .. } => {
                let data = LintData {
                    path: self.name().map_or_else(
                        || data.path.clone(),
                        |name| format!("{}/{}", data.path, name.value),
                    ),
                    localizations: data.localizations.clone(),
                };
                properties
                    .iter()
                    .flat_map(|p| p.analyze(&data, project, processed, manager))
                    .collect::<Vec<_>>()
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
        let mut codes = vec![];
        codes.extend(manager.run(data, project, Some(processed), self));
        codes.extend(match self {
            Self::Entry { value, .. } => {
                let data = LintData {
                    path: format!("{}.{}", data.path, self.name().value),
                    localizations: data.localizations.clone(),
                };
                value.analyze(&data, project, processed, manager)
            }
            Self::Class(c) => c.analyze(data, project, processed, manager),
            Self::Delete(_) | Self::MissingSemicolon(_, _) => vec![],
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
        let mut codes = vec![];
        codes.extend(manager.run(data, project, Some(processed), self));
        codes.extend(match self {
            Self::Str(s) => s.analyze(data, project, processed, manager),
            Self::Number(n) => n.analyze(data, project, processed, manager),
            Self::Expression(e) => e.analyze(data, project, processed, manager),
            Self::Array(a) => a.analyze(data, project, processed, manager),
            Self::UnexpectedArray(a) => a.analyze(data, project, processed, manager),
            Self::Invalid(_) => vec![],
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
        let mut codes = vec![];
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
        let mut codes = vec![];
        codes.extend(manager.run(data, project, Some(processed), self));
        codes.extend(match self {
            Self::Str(s) => s.analyze(data, project, processed, manager),
            Self::Number(n) => n.analyze(data, project, processed, manager),
            Self::Array(a) => a
                .iter()
                .flat_map(|i| i.analyze(data, project, processed, manager))
                .collect::<Vec<_>>(),
            Self::Invalid(_) => vec![],
            Self::Macro { name, args, .. } => {
                let mut codes = vec![];
                codes.extend(name.analyze(data, project, processed, manager));
                for arg in args {
                    codes.extend(arg.analyze(data, project, processed, manager));
                }
                codes
            }
        });
        codes
    }
}

pub fn analyze_array(array: &Array) -> Vec<String> {
    let mut result = Vec::new();
    for item in array.items() {
        match item {
            Item::Str(s) => result.push(s.value().to_string()),
            Item::Number(n) => result.push(n.to_string()),
            Item::Array(items) => {
                for item in items {
                    match item {
                        Item::Str(s) => result.push(s.value().to_string()),
                        Item::Number(n) => result.push(n.to_string()),
                        Item::Array(_) => {}
                        Item::Invalid(_) => {}
                        Item::Macro { args, .. } => {
                            // For macros, add the first argument's value (if available)
                            if let Some(first_arg) = args.first() {
                                result.push(first_arg.value.clone());
                            }
                        }
                    }
                }
            }
            Item::Invalid(_) => {}
            Item::Macro { args, .. } => {
                // For macros, add the first argument's value (if available)
                if let Some(first_arg) = args.first() {
                    result.push(first_arg.value.clone());
                }
            }
        }
    }
    result
}

pub fn analyze_class(class: &Class) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::new();
    for property in class.properties() {
        if let Property::Entry {
            name,
            value: Value::Array(array),
            ..
        } = property
        {
            result.insert(name.value.clone(), analyze_array(array));
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Ident, Str};

    #[test]
    fn analyze_array_test() {
        let array = Array {
            expand: false,
            items: vec![
                Item::Str(Str {
                    value: "first".to_string(),
                    span: 0..5,
                }),
                Item::Number(Number::Int32 {
                    value: 123,
                    span: 6..9,
                }),
                Item::Array(vec![
                    Item::Str(Str {
                        value: "nested".to_string(),
                        span: 11..17,
                    }),
                ]),
                Item::Macro {
                    name: Str {
                        value: "LIST_2".to_string(),
                        span: 19..25,
                    },
                    args: vec![Str {
                        value: "macro".to_string(), 
                        span: 26..31,
                    }],
                    span: 19..31,
                },
            ],
            span: 0..33,
        };

        assert_eq!(
            analyze_array(&array),
            vec![
                "first".to_string(),
                "123".to_string(),
                "nested".to_string(),
                "macro".to_string(),
            ]
        );
    }

    #[test]
    fn analyze_class_test() {
        let class = Class::Local {
            name: Ident {
                value: "TestClass".to_string(),
                span: 0..9,
            },
            parent: None,
            properties: vec![Property::Entry {
                name: Ident {
                    value: "items".to_string(),
                    span: 11..16,
                },
                value: Value::Array(Array {
                    expand: false,
                    items: vec![
                        Item::Str(Str {
                            value: "item1".to_string(),
                            span: 19..24,
                        }),
                        Item::Str(Str {
                            value: "item2".to_string(),
                            span: 26..31,
                        }),
                    ],
                    span: 18..32,
                }),
                expected_array: true,
            }],
            err_missing_braces: false,
        };

        let mut expected = HashMap::new();
        expected.insert(
            "items".to_string(),
            vec!["item1".to_string(), "item2".to_string()],
        );

        assert_eq!(analyze_class(&class), expected);
    }
}
